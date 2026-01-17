#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    let data = Vec::new();
    // read test excel

    Self { data }
  }
}

impl eframe::App for ExcelReader {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {});
  }
}
