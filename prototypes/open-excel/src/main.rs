use calamine::{Reader, Xlsx, open_workbook};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = format!("{}\\tests\\test.xlsx", env!("CARGO_MANIFEST_DIR"));
  let mut excel: Xlsx<_> = open_workbook(path)?;
  if let Ok(r) = excel.worksheet_range("Подія") {
    for row in r.rows() {
      println!("row={:?}, row[0]={:?}", row, row[0]);
    }
  }

  Ok(())
}
