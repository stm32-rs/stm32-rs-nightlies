#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bcr1: BCR1,
    btr1: BTR1,
    bcr2: BCR2,
    btr2: BTR2,
    bcr3: BCR3,
    btr3: BTR3,
    bcr4: BCR4,
    btr4: BTR4,
    pcscntr: PCSCNTR,
    _reserved9: [u8; 0x5c],
    pcr: PCR,
    sr: SR,
    pmem: PMEM,
    patt: PATT,
    hpr: HPR,
    heccr: HECCR,
    _reserved15: [u8; 0x6c],
    bwtr1: BWTR1,
    _reserved16: [u8; 0x04],
    bwtr2: BWTR2,
    _reserved17: [u8; 0x04],
    bwtr3: BWTR3,
    _reserved18: [u8; 0x04],
    bwtr4: BWTR4,
    _reserved19: [u8; 0xe0],
    csqcr: CSQCR,
    csqcfgr1: CSQCFGR1,
    csqcfgr2: CSQCFGR2,
    csqcfgr3: CSQCFGR3,
    csqar1: CSQAR1,
    csqar2: CSQAR2,
    _reserved25: [u8; 0x08],
    csqier: CSQIER,
    csqisr: CSQISR,
    csqicr: CSQICR,
    _reserved28: [u8; 0x04],
    csqemsr: CSQEMSR,
    _reserved29: [u8; 0x1c],
    bchier: BCHIER,
    bchisr: BCHISR,
    bchicr: BCHICR,
    _reserved32: [u8; 0x04],
    bchpbr1: BCHPBR1,
    bchpbr2: BCHPBR2,
    bchpbr3: BCHPBR3,
    bchpbr4: BCHPBR4,
    _reserved36: [u8; 0x0c],
    bchdsr0: BCHDSR0,
    bchdsr1: BCHDSR1,
    bchdsr2: BCHDSR2,
    bchdsr3: BCHDSR3,
    bchdsr4: BCHDSR4,
    _reserved41: [u8; 0x015c],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    ///0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR1 {
        &self.btr1
    }
    ///0x08 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    ///0x0c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR2 {
        &self.btr2
    }
    ///0x10 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR3 {
        &self.bcr3
    }
    ///0x14 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR3 {
        &self.btr3
    }
    ///0x18 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR4 {
        &self.bcr4
    }
    ///0x1c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR4 {
        &self.btr4
    }
    ///0x20 - This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h
    #[inline(always)]
    pub const fn pcscntr(&self) -> &PCSCNTR {
        &self.pcscntr
    }
    ///0x80 - NAND Flash Programmable control register
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    ///0x84 - This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x88 - The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses.
    #[inline(always)]
    pub const fn pmem(&self) -> &PMEM {
        &self.pmem
    }
    ///0x8c - The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).
    #[inline(always)]
    pub const fn patt(&self) -> &PATT {
        &self.patt
    }
    ///0x90 - This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.
    #[inline(always)]
    pub const fn hpr(&self) -> &HPR {
        &self.hpr
    }
    ///0x94 - This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.
    #[inline(always)]
    pub const fn heccr(&self) -> &HECCR {
        &self.heccr
    }
    ///0x104 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR1 {
        &self.bwtr1
    }
    ///0x10c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR2 {
        &self.bwtr2
    }
    ///0x114 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR3 {
        &self.bwtr3
    }
    ///0x11c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR4 {
        &self.bwtr4
    }
    ///0x200 - FMC NAND Command Sequencer Control Register
    #[inline(always)]
    pub const fn csqcr(&self) -> &CSQCR {
        &self.csqcr
    }
    ///0x204 - FMC NAND Command Sequencer Configuration Register 1
    #[inline(always)]
    pub const fn csqcfgr1(&self) -> &CSQCFGR1 {
        &self.csqcfgr1
    }
    ///0x208 - This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .
    #[inline(always)]
    pub const fn csqcfgr2(&self) -> &CSQCFGR2 {
        &self.csqcfgr2
    }
    ///0x20c - FMC NAND sequencer configuration register 3
    #[inline(always)]
    pub const fn csqcfgr3(&self) -> &CSQCFGR3 {
        &self.csqcfgr3
    }
    ///0x210 - This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.
    #[inline(always)]
    pub const fn csqar1(&self) -> &CSQAR1 {
        &self.csqar1
    }
    ///0x214 - This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.
    #[inline(always)]
    pub const fn csqar2(&self) -> &CSQAR2 {
        &self.csqar2
    }
    ///0x220 - FMC NAND Command Sequencer Interrupt Enable Register
    #[inline(always)]
    pub const fn csqier(&self) -> &CSQIER {
        &self.csqier
    }
    ///0x224 - FMC NAND Command Sequencer Interrupt Status Register
    #[inline(always)]
    pub const fn csqisr(&self) -> &CSQISR {
        &self.csqisr
    }
    ///0x228 - FMC NAND Command Sequencer Interrupt Clear Register
    #[inline(always)]
    pub const fn csqicr(&self) -> &CSQICR {
        &self.csqicr
    }
    ///0x230 - This register holds a sector error mapping status when the whole transfer is complete.
    #[inline(always)]
    pub const fn csqemsr(&self) -> &CSQEMSR {
        &self.csqemsr
    }
    ///0x250 - FMC BCH Interrupt enable register
    #[inline(always)]
    pub const fn bchier(&self) -> &BCHIER {
        &self.bchier
    }
    ///0x254 - This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared.
    #[inline(always)]
    pub const fn bchisr(&self) -> &BCHISR {
        &self.bchisr
    }
    ///0x258 - FMC BCH Interrupt Clear Register
    #[inline(always)]
    pub const fn bchicr(&self) -> &BCHICR {
        &self.bchicr
    }
    ///0x260 - These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\[51:0\] are significant and for the BCH 8-bit BCHPB\[103:0\] are significant.
    #[inline(always)]
    pub const fn bchpbr1(&self) -> &BCHPBR1 {
        &self.bchpbr1
    }
    ///0x264 - FMC BCH Parity Bits Register 2
    #[inline(always)]
    pub const fn bchpbr2(&self) -> &BCHPBR2 {
        &self.bchpbr2
    }
    ///0x268 - FMC BCH Parity Bits Register 3
    #[inline(always)]
    pub const fn bchpbr3(&self) -> &BCHPBR3 {
        &self.bchpbr3
    }
    ///0x26c - FMC BCH Parity Bits Register 4
    #[inline(always)]
    pub const fn bchpbr4(&self) -> &BCHPBR4 {
        &self.bchpbr4
    }
    ///0x27c - This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. .
    #[inline(always)]
    pub const fn bchdsr0(&self) -> &BCHDSR0 {
        &self.bchdsr0
    }
    ///0x280 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors
    #[inline(always)]
    pub const fn bchdsr1(&self) -> &BCHDSR1 {
        &self.bchdsr1
    }
    ///0x284 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively.
    #[inline(always)]
    pub const fn bchdsr2(&self) -> &BCHDSR2 {
        &self.bchdsr2
    }
    ///0x288 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.
    #[inline(always)]
    pub const fn bchdsr3(&self) -> &BCHDSR3 {
        &self.bchdsr3
    }
    ///0x28c - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .
    #[inline(always)]
    pub const fn bchdsr4(&self) -> &BCHDSR4 {
        &self.bchdsr4
    }
    ///0x3ec - FMC Hardware configuration register 2
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3f0 - FMC Hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f4 - FMC Version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - FMC Identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - FMC Size Identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**BCR1 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
pub mod bcr1;
/**BTR1 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).

You can [`read`](crate::Reg::read) this register and get [`btr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BTR1)

For information about available fields see [`mod@btr1`] module*/
pub type BTR1 = crate::Reg<btr1::BTR1rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
pub mod btr1;
/**BCR2 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCR2)

For information about available fields see [`mod@bcr2`] module*/
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
pub mod bcr2;
/**BTR2 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).

You can [`read`](crate::Reg::read) this register and get [`btr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BTR2)

For information about available fields see [`mod@btr2`] module*/
pub type BTR2 = crate::Reg<btr2::BTR2rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
pub mod btr2;
/**BCR3 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.

You can [`read`](crate::Reg::read) this register and get [`bcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCR3)

For information about available fields see [`mod@bcr3`] module*/
pub type BCR3 = crate::Reg<bcr3::BCR3rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
pub mod bcr3;
/**BTR3 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).

You can [`read`](crate::Reg::read) this register and get [`btr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BTR3)

For information about available fields see [`mod@btr3`] module*/
pub type BTR3 = crate::Reg<btr3::BTR3rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
pub mod btr3;
/**BCR4 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.

You can [`read`](crate::Reg::read) this register and get [`bcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCR4)

For information about available fields see [`mod@bcr4`] module*/
pub type BCR4 = crate::Reg<bcr4::BCR4rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.
pub mod bcr4;
/**BTR4 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).

You can [`read`](crate::Reg::read) this register and get [`btr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BTR4)

For information about available fields see [`mod@btr4`] module*/
pub type BTR4 = crate::Reg<btr4::BTR4rs>;
///This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
pub mod btr4;
/**PCSCNTR (rw) register accessor: This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h

You can [`read`](crate::Reg::read) this register and get [`pcscntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcscntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:PCSCNTR)

For information about available fields see [`mod@pcscntr`] module*/
pub type PCSCNTR = crate::Reg<pcscntr::PCSCNTRrs>;
///This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h
pub mod pcscntr;
/**PCR (rw) register accessor: NAND Flash Programmable control register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:PCR)

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///NAND Flash Programmable control register
pub mod pcr;
/**SR (r) register accessor: This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.
pub mod sr;
/**PMEM (rw) register accessor: The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses.

You can [`read`](crate::Reg::read) this register and get [`pmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:PMEM)

For information about available fields see [`mod@pmem`] module*/
pub type PMEM = crate::Reg<pmem::PMEMrs>;
///The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses.
pub mod pmem;
/**PATT (rw) register accessor: The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).

You can [`read`](crate::Reg::read) this register and get [`patt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:PATT)

For information about available fields see [`mod@patt`] module*/
pub type PATT = crate::Reg<patt::PATTrs>;
///The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).
pub mod patt;
/**HPR (r) register accessor: This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.

You can [`read`](crate::Reg::read) this register and get [`hpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:HPR)

For information about available fields see [`mod@hpr`] module*/
pub type HPR = crate::Reg<hpr::HPRrs>;
///This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.
pub mod hpr;
/**HECCR (r) register accessor: This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.

You can [`read`](crate::Reg::read) this register and get [`heccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:HECCR)

For information about available fields see [`mod@heccr`] module*/
pub type HECCR = crate::Reg<heccr::HECCRrs>;
///This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.
pub mod heccr;
/**BWTR1 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.

You can [`read`](crate::Reg::read) this register and get [`bwtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BWTR1)

For information about available fields see [`mod@bwtr1`] module*/
pub type BWTR1 = crate::Reg<bwtr1::BWTR1rs>;
///This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
pub mod bwtr1;
/**BWTR2 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.

You can [`read`](crate::Reg::read) this register and get [`bwtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BWTR2)

For information about available fields see [`mod@bwtr2`] module*/
pub type BWTR2 = crate::Reg<bwtr2::BWTR2rs>;
///This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
pub mod bwtr2;
/**BWTR3 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.

You can [`read`](crate::Reg::read) this register and get [`bwtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BWTR3)

For information about available fields see [`mod@bwtr3`] module*/
pub type BWTR3 = crate::Reg<bwtr3::BWTR3rs>;
///This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
pub mod bwtr3;
/**BWTR4 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.

You can [`read`](crate::Reg::read) this register and get [`bwtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BWTR4)

For information about available fields see [`mod@bwtr4`] module*/
pub type BWTR4 = crate::Reg<bwtr4::BWTR4rs>;
///This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.
pub mod bwtr4;
/**CSQCR (w) register accessor: FMC NAND Command Sequencer Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQCR)

For information about available fields see [`mod@csqcr`] module*/
pub type CSQCR = crate::Reg<csqcr::CSQCRrs>;
///FMC NAND Command Sequencer Control Register
pub mod csqcr;
/**CSQCFGR1 (rw) register accessor: FMC NAND Command Sequencer Configuration Register 1

You can [`read`](crate::Reg::read) this register and get [`csqcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQCFGR1)

For information about available fields see [`mod@csqcfgr1`] module*/
pub type CSQCFGR1 = crate::Reg<csqcfgr1::CSQCFGR1rs>;
///FMC NAND Command Sequencer Configuration Register 1
pub mod csqcfgr1;
/**CSQCFGR2 (rw) register accessor: This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .

You can [`read`](crate::Reg::read) this register and get [`csqcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQCFGR2)

For information about available fields see [`mod@csqcfgr2`] module*/
pub type CSQCFGR2 = crate::Reg<csqcfgr2::CSQCFGR2rs>;
///This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .
pub mod csqcfgr2;
/**CSQCFGR3 (rw) register accessor: FMC NAND sequencer configuration register 3

You can [`read`](crate::Reg::read) this register and get [`csqcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQCFGR3)

For information about available fields see [`mod@csqcfgr3`] module*/
pub type CSQCFGR3 = crate::Reg<csqcfgr3::CSQCFGR3rs>;
///FMC NAND sequencer configuration register 3
pub mod csqcfgr3;
/**CSQAR1 (rw) register accessor: This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.

You can [`read`](crate::Reg::read) this register and get [`csqar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQAR1)

For information about available fields see [`mod@csqar1`] module*/
pub type CSQAR1 = crate::Reg<csqar1::CSQAR1rs>;
///This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.
pub mod csqar1;
/**CSQAR2 (rw) register accessor: This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.

You can [`read`](crate::Reg::read) this register and get [`csqar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQAR2)

For information about available fields see [`mod@csqar2`] module*/
pub type CSQAR2 = crate::Reg<csqar2::CSQAR2rs>;
///This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.
pub mod csqar2;
/**CSQIER (rw) register accessor: FMC NAND Command Sequencer Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`csqier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQIER)

For information about available fields see [`mod@csqier`] module*/
pub type CSQIER = crate::Reg<csqier::CSQIERrs>;
///FMC NAND Command Sequencer Interrupt Enable Register
pub mod csqier;
/**CSQISR (rw) register accessor: FMC NAND Command Sequencer Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`csqisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQISR)

For information about available fields see [`mod@csqisr`] module*/
pub type CSQISR = crate::Reg<csqisr::CSQISRrs>;
///FMC NAND Command Sequencer Interrupt Status Register
pub mod csqisr;
/**CSQICR (w) register accessor: FMC NAND Command Sequencer Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQICR)

For information about available fields see [`mod@csqicr`] module*/
pub type CSQICR = crate::Reg<csqicr::CSQICRrs>;
///FMC NAND Command Sequencer Interrupt Clear Register
pub mod csqicr;
/**CSQEMSR (r) register accessor: This register holds a sector error mapping status when the whole transfer is complete.

You can [`read`](crate::Reg::read) this register and get [`csqemsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQEMSR)

For information about available fields see [`mod@csqemsr`] module*/
pub type CSQEMSR = crate::Reg<csqemsr::CSQEMSRrs>;
///This register holds a sector error mapping status when the whole transfer is complete.
pub mod csqemsr;
/**BCHIER (rw) register accessor: FMC BCH Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`bchier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bchier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHIER)

For information about available fields see [`mod@bchier`] module*/
pub type BCHIER = crate::Reg<bchier::BCHIERrs>;
///FMC BCH Interrupt enable register
pub mod bchier;
/**BCHISR (r) register accessor: This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared.

You can [`read`](crate::Reg::read) this register and get [`bchisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHISR)

For information about available fields see [`mod@bchisr`] module*/
pub type BCHISR = crate::Reg<bchisr::BCHISRrs>;
///This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared.
pub mod bchisr;
/**BCHICR (w) register accessor: FMC BCH Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bchicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHICR)

For information about available fields see [`mod@bchicr`] module*/
pub type BCHICR = crate::Reg<bchicr::BCHICRrs>;
///FMC BCH Interrupt Clear Register
pub mod bchicr;
/**BCHPBR1 (r) register accessor: These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\[51:0\] are significant and for the BCH 8-bit BCHPB\[103:0\] are significant.

You can [`read`](crate::Reg::read) this register and get [`bchpbr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHPBR1)

For information about available fields see [`mod@bchpbr1`] module*/
pub type BCHPBR1 = crate::Reg<bchpbr1::BCHPBR1rs>;
///These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\[51:0\] are significant and for the BCH 8-bit BCHPB\[103:0\] are significant.
pub mod bchpbr1;
/**BCHPBR2 (r) register accessor: FMC BCH Parity Bits Register 2

You can [`read`](crate::Reg::read) this register and get [`bchpbr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHPBR2)

For information about available fields see [`mod@bchpbr2`] module*/
pub type BCHPBR2 = crate::Reg<bchpbr2::BCHPBR2rs>;
///FMC BCH Parity Bits Register 2
pub mod bchpbr2;
/**BCHPBR3 (r) register accessor: FMC BCH Parity Bits Register 3

You can [`read`](crate::Reg::read) this register and get [`bchpbr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHPBR3)

For information about available fields see [`mod@bchpbr3`] module*/
pub type BCHPBR3 = crate::Reg<bchpbr3::BCHPBR3rs>;
///FMC BCH Parity Bits Register 3
pub mod bchpbr3;
/**BCHPBR4 (r) register accessor: FMC BCH Parity Bits Register 4

You can [`read`](crate::Reg::read) this register and get [`bchpbr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHPBR4)

For information about available fields see [`mod@bchpbr4`] module*/
pub type BCHPBR4 = crate::Reg<bchpbr4::BCHPBR4rs>;
///FMC BCH Parity Bits Register 4
pub mod bchpbr4;
/**BCHDSR0 (r) register accessor: This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. .

You can [`read`](crate::Reg::read) this register and get [`bchdsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHDSR0)

For information about available fields see [`mod@bchdsr0`] module*/
pub type BCHDSR0 = crate::Reg<bchdsr0::BCHDSR0rs>;
///This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. .
pub mod bchdsr0;
/**BCHDSR1 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors

You can [`read`](crate::Reg::read) this register and get [`bchdsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHDSR1)

For information about available fields see [`mod@bchdsr1`] module*/
pub type BCHDSR1 = crate::Reg<bchdsr1::BCHDSR1rs>;
///The maximum error correction capability of the BCH block embedded in the FMC is 8 errors
pub mod bchdsr1;
/**BCHDSR2 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively.

You can [`read`](crate::Reg::read) this register and get [`bchdsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHDSR2)

For information about available fields see [`mod@bchdsr2`] module*/
pub type BCHDSR2 = crate::Reg<bchdsr2::BCHDSR2rs>;
///The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively.
pub mod bchdsr2;
/**BCHDSR3 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.

You can [`read`](crate::Reg::read) this register and get [`bchdsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHDSR3)

For information about available fields see [`mod@bchdsr3`] module*/
pub type BCHDSR3 = crate::Reg<bchdsr3::BCHDSR3rs>;
///The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.
pub mod bchdsr3;
/**BCHDSR4 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .

You can [`read`](crate::Reg::read) this register and get [`bchdsr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHDSR4)

For information about available fields see [`mod@bchdsr4`] module*/
pub type BCHDSR4 = crate::Reg<bchdsr4::BCHDSR4rs>;
///The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .
pub mod bchdsr4;
/**HWCFGR2 (r) register accessor: FMC Hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///FMC Hardware configuration register 2
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: FMC Hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///FMC Hardware configuration register 1
pub mod hwcfgr1;
/**VERR (r) register accessor: FMC Version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///FMC Version register
pub mod verr;
/**IPIDR (r) register accessor: FMC Identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///FMC Identification register
pub mod ipidr;
/**SIDR (r) register accessor: FMC Size Identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///FMC Size Identification register
pub mod sidr;
