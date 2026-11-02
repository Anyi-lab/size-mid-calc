// 尺寸中间值计算 · Tauri 外壳
// 页面本身是纯静态 HTML，不需要任何后端命令。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动失败");
}
