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
