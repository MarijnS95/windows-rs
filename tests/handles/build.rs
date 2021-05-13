fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::HANDLE,
        Windows::Win32::Graphics::Gdi::HGDIOBJ,
    );
}
