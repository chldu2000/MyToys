#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

pub mod table_info;

use std::cell::RefCell;

use nwd::NwgUi;
use nwg::NativeUi;

use table_info::*;

#[derive(Default, NwgUi)]
pub struct GUIAnalyzer {
    // 储存分析表
    table_info: RefCell<TableInfo>,

    // 分析表、分析过程所用字体
    #[nwg_control(size: 16, family: "Consolas", weight: 1000)]
    consolas_font: nwg::Font,

    // 主窗口相关
    #[nwg_control(source_file: Some("afool.ico"))]
    my_icon: nwg::Icon,
    #[nwg_control(
        size: (800, 600),
        position: (200, 200),
        title: "LL(1)-analyze",
        icon: Some(&data.my_icon),
        flags: "WINDOW|VISIBLE"
    )]
    #[nwg_events( OnWindowClose: [GUIAnalyzer::exit] )]
    window: nwg::Window,

    // 文件对话框
    #[nwg_resource(
        title: "选择一个文件",
        action: nwg::FileDialogAction::Open,
        multiselect: false
    )]
    file_picker: nwg::FileDialog,

    // 文件名
    #[nwg_control(size: (670, 30), position: (10, 10), readonly: true)]
    file_name_area: nwg::TextInput,

    // 按钮，触发 select_file
    #[nwg_control(text: "选择文件", size: (100, 30), position: (690, 10))]
    #[nwg_events( OnButtonClick: [GUIAnalyzer::select_file] )]
    file_select_btn: nwg::Button,

    // 展示分析表
    #[nwg_control(size: (780, 240), position: (10, 50), font: Some(&data.consolas_font))]
    analyze_table_area: nwg::TextBox,

    // 输入待分析的串
    #[nwg_control(
        placeholder_text: Some("输入串，符号之间用空格分隔"),
        size: (670, 30),
        position: (10, 300)
    )]
    input_string: nwg::TextInput,

    // 按钮，触发analyze_string
    #[nwg_control(text: "开始分析", size: (100, 30), position: (690, 300))]
    #[nwg_events( OnButtonClick: [GUIAnalyzer::analyze_string] )]
    analyze_string_btn: nwg::Button,

    // 展示分析过程
    #[nwg_control(size: (780, 250), position: (10, 340), font: Some(&data.consolas_font))]
    analyze_process_area: nwg::TextBox,
}

impl GUIAnalyzer {
    // 选择文件，获取分析表
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
                self.analyze_table_area.clear();

                // 读取、显示分析表
                if !self.table_info.borrow_mut().init_table(&file_n) {
                    nwg::modal_error_message(&self.window, "错误", "该文法不是 LL(1) 文法。");
                    return;
                }
                let table = self.table_info.borrow().get_analyze_table().unwrap();
                self.analyze_table_area.set_text(&table);
            }
        }
    }

    // 分析输入的串
    fn analyze_string(&self) {
        // 保证已经选择了存有分析表的文件
        if self.file_name_area.text().is_empty() {
            nwg::modal_error_message(&self.window, "错误", "请打开正确的分析表文件。");
            return;
        }
        self.analyze_process_area.clear();

        // 分析输入串，展示过程
        let (ok, process) = self
            .table_info
            .borrow()
            .analyze_input(&self.input_string.text());
        self.analyze_process_area.set_text(&process.unwrap());
        if ok {
            nwg::modal_info_message(&self.window, "分析完成", "输入串符合文法。");
        } else {
            nwg::modal_error_message(&self.window, "分析完成", "输入串不符合文法。");
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
