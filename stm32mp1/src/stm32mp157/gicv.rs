#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICV virtual machine control register"]
    pub gicv_ctlr: crate::Reg<gicv_ctlr::GICV_CTLR_SPEC>,
    #[doc = "0x04 - GICV VM priority mask register"]
    pub gicv_pmr: crate::Reg<gicv_pmr::GICV_PMR_SPEC>,
    #[doc = "0x08 - GICV VM binary point register"]
    pub gicv_bpr: crate::Reg<gicv_bpr::GICV_BPR_SPEC>,
    #[doc = "0x0c - GICV VM interrupt acknowledge register"]
    pub gicv_iar: crate::Reg<gicv_iar::GICV_IAR_SPEC>,
    #[doc = "0x10 - GICV VM end of interrupt register"]
    pub gicv_eoir: crate::Reg<gicv_eoir::GICV_EOIR_SPEC>,
    #[doc = "0x14 - GICV VM running priority register"]
    pub gicv_rpr: crate::Reg<gicv_rpr::GICV_RPR_SPEC>,
    #[doc = "0x18 - GICV VM highest priority pending interrupt register"]
    pub gicv_hppir: crate::Reg<gicv_hppir::GICV_HPPIR_SPEC>,
    #[doc = "0x1c - GICV VM aliased binary point register"]
    pub gicv_abpr: crate::Reg<gicv_abpr::GICV_ABPR_SPEC>,
    #[doc = "0x20 - GICV VM aliased interrupt register"]
    pub gicv_aiar: crate::Reg<gicv_aiar::GICV_AIAR_SPEC>,
    #[doc = "0x24 - GICV VM aliased end of interrupt register"]
    pub gicv_aeoir: crate::Reg<gicv_aeoir::GICV_AEOIR_SPEC>,
    #[doc = "0x28 - GICV VM aliased highest priority pending interrupt register"]
    pub gicv_ahppir: crate::Reg<gicv_ahppir::GICV_AHPPIR_SPEC>,
    _reserved11: [u8; 0xa4],
    #[doc = "0xd0 - The GICV_APR0 is an alias of GICH_APR."]
    pub gicv_apr0: crate::Reg<gicv_apr0::GICV_APR0_SPEC>,
    _reserved12: [u8; 0x28],
    #[doc = "0xfc - The GICV_IIDR is an alias of GICC_IIDR."]
    pub gicv_iidr: crate::Reg<gicv_iidr::GICV_IIDR_SPEC>,
    _reserved13: [u8; 0x0f00],
    #[doc = "0x1000 - GICV VM deactivate interrupt register"]
    pub gicv_dir: crate::Reg<gicv_dir::GICV_DIR_SPEC>,
}
#[doc = "GICV_CTLR register accessor: an alias for `Reg<GICV_CTLR_SPEC>`"]
pub type GICV_CTLR = crate::Reg<gicv_ctlr::GICV_CTLR_SPEC>;
#[doc = "GICV virtual machine control register"]
pub mod gicv_ctlr;
#[doc = "GICV_PMR register accessor: an alias for `Reg<GICV_PMR_SPEC>`"]
pub type GICV_PMR = crate::Reg<gicv_pmr::GICV_PMR_SPEC>;
#[doc = "GICV VM priority mask register"]
pub mod gicv_pmr;
#[doc = "GICV_BPR register accessor: an alias for `Reg<GICV_BPR_SPEC>`"]
pub type GICV_BPR = crate::Reg<gicv_bpr::GICV_BPR_SPEC>;
#[doc = "GICV VM binary point register"]
pub mod gicv_bpr;
#[doc = "GICV_IAR register accessor: an alias for `Reg<GICV_IAR_SPEC>`"]
pub type GICV_IAR = crate::Reg<gicv_iar::GICV_IAR_SPEC>;
#[doc = "GICV VM interrupt acknowledge register"]
pub mod gicv_iar;
#[doc = "GICV_EOIR register accessor: an alias for `Reg<GICV_EOIR_SPEC>`"]
pub type GICV_EOIR = crate::Reg<gicv_eoir::GICV_EOIR_SPEC>;
#[doc = "GICV VM end of interrupt register"]
pub mod gicv_eoir;
#[doc = "GICV_RPR register accessor: an alias for `Reg<GICV_RPR_SPEC>`"]
pub type GICV_RPR = crate::Reg<gicv_rpr::GICV_RPR_SPEC>;
#[doc = "GICV VM running priority register"]
pub mod gicv_rpr;
#[doc = "GICV_HPPIR register accessor: an alias for `Reg<GICV_HPPIR_SPEC>`"]
pub type GICV_HPPIR = crate::Reg<gicv_hppir::GICV_HPPIR_SPEC>;
#[doc = "GICV VM highest priority pending interrupt register"]
pub mod gicv_hppir;
#[doc = "GICV_ABPR register accessor: an alias for `Reg<GICV_ABPR_SPEC>`"]
pub type GICV_ABPR = crate::Reg<gicv_abpr::GICV_ABPR_SPEC>;
#[doc = "GICV VM aliased binary point register"]
pub mod gicv_abpr;
#[doc = "GICV_AIAR register accessor: an alias for `Reg<GICV_AIAR_SPEC>`"]
pub type GICV_AIAR = crate::Reg<gicv_aiar::GICV_AIAR_SPEC>;
#[doc = "GICV VM aliased interrupt register"]
pub mod gicv_aiar;
#[doc = "GICV_AEOIR register accessor: an alias for `Reg<GICV_AEOIR_SPEC>`"]
pub type GICV_AEOIR = crate::Reg<gicv_aeoir::GICV_AEOIR_SPEC>;
#[doc = "GICV VM aliased end of interrupt register"]
pub mod gicv_aeoir;
#[doc = "GICV_AHPPIR register accessor: an alias for `Reg<GICV_AHPPIR_SPEC>`"]
pub type GICV_AHPPIR = crate::Reg<gicv_ahppir::GICV_AHPPIR_SPEC>;
#[doc = "GICV VM aliased highest priority pending interrupt register"]
pub mod gicv_ahppir;
#[doc = "GICV_APR0 register accessor: an alias for `Reg<GICV_APR0_SPEC>`"]
pub type GICV_APR0 = crate::Reg<gicv_apr0::GICV_APR0_SPEC>;
#[doc = "The GICV_APR0 is an alias of GICH_APR."]
pub mod gicv_apr0;
#[doc = "GICV_IIDR register accessor: an alias for `Reg<GICV_IIDR_SPEC>`"]
pub type GICV_IIDR = crate::Reg<gicv_iidr::GICV_IIDR_SPEC>;
#[doc = "The GICV_IIDR is an alias of GICC_IIDR."]
pub mod gicv_iidr;
#[doc = "GICV_DIR register accessor: an alias for `Reg<GICV_DIR_SPEC>`"]
pub type GICV_DIR = crate::Reg<gicv_dir::GICV_DIR_SPEC>;
#[doc = "GICV VM deactivate interrupt register"]
pub mod gicv_dir;
