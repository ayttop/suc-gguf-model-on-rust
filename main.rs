use kalosm::language::*;
use std::path::PathBuf;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // تحميل النموذج من ملف GGUF الخاص بك
    let model = Llama::builder()
        .with_source(LlamaSource::new(
            FileSource::Local(PathBuf::from("models/llama-3.2-3b-instruct-uncensored-q8_0.gguf"))
        ))
        .build()
        .await?;

    // إنشاء جلسة شات مع prompt نظام
    let mut chat = model
        .chat()
        .with_system_prompt("You are a helpful assistant.");

    println!("Type 'exit' or 'quit' to end the chat.");

    loop {
        // قراءة مدخل المستخدم
        print!("\nYou: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // خروج عند كتابة exit أو quit
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Exiting chat. Goodbye!");
            break;
        }

        // ارسال للموديل وطباعة الرد
        let mut response_stream = chat(input);
        response_stream.to_std_out().await?;
    }

    Ok(())
}