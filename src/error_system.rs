pub enum OsuKeyboardError {
    InitializationFailed,
    Unknown,
}

pub fn report(error: OsuKeyboardError) -> ! {
    let error = match error {
        _ => ||{},
    };

    loop {
        (error)();
        arduino_uno::delay_ms(1000_u16)
    }
}

// fn stutter_blink(led: &mut PB5<Output>, times: usize) {
//     (0..times).map(|i| i * 10).for_each(|i| {
//         led.toggle().void_unwrap();
//         arduino_uno::delay_ms(i as u16);
//     });
// }
