#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: crate::Reg<bcr1::BCR1_SPEC>,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: crate::Reg<btr::BTR_SPEC>,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr2: crate::Reg<btr::BTR_SPEC>,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr3: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr3: crate::Reg<btr::BTR_SPEC>,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr4: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr4: crate::Reg<btr::BTR_SPEC>,
    _reserved8: [u8; 0x60],
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem: crate::Reg<pmem::PMEM_SPEC>,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt: crate::Reg<patt::PATT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr: crate::Reg<eccr::ECCR_SPEC>,
    _reserved13: [u8; 0x6c],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: crate::Reg<bwtr::BWTR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: crate::Reg<bwtr::BWTR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: crate::Reg<bwtr::BWTR_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr4: crate::Reg<bwtr::BWTR_SPEC>,
}
#[doc = "BCR1 register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr;
#[doc = "BCR register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "FIFO status and interrupt register 3"]
pub mod sr;
#[doc = "PMEM register accessor: an alias for `Reg<PMEM_SPEC>`"]
pub type PMEM = crate::Reg<pmem::PMEM_SPEC>;
#[doc = "Common memory space timing register 3"]
pub mod pmem;
#[doc = "PATT register accessor: an alias for `Reg<PATT_SPEC>`"]
pub type PATT = crate::Reg<patt::PATT_SPEC>;
#[doc = "Attribute memory space timing register 3"]
pub mod patt;
#[doc = "ECCR register accessor: an alias for `Reg<ECCR_SPEC>`"]
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
#[doc = "ECC result register 3"]
pub mod eccr;
#[doc = "BWTR register accessor: an alias for `Reg<BWTR_SPEC>`"]
pub type BWTR = crate::Reg<bwtr::BWTR_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr;