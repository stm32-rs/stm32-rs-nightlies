#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    icr: ICR,
    ascr1: ASCR1,
    ascr2: ASCR2,
    hyscr1: HYSCR1,
    hyscr2: HYSCR2,
    hyscr3: HYSCR3,
    hyscr4: HYSCR4,
}
impl RegisterBlock {
    #[doc = "0x04 - RI input capture register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x08 - RI analog switches control register 1"]
    #[inline(always)]
    pub const fn ascr1(&self) -> &ASCR1 {
        &self.ascr1
    }
    #[doc = "0x0c - RI analog switches control register 2"]
    #[inline(always)]
    pub const fn ascr2(&self) -> &ASCR2 {
        &self.ascr2
    }
    #[doc = "0x10 - RI hysteresis control register 1"]
    #[inline(always)]
    pub const fn hyscr1(&self) -> &HYSCR1 {
        &self.hyscr1
    }
    #[doc = "0x14 - RI hysteresis control register 2"]
    #[inline(always)]
    pub const fn hyscr2(&self) -> &HYSCR2 {
        &self.hyscr2
    }
    #[doc = "0x18 - RI hysteresis control register 3"]
    #[inline(always)]
    pub const fn hyscr3(&self) -> &HYSCR3 {
        &self.hyscr3
    }
    #[doc = "0x1c - Hysteresis control register"]
    #[inline(always)]
    pub const fn hyscr4(&self) -> &HYSCR4 {
        &self.hyscr4
    }
}
#[doc = "ICR (rw) register accessor: RI input capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "RI input capture register"]
pub mod icr;
#[doc = "ASCR1 (rw) register accessor: RI analog switches control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ascr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ascr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ascr1`]
module"]
pub type ASCR1 = crate::Reg<ascr1::ASCR1rs>;
#[doc = "RI analog switches control register 1"]
pub mod ascr1;
#[doc = "ASCR2 (rw) register accessor: RI analog switches control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ascr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ascr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ascr2`]
module"]
pub type ASCR2 = crate::Reg<ascr2::ASCR2rs>;
#[doc = "RI analog switches control register 2"]
pub mod ascr2;
#[doc = "HYSCR1 (rw) register accessor: RI hysteresis control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hyscr1`]
module"]
pub type HYSCR1 = crate::Reg<hyscr1::HYSCR1rs>;
#[doc = "RI hysteresis control register 1"]
pub mod hyscr1;
#[doc = "HYSCR2 (rw) register accessor: RI hysteresis control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hyscr2`]
module"]
pub type HYSCR2 = crate::Reg<hyscr2::HYSCR2rs>;
#[doc = "RI hysteresis control register 2"]
pub mod hyscr2;
#[doc = "HYSCR3 (rw) register accessor: RI hysteresis control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hyscr3`]
module"]
pub type HYSCR3 = crate::Reg<hyscr3::HYSCR3rs>;
#[doc = "RI hysteresis control register 3"]
pub mod hyscr3;
#[doc = "HYSCR4 (rw) register accessor: Hysteresis control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hyscr4`]
module"]
pub type HYSCR4 = crate::Reg<hyscr4::HYSCR4rs>;
#[doc = "Hysteresis control register"]
pub mod hyscr4;
