#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

pub mod grammer_info;

use std::{cell::RefCell, fs::File, io::Write};

use nwd::NwgUi;
use nwg::NativeUi;

use grammer_info::*;

#[derive(Default, NwgUi)]
pub struct GUIAnalyzer {
    // 储存文法信息
    grammer_info: RefCell<GrammerInfo>,

    #[nwg_control(size: 16, family: "Consolas", weight: 1000)]
    consolas_font: nwg::Font,

    // 主窗口相关
    #[nwg_control(source_file: Some("afool.ico"))]
    my_icon: nwg::Icon,
    #[nwg_control(
        size: (800, 350),
        position: (200, 200),
        title: "LL(1)-construct",
        icon: Some(&data.my_icon),
        flags: "WINDOW|VISIBLE"
    )]
    #[nwg_events( OnWindowClose: [GUIAnalyzer::exit] )]
    window: nwg::Window,

    // 打开文件对话框
    #[nwg_resource(
        title: "选择一个文件",
        action: nwg::FileDialogAction::Open,
        multiselect: false
    )]
    file_picker: nwg::FileDialog,

    // 读取的文件名
    #[nwg_control(size: (670, 30), position: (10, 10), readonly: true)]
    file_name_area: nwg::TextInput,

    // 保存文件对话框
    #[nwg_resource(
        title: "保存为",
        action: nwg::FileDialogAction::Save,
    )]
    file_saver: nwg::FileDialog,

    // 按钮，触发 select_file
    #[nwg_control(text: "选择文件", size: (100, 30), position: (690, 10))]
    #[nwg_events( OnButtonClick: [GUIAnalyzer::select_file] )]
    file_select_btn: nwg::Button,

    // 展示 First 和 Follow 集合
    #[nwg_control(size: (780, 240), position: (10, 50), font: Some(&data.consolas_font))]
    display_area: nwg::TextBox,

    // 按钮，触发 save_table
    #[nwg_control(text: "生成分析表", size: (120, 30), position: (10, 300))]
    #[nwg_events( OnButtonClick: [GUIAnalyzer::save_table] )]
    analyze_string_btn: nwg::Button,
}

impl GUIAnalyzer {
    // 选择文件，获取文法信息
    fn select_file(&self) {
        // 打开项目所在文件夹，选择文件
        let current_dir = std::env::current_dir().unwrap();
        self.file_picker
            .set_default_folder(current_dir.to_str().unwrap())
            .expect("Failed to set default folder.");
        if self.file_picker.run(Some(&self.window)) {
            if let Ok(file_name) = self.file_picker.get_selected_item() {
                // 显示文件路径
                let file_n = file_name.into_string().unwrap();
                self.file_name_area.set_text(&file_n);
                self.display_area.clear();

                // 读取文法
                let mut grammer = self.grammer_info.borrow_mut();
                grammer.init_grammer(file_n);
                // 展示 first 和 follow 集合
                let text = grammer.get_first() + "\r\n" + &grammer.get_follow().clone();
                self.display_area.set_text(&text);
            }
        }
    }

    fn save_table(&self) {
        // 打开项目所在文件夹
        let current_dir = std::env::current_dir().unwrap();
        self.file_saver
            .set_default_folder(current_dir.to_str().unwrap())
            .expect("Failed to set default folder.");
        if self.file_saver.run(Some(&self.window)) {
            if let Ok(file_name) = self.file_saver.get_selected_item() {
                // 文件路径
                let file_n = file_name.into_string().unwrap();

                let table = self.grammer_info.borrow().get_table();
                let mut file = File::create(file_n).expect("Failed to create file!");
                let result = file.write_all(table.as_bytes());
                match result {
                    Ok(_save) => {
                        nwg::modal_info_message(&self.window, "OK", "保存成功。");
                    }
                    Err(_e) => {
                        nwg::modal_info_message(&self.window, "ERROR", "保存失败。");
                    }
                };
            }
        }
    }
    // 关闭窗口的动作
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = GUIAnalyzer::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
