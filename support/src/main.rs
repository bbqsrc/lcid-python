extern crate winapi;
extern crate json;

use std::ffi::{OsString, OsStr};
use winapi::shared::minwindef::*;
use winapi::um::winnt::*;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use std::iter::once;

unsafe fn lpwstr_len(lpw: &LPWSTR) -> usize {
    let mut i = 0isize;

    while *lpw.offset(i) != 0 {
        i += 1;
    }
    
    i as usize
}

fn main() {
    let mut out = json::object::Object::new();

    for l in system_locales() {
        if l == "" {
            continue;
        }
        let lcid = locale_name_to_lcid(&l);
        if lcid == 0x1000 {
            continue;
        }

        out.insert(&l, json::JsonValue::Number(lcid.into()));
    }

    println!("{}", out.pretty(2));
}

fn system_locales() -> Vec<String> {
    unsafe extern "system" fn callback(locale: LPWSTR, _: DWORD, l_param: LPARAM) -> i32 {
        let loc_slice = std::slice::from_raw_parts(locale, lpwstr_len(&locale));
        let s = from_wide_string(loc_slice).unwrap();
        let vec = l_param as *mut Vec<String>;
        (*vec).push(s);
        1
    }
    let raw_vec = Box::into_raw(Box::new(vec![]));
    unsafe {
        winapi::um::winnls::EnumSystemLocalesEx(Some(callback), 0, raw_vec as LPARAM, null_mut());
        *Box::from_raw(raw_vec)
    }
}

fn from_wide_string(vec: &[u16]) -> Result<String, OsString> {
    let s = OsString::from_wide(&vec)
        .into_string()?;
    
    Ok(s.split('\0').next().unwrap().to_owned())
}

fn locale_name_to_lcid(locale_name: &str) -> u32 {
    let loc_name: Vec<u16> = OsStr::new(locale_name)
        .encode_wide()
        .chain(once(0))
        .collect();
    
    unsafe {
        winapi::um::winnls::LocaleNameToLCID(loc_name.as_ptr(), 0)
    }
}