#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RAMECC interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - RAMECC monitor x configuration register"]
    pub m1cr: crate::Reg<m1cr::M1CR_SPEC>,
    #[doc = "0x24 - RAMECC monitor x status register"]
    pub m1sr: crate::Reg<m1sr::M1SR_SPEC>,
    #[doc = "0x28 - RAMECC monitor x failing address register"]
    pub m1far: crate::Reg<m1far::M1FAR_SPEC>,
    #[doc = "0x2c - RAMECC monitor x failing data low register"]
    pub m1fdrl: crate::Reg<m1fdrl::M1FDRL_SPEC>,
    #[doc = "0x30 - RAMECC monitor x failing data high register"]
    pub m1fdrh: crate::Reg<m1fdrh::M1FDRH_SPEC>,
    #[doc = "0x34 - RAMECC monitor x failing ECC error code register"]
    pub m1fecr: crate::Reg<m1fecr::M1FECR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x40 - RAMECC monitor x configuration register"]
    pub m2cr: crate::Reg<m2cr::M2CR_SPEC>,
    #[doc = "0x44 - RAMECC monitor x status register"]
    pub m2sr: crate::Reg<m2sr::M2SR_SPEC>,
    #[doc = "0x48 - RAMECC monitor x failing address register"]
    pub m2far: crate::Reg<m2far::M2FAR_SPEC>,
    #[doc = "0x4c - RAMECC monitor x failing data low register"]
    pub m2fdrl: crate::Reg<m2fdrl::M2FDRL_SPEC>,
    #[doc = "0x50 - RAMECC monitor x failing data high register"]
    pub m2fdrh: crate::Reg<m2fdrh::M2FDRH_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x58 - RAMECC monitor x failing ECC error code register"]
    pub m2fecr: crate::Reg<m2fecr::M2FECR_SPEC>,
}
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "RAMECC interrupt enable register"]
pub mod ier;
#[doc = "M1CR register accessor: an alias for `Reg<M1CR_SPEC>`"]
pub type M1CR = crate::Reg<m1cr::M1CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m1cr;
#[doc = "M2CR register accessor: an alias for `Reg<M2CR_SPEC>`"]
pub type M2CR = crate::Reg<m2cr::M2CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m2cr;
#[doc = "M1SR register accessor: an alias for `Reg<M1SR_SPEC>`"]
pub type M1SR = crate::Reg<m1sr::M1SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m1sr;
#[doc = "M2SR register accessor: an alias for `Reg<M2SR_SPEC>`"]
pub type M2SR = crate::Reg<m2sr::M2SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m2sr;
#[doc = "M1FAR register accessor: an alias for `Reg<M1FAR_SPEC>`"]
pub type M1FAR = crate::Reg<m1far::M1FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m1far;
#[doc = "M2FAR register accessor: an alias for `Reg<M2FAR_SPEC>`"]
pub type M2FAR = crate::Reg<m2far::M2FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m2far;
#[doc = "M1FDRL register accessor: an alias for `Reg<M1FDRL_SPEC>`"]
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m1fdrl;
#[doc = "M2FDRL register accessor: an alias for `Reg<M2FDRL_SPEC>`"]
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m2fdrl;
#[doc = "M1FDRH register accessor: an alias for `Reg<M1FDRH_SPEC>`"]
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m1fdrh;
#[doc = "M2FDRH register accessor: an alias for `Reg<M2FDRH_SPEC>`"]
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m2fdrh;
#[doc = "M1FECR register accessor: an alias for `Reg<M1FECR_SPEC>`"]
pub type M1FECR = crate::Reg<m1fecr::M1FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m1fecr;
#[doc = "M2FECR register accessor: an alias for `Reg<M2FECR_SPEC>`"]
pub type M2FECR = crate::Reg<m2fecr::M2FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m2fecr;
