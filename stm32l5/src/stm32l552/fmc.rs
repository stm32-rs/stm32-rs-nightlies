#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMC_BCR1"]
    pub fmc_bcr1: crate::Reg<fmc_bcr1::FMC_BCR1_SPEC>,
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr1: crate::Reg<fmc_btr1::FMC_BTR1_SPEC>,
    #[doc = "0x08 - FMC_BCR2"]
    pub fmc_bcr2: crate::Reg<fmc_bcr2::FMC_BCR2_SPEC>,
    #[doc = "0x0c - FMC_BTR2"]
    pub fmc_btr2: crate::Reg<fmc_btr2::FMC_BTR2_SPEC>,
    #[doc = "0x10 - >FMC_BCR3"]
    pub fmc_bcr3: crate::Reg<fmc_bcr3::FMC_BCR3_SPEC>,
    #[doc = "0x14 - FMC_BTR3"]
    pub fmc_btr3: crate::Reg<fmc_btr3::FMC_BTR3_SPEC>,
    #[doc = "0x18 - >FMC_BCR4"]
    pub fmc_bcr4: crate::Reg<fmc_bcr4::FMC_BCR4_SPEC>,
    #[doc = "0x1c - FMC_BTR4"]
    pub fmc_btr4: crate::Reg<fmc_btr4::FMC_BTR4_SPEC>,
    #[doc = "0x20 - PCSCNTR"]
    pub pcscntr: crate::Reg<pcscntr::PCSCNTR_SPEC>,
    _reserved9: [u8; 0x5c],
    #[doc = "0x80 - NAND Flash control registers"]
    pub fmc_pcr: crate::Reg<fmc_pcr::FMC_PCR_SPEC>,
    #[doc = "0x84 - This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
    pub fmc_sr: crate::Reg<fmc_sr::FMC_SR_SPEC>,
    #[doc = "0x88 - The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
    pub fmc_pmem: crate::Reg<fmc_pmem::FMC_PMEM_SPEC>,
    #[doc = "0x8c - The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
    pub fmc_patt: crate::Reg<fmc_patt::FMC_PATT_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x94 - This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub fmc_eccr: crate::Reg<fmc_eccr::FMC_ECCR_SPEC>,
    _reserved14: [u8; 0x6c],
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr1: crate::Reg<fmc_bwtr1::FMC_BWTR1_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr2: crate::Reg<fmc_bwtr2::FMC_BWTR2_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr3: crate::Reg<fmc_bwtr3::FMC_BWTR3_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr4: crate::Reg<fmc_bwtr4::FMC_BWTR4_SPEC>,
}
#[doc = "FMC_BCR1 register accessor: an alias for `Reg<FMC_BCR1_SPEC>`"]
pub type FMC_BCR1 = crate::Reg<fmc_bcr1::FMC_BCR1_SPEC>;
#[doc = "FMC_BCR1"]
pub mod fmc_bcr1;
#[doc = "FMC_BCR2 register accessor: an alias for `Reg<FMC_BCR2_SPEC>`"]
pub type FMC_BCR2 = crate::Reg<fmc_bcr2::FMC_BCR2_SPEC>;
#[doc = "FMC_BCR2"]
pub mod fmc_bcr2;
#[doc = "FMC_BCR3 register accessor: an alias for `Reg<FMC_BCR3_SPEC>`"]
pub type FMC_BCR3 = crate::Reg<fmc_bcr3::FMC_BCR3_SPEC>;
#[doc = ">FMC_BCR3"]
pub mod fmc_bcr3;
#[doc = "FMC_BCR4 register accessor: an alias for `Reg<FMC_BCR4_SPEC>`"]
pub type FMC_BCR4 = crate::Reg<fmc_bcr4::FMC_BCR4_SPEC>;
#[doc = ">FMC_BCR4"]
pub mod fmc_bcr4;
#[doc = "FMC_BTR1 register accessor: an alias for `Reg<FMC_BTR1_SPEC>`"]
pub type FMC_BTR1 = crate::Reg<fmc_btr1::FMC_BTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr1;
#[doc = "FMC_BTR2 register accessor: an alias for `Reg<FMC_BTR2_SPEC>`"]
pub type FMC_BTR2 = crate::Reg<fmc_btr2::FMC_BTR2_SPEC>;
#[doc = "FMC_BTR2"]
pub mod fmc_btr2;
#[doc = "FMC_BTR3 register accessor: an alias for `Reg<FMC_BTR3_SPEC>`"]
pub type FMC_BTR3 = crate::Reg<fmc_btr3::FMC_BTR3_SPEC>;
#[doc = "FMC_BTR3"]
pub mod fmc_btr3;
#[doc = "FMC_BTR4 register accessor: an alias for `Reg<FMC_BTR4_SPEC>`"]
pub type FMC_BTR4 = crate::Reg<fmc_btr4::FMC_BTR4_SPEC>;
#[doc = "FMC_BTR4"]
pub mod fmc_btr4;
#[doc = "FMC_PCR register accessor: an alias for `Reg<FMC_PCR_SPEC>`"]
pub type FMC_PCR = crate::Reg<fmc_pcr::FMC_PCR_SPEC>;
#[doc = "NAND Flash control registers"]
pub mod fmc_pcr;
#[doc = "FMC_SR register accessor: an alias for `Reg<FMC_SR_SPEC>`"]
pub type FMC_SR = crate::Reg<fmc_sr::FMC_SR_SPEC>;
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
pub mod fmc_sr;
#[doc = "FMC_PMEM register accessor: an alias for `Reg<FMC_PMEM_SPEC>`"]
pub type FMC_PMEM = crate::Reg<fmc_pmem::FMC_PMEM_SPEC>;
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
pub mod fmc_pmem;
#[doc = "FMC_PATT register accessor: an alias for `Reg<FMC_PATT_SPEC>`"]
pub type FMC_PATT = crate::Reg<fmc_patt::FMC_PATT_SPEC>;
#[doc = "The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
pub mod fmc_patt;
#[doc = "FMC_ECCR register accessor: an alias for `Reg<FMC_ECCR_SPEC>`"]
pub type FMC_ECCR = crate::Reg<fmc_eccr::FMC_ECCR_SPEC>;
#[doc = "This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_eccr;
#[doc = "FMC_BWTR1 register accessor: an alias for `Reg<FMC_BWTR1_SPEC>`"]
pub type FMC_BWTR1 = crate::Reg<fmc_bwtr1::FMC_BWTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr1;
#[doc = "FMC_BWTR2 register accessor: an alias for `Reg<FMC_BWTR2_SPEC>`"]
pub type FMC_BWTR2 = crate::Reg<fmc_bwtr2::FMC_BWTR2_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr2;
#[doc = "FMC_BWTR3 register accessor: an alias for `Reg<FMC_BWTR3_SPEC>`"]
pub type FMC_BWTR3 = crate::Reg<fmc_bwtr3::FMC_BWTR3_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr3;
#[doc = "FMC_BWTR4 register accessor: an alias for `Reg<FMC_BWTR4_SPEC>`"]
pub type FMC_BWTR4 = crate::Reg<fmc_bwtr4::FMC_BWTR4_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr4;
#[doc = "PCSCNTR register accessor: an alias for `Reg<PCSCNTR_SPEC>`"]
pub type PCSCNTR = crate::Reg<pcscntr::PCSCNTR_SPEC>;
#[doc = "PCSCNTR"]
pub mod pcscntr;
