use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(val) => format!("{:?}", val),
        _       => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut res: Vec<ResistorColor> = Vec::new();
    let mut i = 0;

    loop {
        if i >= ResistorColor::VARIANT_COUNT {
            break;
        }
        res.push(ResistorColor::from_int(i).unwrap());
        i += 1;
    }
    res
}
