#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fmc_bcr1: FMC_BCR1,
    fmc_btr1: FMC_BTR1,
    fmc_bcr2: FMC_BCR2,
    fmc_btr2: FMC_BTR2,
    fmc_bcr3: FMC_BCR3,
    fmc_btr3: FMC_BTR3,
    fmc_bcr4: FMC_BCR4,
    fmc_btr4: FMC_BTR4,
    fmc_pcscntr: FMC_PCSCNTR,
    _reserved9: [u8; 0x5c],
    fmc_pcr: FMC_PCR,
    fmc_sr: FMC_SR,
    fmc_pmem: FMC_PMEM,
    fmc_patt: FMC_PATT,
    fmc_hpr: FMC_HPR,
    fmc_heccr: FMC_HECCR,
    _reserved15: [u8; 0x6c],
    fmc_bwtr1: FMC_BWTR1,
    _reserved16: [u8; 0x04],
    fmc_bwtr2: FMC_BWTR2,
    _reserved17: [u8; 0x04],
    fmc_bwtr3: FMC_BWTR3,
    _reserved18: [u8; 0x04],
    fmc_bwtr4: FMC_BWTR4,
    _reserved19: [u8; 0xe0],
    fmc_csqcr: FMC_CSQCR,
    fmc_csqcfgr1: FMC_CSQCFGR1,
    fmc_csqcfgr2: FMC_CSQCFGR2,
    fmc_csqcfgr3: FMC_CSQCFGR3,
    fmc_csqar1: FMC_CSQAR1,
    fmc_csqar2: FMC_CSQAR2,
    _reserved25: [u8; 0x08],
    fmc_csqier: FMC_CSQIER,
    fmc_csqisr: FMC_CSQISR,
    fmc_csqicr: FMC_CSQICR,
    _reserved28: [u8; 0x04],
    fmc_csqemsr: FMC_CSQEMSR,
    _reserved29: [u8; 0x1c],
    fmc_bchier: FMC_BCHIER,
    fmc_bchisr: FMC_BCHISR,
    fmc_bchicr: FMC_BCHICR,
    _reserved32: [u8; 0x04],
    fmc_bchpbr1: FMC_BCHPBR1,
    fmc_bchpbr2: FMC_BCHPBR2,
    fmc_bchpbr3: FMC_BCHPBR3,
    fmc_bchpbr4: FMC_BCHPBR4,
    _reserved36: [u8; 0x0c],
    fmc_bchdsr0: FMC_BCHDSR0,
    fmc_bchdsr1: FMC_BCHDSR1,
    fmc_bchdsr2: FMC_BCHDSR2,
    fmc_bchdsr3: FMC_BCHDSR3,
    fmc_bchdsr4: FMC_BCHDSR4,
    _reserved41: [u8; 0x015c],
    fmc_hwcfgr2: FMC_HWCFGR2,
    fmc_hwcfgr1: FMC_HWCFGR1,
    fmc_verr: FMC_VERR,
    fmc_ipidr: FMC_IPIDR,
    fmc_sidr: FMC_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    #[inline(always)]
    pub const fn fmc_bcr1(&self) -> &FMC_BCR1 {
        &self.fmc_bcr1
    }
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    #[inline(always)]
    pub const fn fmc_btr1(&self) -> &FMC_BTR1 {
        &self.fmc_btr1
    }
    #[doc = "0x08 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    #[inline(always)]
    pub const fn fmc_bcr2(&self) -> &FMC_BCR2 {
        &self.fmc_bcr2
    }
    #[doc = "0x0c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    #[inline(always)]
    pub const fn fmc_btr2(&self) -> &FMC_BTR2 {
        &self.fmc_btr2
    }
    #[doc = "0x10 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    #[inline(always)]
    pub const fn fmc_bcr3(&self) -> &FMC_BCR3 {
        &self.fmc_bcr3
    }
    #[doc = "0x14 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    #[inline(always)]
    pub const fn fmc_btr3(&self) -> &FMC_BTR3 {
        &self.fmc_btr3
    }
    #[doc = "0x18 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    #[inline(always)]
    pub const fn fmc_bcr4(&self) -> &FMC_BCR4 {
        &self.fmc_bcr4
    }
    #[doc = "0x1c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    #[inline(always)]
    pub const fn fmc_btr4(&self) -> &FMC_BTR4 {
        &self.fmc_btr4
    }
    #[doc = "0x20 - This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h"]
    #[inline(always)]
    pub const fn fmc_pcscntr(&self) -> &FMC_PCSCNTR {
        &self.fmc_pcscntr
    }
    #[doc = "0x80 - NAND Flash Programmable control register"]
    #[inline(always)]
    pub const fn fmc_pcr(&self) -> &FMC_PCR {
        &self.fmc_pcr
    }
    #[doc = "0x84 - This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits."]
    #[inline(always)]
    pub const fn fmc_sr(&self) -> &FMC_SR {
        &self.fmc_sr
    }
    #[doc = "0x88 - The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses."]
    #[inline(always)]
    pub const fn fmc_pmem(&self) -> &FMC_PMEM {
        &self.fmc_pmem
    }
    #[doc = "0x8c - The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function)."]
    #[inline(always)]
    pub const fn fmc_patt(&self) -> &FMC_PATT {
        &self.fmc_patt
    }
    #[doc = "0x90 - This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    #[inline(always)]
    pub const fn fmc_hpr(&self) -> &FMC_HPR {
        &self.fmc_hpr
    }
    #[doc = "0x94 - This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    #[inline(always)]
    pub const fn fmc_heccr(&self) -> &FMC_HECCR {
        &self.fmc_heccr
    }
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn fmc_bwtr1(&self) -> &FMC_BWTR1 {
        &self.fmc_bwtr1
    }
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn fmc_bwtr2(&self) -> &FMC_BWTR2 {
        &self.fmc_bwtr2
    }
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn fmc_bwtr3(&self) -> &FMC_BWTR3 {
        &self.fmc_bwtr3
    }
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn fmc_bwtr4(&self) -> &FMC_BWTR4 {
        &self.fmc_bwtr4
    }
    #[doc = "0x200 - FMC NAND Command Sequencer Control Register"]
    #[inline(always)]
    pub const fn fmc_csqcr(&self) -> &FMC_CSQCR {
        &self.fmc_csqcr
    }
    #[doc = "0x204 - FMC NAND Command Sequencer Configuration Register 1"]
    #[inline(always)]
    pub const fn fmc_csqcfgr1(&self) -> &FMC_CSQCFGR1 {
        &self.fmc_csqcfgr1
    }
    #[doc = "0x208 - This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. ."]
    #[inline(always)]
    pub const fn fmc_csqcfgr2(&self) -> &FMC_CSQCFGR2 {
        &self.fmc_csqcfgr2
    }
    #[doc = "0x20c - FMC NAND sequencer configuration register 3"]
    #[inline(always)]
    pub const fn fmc_csqcfgr3(&self) -> &FMC_CSQCFGR3 {
        &self.fmc_csqcfgr3
    }
    #[doc = "0x210 - This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer."]
    #[inline(always)]
    pub const fn fmc_csqar1(&self) -> &FMC_CSQAR1 {
        &self.fmc_csqar1
    }
    #[doc = "0x214 - This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable."]
    #[inline(always)]
    pub const fn fmc_csqar2(&self) -> &FMC_CSQAR2 {
        &self.fmc_csqar2
    }
    #[doc = "0x220 - FMC NAND Command Sequencer Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fmc_csqier(&self) -> &FMC_CSQIER {
        &self.fmc_csqier
    }
    #[doc = "0x224 - FMC NAND Command Sequencer Interrupt Status Register"]
    #[inline(always)]
    pub const fn fmc_csqisr(&self) -> &FMC_CSQISR {
        &self.fmc_csqisr
    }
    #[doc = "0x228 - FMC NAND Command Sequencer Interrupt Clear Register"]
    #[inline(always)]
    pub const fn fmc_csqicr(&self) -> &FMC_CSQICR {
        &self.fmc_csqicr
    }
    #[doc = "0x230 - This register holds a sector error mapping status when the whole transfer is complete."]
    #[inline(always)]
    pub const fn fmc_csqemsr(&self) -> &FMC_CSQEMSR {
        &self.fmc_csqemsr
    }
    #[doc = "0x250 - FMC BCH Interrupt enable register"]
    #[inline(always)]
    pub const fn fmc_bchier(&self) -> &FMC_BCHIER {
        &self.fmc_bchier
    }
    #[doc = "0x254 - This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared."]
    #[inline(always)]
    pub const fn fmc_bchisr(&self) -> &FMC_BCHISR {
        &self.fmc_bchisr
    }
    #[doc = "0x258 - FMC BCH Interrupt Clear Register"]
    #[inline(always)]
    pub const fn fmc_bchicr(&self) -> &FMC_BCHICR {
        &self.fmc_bchicr
    }
    #[doc = "0x260 - These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant."]
    #[inline(always)]
    pub const fn fmc_bchpbr1(&self) -> &FMC_BCHPBR1 {
        &self.fmc_bchpbr1
    }
    #[doc = "0x264 - FMC BCH Parity Bits Register 2"]
    #[inline(always)]
    pub const fn fmc_bchpbr2(&self) -> &FMC_BCHPBR2 {
        &self.fmc_bchpbr2
    }
    #[doc = "0x268 - FMC BCH Parity Bits Register 3"]
    #[inline(always)]
    pub const fn fmc_bchpbr3(&self) -> &FMC_BCHPBR3 {
        &self.fmc_bchpbr3
    }
    #[doc = "0x26c - FMC BCH Parity Bits Register 4"]
    #[inline(always)]
    pub const fn fmc_bchpbr4(&self) -> &FMC_BCHPBR4 {
        &self.fmc_bchpbr4
    }
    #[doc = "0x27c - This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. ."]
    #[inline(always)]
    pub const fn fmc_bchdsr0(&self) -> &FMC_BCHDSR0 {
        &self.fmc_bchdsr0
    }
    #[doc = "0x280 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors"]
    #[inline(always)]
    pub const fn fmc_bchdsr1(&self) -> &FMC_BCHDSR1 {
        &self.fmc_bchdsr1
    }
    #[doc = "0x284 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively."]
    #[inline(always)]
    pub const fn fmc_bchdsr2(&self) -> &FMC_BCHDSR2 {
        &self.fmc_bchdsr2
    }
    #[doc = "0x288 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors."]
    #[inline(always)]
    pub const fn fmc_bchdsr3(&self) -> &FMC_BCHDSR3 {
        &self.fmc_bchdsr3
    }
    #[doc = "0x28c - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. ."]
    #[inline(always)]
    pub const fn fmc_bchdsr4(&self) -> &FMC_BCHDSR4 {
        &self.fmc_bchdsr4
    }
    #[doc = "0x3ec - FMC Hardware configuration register 2"]
    #[inline(always)]
    pub const fn fmc_hwcfgr2(&self) -> &FMC_HWCFGR2 {
        &self.fmc_hwcfgr2
    }
    #[doc = "0x3f0 - FMC Hardware configuration register 1"]
    #[inline(always)]
    pub const fn fmc_hwcfgr1(&self) -> &FMC_HWCFGR1 {
        &self.fmc_hwcfgr1
    }
    #[doc = "0x3f4 - FMC Version register"]
    #[inline(always)]
    pub const fn fmc_verr(&self) -> &FMC_VERR {
        &self.fmc_verr
    }
    #[doc = "0x3f8 - FMC Identification register"]
    #[inline(always)]
    pub const fn fmc_ipidr(&self) -> &FMC_IPIDR {
        &self.fmc_ipidr
    }
    #[doc = "0x3fc - FMC Size Identification register"]
    #[inline(always)]
    pub const fn fmc_sidr(&self) -> &FMC_SIDR {
        &self.fmc_sidr
    }
}
#[doc = "FMC_BCR1 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bcr1`]
module"]
pub type FMC_BCR1 = crate::Reg<fmc_bcr1::FMC_BCR1rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr1;
#[doc = "FMC_BTR1 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_btr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_btr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_btr1`]
module"]
pub type FMC_BTR1 = crate::Reg<fmc_btr1::FMC_BTR1rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr1;
#[doc = "FMC_BCR2 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bcr2`]
module"]
pub type FMC_BCR2 = crate::Reg<fmc_bcr2::FMC_BCR2rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr2;
#[doc = "FMC_BTR2 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_btr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_btr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_btr2`]
module"]
pub type FMC_BTR2 = crate::Reg<fmc_btr2::FMC_BTR2rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr2;
#[doc = "FMC_BCR3 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bcr3`]
module"]
pub type FMC_BCR3 = crate::Reg<fmc_bcr3::FMC_BCR3rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr3;
#[doc = "FMC_BTR3 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_btr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_btr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_btr3`]
module"]
pub type FMC_BTR3 = crate::Reg<fmc_btr3::FMC_BTR3rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr3;
#[doc = "FMC_BCR4 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bcr4`]
module"]
pub type FMC_BCR4 = crate::Reg<fmc_bcr4::FMC_BCR4rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr4;
#[doc = "FMC_BTR4 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_btr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_btr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_btr4`]
module"]
pub type FMC_BTR4 = crate::Reg<fmc_btr4::FMC_BTR4rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr4;
#[doc = "FMC_PCSCNTR (rw) register accessor: This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_pcscntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_pcscntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_pcscntr`]
module"]
pub type FMC_PCSCNTR = crate::Reg<fmc_pcscntr::FMC_PCSCNTRrs>;
#[doc = "This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h"]
pub mod fmc_pcscntr;
#[doc = "FMC_PCR (rw) register accessor: NAND Flash Programmable control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_pcr`]
module"]
pub type FMC_PCR = crate::Reg<fmc_pcr::FMC_PCRrs>;
#[doc = "NAND Flash Programmable control register"]
pub mod fmc_pcr;
#[doc = "FMC_SR (r) register accessor: This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_sr`]
module"]
pub type FMC_SR = crate::Reg<fmc_sr::FMC_SRrs>;
#[doc = "This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits."]
pub mod fmc_sr;
#[doc = "FMC_PMEM (rw) register accessor: The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_pmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_pmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_pmem`]
module"]
pub type FMC_PMEM = crate::Reg<fmc_pmem::FMC_PMEMrs>;
#[doc = "The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses."]
pub mod fmc_pmem;
#[doc = "FMC_PATT (rw) register accessor: The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_patt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_patt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_patt`]
module"]
pub type FMC_PATT = crate::Reg<fmc_patt::FMC_PATTrs>;
#[doc = "The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function)."]
pub mod fmc_patt;
#[doc = "FMC_HPR (r) register accessor: This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_hpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_hpr`]
module"]
pub type FMC_HPR = crate::Reg<fmc_hpr::FMC_HPRrs>;
#[doc = "This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_hpr;
#[doc = "FMC_HECCR (r) register accessor: This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_heccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_heccr`]
module"]
pub type FMC_HECCR = crate::Reg<fmc_heccr::FMC_HECCRrs>;
#[doc = "This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_heccr;
#[doc = "FMC_BWTR1 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bwtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bwtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bwtr1`]
module"]
pub type FMC_BWTR1 = crate::Reg<fmc_bwtr1::FMC_BWTR1rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr1;
#[doc = "FMC_BWTR2 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bwtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bwtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bwtr2`]
module"]
pub type FMC_BWTR2 = crate::Reg<fmc_bwtr2::FMC_BWTR2rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr2;
#[doc = "FMC_BWTR3 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bwtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bwtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bwtr3`]
module"]
pub type FMC_BWTR3 = crate::Reg<fmc_bwtr3::FMC_BWTR3rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr3;
#[doc = "FMC_BWTR4 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bwtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bwtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bwtr4`]
module"]
pub type FMC_BWTR4 = crate::Reg<fmc_bwtr4::FMC_BWTR4rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr4;
#[doc = "FMC_CSQCR (w) register accessor: FMC NAND Command Sequencer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqcr`]
module"]
pub type FMC_CSQCR = crate::Reg<fmc_csqcr::FMC_CSQCRrs>;
#[doc = "FMC NAND Command Sequencer Control Register"]
pub mod fmc_csqcr;
#[doc = "FMC_CSQCFGR1 (rw) register accessor: FMC NAND Command Sequencer Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqcfgr1`]
module"]
pub type FMC_CSQCFGR1 = crate::Reg<fmc_csqcfgr1::FMC_CSQCFGR1rs>;
#[doc = "FMC NAND Command Sequencer Configuration Register 1"]
pub mod fmc_csqcfgr1;
#[doc = "FMC_CSQCFGR2 (rw) register accessor: This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqcfgr2`]
module"]
pub type FMC_CSQCFGR2 = crate::Reg<fmc_csqcfgr2::FMC_CSQCFGR2rs>;
#[doc = "This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. ."]
pub mod fmc_csqcfgr2;
#[doc = "FMC_CSQCFGR3 (rw) register accessor: FMC NAND sequencer configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqcfgr3`]
module"]
pub type FMC_CSQCFGR3 = crate::Reg<fmc_csqcfgr3::FMC_CSQCFGR3rs>;
#[doc = "FMC NAND sequencer configuration register 3"]
pub mod fmc_csqcfgr3;
#[doc = "FMC_CSQAR1 (rw) register accessor: This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqar1`]
module"]
pub type FMC_CSQAR1 = crate::Reg<fmc_csqar1::FMC_CSQAR1rs>;
#[doc = "This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer."]
pub mod fmc_csqar1;
#[doc = "FMC_CSQAR2 (rw) register accessor: This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqar2`]
module"]
pub type FMC_CSQAR2 = crate::Reg<fmc_csqar2::FMC_CSQAR2rs>;
#[doc = "This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable."]
pub mod fmc_csqar2;
#[doc = "FMC_CSQIER (rw) register accessor: FMC NAND Command Sequencer Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqier`]
module"]
pub type FMC_CSQIER = crate::Reg<fmc_csqier::FMC_CSQIERrs>;
#[doc = "FMC NAND Command Sequencer Interrupt Enable Register"]
pub mod fmc_csqier;
#[doc = "FMC_CSQISR (rw) register accessor: FMC NAND Command Sequencer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqisr`]
module"]
pub type FMC_CSQISR = crate::Reg<fmc_csqisr::FMC_CSQISRrs>;
#[doc = "FMC NAND Command Sequencer Interrupt Status Register"]
pub mod fmc_csqisr;
#[doc = "FMC_CSQICR (w) register accessor: FMC NAND Command Sequencer Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqicr`]
module"]
pub type FMC_CSQICR = crate::Reg<fmc_csqicr::FMC_CSQICRrs>;
#[doc = "FMC NAND Command Sequencer Interrupt Clear Register"]
pub mod fmc_csqicr;
#[doc = "FMC_CSQEMSR (r) register accessor: This register holds a sector error mapping status when the whole transfer is complete.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqemsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_csqemsr`]
module"]
pub type FMC_CSQEMSR = crate::Reg<fmc_csqemsr::FMC_CSQEMSRrs>;
#[doc = "This register holds a sector error mapping status when the whole transfer is complete."]
pub mod fmc_csqemsr;
#[doc = "FMC_BCHIER (rw) register accessor: FMC BCH Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bchier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchier`]
module"]
pub type FMC_BCHIER = crate::Reg<fmc_bchier::FMC_BCHIERrs>;
#[doc = "FMC BCH Interrupt enable register"]
pub mod fmc_bchier;
#[doc = "FMC_BCHISR (r) register accessor: This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchisr`]
module"]
pub type FMC_BCHISR = crate::Reg<fmc_bchisr::FMC_BCHISRrs>;
#[doc = "This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared."]
pub mod fmc_bchisr;
#[doc = "FMC_BCHICR (w) register accessor: FMC BCH Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bchicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchicr`]
module"]
pub type FMC_BCHICR = crate::Reg<fmc_bchicr::FMC_BCHICRrs>;
#[doc = "FMC BCH Interrupt Clear Register"]
pub mod fmc_bchicr;
#[doc = "FMC_BCHPBR1 (r) register accessor: These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchpbr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchpbr1`]
module"]
pub type FMC_BCHPBR1 = crate::Reg<fmc_bchpbr1::FMC_BCHPBR1rs>;
#[doc = "These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant."]
pub mod fmc_bchpbr1;
#[doc = "FMC_BCHPBR2 (r) register accessor: FMC BCH Parity Bits Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchpbr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchpbr2`]
module"]
pub type FMC_BCHPBR2 = crate::Reg<fmc_bchpbr2::FMC_BCHPBR2rs>;
#[doc = "FMC BCH Parity Bits Register 2"]
pub mod fmc_bchpbr2;
#[doc = "FMC_BCHPBR3 (r) register accessor: FMC BCH Parity Bits Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchpbr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchpbr3`]
module"]
pub type FMC_BCHPBR3 = crate::Reg<fmc_bchpbr3::FMC_BCHPBR3rs>;
#[doc = "FMC BCH Parity Bits Register 3"]
pub mod fmc_bchpbr3;
#[doc = "FMC_BCHPBR4 (r) register accessor: FMC BCH Parity Bits Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchpbr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchpbr4`]
module"]
pub type FMC_BCHPBR4 = crate::Reg<fmc_bchpbr4::FMC_BCHPBR4rs>;
#[doc = "FMC BCH Parity Bits Register 4"]
pub mod fmc_bchpbr4;
#[doc = "FMC_BCHDSR0 (r) register accessor: This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchdsr0`]
module"]
pub type FMC_BCHDSR0 = crate::Reg<fmc_bchdsr0::FMC_BCHDSR0rs>;
#[doc = "This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. ."]
pub mod fmc_bchdsr0;
#[doc = "FMC_BCHDSR1 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchdsr1`]
module"]
pub type FMC_BCHDSR1 = crate::Reg<fmc_bchdsr1::FMC_BCHDSR1rs>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors"]
pub mod fmc_bchdsr1;
#[doc = "FMC_BCHDSR2 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchdsr2`]
module"]
pub type FMC_BCHDSR2 = crate::Reg<fmc_bchdsr2::FMC_BCHDSR2rs>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively."]
pub mod fmc_bchdsr2;
#[doc = "FMC_BCHDSR3 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchdsr3`]
module"]
pub type FMC_BCHDSR3 = crate::Reg<fmc_bchdsr3::FMC_BCHDSR3rs>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors."]
pub mod fmc_bchdsr3;
#[doc = "FMC_BCHDSR4 (r) register accessor: The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_bchdsr4`]
module"]
pub type FMC_BCHDSR4 = crate::Reg<fmc_bchdsr4::FMC_BCHDSR4rs>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. ."]
pub mod fmc_bchdsr4;
#[doc = "FMC_HWCFGR2 (r) register accessor: FMC Hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_hwcfgr2`]
module"]
pub type FMC_HWCFGR2 = crate::Reg<fmc_hwcfgr2::FMC_HWCFGR2rs>;
#[doc = "FMC Hardware configuration register 2"]
pub mod fmc_hwcfgr2;
#[doc = "FMC_HWCFGR1 (r) register accessor: FMC Hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_hwcfgr1`]
module"]
pub type FMC_HWCFGR1 = crate::Reg<fmc_hwcfgr1::FMC_HWCFGR1rs>;
#[doc = "FMC Hardware configuration register 1"]
pub mod fmc_hwcfgr1;
#[doc = "FMC_VERR (r) register accessor: FMC Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_verr`]
module"]
pub type FMC_VERR = crate::Reg<fmc_verr::FMC_VERRrs>;
#[doc = "FMC Version register"]
pub mod fmc_verr;
#[doc = "FMC_IPIDR (r) register accessor: FMC Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_ipidr`]
module"]
pub type FMC_IPIDR = crate::Reg<fmc_ipidr::FMC_IPIDRrs>;
#[doc = "FMC Identification register"]
pub mod fmc_ipidr;
#[doc = "FMC_SIDR (r) register accessor: FMC Size Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_sidr`]
module"]
pub type FMC_SIDR = crate::Reg<fmc_sidr::FMC_SIDRrs>;
#[doc = "FMC Size Identification register"]
pub mod fmc_sidr;
