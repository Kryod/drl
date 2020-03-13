#[macro_use]
extern crate quote;
extern crate proc_macro;

use std::iter::FromIterator;

use proc_macro::TokenStream;
use proc_macro::TokenTree::{ self, Group, Punct, Ident, Literal };

#[derive(PartialEq, Clone)]
enum Arg {
    Handle,
    Other(String, String),
}

impl Arg {
    pub fn to_csharp(&self) -> String {
        match self {
            Arg::Handle => "IntPtr handle".into(),
            Arg::Other(name, t) => {
                format!("{} {}", rust_type_to_csharp(t), mixed_case(name))
            }
        }
    }

    pub fn args_to_csharp(args: std::slice::Iter<Self>) -> String {
        args.map(|a| a.to_csharp()).collect::<Vec<String>>().join(", ")
    }
}

#[derive(Default, Clone)]
struct Method {
    pub name: String,
    pub args: Vec<Arg>,
    pub ret_type: Option<String>,
}

impl Method {
    pub fn name_as_camel_case(&self) -> String {
        camel_case(&self.name)
    }

    pub fn ret_type_as_csharp(&self) -> String {
        rust_type_to_csharp(&self.ret_type.clone().unwrap_or("()".to_string())).into()
    }

    pub fn args_as_csharp(&self) -> String {
        Arg::args_to_csharp(self.args.iter())
    }
}

fn camel_case(txt: &str) -> String {
    use heck::CamelCase;
    txt.to_camel_case()
}

fn mixed_case(txt: &str) -> String {
    use heck::MixedCase;
    txt.to_mixed_case()
}

#[derive(Default)]
struct AttrArgs {
    pub lib_name: String,
    pub target_dir: String,
}

fn extract_attr_args(args: TokenStream) -> AttrArgs {
    let mut it = args.into_iter();
    let mut args = AttrArgs::default();

    args.lib_name = match it.next().expect("Please provide the library name as argument.") {
        Literal(l) => {
            match syn::parse_str::<syn::Lit>(l.to_string().as_ref()).expect("Could not parse library name.") {
                syn::Lit::Str(s) => s.value(),
                _ => panic!("The library name must be a string literal."),
            }
        },
        _ => panic!("The library name must be a string literal."),
    };
    it.next(); // "," between the two args
    args.target_dir = match it.next().expect("Please provide the target directory as argument.") {
        Literal(l) => {
            match syn::parse_str::<syn::Lit>(l.to_string().as_ref()).expect("Could not parse target directory.") {
                syn::Lit::Str(s) => s.value(),
                _ => panic!("The target directory must be a string literal."),
            }
        },
        _ => panic!("The target directory must be a string literal."),
    };

    args
}

fn split_arguments(args: &str) -> Vec<String> {
    let mut split = Vec::new();
    let mut arg = String::new();
    let mut brackets: Vec<char> = Vec::new();
    let available_brackets = [ '[', '(', '<', '{' ];
    let matching_brackets = [ ']', ')', '>', '}' ];

    for c in args.chars() {
        if c == ',' {
            if brackets.is_empty() {
                split.push(arg.trim().to_owned());
                arg = String::new();
            } else {
                arg.push(',');
            }
        } else {
            arg.push(c);

            if available_brackets.contains(&c) {
                brackets.push(c);
            } else if !brackets.is_empty() {
                let last = brackets.last().unwrap();
                match available_brackets.iter().position(|bracket| bracket == last) {
                    Some(last_bracket_idx) => {
                        if matching_brackets[last_bracket_idx] == c {
                            brackets.pop();
                        }
                    },
                    None => { },
                };
            }
        }
    }
    if !arg.trim().is_empty() {
        split.push(arg.trim().to_owned());
    }

    split
}

#[derive(PartialEq)]
enum ReturnTypeSearchMode {
    Dash,
    Arrow,
    Collecting,
}

enum Mode {
    Impl,
    Enum,
    Struct,
}

#[proc_macro_attribute]
pub fn ffi(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut it = input.clone().into_iter();
    let mut mode: Option<Mode> = None;
    while let Some(t) = it.next() {
        match t {
            Ident(i) => {
                let i = i.to_string();
                if &i == "impl" {
                    mode = Some(Mode::Impl);
                    break;
                }
                if &i == "struct" {
                    mode = Some(Mode::Struct);
                    break;
                }
                if &i == "enum" {
                    mode = Some(Mode::Enum);
                    break;
                }
            },
            _ => {},
        };
    };
    let mode = match mode {
        Some(mode) => mode,
        None => panic!("Please use this macro on a struct, an enum, or on an `impl` block."),
    };
    match mode {
        Mode::Impl => process_impl(it, args, input),
        Mode::Enum => process_enum(it, args, input),
        Mode::Struct => process_struct(it, args, input),
    }
}

fn process_impl(mut it: proc_macro::token_stream::IntoIter, args: TokenStream, input: TokenStream) -> TokenStream {
    let class_name = it.next().expect("No struct name after `impl`").to_string();
    let args = extract_attr_args(args);
    let mut methods: Vec<Method> = Vec::new();
    match it.next().expect("Empty struct or `impl` code block.") {
        Group(g) => {
            let mut it = g.stream().into_iter();
            let mut method = Method::default();
            let mut return_type_search = ReturnTypeSearchMode::Dash;
            let mut return_type = String::new();
            let mut public = false;

            while let Some(t) = it.next() {
                match t {
                    Ident(i) => {
                        let i = i.to_string();
                        if i == "pub" {
                            public = true;
                        }
                        if i == "fn" {
                            let next = it.next().expect("Could not find function.");
                            method.name = match next {
                                Ident(i) => i.to_string(),
                                _ => panic!("Invalid `fn`."),
                            };
                        }
                        if return_type_search == ReturnTypeSearchMode::Collecting {
                            return_type.push_str(i.as_ref());
                        }
                    },
                    Punct(p) => {
                        let p = p.as_char();
                        if return_type_search == ReturnTypeSearchMode::Dash && p == '-' {
                            return_type_search = ReturnTypeSearchMode::Arrow;
                        } else if return_type_search == ReturnTypeSearchMode::Arrow && p == '>' {
                            return_type_search = ReturnTypeSearchMode::Collecting;
                        } else if return_type_search == ReturnTypeSearchMode::Collecting {
                            return_type.push(p);
                        } else {
                            return_type_search = ReturnTypeSearchMode::Dash;
                        }
                    },
                    Group(g) => {
                        match g.delimiter() {
                            proc_macro::Delimiter::Parenthesis => {
                                if public && return_type_search == ReturnTypeSearchMode::Collecting {
                                    if g.stream().is_empty() {
                                        return_type = "".into();
                                    } else {
                                        return_type = g.to_string();
                                    }
                                } else {
                                    let args = g.stream().to_string();
                                    let args = split_arguments(&args);
                                    for arg in args {
                                        if arg.contains("self") && !arg.contains(":") {
                                            method.args.push(Arg::Handle);
                                        } else {
                                            let parts: Vec<&str> = arg.split(':').map(|s| s.trim()).collect();
                                            if parts.len() >= 2 {
                                                method.args.push(Arg::Other(
                                                    parts[0].into(),
                                                    parts[1..].join(":"),
                                                ));
                                            }
                                        }
                                    }
                                }
                            },
                            proc_macro::Delimiter::Brace => {
                                if !return_type.is_empty() {
                                    method.ret_type = Some(return_type.clone());
                                }
                                if public {
                                    methods.push(method.clone());
                                }
                                method = Method::default();
                                return_type_search = ReturnTypeSearchMode::Dash;
                                return_type = "".into();
                                public = false;
                            },
                            proc_macro::Delimiter::Bracket => {
                                if public && return_type_search == ReturnTypeSearchMode::Collecting {
                                    return_type = g.to_string();
                                }
                            },
                            _ => {},
                        };
                    },
                    _ => {},
                };
            };
        },
        _ => {},
    };

    write_csharp_file_for_impl(&args.lib_name, &class_name, methods, &args.target_dir);

    let mut ret: Vec<TokenTree> = input.into_iter().collect();
    let class_name = syn::Ident::new(&class_name, proc_macro2::Span::call_site());
    let init_dispose_impl: TokenStream = quote! {
        impl #class_name {
            #[no_mangle]
            pub extern fn initialize() -> *mut Self {
                let boxed = Box::new(Self::new());
                Box::into_raw(boxed)
            }

            #[no_mangle]
            pub unsafe extern fn dispose(ptr: *mut Self) {
                if !ptr.is_null() {
                    let _boxed_self: Box<Self> = Box::from_raw(ptr);
                }
            }
        }
    }.into();
    for t in init_dispose_impl {
        ret.push(t.into());
    }
    TokenStream::from_iter(ret)
}

fn process_struct(mut it: proc_macro::token_stream::IntoIter, args: TokenStream, input: TokenStream) -> TokenStream {
    let args = extract_attr_args(args);
    let struct_name = it.next().expect("No struct name after `struct`").to_string();
    let mut struct_fields: Vec<Arg> = Vec::new();

    while let Some(t) = it.next() {
        match t {
            Group(g) => {
                let mut it = g.stream().into_iter();
                let mut as_str = String::new();
                while let Some(t) = it.next() {
                    as_str.push_str(&format!("{} ", t));
                };
                let fields: Vec<String> = as_str.split(',').filter_map(|arg| {
                    let arg = arg.replace("pub ", "");
                    let arg = arg.trim().to_owned();
                    if !arg.is_empty() { Some(arg) } else { None }
                }).collect();
                for field in fields {
                    let parts: Vec<&str> = field.split(':').map(|s| s.trim()).collect();
                    if parts.len() >= 2 {
                        struct_fields.push(Arg::Other(
                            parts[0].into(),
                            parts[1..].join(":"),
                        ));
                    }
                }
            },
            _ => {},
        };
    };

    write_csharp_file_for_struct(&struct_name, struct_fields, &args.target_dir);

    let mut ret: Vec<TokenTree> = Vec::new();
    let c_repr: TokenStream = quote! {
        #[repr(C)]
    }.into();
    for t in c_repr {
        ret.push(t);
    }
    for t in input.into_iter() {
        ret.push(t);
    }
    TokenStream::from_iter(ret)
}

fn process_enum(mut it: proc_macro::token_stream::IntoIter, args: TokenStream, input: TokenStream) -> TokenStream {
    let enum_name = it.next().expect("No enum name after `enum`").to_string();
    let args = extract_attr_args(args);
    let mut fields = Vec::new();

    match it.next().expect("Empty `enum` code block.") {
        Group(g) => {
            let mut it = g.stream().into_iter();
            let mut field_name = "".to_string();
            let mut value = None;
            while let Some(t) = it.next() {
                match t {
                    Group(_g) => panic!("Data in enum variants are not supported"),
                    Ident(i) => {
                        if field_name != "" {
                            fields.push((field_name, value));
                            value = None;
                        }
                        field_name = i.to_string();
                    },
                    Literal(l) => value = Some(l.to_string()),
                    _ => { },
                };
            };
            if field_name != "" {
                fields.push((field_name, value));
            }
        },
        _ => {},
    };

    write_csharp_file_for_enum(&enum_name, fields, &args.target_dir);
    TokenStream::from_iter(input)
}

fn is_pod_type(t: &str) -> bool {
    ["bool", "i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64", "usize", "f32", "f64"].contains(&t)
}

fn rust_type_to_csharp(t: &str) -> String {
    use regex::Regex;

    if t == "()" { return "void".into(); }
    if t == "bool" { return "bool".into(); }
    if t == "i8" { return "char".into(); }
    if t == "i16" { return "short".into(); }
    if t == "i32" { return "int".into(); }
    if t == "i64" { return "long".into(); }
    if t == "isize" { return "IntPtr".into(); }
    if t == "u8" { return "byte".into(); }
    if t == "u16" { return "ushort".into(); }
    if t == "u32" { return "uint".into(); }
    if t == "u64" { return "ulong".into(); }
    if t == "usize" { return "UIntPtr".into(); }
    if t == "f32" { return "float".into(); }
    if t == "f64" { return "double".into(); }
    if t == "Self" { return "IntPtr".into(); }
    if t.ends_with("str") { return "string".into(); }
    if t == "String" { return "string".into(); }
    if t.contains("* const") && t.contains("c_char") { return "string".into(); }

    let re = Regex::new(r"^\*\s?(mut|const)\s*([a-zA-Z0-9_]*)$").unwrap();
    if let Some(captures) = re.captures(t) {
        if captures.len() == 3 {
            let t = captures.get(2).unwrap().as_str();
            let csharp_type = rust_type_to_csharp(t);
            if is_pod_type(t) {
                //return format!("{}[]", csharp_type);
                return format!("{}*", csharp_type);
            } else {
                return "IntPtr".into();
            }
        }
    }

    let re = Regex::new(r"^Option<(.*)>$").unwrap();
    if let Some(captures) = re.captures(t) {
        if captures.len() == 2 { return rust_type_to_csharp(captures.get(1).unwrap().as_str()); }
    }

    let re = Regex::new(r"^Vec<(.*)>$").unwrap();
    if let Some(captures) = re.captures(t) {
        if captures.len() == 2 {
            let vec_type = rust_type_to_csharp(captures.get(1).unwrap().as_str());
            return format!("{}[]", vec_type);
        }
    }

    let re = Regex::new(r"fn \((.*)\)").unwrap();
    if let Some(captures) = re.captures(t) {
        if captures.len() == 2 {
            let args = split_arguments(captures.get(1).unwrap().as_str()).iter()
                            .map(|arg| rust_type_to_csharp(arg))
                            .collect::<Vec<String>>();
            if args.is_empty() {
                return "Action".into();
            } else {
                return format!("Action<{}>", args.join(", "));
            }
        }
    }

    if t.starts_with('[') && t.ends_with(']') {
        let t = t.replace('[', "").replace(']', "");
        let mut split = t.split(';');
        match split.next() {
            Some(arr_type) => return format!("{}[]", rust_type_to_csharp(arr_type.trim())),
            None => {},
        };
    }
    if t.starts_with('(') && t.ends_with(')') {
        let t = t.replace('(', "").replace(')', "");
        let types = t.split(',').map(|t| {
            rust_type_to_csharp(t.trim())
        }).collect::<Vec<String>>().join(", ");
        return format!("Tuple<{}>", types);
    }

    format!("NativePlugins.Types.{}", t)
}

fn append_dll_import_fn(code: &mut String, class: &str, m: &Method) {
    code.push_str(&format!(r#"
        [DllImport({}Base.DynamicLibraryName, EntryPoint = "{}", CallingConvention = CallingConvention.Cdecl)]
        private static extern {} _{}({});
"#,
        class,
        m.name,
        m.ret_type_as_csharp(),
        m.name_as_camel_case(),
        m.args_as_csharp(),
    ));
}

fn append_dll_delegate(code: &mut String, m: &Method) {
    code.push_str(&format!(r#"
        private delegate {} Dll{}({});"#,
        m.ret_type_as_csharp(),
        m.name_as_camel_case(),
        m.args_as_csharp(),
    ));
}

fn append_member_method(code: &mut String, m: &Method) {
    let camel = m.name_as_camel_case();
    let received_args = m.args.iter().filter_map(|a| {
        match a {
            Arg::Handle => None,
            Arg::Other(_, _) => Some(a.clone()),
        }
    }).collect::<Vec<Arg>>();
    let received_args = Arg::args_to_csharp(received_args.iter());
    let passed_args = m.args.iter().map(|a| {
        match a {
            Arg::Handle => "this.Handle".to_string(),
            Arg::Other(name, _t) => mixed_case(name),
        }
    }).collect::<Vec<String>>().join(", ");
    let return_str = match m.ret_type {
        Some(_) => "return ",
        None => "",
    };
    let internal_call = format!("{}_{}({});", return_str, camel, passed_args);
    let delegate_call = format!(r#"{}NativePluginLoader.GetDelegate<Dll{}>(this.DllHandle, "{}")({});"#, return_str, camel, m.name, passed_args);
    code.push_str(&format!(r#"
        public {} {}({})
        {{
            this.CheckHandle();
#if !UNITY_EDITOR
            {}
#else
            {}
#endif
        }}
"#,
        m.ret_type_as_csharp(), camel, received_args,
        internal_call,
        delegate_call,
    ));
}

fn write_csharp_file_for_impl(lib: &str, class: &str, methods: Vec<Method>, target_dir: &str) {
    let mut dir = std::env::current_dir().expect("Could not get current working directory.");
    dir.pop();
    dir.push(target_dir);
    dir.push("Assets");
    dir.push("Scripts");
    dir.push("NativePlugins");
    let _ = std::fs::create_dir_all(&dir);

    let init = Method {
        name: "initialize".into(),
        args: Vec::new(),
        ret_type: Some("*mut Self".into()),
    };
    let dispose = Method {
        name: "dispose".into(),
        args: vec![ Arg::Other("handle".into(), "*mut Self".into()) ],
        ret_type: None,
    };

    let mut contents = format!("/* THIS FILE IS GENERATED AUTOMATICALLY
 * DO NOT EDIT THIS FILE
 */

using System;
using Utils;

#if !UNITY_EDITOR
using System.Runtime.InteropServices;
#endif

namespace NativePlugins
{{
    public unsafe abstract class {}Base : NativePlugin
    {{
#if UNITY_STANDALONE_WIN
        protected const string DynamicLibraryName = \"{}.dll\";
#elif UNITY_STANDALONE_OSX
        protected const string DynamicLibraryName = \"{}.dylib\";
#else
        protected const string DynamicLibraryName = \"{}.so\";
#endif

#if !UNITY_EDITOR", class, lib, lib, lib);
    append_dll_import_fn(&mut contents, class, &init);
    append_dll_import_fn(&mut contents, class, &dispose);
    for m in methods.iter() {
        append_dll_import_fn(&mut contents, class, m);
    }
    contents.push_str("#else");
    append_dll_delegate(&mut contents, &init);
    append_dll_delegate(&mut contents, &dispose);
    for m in methods.iter() {
        append_dll_delegate(&mut contents, m);
    }
    contents.push_str(&format!(r#"
#endif

        public {}Base()
        {{
#if !UNITY_EDITOR
            this.Handle = _Initialize();
#else
            this.DllHandle = NativePluginLoader.LoadPlugin("{}");
            this.Handle = NativePluginLoader.GetDelegate<DllInitialize>(this.DllHandle, "initialize")();
#endif
        }}

        public void Dispose()
        {{
            this.CheckHandle();

#if !UNITY_EDITOR
            _Dispose(this.Handle);
#else
            NativePluginLoader.GetDelegate<DllDispose>(this.DllHandle, "dispose")(this.Handle);
            NativePluginLoader.FreePlugin(this.DllHandle);
            this.DllHandle = IntPtr.Zero;
#endif
            this.Handle = IntPtr.Zero;
        }}
"#, class, lib));
    for m in methods.iter() {
        append_member_method(&mut contents, m);
    }
    contents.push_str("
    }
}
");

    let mut file = dir.clone();
    file.push(format!("{}Base.cs", class));
    std::fs::write(&file, contents).expect("Could not write base class.");

    file.pop();
    file.push(format!("{}.cs", class));
    if !file.exists() {
        let contents = format!("namespace NativePlugins
{{
    public class {} : {}Base
    {{

    }}
}}
", class, class);
        std::fs::write(&file, contents).expect("Could not write child class.");
    }
}

fn write_csharp_file_for_struct(name: &str, fields: Vec<Arg>, target_dir: &str) {
    let mut dir = std::env::current_dir().expect("Could not get current working directory.");
    dir.pop();
    dir.push(target_dir);
    dir.push("Assets");
    dir.push("Scripts");
    dir.push("NativePlugins");
    dir.push("Types");
    let _ = std::fs::create_dir_all(&dir);

    let fields = fields.iter().filter_map(|f| {
        match f {
            Arg::Other(field_name, field_type) => {
                Some(format!("        public {} {};", rust_type_to_csharp(field_type), field_name))
            },
            _ => None,
        }
    }).collect::<Vec<String>>().join("\n");

    let contents = format!("/* THIS FILE IS GENERATED AUTOMATICALLY
 * DO NOT EDIT THIS FILE
 */

using System.Runtime.InteropServices;

namespace NativePlugins.Types
{{
    [StructLayout(LayoutKind.Sequential)]
    public struct {}
    {{
{}
    }}
}}
", name, fields);

    let mut file = dir;
    file.push(format!("{}.cs", name));
    std::fs::write(&file, contents).expect("Could not write struct.");
}

fn write_csharp_file_for_enum(name: &str, fields: Vec<(String, Option<String>)>, target_dir: &str) {
    let mut dir = std::env::current_dir().expect("Could not get current working directory.");
    dir.pop();
    dir.push(target_dir);
    dir.push("Assets");
    dir.push("Scripts");
    dir.push("NativePlugins");
    dir.push("Types");
    let _ = std::fs::create_dir_all(&dir);

    let fields = fields
                    .iter()
                    .map(|(field_name, val)| match val {
                        Some(val) => format!("        {} = {},", field_name, val),
                        None => format!("        {},", field_name),
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

    let contents = format!("/* THIS FILE IS GENERATED AUTOMATICALLY
 * DO NOT EDIT THIS FILE
 */

namespace NativePlugins.Types
{{
    public enum {}
    {{
{}
    }}
}}
", name, fields);

    let mut file = dir;
    file.push(format!("{}.cs", name));
    std::fs::write(&file, contents).expect("Could not write enum.");
}
