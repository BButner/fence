pub mod windows_cursor;

#[tokio::main]
async fn main() {
    let cursor = windows_cursor::WindowsCursorHandler::new().await;

    cursor.start_mouse_hook();

    loop {}
}
