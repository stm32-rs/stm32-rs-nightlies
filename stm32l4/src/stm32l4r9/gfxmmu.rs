#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    fcr: FCR,
    _reserved3: [u8; 0x04],
    dvr: DVR,
    _reserved4: [u8; 0x0c],
    b0cr: B0CR,
    b1cr: B1CR,
    b2cr: B2CR,
    b3cr: B3CR,
    _reserved8: [u8; 0x0fc4],
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
    lut: [LUT; 1024],
}
impl RegisterBlock {
    #[doc = "0x00 - Graphic MMU configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Graphic MMU status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - Graphic MMU flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x10 - Graphic MMU default value register"]
    #[inline(always)]
    pub const fn dvr(&self) -> &DVR {
        &self.dvr
    }
    #[doc = "0x20 - Graphic MMU buffer 0 configuration register"]
    #[inline(always)]
    pub const fn b0cr(&self) -> &B0CR {
        &self.b0cr
    }
    #[doc = "0x24 - Graphic MMU buffer 1 configuration register"]
    #[inline(always)]
    pub const fn b1cr(&self) -> &B1CR {
        &self.b1cr
    }
    #[doc = "0x28 - Graphic MMU buffer 2 configuration register"]
    #[inline(always)]
    pub const fn b2cr(&self) -> &B2CR {
        &self.b2cr
    }
    #[doc = "0x2c - Graphic MMU buffer 3 configuration register"]
    #[inline(always)]
    pub const fn b3cr(&self) -> &B3CR {
        &self.b3cr
    }
    #[doc = "0xff4 - Graphic MMU version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0xff8 - Graphic MMU identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0xffc - Graphic MMU size identification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
    #[doc = "0x1000..0x3000 - Cluster LUT%s, containing LUT*L, LUT*H"]
    #[inline(always)]
    pub const fn lut(&self, n: usize) -> &LUT {
        &self.lut[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x3000 - Cluster LUT%s, containing LUT*L, LUT*H"]
    #[inline(always)]
    pub fn lut_iter(&self) -> impl Iterator<Item = &LUT> {
        self.lut.iter()
    }
}
#[doc = "CR (rw) register accessor: Graphic MMU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Graphic MMU configuration register"]
pub mod cr;
#[doc = "SR (r) register accessor: Graphic MMU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Graphic MMU status register"]
pub mod sr;
#[doc = "FCR (w) register accessor: Graphic MMU flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "Graphic MMU flag clear register"]
pub mod fcr;
#[doc = "DVR (rw) register accessor: Graphic MMU default value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvr`]
module"]
pub type DVR = crate::Reg<dvr::DVRrs>;
#[doc = "Graphic MMU default value register"]
pub mod dvr;
#[doc = "B0CR (rw) register accessor: Graphic MMU buffer 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0cr`]
module"]
pub type B0CR = crate::Reg<b0cr::B0CRrs>;
#[doc = "Graphic MMU buffer 0 configuration register"]
pub mod b0cr;
#[doc = "B1CR (rw) register accessor: Graphic MMU buffer 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b1cr`]
module"]
pub type B1CR = crate::Reg<b1cr::B1CRrs>;
#[doc = "Graphic MMU buffer 1 configuration register"]
pub mod b1cr;
#[doc = "B2CR (rw) register accessor: Graphic MMU buffer 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2cr`]
module"]
pub type B2CR = crate::Reg<b2cr::B2CRrs>;
#[doc = "Graphic MMU buffer 2 configuration register"]
pub mod b2cr;
#[doc = "B3CR (rw) register accessor: Graphic MMU buffer 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b3cr`]
module"]
pub type B3CR = crate::Reg<b3cr::B3CRrs>;
#[doc = "Graphic MMU buffer 3 configuration register"]
pub mod b3cr;
#[doc = "VERR (r) register accessor: Graphic MMU version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "Graphic MMU version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: Graphic MMU identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "Graphic MMU identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: Graphic MMU size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "Graphic MMU size identification register"]
pub mod sidr;
#[doc = "Cluster LUT%s, containing LUT*L, LUT*H"]
pub use self::lut::LUT;
#[doc = r"Cluster"]
#[doc = "Cluster LUT%s, containing LUT*L, LUT*H"]
pub mod lut;
