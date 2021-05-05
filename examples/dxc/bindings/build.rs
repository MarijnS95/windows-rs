fn main() {
    windows::build! {
        Windows::Win32::{
            Graphics::Hlsl::*, System::Diagnostics::Debug::WIN32_ERROR,
        },
    };
}
