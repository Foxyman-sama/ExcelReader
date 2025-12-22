use calamine::{Reader, Xlsx, open_workbook};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = format!("{}\\tests\\test.xlsx", env!("CARGO_MANIFEST_DIR"));
  let mut excel: Xlsx<_> = open_workbook(path)?;
  if let Some(Ok(r)) = excel.worksheet_range_at(0) {
    for row in r.rows() {
      println!("{:?}", row);
    }
  }

  Ok(())
}
