#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICC control register"]
    pub gicc_ctlr: crate::Reg<gicc_ctlr::GICC_CTLR_SPEC>,
    #[doc = "0x04 - GICC input priority mask register"]
    pub gicc_pmr: crate::Reg<gicc_pmr::GICC_PMR_SPEC>,
    #[doc = "0x08 - GICC binary point register"]
    pub gicc_bpr: crate::Reg<gicc_bpr::GICC_BPR_SPEC>,
    #[doc = "0x0c - GICC interrupt acknowledge register"]
    pub gicc_iar: crate::Reg<gicc_iar::GICC_IAR_SPEC>,
    #[doc = "0x10 - GICC end of interrupt register"]
    pub gicc_eoir: crate::Reg<gicc_eoir::GICC_EOIR_SPEC>,
    #[doc = "0x14 - GICC running priority register"]
    pub gicc_rpr: crate::Reg<gicc_rpr::GICC_RPR_SPEC>,
    #[doc = "0x18 - GICC highest priority pending interrupt register"]
    pub gicc_hppir: crate::Reg<gicc_hppir::GICC_HPPIR_SPEC>,
    #[doc = "0x1c - GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR."]
    pub gicc_abpr: crate::Reg<gicc_abpr::GICC_ABPR_SPEC>,
    #[doc = "0x20 - GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR."]
    pub gicc_aiar: crate::Reg<gicc_aiar::GICC_AIAR_SPEC>,
    #[doc = "0x24 - GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used."]
    pub gicc_aeoir: crate::Reg<gicc_aeoir::GICC_AEOIR_SPEC>,
    #[doc = "0x28 - ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR."]
    pub gicc_ahppir: crate::Reg<gicc_ahppir::GICC_AHPPIR_SPEC>,
    _reserved11: [u8; 0xa4],
    #[doc = "0xd0 - GICC active priority register"]
    pub gicc_apr0: crate::Reg<gicc_apr0::GICC_APR0_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0xe0 - GICC non-secure active priority register"]
    pub gicc_nsapr0: crate::Reg<gicc_nsapr0::GICC_NSAPR0_SPEC>,
    _reserved13: [u8; 0x18],
    #[doc = "0xfc - GICC interface identification register"]
    pub gicc_iidr: crate::Reg<gicc_iidr::GICC_IIDR_SPEC>,
    _reserved14: [u8; 0x0f00],
    #[doc = "0x1000 - GICC deactivate interrupt register"]
    pub gicc_dir: crate::Reg<gicc_dir::GICC_DIR_SPEC>,
}
#[doc = "GICC_CTLR register accessor: an alias for `Reg<GICC_CTLR_SPEC>`"]
pub type GICC_CTLR = crate::Reg<gicc_ctlr::GICC_CTLR_SPEC>;
#[doc = "GICC control register"]
pub mod gicc_ctlr;
#[doc = "GICC_PMR register accessor: an alias for `Reg<GICC_PMR_SPEC>`"]
pub type GICC_PMR = crate::Reg<gicc_pmr::GICC_PMR_SPEC>;
#[doc = "GICC input priority mask register"]
pub mod gicc_pmr;
#[doc = "GICC_BPR register accessor: an alias for `Reg<GICC_BPR_SPEC>`"]
pub type GICC_BPR = crate::Reg<gicc_bpr::GICC_BPR_SPEC>;
#[doc = "GICC binary point register"]
pub mod gicc_bpr;
#[doc = "GICC_IAR register accessor: an alias for `Reg<GICC_IAR_SPEC>`"]
pub type GICC_IAR = crate::Reg<gicc_iar::GICC_IAR_SPEC>;
#[doc = "GICC interrupt acknowledge register"]
pub mod gicc_iar;
#[doc = "GICC_EOIR register accessor: an alias for `Reg<GICC_EOIR_SPEC>`"]
pub type GICC_EOIR = crate::Reg<gicc_eoir::GICC_EOIR_SPEC>;
#[doc = "GICC end of interrupt register"]
pub mod gicc_eoir;
#[doc = "GICC_RPR register accessor: an alias for `Reg<GICC_RPR_SPEC>`"]
pub type GICC_RPR = crate::Reg<gicc_rpr::GICC_RPR_SPEC>;
#[doc = "GICC running priority register"]
pub mod gicc_rpr;
#[doc = "GICC_HPPIR register accessor: an alias for `Reg<GICC_HPPIR_SPEC>`"]
pub type GICC_HPPIR = crate::Reg<gicc_hppir::GICC_HPPIR_SPEC>;
#[doc = "GICC highest priority pending interrupt register"]
pub mod gicc_hppir;
#[doc = "GICC_ABPR register accessor: an alias for `Reg<GICC_ABPR_SPEC>`"]
pub type GICC_ABPR = crate::Reg<gicc_abpr::GICC_ABPR_SPEC>;
#[doc = "GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR."]
pub mod gicc_abpr;
#[doc = "GICC_AIAR register accessor: an alias for `Reg<GICC_AIAR_SPEC>`"]
pub type GICC_AIAR = crate::Reg<gicc_aiar::GICC_AIAR_SPEC>;
#[doc = "GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR."]
pub mod gicc_aiar;
#[doc = "GICC_AEOIR register accessor: an alias for `Reg<GICC_AEOIR_SPEC>`"]
pub type GICC_AEOIR = crate::Reg<gicc_aeoir::GICC_AEOIR_SPEC>;
#[doc = "GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used."]
pub mod gicc_aeoir;
#[doc = "GICC_AHPPIR register accessor: an alias for `Reg<GICC_AHPPIR_SPEC>`"]
pub type GICC_AHPPIR = crate::Reg<gicc_ahppir::GICC_AHPPIR_SPEC>;
#[doc = "ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR."]
pub mod gicc_ahppir;
#[doc = "GICC_APR0 register accessor: an alias for `Reg<GICC_APR0_SPEC>`"]
pub type GICC_APR0 = crate::Reg<gicc_apr0::GICC_APR0_SPEC>;
#[doc = "GICC active priority register"]
pub mod gicc_apr0;
#[doc = "GICC_NSAPR0 register accessor: an alias for `Reg<GICC_NSAPR0_SPEC>`"]
pub type GICC_NSAPR0 = crate::Reg<gicc_nsapr0::GICC_NSAPR0_SPEC>;
#[doc = "GICC non-secure active priority register"]
pub mod gicc_nsapr0;
#[doc = "GICC_IIDR register accessor: an alias for `Reg<GICC_IIDR_SPEC>`"]
pub type GICC_IIDR = crate::Reg<gicc_iidr::GICC_IIDR_SPEC>;
#[doc = "GICC interface identification register"]
pub mod gicc_iidr;
#[doc = "GICC_DIR register accessor: an alias for `Reg<GICC_DIR_SPEC>`"]
pub type GICC_DIR = crate::Reg<gicc_dir::GICC_DIR_SPEC>;
#[doc = "GICC deactivate interrupt register"]
pub mod gicc_dir;
