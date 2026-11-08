//! Встраивает в exe иконку, манифест и информацию о версии.
//! Релиз: манифест требует права администратора (UAC при запуске).
//! Debug: `asInvoker`, чтобы `cargo run` работал из обычного терминала.

#[path = "src/icon.rs"]
mod icon;

use std::{env, fs, path::Path, path::PathBuf};

const ICO_SIZES: &[u32] = &[16, 20, 24, 32, 40, 48, 64, 256];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/icon.rs");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let release = env::var("PROFILE").as_deref() == Ok("release");

    let ico = out.join("gds.ico");
    fs::write(&ico, build_ico(ICO_SIZES)).unwrap();

    let level = if release {
        "requireAdministrator"
    } else {
        "asInvoker"
    };
    let manifest = out.join("gds.manifest");
    fs::write(&manifest, MANIFEST.replace("{LEVEL}", level)).unwrap();

    let ver_commas = format!("{},0", version.replace('.', ","));
    let rc = RC
        .replace("{ICO}", &rc_path(&ico))
        .replace("{MANIFEST}", &rc_path(&manifest))
        .replace("{VER_COMMAS}", &ver_commas)
        .replace("{VER}", &version);
    let rc_file = out.join("gds.rc");
    fs::write(&rc_file, rc).unwrap();

    embed_resource::compile(&rc_file, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}

fn rc_path(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "\\\\")
}

/// ICO из 32-битных DIB-картинок (без PNG-кодировщика и зависимостей).
fn build_ico(sizes: &[u32]) -> Vec<u8> {
    let images: Vec<Vec<u8>> = sizes.iter().map(|&s| dib(s, &icon::render(s))).collect();

    let mut out = Vec::new();
    out.extend(0u16.to_le_bytes()); // reserved
    out.extend(1u16.to_le_bytes()); // type: icon
    out.extend((sizes.len() as u16).to_le_bytes());

    let mut offset = 6 + 16 * sizes.len() as u32;
    for (&s, img) in sizes.iter().zip(&images) {
        let d = if s >= 256 { 0 } else { s as u8 };
        out.extend([d, d, 0, 0]); // width, height, colors, reserved
        out.extend(1u16.to_le_bytes()); // planes
        out.extend(32u16.to_le_bytes()); // bpp
        out.extend((img.len() as u32).to_le_bytes());
        out.extend(offset.to_le_bytes());
        offset += img.len() as u32;
    }
    for img in images {
        out.extend(img);
    }
    out
}

fn dib(s: u32, rgba: &[u8]) -> Vec<u8> {
    let mask_row = s.div_ceil(32) * 4;
    let mut out = Vec::new();
    // BITMAPINFOHEADER (высота удвоена: XOR + AND маски)
    out.extend(40u32.to_le_bytes());
    out.extend((s as i32).to_le_bytes());
    out.extend((2 * s as i32).to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(32u16.to_le_bytes());
    out.extend(0u32.to_le_bytes()); // BI_RGB
    out.extend((s * s * 4 + mask_row * s).to_le_bytes());
    out.extend([0u8; 16]);
    // Пиксели снизу вверх, BGRA
    for y in (0..s).rev() {
        for x in 0..s {
            let i = ((y * s + x) * 4) as usize;
            out.extend([rgba[i + 2], rgba[i + 1], rgba[i], rgba[i + 3]]);
        }
    }
    // AND-маска (нули — прозрачность берётся из альфы)
    out.extend(std::iter::repeat_n(0u8, (mask_row * s) as usize));
    out
}

const MANIFEST: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="{LEVEL}" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
    </application>
  </compatibility>
</assembly>
"#;

const RC: &str = r#"1 ICON "{ICO}"
1 24 "{MANIFEST}"

1 VERSIONINFO
FILEVERSION {VER_COMMAS}
PRODUCTVERSION {VER_COMMAS}
FILEOS 0x40004
FILETYPE 0x1
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904B0"
    BEGIN
      VALUE "FileDescription", "GDS - Genshin Dialogs Skipper"
      VALUE "ProductName", "GDS - Genshin Dialogs Skipper"
      VALUE "FileVersion", "{VER}"
      VALUE "ProductVersion", "{VER}"
      VALUE "OriginalFilename", "gds.exe"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x409, 1200
  END
END
"#;
