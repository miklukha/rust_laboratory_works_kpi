use rfd::FileDialog;
use std::fs;

use chrono::{DateTime, Local};

use crate::todo_item::ListItemData;
use crate::{AppConfig, AppLogic, AppWindow, ListItem};

use resolve_path::PathResolveExt;
use rmp_serde as rmps;
use slint::{ComponentHandle, Model};

pub fn callback_declare_dump_list_items(app: &AppWindow) {
    let logic = app.global::<AppLogic>();

    let weak_app = app.as_weak();
    logic.on_dump_list_items(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let items: Vec<ListItemData> = cfg.get_list_items().iter().map(|li| li.into()).collect();
        let item_buf = rmps::to_vec(&items).unwrap();

        let data_path = cfg.get_data_path();
        fs::write(data_path.as_str().resolve(), item_buf)
            .map_err(|err| eprintln!("{err:?}"))
            .unwrap_or_default();
    });
}

pub fn callback_declare_load_list_items(app: &AppWindow) {
    let logic = app.global::<AppLogic>();

    let weak_app = app.as_weak();
    logic.on_load_list_items(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let data_path = cfg.get_data_path();
        let data: Vec<u8> = fs::read(data_path.as_str().resolve())
            .map_err(|err| eprintln!("{err:?}"))
            .unwrap_or_default();
        if data.is_empty() {
            return;
        }

        let items: Vec<ListItem> = rmps::from_slice::<Vec<ListItemData>>(&data)
            .unwrap()
            .iter()
            .map(|li| li.to_owned().into())
            .collect();

        let items_model = std::rc::Rc::new(slint::VecModel::from(items));
        cfg.set_list_items(items_model.into());
    });

    logic.invoke_load_list_items();
}

pub fn callback_declare_pop_list_item(app: &AppWindow) {
    let logic = app.global::<AppLogic>();

    let weak_app = app.as_weak();
    logic.on_pop_list_item(move |idx| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();
        let mut items: Vec<ListItem> = cfg.get_list_items().iter().collect();
        let item = items.remove(idx as usize);
        let items_model = std::rc::Rc::new(slint::VecModel::from(items));
        cfg.set_list_items(items_model.into());
        item
    });
}

pub fn callback_declare_put_list_item(app: &AppWindow) {
    let logic = app.global::<AppLogic>();

    let weak_app = app.as_weak();
    logic.on_put_list_item(move |idx, mut item| {
        let desc = item.description.trim();
        if desc == "" {
            return;
        }

        let chars = desc.chars().collect::<Vec<_>>();
        if chars.len() > 250 {
            item.description = chars[0..250].iter().collect::<String>().into();
        }

        let current_local: DateTime<Local> = Local::now();
        let datetime = current_local.format("%d-%m-%Y • %H:%M").to_string();
        item.datetime = datetime.into();

        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();
        let mut items: Vec<ListItem> = cfg.get_list_items().iter().collect();
        items.insert(idx as usize, item);
        let items_model = std::rc::Rc::new(slint::VecModel::from(items));
        cfg.set_list_items(items_model.into());
    });
}

pub fn callback_declare_edit_list_item(app: &AppWindow) {
    let logic = app.global::<AppLogic>();

    let weak_app = app.as_weak();
    logic.on_edit_list_item(move |idx, mut item| {
        let desc = item.description.trim();
        if desc == "" {
            return;
        }

        let chars = desc.chars().collect::<Vec<_>>();
        if chars.len() > 250 {
            item.description = chars[0..250].iter().collect::<String>().into();
        }

        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();
        let mut items: Vec<ListItem> = cfg.get_list_items().iter().collect();

        if idx < items.len() as i32 {
            items[idx as usize] = item;
            let items_model = std::rc::Rc::new(slint::VecModel::from(items));
            cfg.set_list_items(items_model.into());
        }
    });
}

pub fn callback_declare_export_list_items(app: &AppWindow) {
    let logic = app.global::<AppLogic>();

    let weak_app = app.as_weak();
    logic.on_export_list_items(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        if let Some(path) = FileDialog::new()
            .set_title("Експорт списку справ")
            .add_filter("Text Files", &["txt"]) // Змінено на .txt
            .save_file()
        {
            let items: Vec<ListItemData> =
                cfg.get_list_items().iter().map(|li| li.into()).collect();

            // Перетворення в текстовий формат
            let items_text = items
                .iter()
                .map(|item| {
                    format!(
                        "зроблено: {}, справа: {}, дата: {}",
                        item.completed,
                        item.description.replace(',', "\\,"),
                        item.datetime
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            // Зберігаємо файл
            fs::write(&path, items_text)
                .map_err(|err| eprintln!("Помилка експорту: {err:?}"))
                .unwrap_or_default();
        }
    });
}

pub fn callback_declare_import_list_items(app: &AppWindow) {
    let logic = app.global::<AppLogic>();

    let weak_app = app.as_weak();
    logic.on_import_list_items(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        if let Some(path) = FileDialog::new()
            .set_title("Імпорт списку справ")
            .add_filter("Text Files", &["txt"]) // Змінено на .txt
            .pick_file()
        {
            let data = fs::read_to_string(&path)
                .map_err(|err| eprintln!("Помилка читання файлу: {err:?}"))
                .unwrap_or_default();

            if data.is_empty() {
                return;
            }

            // Парсинг текстового файлу
            let items: Vec<ListItem> = data
                .lines()
                .filter_map(|line| {
                    let completed_match = line.contains("зроблено: true");
                    let description_match =
                        line.split("справа: ").nth(1)?.split(", дата:").next()?;
                    let datetime_match = line.split("дата: ").nth(1);

                    Some(ListItem {
                        completed: completed_match,
                        description: description_match.replace("\\,", ",").into(),
                        datetime: datetime_match?.to_string().into(),
                    })
                })
                .collect();

            let items_model = std::rc::Rc::new(slint::VecModel::from(items));
            cfg.set_list_items(items_model.into());

            // Зберігаємо в основний файл даних
            let item_buf = rmps::to_vec(
                &cfg.get_list_items()
                    .iter()
                    .map(|li| li.into())
                    .collect::<Vec<ListItemData>>(),
            )
            .unwrap();
            fs::write(cfg.get_data_path().as_str().resolve(), item_buf)
                .map_err(|err| eprintln!("Помилка збереження: {err:?}"))
                .unwrap_or_default();
        }
    });
}

// pub fn callback_declare_export_list_items(app: &AppWindow) {
//     let logic = app.global::<AppLogic>();

//     let weak_app = app.as_weak();
//     logic.on_export_list_items(move || {
//         let app = weak_app.upgrade().unwrap();
//         let cfg = app.global::<AppConfig>();

//         // Вибираємо файл для експорту
//         if let Some(path) = FileDialog::new()
//             .set_title("Експорт списку справ")
//             .add_filter("MessagePack Files", &["mpack"])
//             .save_file()
//         {
//             let items: Vec<ListItemData> =
//                 cfg.get_list_items().iter().map(|li| li.into()).collect();
//             let item_buf = rmps::to_vec(&items).unwrap();

//             // Зберігаємо файл
//             fs::write(&path, item_buf)
//                 .map_err(|err| eprintln!("Помилка експорту: {err:?}"))
//                 .unwrap_or_default();
//         }
//     });
// }

// pub fn callback_declare_import_list_items(app: &AppWindow) {
//     let logic = app.global::<AppLogic>();

//     let weak_app = app.as_weak();
//     logic.on_import_list_items(move || {
//         let app = weak_app.upgrade().unwrap();
//         let cfg = app.global::<AppConfig>();

//         // Вибираємо файл для імпорту
//         if let Some(path) = FileDialog::new()
//             .set_title("Імпорт списку справ")
//             .add_filter("MessagePack Files", &["mpack"])
//             .pick_file()
//         {
//             let data: Vec<u8> = fs::read(&path)
//                 .map_err(|err| eprintln!("Помилка читання файлу: {err:?}"))
//                 .unwrap_or_default();

//             if data.is_empty() {
//                 return;
//             }

//             let items: Vec<ListItem> = rmps::from_slice::<Vec<ListItemData>>(&data)
//                 .unwrap()
//                 .iter()
//                 .map(|li| li.to_owned().into())
//                 .collect();

//             let items_model = std::rc::Rc::new(slint::VecModel::from(items));
//             cfg.set_list_items(items_model.into());

//             // Одразу зберігаємо імпортовані дані
//             let item_buf = rmps::to_vec(
//                 &cfg.get_list_items()
//                     .iter()
//                     .map(|li| li.into())
//                     .collect::<Vec<ListItemData>>(),
//             )
//             .unwrap();
//             fs::write(cfg.get_data_path().as_str().resolve(), item_buf)
//                 .map_err(|err| eprintln!("Помилка збереження: {err:?}"))
//                 .unwrap_or_default();
//         }
//     });
// }
