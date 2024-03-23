#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ier: IER,
    _reserved1: [u8; 0x1c],
    m1cr: M1CR,
    m1sr: M1SR,
    m1far: M1FAR,
    m1fdrl: M1FDRL,
    m1fdrh: M1FDRH,
    m1fecr: M1FECR,
    _reserved7: [u8; 0x08],
    m2cr: M2CR,
    m2sr: M2SR,
    m2far: M2FAR,
    m2fdrl: M2FDRL,
    m2fdrh: M2FDRH,
    m2fecr: M2FECR,
    _reserved13: [u8; 0x08],
    m3cr: M3CR,
    m3sr: M3SR,
    m3far: M3FAR,
    m3fdrl: M3FDRL,
    m3fdrh: M3FDRH,
    m3fecr: M3FECR,
    _reserved19: [u8; 0x08],
    m4cr: M4CR,
    m4sr: M4SR,
    m4far: M4FAR,
    m4fdrl: M4FDRL,
    m4fdrh: M4FDRH,
    m4fecr: M4FECR,
    _reserved25: [u8; 0x08],
    m5cr: M5CR,
    m5sr: M5SR,
    m5far: M5FAR,
    m5fdrl: M5FDRL,
    m5fdrh: M5FDRH,
    m5fecr: M5FECR,
}
impl RegisterBlock {
    #[doc = "0x00 - RAMECC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x20 - RAMECC monitor 1 configuration register"]
    #[inline(always)]
    pub const fn m1cr(&self) -> &M1CR {
        &self.m1cr
    }
    #[doc = "0x24 - RAMECC monitor 1 status register"]
    #[inline(always)]
    pub const fn m1sr(&self) -> &M1SR {
        &self.m1sr
    }
    #[doc = "0x28 - RAMECC monitor 1 failing address register"]
    #[inline(always)]
    pub const fn m1far(&self) -> &M1FAR {
        &self.m1far
    }
    #[doc = "0x2c - RAMECC monitor 1 failing data low register"]
    #[inline(always)]
    pub const fn m1fdrl(&self) -> &M1FDRL {
        &self.m1fdrl
    }
    #[doc = "0x30 - RAMECC monitor 1 failing data high register"]
    #[inline(always)]
    pub const fn m1fdrh(&self) -> &M1FDRH {
        &self.m1fdrh
    }
    #[doc = "0x34 - RAMECC monitor 1 failing error code register"]
    #[inline(always)]
    pub const fn m1fecr(&self) -> &M1FECR {
        &self.m1fecr
    }
    #[doc = "0x40 - RAMECC monitor 2 configuration register"]
    #[inline(always)]
    pub const fn m2cr(&self) -> &M2CR {
        &self.m2cr
    }
    #[doc = "0x44 - RAMECC monitor 2 status register"]
    #[inline(always)]
    pub const fn m2sr(&self) -> &M2SR {
        &self.m2sr
    }
    #[doc = "0x48 - RAMECC monitor 2 failing address register"]
    #[inline(always)]
    pub const fn m2far(&self) -> &M2FAR {
        &self.m2far
    }
    #[doc = "0x4c - RAMECC monitor 2 failing data low register"]
    #[inline(always)]
    pub const fn m2fdrl(&self) -> &M2FDRL {
        &self.m2fdrl
    }
    #[doc = "0x50 - RAMECC monitor 2 failing data high register"]
    #[inline(always)]
    pub const fn m2fdrh(&self) -> &M2FDRH {
        &self.m2fdrh
    }
    #[doc = "0x54 - RAMECC monitor 2 failing error code register"]
    #[inline(always)]
    pub const fn m2fecr(&self) -> &M2FECR {
        &self.m2fecr
    }
    #[doc = "0x60 - RAMECC monitor 3 configuration register"]
    #[inline(always)]
    pub const fn m3cr(&self) -> &M3CR {
        &self.m3cr
    }
    #[doc = "0x64 - RAMECC monitor 3 status register"]
    #[inline(always)]
    pub const fn m3sr(&self) -> &M3SR {
        &self.m3sr
    }
    #[doc = "0x68 - RAMECC monitor 3 failing address register"]
    #[inline(always)]
    pub const fn m3far(&self) -> &M3FAR {
        &self.m3far
    }
    #[doc = "0x6c - RAMECC monitor 3 failing data low register"]
    #[inline(always)]
    pub const fn m3fdrl(&self) -> &M3FDRL {
        &self.m3fdrl
    }
    #[doc = "0x70 - RAMECC monitor 3 failing data high register"]
    #[inline(always)]
    pub const fn m3fdrh(&self) -> &M3FDRH {
        &self.m3fdrh
    }
    #[doc = "0x74 - RAMECC monitor 3 failing error code register"]
    #[inline(always)]
    pub const fn m3fecr(&self) -> &M3FECR {
        &self.m3fecr
    }
    #[doc = "0x80 - RAMECC monitor 4 configuration register"]
    #[inline(always)]
    pub const fn m4cr(&self) -> &M4CR {
        &self.m4cr
    }
    #[doc = "0x84 - RAMECC monitor 4 status register"]
    #[inline(always)]
    pub const fn m4sr(&self) -> &M4SR {
        &self.m4sr
    }
    #[doc = "0x88 - RAMECC monitor 4 failing address register"]
    #[inline(always)]
    pub const fn m4far(&self) -> &M4FAR {
        &self.m4far
    }
    #[doc = "0x8c - RAMECC monitor 4 failing data low register"]
    #[inline(always)]
    pub const fn m4fdrl(&self) -> &M4FDRL {
        &self.m4fdrl
    }
    #[doc = "0x90 - RAMECC monitor 4 failing data high register"]
    #[inline(always)]
    pub const fn m4fdrh(&self) -> &M4FDRH {
        &self.m4fdrh
    }
    #[doc = "0x94 - RAMECC monitor 4 failing error code register"]
    #[inline(always)]
    pub const fn m4fecr(&self) -> &M4FECR {
        &self.m4fecr
    }
    #[doc = "0xa0 - RAMECC monitor 5 configuration register"]
    #[inline(always)]
    pub const fn m5cr(&self) -> &M5CR {
        &self.m5cr
    }
    #[doc = "0xa4 - RAMECC monitor 5 status register"]
    #[inline(always)]
    pub const fn m5sr(&self) -> &M5SR {
        &self.m5sr
    }
    #[doc = "0xa8 - RAMECC monitor 5 failing address register"]
    #[inline(always)]
    pub const fn m5far(&self) -> &M5FAR {
        &self.m5far
    }
    #[doc = "0xac - RAMECC monitor 5 failing data low register"]
    #[inline(always)]
    pub const fn m5fdrl(&self) -> &M5FDRL {
        &self.m5fdrl
    }
    #[doc = "0xb0 - RAMECC monitor 5 failing data high register"]
    #[inline(always)]
    pub const fn m5fdrh(&self) -> &M5FDRH {
        &self.m5fdrh
    }
    #[doc = "0xb4 - RAMECC monitor 5 failing error code register"]
    #[inline(always)]
    pub const fn m5fecr(&self) -> &M5FECR {
        &self.m5fecr
    }
}
#[doc = "IER (rw) register accessor: RAMECC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "RAMECC interrupt enable register"]
pub mod ier;
#[doc = "M1CR (rw) register accessor: RAMECC monitor 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1cr`]
module"]
pub type M1CR = crate::Reg<m1cr::M1CRrs>;
#[doc = "RAMECC monitor 1 configuration register"]
pub mod m1cr;
#[doc = "M1SR (rw) register accessor: RAMECC monitor 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1sr`]
module"]
pub type M1SR = crate::Reg<m1sr::M1SRrs>;
#[doc = "RAMECC monitor 1 status register"]
pub mod m1sr;
#[doc = "M1FAR (rw) register accessor: RAMECC monitor 1 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1far`]
module"]
pub type M1FAR = crate::Reg<m1far::M1FARrs>;
#[doc = "RAMECC monitor 1 failing address register"]
pub mod m1far;
#[doc = "M1FDRL (rw) register accessor: RAMECC monitor 1 failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fdrl`]
module"]
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRLrs>;
#[doc = "RAMECC monitor 1 failing data low register"]
pub mod m1fdrl;
#[doc = "M1FDRH (rw) register accessor: RAMECC monitor 1 failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fdrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fdrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fdrh`]
module"]
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRHrs>;
#[doc = "RAMECC monitor 1 failing data high register"]
pub mod m1fdrh;
#[doc = "M1FECR (rw) register accessor: RAMECC monitor 1 failing error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fecr`]
module"]
pub type M1FECR = crate::Reg<m1fecr::M1FECRrs>;
#[doc = "RAMECC monitor 1 failing error code register"]
pub mod m1fecr;
#[doc = "M2CR (rw) register accessor: RAMECC monitor 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2cr`]
module"]
pub type M2CR = crate::Reg<m2cr::M2CRrs>;
#[doc = "RAMECC monitor 2 configuration register"]
pub mod m2cr;
#[doc = "M2SR (rw) register accessor: RAMECC monitor 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2sr`]
module"]
pub type M2SR = crate::Reg<m2sr::M2SRrs>;
#[doc = "RAMECC monitor 2 status register"]
pub mod m2sr;
#[doc = "M2FAR (rw) register accessor: RAMECC monitor 2 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2far`]
module"]
pub type M2FAR = crate::Reg<m2far::M2FARrs>;
#[doc = "RAMECC monitor 2 failing address register"]
pub mod m2far;
#[doc = "M2FDRL (rw) register accessor: RAMECC monitor 2 failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fdrl`]
module"]
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRLrs>;
#[doc = "RAMECC monitor 2 failing data low register"]
pub mod m2fdrl;
#[doc = "M2FDRH (rw) register accessor: RAMECC monitor 2 failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fdrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2fdrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fdrh`]
module"]
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRHrs>;
#[doc = "RAMECC monitor 2 failing data high register"]
pub mod m2fdrh;
#[doc = "M2FECR (rw) register accessor: RAMECC monitor 2 failing error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2fecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fecr`]
module"]
pub type M2FECR = crate::Reg<m2fecr::M2FECRrs>;
#[doc = "RAMECC monitor 2 failing error code register"]
pub mod m2fecr;
#[doc = "M3CR (rw) register accessor: RAMECC monitor 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3cr`]
module"]
pub type M3CR = crate::Reg<m3cr::M3CRrs>;
#[doc = "RAMECC monitor 3 configuration register"]
pub mod m3cr;
#[doc = "M3SR (rw) register accessor: RAMECC monitor 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3sr`]
module"]
pub type M3SR = crate::Reg<m3sr::M3SRrs>;
#[doc = "RAMECC monitor 3 status register"]
pub mod m3sr;
#[doc = "M3FAR (rw) register accessor: RAMECC monitor 3 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3far`]
module"]
pub type M3FAR = crate::Reg<m3far::M3FARrs>;
#[doc = "RAMECC monitor 3 failing address register"]
pub mod m3far;
#[doc = "M3FDRL (rw) register accessor: RAMECC monitor 3 failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3fdrl`]
module"]
pub type M3FDRL = crate::Reg<m3fdrl::M3FDRLrs>;
#[doc = "RAMECC monitor 3 failing data low register"]
pub mod m3fdrl;
#[doc = "M3FDRH (rw) register accessor: RAMECC monitor 3 failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3fdrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3fdrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3fdrh`]
module"]
pub type M3FDRH = crate::Reg<m3fdrh::M3FDRHrs>;
#[doc = "RAMECC monitor 3 failing data high register"]
pub mod m3fdrh;
#[doc = "M3FECR (rw) register accessor: RAMECC monitor 3 failing error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3fecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3fecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3fecr`]
module"]
pub type M3FECR = crate::Reg<m3fecr::M3FECRrs>;
#[doc = "RAMECC monitor 3 failing error code register"]
pub mod m3fecr;
#[doc = "M4CR (rw) register accessor: RAMECC monitor 4 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4cr`]
module"]
pub type M4CR = crate::Reg<m4cr::M4CRrs>;
#[doc = "RAMECC monitor 4 configuration register"]
pub mod m4cr;
#[doc = "M4SR (rw) register accessor: RAMECC monitor 4 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4sr`]
module"]
pub type M4SR = crate::Reg<m4sr::M4SRrs>;
#[doc = "RAMECC monitor 4 status register"]
pub mod m4sr;
#[doc = "M4FAR (rw) register accessor: RAMECC monitor 4 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4far`]
module"]
pub type M4FAR = crate::Reg<m4far::M4FARrs>;
#[doc = "RAMECC monitor 4 failing address register"]
pub mod m4far;
#[doc = "M4FDRL (rw) register accessor: RAMECC monitor 4 failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4fdrl`]
module"]
pub type M4FDRL = crate::Reg<m4fdrl::M4FDRLrs>;
#[doc = "RAMECC monitor 4 failing data low register"]
pub mod m4fdrl;
#[doc = "M4FDRH (rw) register accessor: RAMECC monitor 4 failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fdrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4fdrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4fdrh`]
module"]
pub type M4FDRH = crate::Reg<m4fdrh::M4FDRHrs>;
#[doc = "RAMECC monitor 4 failing data high register"]
pub mod m4fdrh;
#[doc = "M4FECR (rw) register accessor: RAMECC monitor 4 failing error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4fecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4fecr`]
module"]
pub type M4FECR = crate::Reg<m4fecr::M4FECRrs>;
#[doc = "RAMECC monitor 4 failing error code register"]
pub mod m4fecr;
#[doc = "M5CR (rw) register accessor: RAMECC monitor 5 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5cr`]
module"]
pub type M5CR = crate::Reg<m5cr::M5CRrs>;
#[doc = "RAMECC monitor 5 configuration register"]
pub mod m5cr;
#[doc = "M5SR (rw) register accessor: RAMECC monitor 5 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5sr`]
module"]
pub type M5SR = crate::Reg<m5sr::M5SRrs>;
#[doc = "RAMECC monitor 5 status register"]
pub mod m5sr;
#[doc = "M5FAR (rw) register accessor: RAMECC monitor 5 failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5far`]
module"]
pub type M5FAR = crate::Reg<m5far::M5FARrs>;
#[doc = "RAMECC monitor 5 failing address register"]
pub mod m5far;
#[doc = "M5FDRL (rw) register accessor: RAMECC monitor 5 failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5fdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5fdrl`]
module"]
pub type M5FDRL = crate::Reg<m5fdrl::M5FDRLrs>;
#[doc = "RAMECC monitor 5 failing data low register"]
pub mod m5fdrl;
#[doc = "M5FDRH (rw) register accessor: RAMECC monitor 5 failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fdrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5fdrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5fdrh`]
module"]
pub type M5FDRH = crate::Reg<m5fdrh::M5FDRHrs>;
#[doc = "RAMECC monitor 5 failing data high register"]
pub mod m5fdrh;
#[doc = "M5FECR (rw) register accessor: RAMECC monitor 5 failing error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5fecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5fecr`]
module"]
pub type M5FECR = crate::Reg<m5fecr::M5FECRrs>;
#[doc = "RAMECC monitor 5 failing error code register"]
pub mod m5fecr;
