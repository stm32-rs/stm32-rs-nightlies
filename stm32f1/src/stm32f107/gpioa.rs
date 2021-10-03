#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CRL)"]
    pub crl: crate::Reg<crl::CRL_SPEC>,
    #[doc = "0x04 - Port configuration register high (GPIOn_CRL)"]
    pub crh: crate::Reg<crh::CRH_SPEC>,
    #[doc = "0x08 - Port input data register (GPIOn_IDR)"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x0c - Port output data register (GPIOn_ODR)"]
    pub odr: crate::Reg<odr::ODR_SPEC>,
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSRR)"]
    pub bsrr: crate::Reg<bsrr::BSRR_SPEC>,
    #[doc = "0x14 - Port bit reset register (GPIOn_BRR)"]
    pub brr: crate::Reg<brr::BRR_SPEC>,
    #[doc = "0x18 - Port configuration lock register"]
    pub lckr: crate::Reg<lckr::LCKR_SPEC>,
}
#[doc = "CRL register accessor: an alias for `Reg<CRL_SPEC>`"]
pub type CRL = crate::Reg<crl::CRL_SPEC>;
#[doc = "Port configuration register low (GPIOn_CRL)"]
pub mod crl;
#[doc = "CRH register accessor: an alias for `Reg<CRH_SPEC>`"]
pub type CRH = crate::Reg<crh::CRH_SPEC>;
#[doc = "Port configuration register high (GPIOn_CRL)"]
pub mod crh;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Port input data register (GPIOn_IDR)"]
pub mod idr;
#[doc = "ODR register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "Port output data register (GPIOn_ODR)"]
pub mod odr;
#[doc = "BSRR register accessor: an alias for `Reg<BSRR_SPEC>`"]
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
#[doc = "Port bit set/reset register (GPIOn_BSRR)"]
pub mod bsrr;
#[doc = "BRR register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Port bit reset register (GPIOn_BRR)"]
pub mod brr;
#[doc = "LCKR register accessor: an alias for `Reg<LCKR_SPEC>`"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "Port configuration lock register"]
pub mod lckr;
