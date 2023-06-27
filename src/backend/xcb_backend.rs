use super::{Backend, Key, KeyMap};

use xcb::{x::ModMask, xkb};

pub struct XcbBackend {
    conn: xcb::Connection,
}

impl XcbBackend {
    pub fn new() -> anyhow::Result<XcbBackend> {
        let (conn, _) =
            xcb::Connection::connect_with_extensions(None, &[xcb::Extension::Xkb], &[])?;

        {
            let xkb_ver = conn.wait_for_reply(conn.send_request(&xkb::UseExtension {
                wanted_major: 1,
                wanted_minor: 0,
            }))?;

            if !xkb_ver.supported() {
                return Err(anyhow::format_err!("Failed to find support for xkb 1.0"));
            }
        }

        let events = xkb::EventType::STATE_NOTIFY;
        let map_parts = xkb::MapPart::MODIFIER_MAP;
        let cookie = conn.send_request_checked(&xkb::SelectEvents {
            device_spec: xkb::Id::UseCoreKbd as xkb::DeviceSpec,
            affect_which: events,
            clear: xkb::EventType::empty(),
            select_all: events,
            affect_map: map_parts,
            map: map_parts,
            details: &[],
        });
        conn.check_request(cookie)?;
        Ok(XcbBackend { conn })
    }
    // add code here
}

const MODIFIERS: [(ModMask, Key); 4] = [
    (ModMask::N1, Key::Alt),
    (ModMask::CONTROL, Key::Ctrl),
    (ModMask::SHIFT, Key::Shift),
    (ModMask::N4, Key::Super),
];

fn get_keymap(mods: ModMask) -> KeyMap {
    return MODIFIERS
        .iter()
        .map(|(m, k)| (*k, mods.contains(*m)))
        .collect();
}
impl Backend for XcbBackend {
    fn poll(&self) -> anyhow::Result<KeyMap> {
        loop {
            match self.conn.wait_for_event()? {
                xcb::Event::Xkb(xkb::Event::StateNotify(ev)) => {
                    return Ok(get_keymap(ev.mods()));
                }
                _ => (),
            };
        }
    }

    fn get(&self) -> anyhow::Result<KeyMap> {
        let state = xcb::xkb::GetState {
            device_spec: xkb::Id::UseCoreKbd as xkb::DeviceSpec,
        };
        let state = self.conn.send_request(&state);
        let state = self.conn.wait_for_reply(state)?;
        Ok(get_keymap(state.mods()))
    }
}
