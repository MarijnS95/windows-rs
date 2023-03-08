//! If running into segfaults on Linux, make sure DXC is compiled with this PR included:
//! https://github.com/microsoft/DirectXShaderCompiler/pull/3793

use anyhow::Result;
use libloading::{Library, Symbol};
use std::path::Path;
use windows::{
    core::{w, ComInterface},
    Win32::{Foundation::BOOL, Graphics::Direct3D::Dxc::*},
};

#[cfg(not(windows))]
use windows::Win32::{
    Foundation::HANDLE,
    System::{Memory::HEAP_FLAGS, SystemServices::MEMORY_ALLOCATION_ALIGNMENT},
};
#[cfg(not(windows))]
const PROCESS_HEAP: HANDLE = HANDLE(1);

#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn FormatMessageW() {
    std::process::abort()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn GetProcessHeap() -> HANDLE {
    PROCESS_HEAP
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn SysStringLen() {
    std::process::abort()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn HeapAlloc(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    dwbytes: usize,
) -> *mut ::core::ffi::c_void {
    dbg!(hheap, dwflags, dwbytes);
    assert_eq!(PROCESS_HEAP.0, 1);
    let layout =
        std::alloc::Layout::from_size_align(dwbytes, MEMORY_ALLOCATION_ALIGNMENT as usize).unwrap();
    dbg!(unsafe { std::alloc::alloc(layout) }.cast())
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn LoadLibraryExA() {
    std::process::abort()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn GetProcAddress() {
    std::process::abort()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn GetErrorInfo() {
    std::process::abort()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn GetLastError() {
    std::process::abort()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn FreeLibrary() {
    std::process::abort()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn HeapFree(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    lpmem: *const ::core::ffi::c_void,
) -> BOOL {
    dbg!(hheap, dwflags, lpmem);
    assert_eq!(PROCESS_HEAP.0, 1);
    /* TODO: Store length in prefix */
    let layout =
        std::alloc::Layout::from_size_align(0, MEMORY_ALLOCATION_ALIGNMENT as usize).unwrap();
    unsafe { std::alloc::dealloc(lpmem as *mut _, layout) };
    true.into()
}
#[cfg(not(windows))]
#[no_mangle]
pub extern "system" fn SysFreeString() {
    std::process::abort()
}

#[cfg(target_os = "windows")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("dxcompiler.dll")
}

#[cfg(target_os = "linux")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("libdxcompiler.so")
}

#[cfg(target_os = "macos")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("libdxcompiler.dynlib")
}

fn blob_encoding_as_str(blob: &IDxcBlobEncoding) -> &str {
    if unsafe { blob.GetBufferSize() } == 0 {
        return "No error details (string) provided!";
    }

    let mut known: BOOL = false.into();
    let mut cp = DXC_CP::default();
    unsafe { blob.GetEncoding(&mut known as *mut _ as _, &mut cp as *mut _ as _) }.unwrap();
    assert!(bool::from(known));
    assert_eq!(cp, DXC_CP_UTF8);
    unsafe {
        let slice = std::slice::from_raw_parts(
            blob.GetBufferPointer() as *const u8,
            blob.GetBufferSize() - 1,
        );
        std::str::from_utf8_unchecked(slice)
    }
}

fn main() -> Result<()> {
    let lib = unsafe { Library::new(dxcompiler_lib_name()) }?;
    let create: Symbol<DxcCreateInstanceProc> = unsafe { lib.get(b"DxcCreateInstance\0") }?;
    dbg!(&create);

    // TODO: Re-add helper callers for symbols
    // This depends on fixed SAL for DXC:
    // https://github.com/microsoft/DirectXShaderCompiler/pull/4524
    // And support for helper generation here in windows-rs:
    // https://github.com/microsoft/windows-rs/issues/1835
    // let compiler: IDxcCompiler3 = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcCompiler) }?;
    // let library: IDxcLibrary = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcLibrary) }?;
    let mut compiler = None::<IDxcCompiler3>;
    unsafe {
        (create.unwrap())(
            &CLSID_DxcCompiler,
            &IDxcCompiler3::IID,
            <*mut _>::cast(&mut compiler),
        )
        .ok()?
    };
    let mut library = None::<IDxcLibrary>;
    unsafe {
        (create.unwrap())(
            &CLSID_DxcLibrary,
            &IDxcLibrary::IID,
            <*mut _>::cast(&mut library),
        )
        .ok()?
    };
    let compiler = compiler.unwrap();
    let library = library.unwrap();

    dbg!(&compiler, &library);

    let main_shader = include_str!("copy.hlsl");

    // let shader_blob = unsafe {
    //     library.CreateBlobWithEncodingFromPinned(
    //         main_shader.as_ptr() as *const _,
    //         main_shader.len() as u32,
    //         DXC_CP_UTF8,
    //     )
    // }?;
    // dbg!(&shader_blob);

    let shader_blob = DxcBuffer {
        Ptr: main_shader.as_ptr().cast(),
        Size: main_shader.len(),
        Encoding: DXC_CP_UTF8.0,
    };

    // let defines = vec![];

    let result: IDxcResult = unsafe {
        compiler.Compile(
            &shader_blob,
            // , w!("-Fc"), w!("HELLO")
            Some(&[
                w!("-T"),
                w!("cs_6_5"),
                w!("-E"),
                w!("copyCs"),
                // w!("-spirv"),
                w!("-Zi"),
                w!("-Qembed_debug"),
            ]),
            None,
        )
        // compiler.Compile(
        //     &shader_blob,
        //     w!("copy.hlsl"),
        //     w!("copyCs"),
        //     w!("cs_6_5"),
        //     None,
        //     &defines,
        //     None,
        // )
    }?;

    let status = unsafe { result.GetStatus() }?;
    if status.is_err() {
        let errors = unsafe { result.GetErrorBuffer() }?;
        let errors = blob_encoding_as_str(&errors);
        eprintln!("Compilation failed with {:?}: `{}`", status, errors);
        Ok(status.ok()?)
    } else {
        let blob = unsafe { result.GetResult() }?;

        let types = (0..unsafe { result.GetNumOutputs() })
            .map(|i| unsafe { result.GetOutputByIndex(i) })
            .collect::<Vec<_>>();
        dbg!(types);

        let shader = unsafe {
            std::slice::from_raw_parts(blob.GetBufferPointer().cast::<u8>(), blob.GetBufferSize())
        };

        let mut blob = None::<IDxcBlob>;
        let mut outname = None::<IDxcBlobUtf16>;

        unsafe { result.GetOutput(DXC_OUT_PDB, &mut outname, &mut blob) }?;
        let blob = blob.unwrap();
        let outname = outname.unwrap();
        // dbg!(unsafe { outname.GetStringPointer().to_string() });
        // let shader2 = unsafe {
        //     std::slice::from_raw_parts(blob.GetBufferPointer().cast::<u8>(), blob.GetBufferSize())
        // };
        // assert_eq!(shader, shader2);

        let result: IDxcResult = unsafe {
            compiler.Disassemble(dbg!(&DxcBuffer {
                Ptr: blob.GetBufferPointer().cast(),
                Size: blob.GetBufferSize(),
                Encoding: 0,
            }))
        }?;

        let blob = unsafe { result.GetResult() }?;

        let types = (0..unsafe { result.GetNumOutputs() })
            .map(|i| unsafe { result.GetOutputByIndex(i) })
            .collect::<Vec<_>>();
        dbg!(types);

        let disasm = unsafe {
            std::slice::from_raw_parts(blob.GetBufferPointer().cast::<u8>(), blob.GetBufferSize())
        };
        println!("{}", std::str::from_utf8(disasm).unwrap());

        // std::fs::write("result.spv", shader)?;
        // for c in shader.chunks(16) {
        //     println!("{:02x?}", c);
        // }
        Ok(())
    }
}
