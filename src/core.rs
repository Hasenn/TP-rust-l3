use crate::instruction::{Instruction, OpCode};
use crate::{Error, MAX_REGISTER_INDEX, MEMORY_SIZE};

pub struct Core {
    registers: [u16; MAX_REGISTER_INDEX as usize + 1]
}

impl Core {
    /// Instantiate our core
    pub fn new() -> Self {
        todo!()
    }

    /// Get the value from the register at `index`
    pub fn register(&self, index: u8) -> Result<u16, Error> {
        todo!()
    }
    /// Print the current state of this core
    pub fn dump(&self, preamble: &str) {
        println!("do-core1: {}:", preamble);
        for (index, register) in self.registers.iter().enumerate() {
            println!("\tR{}: {:#x?}", index, *register);
        }
    }
    /// Decode an instruction encoded as a u16 into an [Instruction]
    pub fn decode(&mut self, insn: u16) -> Result<Instruction, Error> {
        unimplemented!()
    }
    /// Run an instruction and consequently change the core's state
    pub fn execute(&mut self, insn: Instruction) -> Result<(), Error> {
        unimplemented!()
    }
    // Implementation of the add instruction
    fn add(&mut self, insn: Instruction) -> Result<(), Error> {
        todo!()
    }
    // Implementation of the xor instruction
    fn xor(&mut self, insn: Instruction) -> Result<(), Error> {
        unimplemented!()
    }
    // Implementation of the load instruction
    fn load(&mut self, insn: Instruction) -> Result<(), Error> {
        unimplemented!()
    }
    // Implementation of the store instruction
    fn store(&mut self, insn: Instruction) -> Result<(), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Core;
    use crate::Error;

    #[test]
    fn test_add_all() -> Result<(), Error> {
        for i in 0..8 {
            for j in 0..8 {
                let insn : u16 = 0x200 | i << 4 | j;
                println!("0x{:x}", insn);
                let mut cpu = Core::new();

                let rj = cpu.register(j as u8)?;
                let ri = cpu.register(i as u8)?;

                let decoded_insn = cpu.decode(insn)?;
                cpu.execute(decoded_insn)?;

                let new_ri = cpu.register(i as u8)?;
                let new_rj = cpu.register(j as u8)?;

                assert_eq!(new_ri, ri + rj, "The ADD i j instruction sets Ri to (*Ri + *Rj)");
                if i != j {
                    assert_eq!(new_rj, rj, "The ADD i j instruction doesn't change Rj for i != j");
                }
            }
        }
        Ok(())
    }
    #[test]
    fn test_xor_all() -> Result<(), Error> {
        for i in 0..8 {
            for j in 0..8 {
                let insn : u16 = 0x300 | i << 4 | j;
                //println!("0x{:x}", insn);
                let mut cpu = Core::new();

                let rj = cpu.register(j as u8)?;
                let ri = cpu.register(i as u8)?;

                let decoded_insn = cpu.decode(insn)?;
                cpu.execute(decoded_insn)?;

                let new_ri = cpu.register(i as u8)?;
                let new_rj = cpu.register(j as u8)?;

                assert_eq!(new_ri, ri ^ rj, "The XOR i j instruction sets Ri to (*Ri ^ *Rj)");
                if i != j {
                    assert_eq!(new_rj, rj, "The XOR i j instruction doesn't change Rj for i != j");
                }
            }
        }
        Ok(())
    }
}
