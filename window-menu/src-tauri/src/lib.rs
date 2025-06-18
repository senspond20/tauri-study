#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItem, SubmenuBuilder},
};
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .setup(|app| {
            let check_sub_item_en = CheckMenuItemBuilder::with_id("en", "EN")
                .checked(true)
                .build(app)?;

            let check_sub_item_zh = CheckMenuItemBuilder::with_id("zh", "ZH")
                .checked(false)
                .build(app)?;

            let text_menu = MenuItem::with_id(
                app,
                "change_text",
                &"Change menu".to_string(),
                true,
                Some("Ctrl+Z"),
            )
            .unwrap();

            let menu_item = SubmenuBuilder::new(app, "Change menu")
                .item(&text_menu)
                .items(&[&check_sub_item_en, &check_sub_item_zh])
                .build()?;
            let menu = MenuBuilder::new(app).items(&[&menu_item]).build()?;
            app.set_menu(menu)?;
            app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
                match event.id().0.as_str() {
                    "change_text" => {
                        text_menu
                            .set_text("changed menu text")
                            .expect("Change text error");

                        text_menu
                            .set_text("changed menu text")
                            .expect("Change text error");
                    }

                    "en" | "zh" => {
                        check_sub_item_en
                            .set_checked(event.id().0.as_str() == "en")
                            .expect("Change check error");
                        check_sub_item_zh
                            .set_checked(event.id().0.as_str() == "zh")
                            .expect("Change check error");
                        check_sub_item_zh.set_accelerator(Some("Ctrl+L"))
                        .expect("Change accelerator error");
                    }
                    _ => {
                        println!("unexpected menu event");
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
