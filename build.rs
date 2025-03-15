use std::io;
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        WindowsResource::new().set_icon("newicon.ico").compile()?;
    }
    Ok(())
}
