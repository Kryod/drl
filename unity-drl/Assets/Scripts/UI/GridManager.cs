using UnityEngine;
using UnityEngine.UI;

namespace UI
{
    public class GridManager : Singleton<GridManager>
    {
        [SerializeField] private GridLayoutGroup gridLayout;
        [SerializeField] private GridCell cellPrefab;
        private GridCell[][] cells;
        public int cols { get; private set; }
        public int rows { get; private set; }

        public void CreateGrid(int cols, int rows)
        {
            this.SetNumCols(cols);
            this.SetNumRows(rows);
            this.Fill();
        }

        public void CreateLine(int cells)
        {
            this.SetNumCols(cells);
            this.SetNumRows(1);
            this.gridLayout.cellSize = new Vector2(this.gridLayout.cellSize.x, this.gridLayout.cellSize.x); // Make a square
            this.Fill();
        }

        public GridCell Get(int x, int y)
        {
            return this.cells[y][x];
        }

        public GridCell Get(int idx)
        {
            return this.cells[idx / this.cols][idx % this.cols];
        }

        private void Fill()
        {
            this.cells = new GridCell[this.rows][];
            for (int y = 0; y < this.rows; ++y)
            {
                this.cells[y] = new GridCell[this.cols];
                for (int x = 0; x < this.cols; ++x)
                {
                    GridCell cell = GameObject.Instantiate(this.cellPrefab);
                    cell.gameObject.transform.SetParent(this.gridLayout.transform, false);
                    this.cells[y][x] = cell;
                }
            }
        }

        public void Reset()
        {
            foreach (Transform child in this.gridLayout.transform)
            {
                GameObject.Destroy(child.gameObject);
            }
        }

        private void SetNumCols(int cols)
        {
            this.cols = cols;
            float sizeAvailable = this.gridLayout.GetComponent<RectTransform>().rect.width - (this.gridLayout.spacing.x * cols);
            float x = sizeAvailable / (float)cols;
            this.gridLayout.cellSize = new Vector2(x, this.gridLayout.cellSize.y);
        }

        private void SetNumRows(int rows)
        {
            this.rows = rows;
            float sizeAvailable = this.gridLayout.GetComponent<RectTransform>().rect.height - (this.gridLayout.spacing.y * rows);
            float y = sizeAvailable / (float)rows;
            this.gridLayout.cellSize = new Vector2(this.gridLayout.cellSize.x, y);
        }
    }
}
