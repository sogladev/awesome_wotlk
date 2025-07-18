use anyhow::{Context, Result};
use goblin::pe::PE;
use std::fs;
use std::path::Path;

use crate::patches::Patch;

/// PE file patcher that can apply binary patches to Windows executables
pub struct PePatcher {
    file_path: std::path::PathBuf,
    file_data: Vec<u8>,
}

impl PePatcher {
    /// Create a new PE patcher for the given executable
    pub fn new(path: &Path) -> Result<Self> {
        let file_data =
            fs::read(path).with_context(|| format!("Failed to read file: {}", path.display()))?;

        // Validate that it's a PE file by trying to parse it
        PE::parse(&file_data)
            .context("Failed to parse PE file - not a valid Windows executable")?;

        Ok(PePatcher {
            file_path: path.to_path_buf(),
            file_data,
        })
    }

    /// Apply a single patch to the PE file
    pub fn apply_patch(&mut self, patch: &Patch) -> Result<()> {
        let bytes = patch
            .to_bytes()
            .map_err(|e| anyhow::anyhow!("Invalid patch bytes: {}", e))?;

        // Convert virtual address to file offset
        let file_offset = self
            .virtual_address_to_file_offset(patch.virtual_address)
            .with_context(|| {
                format!(
                    "Failed to convert virtual address 0x{:08X} to file offset",
                    patch.virtual_address
                )
            })?;

        // Validate bounds
        if file_offset + bytes.len() > self.file_data.len() {
            anyhow::bail!(
                "Patch extends beyond file bounds: offset={}, size={}, file_size={}",
                file_offset,
                bytes.len(),
                self.file_data.len()
            );
        }

        // Apply patch
        self.file_data[file_offset..file_offset + bytes.len()].copy_from_slice(&bytes);

        Ok(())
    }

    /// Save the patched file back to disk
    pub fn save(&self) -> Result<()> {
        fs::write(&self.file_path, &self.file_data).with_context(|| {
            format!("Failed to write patched file: {}", self.file_path.display())
        })?;

        Ok(())
    }

    /// Convert a virtual address to file offset using PE section information
    fn virtual_address_to_file_offset(&self, virtual_address: u32) -> Result<usize> {
        // Parse PE on-demand to avoid lifetime issues
        let pe = PE::parse(&self.file_data).context("Failed to parse PE file")?;

        let image_base = pe
            .header
            .optional_header
            .ok_or_else(|| anyhow::anyhow!("PE file has no optional header"))?
            .windows_fields
            .image_base as u32;

        // Calculate RVA (Relative Virtual Address)
        if virtual_address < image_base {
            anyhow::bail!(
                "Virtual address 0x{:08X} is below image base 0x{:08X}",
                virtual_address,
                image_base
            );
        }

        let rva = virtual_address - image_base;

        // Find the section containing this RVA
        for section in &pe.sections {
            let section_start = section.virtual_address;
            let section_end = section_start + section.virtual_size;

            if rva >= section_start && rva < section_end {
                // Calculate file offset
                let offset_in_section = rva - section_start;
                let file_offset = section.pointer_to_raw_data + offset_in_section;

                return Ok(file_offset as usize);
            }
        }

        anyhow::bail!(
            "Virtual address 0x{:08X} (RVA 0x{:08X}) not found in any section",
            virtual_address,
            rva
        );
    }
}

#[cfg(test)]
mod tests {

    // Note: These tests would need a sample PE file to work properly
    // For now, they're commented out but show the testing approach

    /*
    #[test]
    fn test_pe_parsing() {
        // Would need a test PE file
        let temp_file = create_test_pe_file();
        let patcher = PePatcher::new(temp_file.path()).unwrap();
        // Test assertions here
    }
    */
}
