// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub const PURE_VALUE_METHODS: &[&str] =
    &["clone", "len", "get", "is_empty", "contains", "iter"];

pub const CUSTOM_TYPE_ELIGIBLE_METHODS: &[&str] = &["clone"];

pub const STD_CONSTRUCTORS: &[&str] = &[
    "Box", "Arc", "Rc", "String", "Vec", "HashMap", "HashSet", "Option", "Result", "Ok", "Err",
    "Some", "None", "Ordering", "Duration", "Path", "PathBuf", "CString", "CStr", "OsString",
    "OsStr", "Default", "Clone",
];

pub const STD_MODULE_CALLS: &[&str] = &[
    "fs::read",
    "fs::write",
    "fs::read_to_string",
    "fs::read_dir",
    "fs::create_dir",
    "fs::create_dir_all",
    "fs::remove_file",
    "fs::remove_dir",
    "fs::remove_dir_all",
    "fs::copy",
    "fs::rename",
    "fs::metadata",
    "env::var",
    "env::args",
    "env::temp_dir",
    "env::current_dir",
    "env::current_exe",
    "env::set_var",
    "process::exit",
    "process::abort",
    "process::id",
    "thread::sleep",
    "thread::spawn",
    "net::TcpStream",
    "net::TcpListener",
    "net::UdpSocket",
];
