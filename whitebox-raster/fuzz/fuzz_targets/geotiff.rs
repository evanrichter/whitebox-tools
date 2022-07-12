#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let r = std::io::Cursor::new(data);
    let _ = whitebox_raster::print_tags_from_reader(r);
});
