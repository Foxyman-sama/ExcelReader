#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use calamine::{Reader, Xlsx, open_workbook};
use eframe::egui;
use egui_extras::{Column, TableBuilder};

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
    ..Default::default()
  };

  eframe::run_native(
    "ExcelReader",
    options,
    Box::new(|_cc| Ok(Box::new(ExcelReader::default()))),
  )
}

struct ExcelReader {
  data: Vec<Vec<String>>,
}

impl Default for ExcelReader {
  fn default() -> Self {
    let mut data = Vec::new();
    let path = format!("{}\\tests\\test.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range_at(0) {
      for row in r.rows() {
        let row_strings: Vec<String> =
          row.iter().map(|cell| cell.to_string()).collect();

        data.push(row_strings);
      }
    }

    Self { data }
  }
}

impl eframe::App for ExcelReader {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("ExcelReader");

      TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .columns(Column::initial(100.0).resizable(true), self.data.len())
        .header(20.0, |mut header| {
          let headers = &self.data[0];
          for col in headers {
            header.col(|ui| {
              ui.strong(col);
            });
          }
        })
        .body(|body| {
          let row_height = 10.0;
          let num_rows = self.data.len() - 1;
          body.rows(row_height, num_rows, |mut row| {
            let row_index = row.index();
            let row_data = &self.data[row_index + 1];
            for cell_text in row_data {
              row.col(|ui| {
                ui.label(cell_text);
              });
            }
          });
        });
    });
  }
}
