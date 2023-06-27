use backend::{xcb_backend::XcbBackend, Key, KeyMap};

use crate::backend::Backend;

mod backend;

const ENCODING: [(backend::Key, &str, &str); 4] = [
    (Key::Alt, "A", "a"),
    (Key::Ctrl, "C", "c"),
    (Key::Shift, "S", "s"),
    (Key::Super, "W", "w"),
];

fn encode(km: KeyMap) -> String {
    ENCODING
        .iter()
        .map(|(k, t, f)| (if *km.get(k).unwrap_or(&false) { t } else { f }).chars())
        .flatten()
        .collect()
}

fn main() {
    let backend = XcbBackend::new().expect("Failed to establish connection to X server.");
    match backend.get() {
        Ok(state) => {
            println!("{}", encode(state));
        }
        Err(err) => {
            println!("err!");
            eprintln!("Failed to obtain keyboard state: {}", err)
        }
    }
    loop {
        match backend.poll() {
            Ok(state) => {
                println!("{}", encode(state));
            }
            Err(err) => {
                println!("err!");
                eprintln!("Failed to obtain keyboard state notification: {}", err)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::backend::Key;

    use super::encode;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode(
                [
                    (Key::Alt, true),
                    (Key::Ctrl, true),
                    (Key::Shift, true),
                    (Key::Super, true)
                ]
                .into()
            ),
            "ACSW".to_owned()
        );

        assert_eq!(
            encode(
                [
                    (Key::Alt, false),
                    (Key::Ctrl, false),
                    (Key::Shift, false),
                    (Key::Super, false)
                ]
                .into()
            ),
            "acsw".to_owned()
        );

        assert_eq!(
            encode(
                [
                    (Key::Alt, true),
                    (Key::Ctrl, false),
                    (Key::Shift, true),
                    (Key::Super, false)
                ]
                .into()
            ),
            "AcSw".to_owned()
        );
    }
}
