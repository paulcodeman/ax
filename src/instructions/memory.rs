use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{debug_log, instructions::registers::SupportedRegister};

use super::{axecutor::Axecutor, errors::AxError};

use std::convert::TryInto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MemoryArea {
    name: Option<String>,
    start: u64,
    length: u64,
    data: Vec<u8>,
}

impl MemoryArea {
    pub fn to_string_ident(&self, i: usize) -> String {
        let mut s = String::new();

        s.push_str(&format!("MemoryArea {{\n"));
        s.push_str(&format!(
            "{}    name: {:?},\n",
            " ".repeat(i * 4),
            self.name
        ));
        s.push_str(&format!(
            "{}    start: {:#x},\n",
            " ".repeat(i * 4),
            self.start
        ));
        s.push_str(&format!(
            "{}    length: {},\n",
            " ".repeat(i * 4),
            self.length
        ));
        s.push_str(&format!("{}    data: [", " ".repeat(i * 4)));

        if self.data.len() > 255 {
            s.push_str("<too long to display>");
        } else {
            for (i, byte) in self.data.iter().enumerate() {
                s.push_str(&format!("0x{:02x}", byte));

                if i != self.data.len() - 1 {
                    s.push_str(", ");
                }
            }
        }

        s.push_str("],\n");
        s.push_str(&format!("{}}}", " ".repeat(i * 4)));

        s
    }
}

#[wasm_bindgen]
impl Axecutor {
    // TODO: Currently cannot read consecutive sections of memory
    #[must_use]
    pub fn mem_read_bytes(&self, address: u64, length: u64) -> Result<Vec<u8>, AxError> {
        debug_log!(
            "Calling Axecutor::mem_read_bytes, address={:#x}, length={}",
            address,
            length
        );

        let mut result = Vec::new();

        for area in &self.state.memory {
            if address >= area.start && address + length <= area.start + area.length {
                let offset = (address - area.start) as usize;
                result.extend_from_slice(&area.data[offset..offset + length as usize]);

                if result.len() <= 100 {
                    debug_log!(
                        "Read from memory area{}, start={:#x}, length={}, read={:?}",
                        match &area.name {
                            Some(name) => format!(" {}", name),
                            None => String::new(),
                        },
                        area.start,
                        area.length,
                        result
                    );
                } else {
                    // Only log the first 50 and last 50 bytes of the memory area, with "<too much data to display>" in the middle
                    debug_log!(
                        "Read from memory area{}, start={:#x}, length={}, read=[{:?}, <too much data to display>, {:?}]",
                        match &area.name {
                            Some(name) => format!(" {}", name),
                            None => String::new(),
                        },
                        area.start,
                        area.length,
                        &result[0..50],
                        &result[result.len() - 50..]
                    );
                }

                return Ok(result);
            }
        }

        Err(AxError::from(format!(
            "Could not read memory at address {:#x}",
            address
        )))
    }

    pub fn mem_read_64(&self, address: u64) -> Result<u64, AxError> {
        let bytes = self.mem_read_bytes(address, 8)?;

        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn mem_read_32(&self, address: u64) -> Result<u32, AxError> {
        let bytes = self.mem_read_bytes(address, 4)?;

        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn mem_read_16(&self, address: u64) -> Result<u16, AxError> {
        let bytes = self.mem_read_bytes(address, 2)?;

        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn mem_read_8(&self, address: u64) -> Result<u8, AxError> {
        let bytes = self.mem_read_bytes(address, 1)?;

        Ok(bytes[0])
    }

    // TODO: Currently cannot write consecutive sections of memory
    // It would also make sense to give better error messages, e.g. if the write start address is within an area, but the data is too long
    #[must_use]
    pub fn mem_write_bytes(&mut self, address: u64, data: &[u8]) -> Result<(), AxError> {
        debug_log!(
            "Calling Axecutor::mem_write_bytes, address={:#x}, data={:?}",
            address,
            data
        );

        for area in &mut self.state.memory {
            if address >= area.start && address + data.len() as u64 <= area.start + area.length {
                let offset = (address - area.start) as usize;
                area.data[offset..offset + data.len()].copy_from_slice(data);

                #[cfg(debug_assertions)]
                if data.len() <= 100 {
                    debug_log!(
                        "Wrote to memory area, start={:#x}, length={}, wrote={:?}{}",
                        area.start,
                        area.length,
                        data,
                        match data.len() {
                            1 => format!(", formatted=0x{:02x}", data[0]),
                            2 => format!(
                                ", formatted=0x{:04x}",
                                u16::from_le_bytes(data.try_into().unwrap())
                            ),
                            4 => format!(
                                ", formatted=0x{:08x}",
                                u32::from_le_bytes(data.try_into().unwrap())
                            ),
                            8 => format!(
                                ", formatted=0x{:016x}",
                                u64::from_le_bytes(data.try_into().unwrap())
                            ),
                            _ => "".to_string(),
                        }
                    );
                } else {
                    // Only log the first 50 and last 50 bytes of data, with "<too much data to display>" in the middle
                    debug_log!(
                        "Wrote to memory area, start={:#x}, length={}, wrote=[{:?}, <too much data to display>, {:?}]",
                        area.start,
                        area.length,
                        &data[0..50],
                        &data[data.len() - 50..]
                    );
                }

                return Ok(());
            }
        }

        Err(AxError::from(format!(
            "Could not write memory at address {:#x}",
            address
        )))
    }

    pub fn mem_write_64(&mut self, address: u64, data: u64) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    pub fn mem_write_32(&mut self, address: u64, data: u32) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    pub fn mem_write_16(&mut self, address: u64, data: u16) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    pub fn mem_write_8(&mut self, address: u64, data: u8) -> Result<(), AxError> {
        self.mem_write_bytes(address, &[data])
    }

    #[must_use]
    pub fn mem_init_area_named(
        &mut self,
        start: u64,
        data: Vec<u8>,
        name: Option<String>,
    ) -> Result<(), AxError> {
        // Make sure there's no overlapping area already defined, including code region
        if start >= self.code_start_address && start < self.code_start_address + self.code_length {
            return Err(AxError::from(format!(
                "Cannot initialize memory area at {:#x} (len={}), as it overlaps with the code section starting at {:#x} (len={})",
                start, data.len(), self.code_start_address, self.code_length
            )));
        }

        for area in &self.state.memory {
            if start >= area.start && start < area.start + area.length {
                return Err(AxError::from(format!(
                    "cannot create memory area with start={:#x}, length={:#x}: overlaps with area with start={:#x}, length={:#x}",
                    start, data.len(), area.start, area.length
                )));
            }
        }

        #[cfg(debug_assertions)]
        let display_name = match &name {
            Some(n) => format!(" {}", n),
            None => "".to_string(),
        };

        let len = data.len() as u64;
        self.state.memory.push(MemoryArea {
            start,
            length: len,
            data,
            name,
        });

        debug_log!(
            "Initialized memory area{}, start={:#x}, length={:#x}",
            display_name,
            start,
            len
        );

        Ok(())
    }

    pub fn mem_init_area(&mut self, start: u64, data: Vec<u8>) -> Result<(), AxError> {
        self.mem_init_area_named(start, data, None)
    }
    pub fn mem_init_zero(&mut self, start: u64, length: u64) -> Result<(), AxError> {
        self.mem_init_area_named(start, vec![0; length as usize], None)
    }

    pub fn mem_init_zero_named(
        &mut self,
        start: u64,
        length: u64,
        name: String,
    ) -> Result<(), AxError> {
        self.mem_init_area_named(start, vec![0; length as usize], Some(name))
    }

    pub fn init_stack(&mut self, length: u64) -> Result<(), AxError> {
        let mut stack_start: u64 = 0x1000;

        loop {
            if stack_start >= 0x7fff_ffff_ffff_ffff {
                return Err(AxError::from(
                    "Could not find a suitable stack start address",
                ));
            }

            if self
                .mem_init_zero_named(stack_start, length, "Stack".to_string())
                .is_ok()
            {
                break;
            }
            stack_start <<= 1;
        }

        let initial_rsp = stack_start + length - 8;
        self.reg_write_64(SupportedRegister::RSP, initial_rsp);
        self.stack_top = stack_start + length;

        Ok(())
    }
}
