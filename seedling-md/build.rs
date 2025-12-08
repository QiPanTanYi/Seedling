// build script for embedding Windows icon via winres

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/seedling.ico");
        if let Err(e) = res.compile() {
            println!("cargo:warning=嵌入图标失败: {}", e);
        }
    }
}
