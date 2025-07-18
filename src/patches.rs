/// Patch definitions for World of Warcraft: Wrath of the Lich King
///
/// These patches modify specific virtual addresses in the WoW executable
/// to inject the AwesomeWotlk library functionality.

#[derive(Debug, Clone)]
pub struct Patch {
    pub virtual_address: u32,
    pub hex_bytes: &'static str,
    pub description: &'static str,
}

impl Patch {
    /// Convert hex string to byte vector
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let hex = self.hex_bytes.replace(" ", ""); // Remove any spaces

        if hex.len() % 2 != 0 {
            return Err(format!("Invalid hex string length: {}", hex.len()));
        }

        let mut bytes = Vec::with_capacity(hex.len() / 2);

        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            match u8::from_str_radix(byte_str, 16) {
                Ok(byte) => bytes.push(byte),
                Err(_) => return Err(format!("Invalid hex byte: {byte_str}")),
            }
        }

        Ok(bytes)
    }
}

/// All patches to be applied to the WoW executable
pub const PATCHES: &[Patch] = &[
    Patch {
        virtual_address: 0x004DCCF0,
        hex_bytes: "B800000000C3", // mov eax, 0; ret
        description: "lua_ScanDllStart hook",
    },
    Patch {
        virtual_address: 0x004E5CB0,
        hex_bytes: concat!(
            "B801000000",                               // mov eax, 1
            "A374B4B600",                               // mov s_isScanDllFinished, eax
            "68E05C4E00",                               // push AwesomeWotlkLib.dll
            "E81C683800",                               // call _loadddll
            "83C404",                                   // add esp, 4
            "55",                                       // push ebp
            "8BEC",                                     // mov ebp, esp
            "E8A110F2FF",                               // call 0x00406D70
            "E9045BF2FF",                               // jmp 0x0040B7D8
            "CCCCCCCCCCCCCCCCCCCCCCCC",                 // int3 (12 times)
            "417765736F6D65576F746C6B4C69622E646C6C00"  // "AwesomeWotlkLib.dll\0"
        ),
        description: "ScanDllStart injection point",
    },
    Patch {
        virtual_address: 0x0040B7D0,
        hex_bytes: "E9DBA40D00909090", // jmp 0x004E5CB0; nop; nop; nop
        description: "StartAddress hook",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let patch = Patch {
            virtual_address: 0x12345678,
            hex_bytes: "B800000000C3",
            description: "test patch",
        };

        let bytes = patch.to_bytes().unwrap();
        assert_eq!(bytes, vec![0xB8, 0x00, 0x00, 0x00, 0x00, 0xC3]);
    }

    #[test]
    fn test_all_patches_valid() {
        for patch in PATCHES {
            assert!(
                patch.to_bytes().is_ok(),
                "Invalid patch: {}",
                patch.description
            );
        }
    }
}
