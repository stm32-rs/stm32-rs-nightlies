#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sr: SR,
    cr1: CR1,
    cr2: CR2,
    smpr1: SMPR1,
    smpr2: SMPR2,
    jofr1: JOFR1,
    jofr2: JOFR2,
    jofr3: JOFR3,
    jofr4: JOFR4,
    htr: HTR,
    ltr: LTR,
    sqr1: SQR1,
    sqr2: SQR2,
    sqr3: SQR3,
    jsqr: JSQR,
    jdr1: JDR1,
    jdr2: JDR2,
    jdr3: JDR3,
    jdr4: JDR4,
    dr: DR,
}
impl RegisterBlock {
    #[doc = "0x00 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x08 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x0c - sample time register 1"]
    #[inline(always)]
    pub const fn smpr1(&self) -> &SMPR1 {
        &self.smpr1
    }
    #[doc = "0x10 - sample time register 2"]
    #[inline(always)]
    pub const fn smpr2(&self) -> &SMPR2 {
        &self.smpr2
    }
    #[doc = "0x14 - injected channel data offset register x"]
    #[inline(always)]
    pub const fn jofr1(&self) -> &JOFR1 {
        &self.jofr1
    }
    #[doc = "0x18 - injected channel data offset register x"]
    #[inline(always)]
    pub const fn jofr2(&self) -> &JOFR2 {
        &self.jofr2
    }
    #[doc = "0x1c - injected channel data offset register x"]
    #[inline(always)]
    pub const fn jofr3(&self) -> &JOFR3 {
        &self.jofr3
    }
    #[doc = "0x20 - injected channel data offset register x"]
    #[inline(always)]
    pub const fn jofr4(&self) -> &JOFR4 {
        &self.jofr4
    }
    #[doc = "0x24 - watchdog higher threshold register"]
    #[inline(always)]
    pub const fn htr(&self) -> &HTR {
        &self.htr
    }
    #[doc = "0x28 - watchdog lower threshold register"]
    #[inline(always)]
    pub const fn ltr(&self) -> &LTR {
        &self.ltr
    }
    #[doc = "0x2c - regular sequence register 1"]
    #[inline(always)]
    pub const fn sqr1(&self) -> &SQR1 {
        &self.sqr1
    }
    #[doc = "0x30 - regular sequence register 2"]
    #[inline(always)]
    pub const fn sqr2(&self) -> &SQR2 {
        &self.sqr2
    }
    #[doc = "0x34 - regular sequence register 3"]
    #[inline(always)]
    pub const fn sqr3(&self) -> &SQR3 {
        &self.sqr3
    }
    #[doc = "0x38 - injected sequence register"]
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    #[doc = "0x3c - injected data register x"]
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR1 {
        &self.jdr1
    }
    #[doc = "0x40 - injected data register x"]
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR2 {
        &self.jdr2
    }
    #[doc = "0x44 - injected data register x"]
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR3 {
        &self.jdr3
    }
    #[doc = "0x48 - injected data register x"]
    #[inline(always)]
    pub const fn jdr4(&self) -> &JDR4 {
        &self.jdr4
    }
    #[doc = "0x4c - regular data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMPR1 (rw) register accessor: sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`]
module"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1rs>;
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr2`]
module"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2rs>;
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "JOFR1 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr1`]
module"]
pub type JOFR1 = crate::Reg<jofr1::JOFR1rs>;
#[doc = "injected channel data offset register x"]
pub mod jofr1;
#[doc = "JOFR2 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr2`]
module"]
pub type JOFR2 = crate::Reg<jofr2::JOFR2rs>;
#[doc = "injected channel data offset register x"]
pub mod jofr2;
#[doc = "JOFR3 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr3`]
module"]
pub type JOFR3 = crate::Reg<jofr3::JOFR3rs>;
#[doc = "injected channel data offset register x"]
pub mod jofr3;
#[doc = "JOFR4 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr4`]
module"]
pub type JOFR4 = crate::Reg<jofr4::JOFR4rs>;
#[doc = "injected channel data offset register x"]
pub mod jofr4;
#[doc = "HTR (rw) register accessor: watchdog higher threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htr`]
module"]
pub type HTR = crate::Reg<htr::HTRrs>;
#[doc = "watchdog higher threshold register"]
pub mod htr;
#[doc = "LTR (rw) register accessor: watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltr`]
module"]
pub type LTR = crate::Reg<ltr::LTRrs>;
#[doc = "watchdog lower threshold register"]
pub mod ltr;
#[doc = "SQR1 (rw) register accessor: regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr1`]
module"]
pub type SQR1 = crate::Reg<sqr1::SQR1rs>;
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr2`]
module"]
pub type SQR2 = crate::Reg<sqr2::SQR2rs>;
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: regular sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr3`]
module"]
pub type SQR3 = crate::Reg<sqr3::SQR3rs>;
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "JSQR (rw) register accessor: injected sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jsqr`]
module"]
pub type JSQR = crate::Reg<jsqr::JSQRrs>;
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "JDR1 (r) register accessor: injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr1`]
module"]
pub type JDR1 = crate::Reg<jdr1::JDR1rs>;
#[doc = "injected data register x"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr2`]
module"]
pub type JDR2 = crate::Reg<jdr2::JDR2rs>;
#[doc = "injected data register x"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr3`]
module"]
pub type JDR3 = crate::Reg<jdr3::JDR3rs>;
#[doc = "injected data register x"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr4`]
module"]
pub type JDR4 = crate::Reg<jdr4::JDR4rs>;
#[doc = "injected data register x"]
pub mod jdr4;
#[doc = "DR (r) register accessor: regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "regular data register"]
pub mod dr;
