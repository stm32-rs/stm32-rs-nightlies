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
    _reserved13: [u8; 0x04],
    #[doc = "0x60 - RAMECC monitor x configuration register"]
    pub m3cr: crate::Reg<m3cr::M3CR_SPEC>,
    #[doc = "0x64 - RAMECC monitor x status register"]
    pub m3sr: crate::Reg<m3sr::M3SR_SPEC>,
    #[doc = "0x68 - RAMECC monitor x failing address register"]
    pub m3far: crate::Reg<m3far::M3FAR_SPEC>,
    #[doc = "0x6c - RAMECC monitor x failing data low register"]
    pub m3fdrl: crate::Reg<m3fdrl::M3FDRL_SPEC>,
    #[doc = "0x70 - RAMECC monitor x failing data high register"]
    pub m3fdrh: crate::Reg<m3fdrh::M3FDRH_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x7c - RAMECC monitor x failing ECC error code register"]
    pub m3fecr: crate::Reg<m3fecr::M3FECR_SPEC>,
    #[doc = "0x80 - RAMECC monitor x configuration register"]
    pub m4cr: crate::Reg<m4cr::M4CR_SPEC>,
    #[doc = "0x84 - RAMECC monitor x status register"]
    pub m4sr: crate::Reg<m4sr::M4SR_SPEC>,
    #[doc = "0x88 - RAMECC monitor x failing address register"]
    pub m4far: crate::Reg<m4far::M4FAR_SPEC>,
    #[doc = "0x8c - RAMECC monitor x failing data low register"]
    pub m4fdrl: crate::Reg<m4fdrl::M4FDRL_SPEC>,
    _reserved_23_m: [u8; 0x04],
    _reserved24: [u8; 0x0c],
    #[doc = "0xa0 - RAMECC monitor x configuration register"]
    pub m5cr: crate::Reg<m5cr::M5CR_SPEC>,
    #[doc = "0xa4 - RAMECC monitor x status register"]
    pub m5sr: crate::Reg<m5sr::M5SR_SPEC>,
    #[doc = "0xa8 - RAMECC monitor x failing address register"]
    pub m5far: crate::Reg<m5far::M5FAR_SPEC>,
    #[doc = "0xac - RAMECC monitor x failing data low register"]
    pub m5fdrl: crate::Reg<m5fdrl::M5FDRL_SPEC>,
    #[doc = "0xb0 - RAMECC monitor x failing data high register"]
    pub m5fdrh: crate::Reg<m5fdrh::M5FDRH_SPEC>,
    #[doc = "0xb4 - RAMECC monitor x failing ECC error code register"]
    pub m5fecr: crate::Reg<m5fecr::M5FECR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x90 - RAMECC monitor x failing ECC error code register"]
    #[inline(always)]
    pub fn m4fecr(&self) -> &crate::Reg<m4fecr::M4FECR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(144usize)
                as *const crate::Reg<m4fecr::M4FECR_SPEC>)
        }
    }
    #[doc = "0x90 - RAMECC monitor x failing data high register"]
    #[inline(always)]
    pub fn m4fdrh(&self) -> &crate::Reg<m4fdrh::M4FDRH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(144usize)
                as *const crate::Reg<m4fdrh::M4FDRH_SPEC>)
        }
    }
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
#[doc = "M3CR register accessor: an alias for `Reg<M3CR_SPEC>`"]
pub type M3CR = crate::Reg<m3cr::M3CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m3cr;
#[doc = "M4CR register accessor: an alias for `Reg<M4CR_SPEC>`"]
pub type M4CR = crate::Reg<m4cr::M4CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m4cr;
#[doc = "M5CR register accessor: an alias for `Reg<M5CR_SPEC>`"]
pub type M5CR = crate::Reg<m5cr::M5CR_SPEC>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m5cr;
#[doc = "M1SR register accessor: an alias for `Reg<M1SR_SPEC>`"]
pub type M1SR = crate::Reg<m1sr::M1SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m1sr;
#[doc = "M2SR register accessor: an alias for `Reg<M2SR_SPEC>`"]
pub type M2SR = crate::Reg<m2sr::M2SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m2sr;
#[doc = "M3SR register accessor: an alias for `Reg<M3SR_SPEC>`"]
pub type M3SR = crate::Reg<m3sr::M3SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m3sr;
#[doc = "M4SR register accessor: an alias for `Reg<M4SR_SPEC>`"]
pub type M4SR = crate::Reg<m4sr::M4SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m4sr;
#[doc = "M5SR register accessor: an alias for `Reg<M5SR_SPEC>`"]
pub type M5SR = crate::Reg<m5sr::M5SR_SPEC>;
#[doc = "RAMECC monitor x status register"]
pub mod m5sr;
#[doc = "M1FAR register accessor: an alias for `Reg<M1FAR_SPEC>`"]
pub type M1FAR = crate::Reg<m1far::M1FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m1far;
#[doc = "M2FAR register accessor: an alias for `Reg<M2FAR_SPEC>`"]
pub type M2FAR = crate::Reg<m2far::M2FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m2far;
#[doc = "M3FAR register accessor: an alias for `Reg<M3FAR_SPEC>`"]
pub type M3FAR = crate::Reg<m3far::M3FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m3far;
#[doc = "M4FAR register accessor: an alias for `Reg<M4FAR_SPEC>`"]
pub type M4FAR = crate::Reg<m4far::M4FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m4far;
#[doc = "M5FAR register accessor: an alias for `Reg<M5FAR_SPEC>`"]
pub type M5FAR = crate::Reg<m5far::M5FAR_SPEC>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m5far;
#[doc = "M1FDRL register accessor: an alias for `Reg<M1FDRL_SPEC>`"]
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m1fdrl;
#[doc = "M2FDRL register accessor: an alias for `Reg<M2FDRL_SPEC>`"]
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m2fdrl;
#[doc = "M3FDRL register accessor: an alias for `Reg<M3FDRL_SPEC>`"]
pub type M3FDRL = crate::Reg<m3fdrl::M3FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m3fdrl;
#[doc = "M4FDRL register accessor: an alias for `Reg<M4FDRL_SPEC>`"]
pub type M4FDRL = crate::Reg<m4fdrl::M4FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m4fdrl;
#[doc = "M5FDRL register accessor: an alias for `Reg<M5FDRL_SPEC>`"]
pub type M5FDRL = crate::Reg<m5fdrl::M5FDRL_SPEC>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m5fdrl;
#[doc = "M1FDRH register accessor: an alias for `Reg<M1FDRH_SPEC>`"]
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m1fdrh;
#[doc = "M2FDRH register accessor: an alias for `Reg<M2FDRH_SPEC>`"]
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m2fdrh;
#[doc = "M3FDRH register accessor: an alias for `Reg<M3FDRH_SPEC>`"]
pub type M3FDRH = crate::Reg<m3fdrh::M3FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m3fdrh;
#[doc = "M4FDRH register accessor: an alias for `Reg<M4FDRH_SPEC>`"]
pub type M4FDRH = crate::Reg<m4fdrh::M4FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m4fdrh;
#[doc = "M5FDRH register accessor: an alias for `Reg<M5FDRH_SPEC>`"]
pub type M5FDRH = crate::Reg<m5fdrh::M5FDRH_SPEC>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m5fdrh;
#[doc = "M1FECR register accessor: an alias for `Reg<M1FECR_SPEC>`"]
pub type M1FECR = crate::Reg<m1fecr::M1FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m1fecr;
#[doc = "M2FECR register accessor: an alias for `Reg<M2FECR_SPEC>`"]
pub type M2FECR = crate::Reg<m2fecr::M2FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m2fecr;
#[doc = "M3FECR register accessor: an alias for `Reg<M3FECR_SPEC>`"]
pub type M3FECR = crate::Reg<m3fecr::M3FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m3fecr;
#[doc = "M4FECR register accessor: an alias for `Reg<M4FECR_SPEC>`"]
pub type M4FECR = crate::Reg<m4fecr::M4FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m4fecr;
#[doc = "M5FECR register accessor: an alias for `Reg<M5FECR_SPEC>`"]
pub type M5FECR = crate::Reg<m5fecr::M5FECR_SPEC>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m5fecr;
