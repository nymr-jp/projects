#[derive(PartialEq)]
pub enum KeyCode {
    Esc,
    Left,
    Right,
    Up,
    Down,
    Home,
    F(u8),
    Something,
}

pub fn parse(buffer: &[u8]) -> std::io::Result<Option<KeyCode>> {
    if buffer.is_empty() {
        return Ok(None);
    }

    match buffer[0] {
        b'\x1B' => {
            if buffer.len() == 1 {
                Ok(Some(KeyCode::Esc))
            } else {
                match buffer[1] {
                    b'[' => {
                        // ANSI escape code: xterm sequence
                        match buffer[2] {
                            b'D' => Ok(Some(KeyCode::Left)),
                            b'C' => Ok(Some(KeyCode::Right)),
                            b'A' => Ok(Some(KeyCode::Up)),
                            b'B' => Ok(Some(KeyCode::Down)),
                            b'H' => Ok(Some(KeyCode::Home)),
                            _ => Ok(None)
                        }
                    },
                    _ => Ok(None)
                }
            }
        },
        _ => Ok(Some(KeyCode::Something)),
    }
}
