#[macro_use]
mod support;

mod bungee_color {
    use truetype::{Value, WindowsMetrics};

    #[test]
    #[cfg_attr(not(feature = "ignore-invalid-component-flags"), should_panic)]
    fn read() {
        match ok!(WindowsMetrics::read(&mut setup!(BungeeColor, "OS/2"))) {
            WindowsMetrics::Version3(_) => {}
            _ => unreachable!(),
        }
    }
}

mod source_serif {
    use truetype::{Tag, Value, WindowsMetrics};

    #[test]
    fn read() {
        match ok!(WindowsMetrics::read(&mut setup!(SourceSerif, "OS/2"))) {
            WindowsMetrics::Version3(ref table) => {
                assert_eq!(table.panose, [2, 4, 6, 3, 5, 4, 5, 2, 2, 4]);
                assert_eq!(table.vendor_id, Tag(*b"ADBE"));
                assert_eq!(table.break_char, 32);
            }
            _ => unreachable!(),
        }
    }
}
