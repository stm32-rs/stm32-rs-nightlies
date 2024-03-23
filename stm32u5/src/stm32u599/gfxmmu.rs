#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    fcr: FCR,
    ccr: CCR,
    dvr: DVR,
    _reserved5: [u8; 0x0c],
    b0cr: B0CR,
    b1cr: B1CR,
    b2cr: B2CR,
    b3cr: B3CR,
    _reserved9: [u8; 0x0fd0],
    lut: [LUT; 1024],
}
impl RegisterBlock {
    #[doc = "0x00 - GFXMMU configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - GFXMMU status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - GFXMMU flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x0c - GFXMMU cache control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x10 - GFXMMU default value register"]
    #[inline(always)]
    pub const fn dvr(&self) -> &DVR {
        &self.dvr
    }
    #[doc = "0x20 - GFXMMU buffer 0 configuration register"]
    #[inline(always)]
    pub const fn b0cr(&self) -> &B0CR {
        &self.b0cr
    }
    #[doc = "0x24 - GFXMMU buffer 1 configuration register"]
    #[inline(always)]
    pub const fn b1cr(&self) -> &B1CR {
        &self.b1cr
    }
    #[doc = "0x28 - GFXMMU buffer 2 configuration register"]
    #[inline(always)]
    pub const fn b2cr(&self) -> &B2CR {
        &self.b2cr
    }
    #[doc = "0x2c - GFXMMU buffer 3 configuration register"]
    #[inline(always)]
    pub const fn b3cr(&self) -> &B3CR {
        &self.b3cr
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
#[doc = "CR (rw) register accessor: GFXMMU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "GFXMMU configuration register"]
pub mod cr;
#[doc = "SR (r) register accessor: GFXMMU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "GFXMMU status register"]
pub mod sr;
#[doc = "FCR (rw) register accessor: GFXMMU flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "GFXMMU flag clear register"]
pub mod fcr;
#[doc = "CCR (rw) register accessor: GFXMMU cache control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "GFXMMU cache control register"]
pub mod ccr;
#[doc = "DVR (rw) register accessor: GFXMMU default value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvr`]
module"]
pub type DVR = crate::Reg<dvr::DVRrs>;
#[doc = "GFXMMU default value register"]
pub mod dvr;
#[doc = "B0CR (rw) register accessor: GFXMMU buffer 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0cr`]
module"]
pub type B0CR = crate::Reg<b0cr::B0CRrs>;
#[doc = "GFXMMU buffer 0 configuration register"]
pub mod b0cr;
#[doc = "B1CR (rw) register accessor: GFXMMU buffer 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b1cr`]
module"]
pub type B1CR = crate::Reg<b1cr::B1CRrs>;
#[doc = "GFXMMU buffer 1 configuration register"]
pub mod b1cr;
#[doc = "B2CR (rw) register accessor: GFXMMU buffer 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2cr`]
module"]
pub type B2CR = crate::Reg<b2cr::B2CRrs>;
#[doc = "GFXMMU buffer 2 configuration register"]
pub mod b2cr;
#[doc = "B3CR (rw) register accessor: GFXMMU buffer 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b3cr`]
module"]
pub type B3CR = crate::Reg<b3cr::B3CRrs>;
#[doc = "GFXMMU buffer 3 configuration register"]
pub mod b3cr;
#[doc = "Cluster LUT%s, containing LUT*L, LUT*H"]
pub use self::lut::LUT;
#[doc = r"Cluster"]
#[doc = "Cluster LUT%s, containing LUT*L, LUT*H"]
pub mod lut;
