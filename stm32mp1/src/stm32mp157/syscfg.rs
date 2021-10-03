#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )"]
    pub syscfg_bootr: crate::Reg<syscfg_bootr::SYSCFG_BOOTR_SPEC>,
    #[doc = "0x04 - SYSCFG peripheral mode configuration set register"]
    pub syscfg_pmcsetr: crate::Reg<syscfg_pmcsetr::SYSCFG_PMCSETR_SPEC>,
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - SYSCFG IO control register"]
    pub syscfg_ioctrlsetr: crate::Reg<syscfg_ioctrlsetr::SYSCFG_IOCTRLSETR_SPEC>,
    #[doc = "0x1c - SYSCFG interconnect control register"]
    pub syscfg_icnr: crate::Reg<syscfg_icnr::SYSCFG_ICNR_SPEC>,
    #[doc = "0x20 - SYSCFG compensation cell control register"]
    pub syscfg_cmpcr: crate::Reg<syscfg_cmpcr::SYSCFG_CMPCR_SPEC>,
    #[doc = "0x24 - SYSCFG compensation cell enable set register"]
    pub syscfg_cmpensetr: crate::Reg<syscfg_cmpensetr::SYSCFG_CMPENSETR_SPEC>,
    #[doc = "0x28 - SYSCFG compensation cell enable set register"]
    pub syscfg_cmpenclrr: crate::Reg<syscfg_cmpenclrr::SYSCFG_CMPENCLRR_SPEC>,
    #[doc = "0x2c - SYSCFG control timer break register"]
    pub syscfg_cbr: crate::Reg<syscfg_cbr::SYSCFG_CBR_SPEC>,
    _reserved8: [u8; 0x14],
    #[doc = "0x44 - SYSCFG peripheral mode configuration clear register"]
    pub syscfg_pmcclrr: crate::Reg<syscfg_pmcclrr::SYSCFG_PMCCLRR_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x58 - SYSCFG IO control register"]
    pub syscfg_ioctrlclrr: crate::Reg<syscfg_ioctrlclrr::SYSCFG_IOCTRLCLRR_SPEC>,
    _reserved10: [u8; 0x0398],
    #[doc = "0x3f4 - SYSCFG version register"]
    pub syscfg_verr: crate::Reg<syscfg_verr::SYSCFG_VERR_SPEC>,
    #[doc = "0x3f8 - SYSCFG identification register"]
    pub syscfg_ipidr: crate::Reg<syscfg_ipidr::SYSCFG_IPIDR_SPEC>,
    #[doc = "0x3fc - SYSCFG size identification register"]
    pub syscfg_sidr: crate::Reg<syscfg_sidr::SYSCFG_SIDR_SPEC>,
}
#[doc = "SYSCFG_BOOTR register accessor: an alias for `Reg<SYSCFG_BOOTR_SPEC>`"]
pub type SYSCFG_BOOTR = crate::Reg<syscfg_bootr::SYSCFG_BOOTR_SPEC>;
#[doc = "This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )"]
pub mod syscfg_bootr;
#[doc = "SYSCFG_PMCSETR register accessor: an alias for `Reg<SYSCFG_PMCSETR_SPEC>`"]
pub type SYSCFG_PMCSETR = crate::Reg<syscfg_pmcsetr::SYSCFG_PMCSETR_SPEC>;
#[doc = "SYSCFG peripheral mode configuration set register"]
pub mod syscfg_pmcsetr;
#[doc = "SYSCFG_IOCTRLSETR register accessor: an alias for `Reg<SYSCFG_IOCTRLSETR_SPEC>`"]
pub type SYSCFG_IOCTRLSETR = crate::Reg<syscfg_ioctrlsetr::SYSCFG_IOCTRLSETR_SPEC>;
#[doc = "SYSCFG IO control register"]
pub mod syscfg_ioctrlsetr;
#[doc = "SYSCFG_ICNR register accessor: an alias for `Reg<SYSCFG_ICNR_SPEC>`"]
pub type SYSCFG_ICNR = crate::Reg<syscfg_icnr::SYSCFG_ICNR_SPEC>;
#[doc = "SYSCFG interconnect control register"]
pub mod syscfg_icnr;
#[doc = "SYSCFG_CMPCR register accessor: an alias for `Reg<SYSCFG_CMPCR_SPEC>`"]
pub type SYSCFG_CMPCR = crate::Reg<syscfg_cmpcr::SYSCFG_CMPCR_SPEC>;
#[doc = "SYSCFG compensation cell control register"]
pub mod syscfg_cmpcr;
#[doc = "SYSCFG_CMPENSETR register accessor: an alias for `Reg<SYSCFG_CMPENSETR_SPEC>`"]
pub type SYSCFG_CMPENSETR = crate::Reg<syscfg_cmpensetr::SYSCFG_CMPENSETR_SPEC>;
#[doc = "SYSCFG compensation cell enable set register"]
pub mod syscfg_cmpensetr;
#[doc = "SYSCFG_CMPENCLRR register accessor: an alias for `Reg<SYSCFG_CMPENCLRR_SPEC>`"]
pub type SYSCFG_CMPENCLRR = crate::Reg<syscfg_cmpenclrr::SYSCFG_CMPENCLRR_SPEC>;
#[doc = "SYSCFG compensation cell enable set register"]
pub mod syscfg_cmpenclrr;
#[doc = "SYSCFG_CBR register accessor: an alias for `Reg<SYSCFG_CBR_SPEC>`"]
pub type SYSCFG_CBR = crate::Reg<syscfg_cbr::SYSCFG_CBR_SPEC>;
#[doc = "SYSCFG control timer break register"]
pub mod syscfg_cbr;
#[doc = "SYSCFG_PMCCLRR register accessor: an alias for `Reg<SYSCFG_PMCCLRR_SPEC>`"]
pub type SYSCFG_PMCCLRR = crate::Reg<syscfg_pmcclrr::SYSCFG_PMCCLRR_SPEC>;
#[doc = "SYSCFG peripheral mode configuration clear register"]
pub mod syscfg_pmcclrr;
#[doc = "SYSCFG_IOCTRLCLRR register accessor: an alias for `Reg<SYSCFG_IOCTRLCLRR_SPEC>`"]
pub type SYSCFG_IOCTRLCLRR = crate::Reg<syscfg_ioctrlclrr::SYSCFG_IOCTRLCLRR_SPEC>;
#[doc = "SYSCFG IO control register"]
pub mod syscfg_ioctrlclrr;
#[doc = "SYSCFG_VERR register accessor: an alias for `Reg<SYSCFG_VERR_SPEC>`"]
pub type SYSCFG_VERR = crate::Reg<syscfg_verr::SYSCFG_VERR_SPEC>;
#[doc = "SYSCFG version register"]
pub mod syscfg_verr;
#[doc = "SYSCFG_IPIDR register accessor: an alias for `Reg<SYSCFG_IPIDR_SPEC>`"]
pub type SYSCFG_IPIDR = crate::Reg<syscfg_ipidr::SYSCFG_IPIDR_SPEC>;
#[doc = "SYSCFG identification register"]
pub mod syscfg_ipidr;
#[doc = "SYSCFG_SIDR register accessor: an alias for `Reg<SYSCFG_SIDR_SPEC>`"]
pub type SYSCFG_SIDR = crate::Reg<syscfg_sidr::SYSCFG_SIDR_SPEC>;
#[doc = "SYSCFG size identification register"]
pub mod syscfg_sidr;
