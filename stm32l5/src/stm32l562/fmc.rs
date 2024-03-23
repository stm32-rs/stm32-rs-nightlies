#[repr(C)]
#[doc = "Register block"]
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
    _reserved13: [u8; 0x04],
    eccr: ECCR,
    _reserved14: [u8; 0x6c],
    bwtr1: BWTR1,
    _reserved15: [u8; 0x04],
    bwtr2: BWTR2,
    _reserved16: [u8; 0x04],
    bwtr3: BWTR3,
    _reserved17: [u8; 0x04],
    bwtr4: BWTR4,
}
impl RegisterBlock {
    #[doc = "0x00 - FMC_BCR1"]
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR1 {
        &self.btr1
    }
    #[doc = "0x08 - FMC_BCR2"]
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    #[doc = "0x0c - FMC_BTR2"]
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR2 {
        &self.btr2
    }
    #[doc = "0x10 - >FMC_BCR3"]
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR3 {
        &self.bcr3
    }
    #[doc = "0x14 - FMC_BTR3"]
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR3 {
        &self.btr3
    }
    #[doc = "0x18 - >FMC_BCR4"]
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR4 {
        &self.bcr4
    }
    #[doc = "0x1c - FMC_BTR4"]
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR4 {
        &self.btr4
    }
    #[doc = "0x20 - PCSCNTR"]
    #[inline(always)]
    pub const fn pcscntr(&self) -> &PCSCNTR {
        &self.pcscntr
    }
    #[doc = "0x80 - NAND Flash control registers"]
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    #[doc = "0x84 - This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x88 - The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
    #[inline(always)]
    pub const fn pmem(&self) -> &PMEM {
        &self.pmem
    }
    #[doc = "0x8c - The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
    #[inline(always)]
    pub const fn patt(&self) -> &PATT {
        &self.patt
    }
    #[doc = "0x94 - This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR1 {
        &self.bwtr1
    }
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR2 {
        &self.bwtr2
    }
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR3 {
        &self.bwtr3
    }
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR4 {
        &self.bwtr4
    }
}
#[doc = "BCR1 (rw) register accessor: FMC_BCR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr1`]
module"]
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
#[doc = "FMC_BCR1"]
pub mod bcr1;
#[doc = "BCR2 (rw) register accessor: FMC_BCR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr2`]
module"]
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
#[doc = "FMC_BCR2"]
pub mod bcr2;
#[doc = "BCR3 (rw) register accessor: >FMC_BCR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr3`]
module"]
pub type BCR3 = crate::Reg<bcr3::BCR3rs>;
#[doc = ">FMC_BCR3"]
pub mod bcr3;
#[doc = "BCR4 (rw) register accessor: >FMC_BCR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr4`]
module"]
pub type BCR4 = crate::Reg<bcr4::BCR4rs>;
#[doc = ">FMC_BCR4"]
pub mod bcr4;
#[doc = "BTR1 (rw) register accessor: This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr1`]
module"]
pub type BTR1 = crate::Reg<btr1::BTR1rs>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod btr1;
#[doc = "BTR2 (rw) register accessor: FMC_BTR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr2`]
module"]
pub type BTR2 = crate::Reg<btr2::BTR2rs>;
#[doc = "FMC_BTR2"]
pub mod btr2;
#[doc = "BTR3 (rw) register accessor: FMC_BTR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr3`]
module"]
pub type BTR3 = crate::Reg<btr3::BTR3rs>;
#[doc = "FMC_BTR3"]
pub mod btr3;
#[doc = "BTR4 (rw) register accessor: FMC_BTR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr4`]
module"]
pub type BTR4 = crate::Reg<btr4::BTR4rs>;
#[doc = "FMC_BTR4"]
pub mod btr4;
#[doc = "PCR (rw) register accessor: NAND Flash control registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCRrs>;
#[doc = "NAND Flash control registers"]
pub mod pcr;
#[doc = "SR (rw) register accessor: This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
pub mod sr;
#[doc = "PMEM (rw) register accessor: The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmem`]
module"]
pub type PMEM = crate::Reg<pmem::PMEMrs>;
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
pub mod pmem;
#[doc = "PATT (rw) register accessor: The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt`]
module"]
pub type PATT = crate::Reg<patt::PATTrs>;
#[doc = "The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
pub mod patt;
#[doc = "ECCR (r) register accessor: This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`]
module"]
pub type ECCR = crate::Reg<eccr::ECCRrs>;
#[doc = "This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod eccr;
#[doc = "BWTR1 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr1`]
module"]
pub type BWTR1 = crate::Reg<bwtr1::BWTR1rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr1;
#[doc = "BWTR2 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr2`]
module"]
pub type BWTR2 = crate::Reg<bwtr2::BWTR2rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr2;
#[doc = "BWTR3 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr3`]
module"]
pub type BWTR3 = crate::Reg<bwtr3::BWTR3rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr3;
#[doc = "BWTR4 (rw) register accessor: This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr4`]
module"]
pub type BWTR4 = crate::Reg<bwtr4::BWTR4rs>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr4;
#[doc = "PCSCNTR (rw) register accessor: PCSCNTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcscntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcscntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcscntr`]
module"]
pub type PCSCNTR = crate::Reg<pcscntr::PCSCNTRrs>;
#[doc = "PCSCNTR"]
pub mod pcscntr;
