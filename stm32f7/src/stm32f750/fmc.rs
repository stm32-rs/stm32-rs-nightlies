#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: crate::Reg<bcr1::BCR1_SPEC>,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: crate::Reg<btr1::BTR1_SPEC>,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: crate::Reg<bcr2::BCR2_SPEC>,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub btr2: crate::Reg<btr2::BTR2_SPEC>,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bcr3: crate::Reg<bcr3::BCR3_SPEC>,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub btr3: crate::Reg<btr3::BTR3_SPEC>,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bcr4: crate::Reg<bcr4::BCR4_SPEC>,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub btr4: crate::Reg<btr4::BTR4_SPEC>,
    _reserved8: [u8; 0x60],
    #[doc = "0x80 - PC Card/NAND Flash control register"]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x84 - FIFO status and interrupt register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x88 - Common memory space timing register"]
    pub pmem: crate::Reg<pmem::PMEM_SPEC>,
    #[doc = "0x8c - Attribute memory space timing register"]
    pub patt: crate::Reg<patt::PATT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x94 - ECC result register"]
    pub eccr: crate::Reg<eccr::ECCR_SPEC>,
    _reserved13: [u8; 0x6c],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: crate::Reg<bwtr1::BWTR1_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bwtr2: crate::Reg<bwtr2::BWTR2_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bwtr3: crate::Reg<bwtr3::BWTR3_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bwtr4: crate::Reg<bwtr4::BWTR4_SPEC>,
    _reserved17: [u8; 0x20],
    #[doc = "0x140 - SDRAM Control Register 1"]
    pub sdcr1: crate::Reg<sdcr1::SDCR1_SPEC>,
    #[doc = "0x144 - SDRAM Control Register 2"]
    pub sdcr2: crate::Reg<sdcr2::SDCR2_SPEC>,
    #[doc = "0x148 - SDRAM Timing register 1"]
    pub sdtr1: crate::Reg<sdtr1::SDTR1_SPEC>,
    #[doc = "0x14c - SDRAM Timing register 2"]
    pub sdtr2: crate::Reg<sdtr2::SDTR2_SPEC>,
    #[doc = "0x150 - SDRAM Command Mode register"]
    pub sdcmr: crate::Reg<sdcmr::SDCMR_SPEC>,
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    pub sdrtr: crate::Reg<sdrtr::SDRTR_SPEC>,
    #[doc = "0x158 - SDRAM Status register"]
    pub sdsr: crate::Reg<sdsr::SDSR_SPEC>,
}
#[doc = "BCR1 register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR1 register accessor: an alias for `Reg<BTR1_SPEC>`"]
pub type BTR1 = crate::Reg<btr1::BTR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "BCR2 register accessor: an alias for `Reg<BCR2_SPEC>`"]
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "BTR2 register accessor: an alias for `Reg<BTR2_SPEC>`"]
pub type BTR2 = crate::Reg<btr2::BTR2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "BCR3 register accessor: an alias for `Reg<BCR3_SPEC>`"]
pub type BCR3 = crate::Reg<bcr3::BCR3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "BTR3 register accessor: an alias for `Reg<BTR3_SPEC>`"]
pub type BTR3 = crate::Reg<btr3::BTR3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "BCR4 register accessor: an alias for `Reg<BCR4_SPEC>`"]
pub type BCR4 = crate::Reg<bcr4::BCR4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "BTR4 register accessor: an alias for `Reg<BTR4_SPEC>`"]
pub type BTR4 = crate::Reg<btr4::BTR4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PC Card/NAND Flash control register"]
pub mod pcr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "FIFO status and interrupt register"]
pub mod sr;
#[doc = "PMEM register accessor: an alias for `Reg<PMEM_SPEC>`"]
pub type PMEM = crate::Reg<pmem::PMEM_SPEC>;
#[doc = "Common memory space timing register"]
pub mod pmem;
#[doc = "PATT register accessor: an alias for `Reg<PATT_SPEC>`"]
pub type PATT = crate::Reg<patt::PATT_SPEC>;
#[doc = "Attribute memory space timing register"]
pub mod patt;
#[doc = "ECCR register accessor: an alias for `Reg<ECCR_SPEC>`"]
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
#[doc = "ECC result register"]
pub mod eccr;
#[doc = "BWTR1 register accessor: an alias for `Reg<BWTR1_SPEC>`"]
pub type BWTR1 = crate::Reg<bwtr1::BWTR1_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "BWTR2 register accessor: an alias for `Reg<BWTR2_SPEC>`"]
pub type BWTR2 = crate::Reg<bwtr2::BWTR2_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "BWTR3 register accessor: an alias for `Reg<BWTR3_SPEC>`"]
pub type BWTR3 = crate::Reg<bwtr3::BWTR3_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "BWTR4 register accessor: an alias for `Reg<BWTR4_SPEC>`"]
pub type BWTR4 = crate::Reg<bwtr4::BWTR4_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
#[doc = "SDCR1 register accessor: an alias for `Reg<SDCR1_SPEC>`"]
pub type SDCR1 = crate::Reg<sdcr1::SDCR1_SPEC>;
#[doc = "SDRAM Control Register 1"]
pub mod sdcr1;
#[doc = "SDCR2 register accessor: an alias for `Reg<SDCR2_SPEC>`"]
pub type SDCR2 = crate::Reg<sdcr2::SDCR2_SPEC>;
#[doc = "SDRAM Control Register 2"]
pub mod sdcr2;
#[doc = "SDTR1 register accessor: an alias for `Reg<SDTR1_SPEC>`"]
pub type SDTR1 = crate::Reg<sdtr1::SDTR1_SPEC>;
#[doc = "SDRAM Timing register 1"]
pub mod sdtr1;
#[doc = "SDTR2 register accessor: an alias for `Reg<SDTR2_SPEC>`"]
pub type SDTR2 = crate::Reg<sdtr2::SDTR2_SPEC>;
#[doc = "SDRAM Timing register 2"]
pub mod sdtr2;
#[doc = "SDCMR register accessor: an alias for `Reg<SDCMR_SPEC>`"]
pub type SDCMR = crate::Reg<sdcmr::SDCMR_SPEC>;
#[doc = "SDRAM Command Mode register"]
pub mod sdcmr;
#[doc = "SDRTR register accessor: an alias for `Reg<SDRTR_SPEC>`"]
pub type SDRTR = crate::Reg<sdrtr::SDRTR_SPEC>;
#[doc = "SDRAM Refresh Timer register"]
pub mod sdrtr;
#[doc = "SDSR register accessor: an alias for `Reg<SDSR_SPEC>`"]
pub type SDSR = crate::Reg<sdsr::SDSR_SPEC>;
#[doc = "SDRAM Status register"]
pub mod sdsr;
