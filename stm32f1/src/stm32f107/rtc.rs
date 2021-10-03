#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    pub crh: crate::Reg<crh::CRH_SPEC>,
    #[doc = "0x04 - RTC Control Register Low"]
    pub crl: crate::Reg<crl::CRL_SPEC>,
    #[doc = "0x08 - RTC Prescaler Load Register High"]
    pub prlh: crate::Reg<prlh::PRLH_SPEC>,
    #[doc = "0x0c - RTC Prescaler Load Register Low"]
    pub prll: crate::Reg<prll::PRLL_SPEC>,
    #[doc = "0x10 - RTC Prescaler Divider Register High"]
    pub divh: crate::Reg<divh::DIVH_SPEC>,
    #[doc = "0x14 - RTC Prescaler Divider Register Low"]
    pub divl: crate::Reg<divl::DIVL_SPEC>,
    #[doc = "0x18 - RTC Counter Register High"]
    pub cnth: crate::Reg<cnth::CNTH_SPEC>,
    #[doc = "0x1c - RTC Counter Register Low"]
    pub cntl: crate::Reg<cntl::CNTL_SPEC>,
    #[doc = "0x20 - RTC Alarm Register High"]
    pub alrh: crate::Reg<alrh::ALRH_SPEC>,
    #[doc = "0x24 - RTC Alarm Register Low"]
    pub alrl: crate::Reg<alrl::ALRL_SPEC>,
}
#[doc = "CRH register accessor: an alias for `Reg<CRH_SPEC>`"]
pub type CRH = crate::Reg<crh::CRH_SPEC>;
#[doc = "RTC Control Register High"]
pub mod crh;
#[doc = "CRL register accessor: an alias for `Reg<CRL_SPEC>`"]
pub type CRL = crate::Reg<crl::CRL_SPEC>;
#[doc = "RTC Control Register Low"]
pub mod crl;
#[doc = "PRLH register accessor: an alias for `Reg<PRLH_SPEC>`"]
pub type PRLH = crate::Reg<prlh::PRLH_SPEC>;
#[doc = "RTC Prescaler Load Register High"]
pub mod prlh;
#[doc = "PRLL register accessor: an alias for `Reg<PRLL_SPEC>`"]
pub type PRLL = crate::Reg<prll::PRLL_SPEC>;
#[doc = "RTC Prescaler Load Register Low"]
pub mod prll;
#[doc = "DIVH register accessor: an alias for `Reg<DIVH_SPEC>`"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "RTC Prescaler Divider Register High"]
pub mod divh;
#[doc = "DIVL register accessor: an alias for `Reg<DIVL_SPEC>`"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "RTC Prescaler Divider Register Low"]
pub mod divl;
#[doc = "CNTH register accessor: an alias for `Reg<CNTH_SPEC>`"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "CNTL register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "ALRH register accessor: an alias for `Reg<ALRH_SPEC>`"]
pub type ALRH = crate::Reg<alrh::ALRH_SPEC>;
#[doc = "RTC Alarm Register High"]
pub mod alrh;
#[doc = "ALRL register accessor: an alias for `Reg<ALRL_SPEC>`"]
pub type ALRL = crate::Reg<alrl::ALRL_SPEC>;
#[doc = "RTC Alarm Register Low"]
pub mod alrl;
