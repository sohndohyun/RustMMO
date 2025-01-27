use dsnet::client;
use eframe::egui;

mod protocol_generated;

use flatbuffers::FlatBufferBuilder;
use protocol_generated::nexus;

struct ChatApp {
    messages: Vec<String>,
    input_text: String,
    user_name: String,
    focus_requested: bool,
    client_app: client::App,
}

impl ChatApp {
    pub async fn new() -> Self {
        Self {
            messages: vec![],
            input_text: String::new(),
            user_name: String::from("Me"), // 기본 이름 설정
            focus_requested: false,
            client_app: client::App::create("127.0.0.1:1234".into())
                .await
                .unwrap(),
        }
    }

    pub fn network_update(&mut self) {
        loop {
            match self.client_app.get_callback() {
                client::Callback::Receive {
                    packet_type,
                    message,
                } => match nexus::PacketType(packet_type) {
                    nexus::PacketType::MESSAGE => {
                        let data = flatbuffers::root::<nexus::Message>(&message).unwrap();
                        self.messages.push(format!(
                            "{}: {}",
                            data.name().unwrap(),
                            data.message().unwrap()
                        ));
                    }
                    _ => {
                        panic!("how can?");
                    }
                },
                _ => break,
            };
        }
    }

    pub fn send_message(&mut self) {
        let mut builder = FlatBufferBuilder::new();
        let name = builder.create_string(&self.user_name);
        let message = builder.create_string(&self.input_text);

        let message = nexus::Message::create(
            &mut builder,
            &nexus::MessageArgs {
                name: Some(name),
                message: Some(message),
            },
        );

        builder.finish(message, None);
        let serialized_data = builder.finished_data();

        let _ = self
            .client_app
            .send_message(nexus::PacketType::MESSAGE.0, serialized_data.to_vec());
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.network_update();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // 채팅 메시지 표시 영역
                egui::ScrollArea::vertical()
                    .id_salt("chat_messages")
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .max_height(300.0)
                    .min_scrolled_height(300.0)
                    .show(ui, |ui| {
                        for message in &self.messages {
                            ui.label(message);
                        }
                    });

                ui.separator();

                // 입력 필드와 전송 버튼
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [50.0, 20.0],
                        egui::TextEdit::singleline(&mut self.user_name),
                    );
                    let input = ui.text_edit_singleline(&mut self.input_text);

                    // 첫 업데이트 시 입력 필드에 포커스 요청
                    if !self.focus_requested {
                        input.request_focus();
                        self.focus_requested = true;
                    }

                    // Enter 키 입력 또는 버튼 클릭으로 메시지 전송
                    if ui.button("Send").clicked()
                        || input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        if !self.input_text.is_empty() {
                            self.send_message();
                            self.input_text.clear();
                        }
                        input.request_focus();
                    }
                });
            });
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([400.0, 345.0]),
        ..Default::default()
    };

    let client_app = ChatApp::new().await;

    eframe::run_native(
        "Chat App",
        native_options,
        Box::new(|_cc| Ok(Box::new(client_app))),
    )
}
