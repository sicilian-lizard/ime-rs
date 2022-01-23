use globals::{
    SAMPLEIME_GUID_COMPARTMENT_DOUBLE_SINGLE_BYTE, SAMPLEIME_GUID_COMPARTMENT_PUNCTUATION,
    SAMPLEIME_GUID_DOUBLE_SINGLE_BYTE_PRESERVE_KEY, SAMPLEIME_GUID_IME_MODE_PRESERVE_KEY,
    SAMPLEIME_GUID_PUNCTUATION_PRESERVE_KEY,
};
use windows::{
    core::{Interface, GUID},
    Win32::{
        Foundation::PWSTR,
        UI::{
            Input::KeyboardAndMouse::{VK_OEM_PERIOD, VK_SHIFT, VK_SPACE},
            TextServices::{
                ITfKeystrokeMgr, ITfThreadMgr, GUID_COMPARTMENT_KEYBOARD_OPENCLOSE, TF_MOD_CONTROL,
                TF_MOD_ON_KEYUP, TF_MOD_SHIFT, TF_PRESERVEDKEY,
            },
        },
    },
};

pub struct PreservedKeys {
    pub keys: [PreservedKeyExtended; 3],
}

pub struct PreservedKeyExtended {
    pub key: TF_PRESERVEDKEY,
    pub key_guid: GUID,
    pub compartment_guid: GUID,
    pub desc: &'static str,
}

impl PreservedKeys {
    pub fn new() -> PreservedKeys {
        PreservedKeys {
            keys: [
                PreservedKeyExtended {
                    key: TF_PRESERVEDKEY {
                        uVKey: VK_SHIFT.0 as u32,
                        uModifiers: TF_MOD_ON_KEYUP,
                    },
                    key_guid: SAMPLEIME_GUID_IME_MODE_PRESERVE_KEY,
                    compartment_guid: GUID_COMPARTMENT_KEYBOARD_OPENCLOSE,
                    desc: "Chinese/English input (Shift)",
                },
                PreservedKeyExtended {
                    key: TF_PRESERVEDKEY {
                        uVKey: VK_SPACE.0 as u32,
                        uModifiers: TF_MOD_SHIFT,
                    },
                    key_guid: SAMPLEIME_GUID_DOUBLE_SINGLE_BYTE_PRESERVE_KEY,
                    compartment_guid: SAMPLEIME_GUID_COMPARTMENT_DOUBLE_SINGLE_BYTE,
                    desc: "Double/Single byte (Shift+Space)",
                },
                PreservedKeyExtended {
                    key: TF_PRESERVEDKEY {
                        uVKey: VK_OEM_PERIOD.0 as u32,
                        uModifiers: TF_MOD_CONTROL,
                    },
                    key_guid: SAMPLEIME_GUID_PUNCTUATION_PRESERVE_KEY,
                    compartment_guid: SAMPLEIME_GUID_COMPARTMENT_PUNCTUATION,
                    desc: "Chinese/English punctuation (Ctrl+.)",
                },
            ],
        }
    }

    pub fn init_keys(&self, thread_mgr: ITfThreadMgr, client_id: u32) -> windows::core::Result<()> {
        // This function is ignoring the failures to follow the original Microsoft demo behavior.
        // It's also probably better to make it partially work than to not work at all.

        let mut keystroke_mgr: Option<ITfKeystrokeMgr> = None;
        unsafe {
            thread_mgr.query(
                &ITfKeystrokeMgr::IID,
                core::mem::transmute(&mut keystroke_mgr),
            )
        }
        .ok()?;

        let keystroke_mgr = keystroke_mgr.unwrap();

        for preserved in &self.keys {
            Self::init_key(preserved, &keystroke_mgr, client_id)?;
        }

        Ok(())
    }

    fn init_key(
        preserved: &PreservedKeyExtended,
        keystroke_mgr: &ITfKeystrokeMgr,
        client_id: u32,
    ) -> windows::core::Result<()> {
        debug_assert!(preserved.key_guid != GUID::zeroed());

        let mut desc: Vec<u16> = preserved.desc.encode_utf16().collect();

        unsafe {
            keystroke_mgr.PreserveKey(
                client_id,
                &preserved.key_guid,
                &preserved.key,
                PWSTR(desc.as_mut_ptr()),
                desc.len() as u32,
            )
        }?;

        Ok(())
    }
}
