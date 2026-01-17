#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui_extras::{Column, TableBuilder};

// 1. Створюємо структуру для збереження стану вашого додатку
struct MyApp {
  data: Vec<Vec<String>>,
}

// 2. Реалізуємо значення за замовчуванням (початковий стан)
impl Default for MyApp {
  fn default() -> Self {
    let mut data = Vec::new();
    for row in 0..100 {
      let mut row_data = Vec::new();
      for col in 0..4 {
        row_data.push(format!("Row {} Col {}", row, col));
      }

      data.push(row_data);
    }

    Self { data }
  }
}

// 3. Реалізуємо основний трейт App, де описуємо UI
impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Table Prototype");

      TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::initial(100.0).resizable(true))
        .column(Column::initial(100.0).resizable(true))
        .column(Column::initial(100.0).resizable(true))
        .column(Column::remainder())
        .header(20.0, |mut header| {
          header.col(|ui| {
            ui.strong("ID");
          });
          header.col(|ui| {
            ui.strong("Name");
          });
          header.col(|ui| {
            ui.strong("Value");
          });
          header.col(|ui| {
            ui.strong("Comment");
          });
        })
        .body(|body| {
          let row_height = 10.0;
          let num_rows = self.data.len();

          body.rows(row_height, num_rows, |mut row| {
            let row_index = row.index();
            let row_data = &self.data[row_index];
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

fn main() -> eframe::Result {
  env_logger::init();

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
    ..Default::default()
  };

  // 4. Використовуємо run_native (стандартний метод запуску в 0.29+)
  eframe::run_native(
    "Excel Table",
    options,
    // Створюємо додаток через Box
    Box::new(|_cc| Ok(Box::new(MyApp::default()))),
  )
}
