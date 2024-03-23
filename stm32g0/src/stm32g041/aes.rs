#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dinr: DINR,
    doutr: DOUTR,
    keyr0: KEYR0,
    keyr1: KEYR1,
    keyr2: KEYR2,
    keyr3: KEYR3,
    ivr0: IVR0,
    ivr1: IVR1,
    ivr2: IVR2,
    ivr3: IVR3,
    keyr4: KEYR4,
    keyr5: KEYR5,
    keyr6: KEYR6,
    keyr7: KEYR7,
    susp0r: SUSP0R,
    susp1r: SUSP1R,
    susp2r: SUSP2R,
    susp3r: SUSP3R,
    susp4r: SUSP4R,
    susp5r: SUSP5R,
    susp6r: SUSP6R,
    susp7r: SUSP7R,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - data input register"]
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    #[doc = "0x0c - data output register"]
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    #[doc = "0x10 - key register 0"]
    #[inline(always)]
    pub const fn keyr0(&self) -> &KEYR0 {
        &self.keyr0
    }
    #[doc = "0x14 - key register 1"]
    #[inline(always)]
    pub const fn keyr1(&self) -> &KEYR1 {
        &self.keyr1
    }
    #[doc = "0x18 - key register 2"]
    #[inline(always)]
    pub const fn keyr2(&self) -> &KEYR2 {
        &self.keyr2
    }
    #[doc = "0x1c - key register 3"]
    #[inline(always)]
    pub const fn keyr3(&self) -> &KEYR3 {
        &self.keyr3
    }
    #[doc = "0x20 - initialization vector register 0"]
    #[inline(always)]
    pub const fn ivr0(&self) -> &IVR0 {
        &self.ivr0
    }
    #[doc = "0x24 - initialization vector register 1"]
    #[inline(always)]
    pub const fn ivr1(&self) -> &IVR1 {
        &self.ivr1
    }
    #[doc = "0x28 - initialization vector register 2"]
    #[inline(always)]
    pub const fn ivr2(&self) -> &IVR2 {
        &self.ivr2
    }
    #[doc = "0x2c - initialization vector register 3"]
    #[inline(always)]
    pub const fn ivr3(&self) -> &IVR3 {
        &self.ivr3
    }
    #[doc = "0x30 - key register 4"]
    #[inline(always)]
    pub const fn keyr4(&self) -> &KEYR4 {
        &self.keyr4
    }
    #[doc = "0x34 - key register 5"]
    #[inline(always)]
    pub const fn keyr5(&self) -> &KEYR5 {
        &self.keyr5
    }
    #[doc = "0x38 - key register 6"]
    #[inline(always)]
    pub const fn keyr6(&self) -> &KEYR6 {
        &self.keyr6
    }
    #[doc = "0x3c - key register 7"]
    #[inline(always)]
    pub const fn keyr7(&self) -> &KEYR7 {
        &self.keyr7
    }
    #[doc = "0x40 - AES suspend register 0"]
    #[inline(always)]
    pub const fn susp0r(&self) -> &SUSP0R {
        &self.susp0r
    }
    #[doc = "0x44 - AES suspend register 1"]
    #[inline(always)]
    pub const fn susp1r(&self) -> &SUSP1R {
        &self.susp1r
    }
    #[doc = "0x48 - AES suspend register 2"]
    #[inline(always)]
    pub const fn susp2r(&self) -> &SUSP2R {
        &self.susp2r
    }
    #[doc = "0x4c - AES suspend register 3"]
    #[inline(always)]
    pub const fn susp3r(&self) -> &SUSP3R {
        &self.susp3r
    }
    #[doc = "0x50 - AES suspend register 4"]
    #[inline(always)]
    pub const fn susp4r(&self) -> &SUSP4R {
        &self.susp4r
    }
    #[doc = "0x54 - AES suspend register 5"]
    #[inline(always)]
    pub const fn susp5r(&self) -> &SUSP5R {
        &self.susp5r
    }
    #[doc = "0x58 - AES suspend register 6"]
    #[inline(always)]
    pub const fn susp6r(&self) -> &SUSP6R {
        &self.susp6r
    }
    #[doc = "0x5c - AES suspend register 7"]
    #[inline(always)]
    pub const fn susp7r(&self) -> &SUSP7R {
        &self.susp7r
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "DINR (rw) register accessor: data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr`]
module"]
pub type DINR = crate::Reg<dinr::DINRrs>;
#[doc = "data input register"]
pub mod dinr;
#[doc = "DOUTR (r) register accessor: data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr`]
module"]
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
#[doc = "data output register"]
pub mod doutr;
#[doc = "KEYR0 (rw) register accessor: key register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr0`]
module"]
pub type KEYR0 = crate::Reg<keyr0::KEYR0rs>;
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "KEYR1 (rw) register accessor: key register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr1`]
module"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1rs>;
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "KEYR2 (rw) register accessor: key register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr2`]
module"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2rs>;
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "KEYR3 (rw) register accessor: key register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr3`]
module"]
pub type KEYR3 = crate::Reg<keyr3::KEYR3rs>;
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "IVR0 (rw) register accessor: initialization vector register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr0`]
module"]
pub type IVR0 = crate::Reg<ivr0::IVR0rs>;
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "IVR1 (rw) register accessor: initialization vector register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr1`]
module"]
pub type IVR1 = crate::Reg<ivr1::IVR1rs>;
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "IVR2 (rw) register accessor: initialization vector register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr2`]
module"]
pub type IVR2 = crate::Reg<ivr2::IVR2rs>;
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "IVR3 (rw) register accessor: initialization vector register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr3`]
module"]
pub type IVR3 = crate::Reg<ivr3::IVR3rs>;
#[doc = "initialization vector register 3"]
pub mod ivr3;
#[doc = "KEYR4 (rw) register accessor: key register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr4`]
module"]
pub type KEYR4 = crate::Reg<keyr4::KEYR4rs>;
#[doc = "key register 4"]
pub mod keyr4;
#[doc = "KEYR5 (rw) register accessor: key register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr5`]
module"]
pub type KEYR5 = crate::Reg<keyr5::KEYR5rs>;
#[doc = "key register 5"]
pub mod keyr5;
#[doc = "KEYR6 (rw) register accessor: key register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr6`]
module"]
pub type KEYR6 = crate::Reg<keyr6::KEYR6rs>;
#[doc = "key register 6"]
pub mod keyr6;
#[doc = "KEYR7 (rw) register accessor: key register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr7`]
module"]
pub type KEYR7 = crate::Reg<keyr7::KEYR7rs>;
#[doc = "key register 7"]
pub mod keyr7;
#[doc = "SUSP0R (rw) register accessor: AES suspend register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp0r`]
module"]
pub type SUSP0R = crate::Reg<susp0r::SUSP0Rrs>;
#[doc = "AES suspend register 0"]
pub mod susp0r;
#[doc = "SUSP1R (rw) register accessor: AES suspend register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp1r`]
module"]
pub type SUSP1R = crate::Reg<susp1r::SUSP1Rrs>;
#[doc = "AES suspend register 1"]
pub mod susp1r;
#[doc = "SUSP2R (rw) register accessor: AES suspend register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp2r`]
module"]
pub type SUSP2R = crate::Reg<susp2r::SUSP2Rrs>;
#[doc = "AES suspend register 2"]
pub mod susp2r;
#[doc = "SUSP3R (rw) register accessor: AES suspend register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp3r`]
module"]
pub type SUSP3R = crate::Reg<susp3r::SUSP3Rrs>;
#[doc = "AES suspend register 3"]
pub mod susp3r;
#[doc = "SUSP4R (rw) register accessor: AES suspend register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp4r`]
module"]
pub type SUSP4R = crate::Reg<susp4r::SUSP4Rrs>;
#[doc = "AES suspend register 4"]
pub mod susp4r;
#[doc = "SUSP5R (rw) register accessor: AES suspend register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp5r`]
module"]
pub type SUSP5R = crate::Reg<susp5r::SUSP5Rrs>;
#[doc = "AES suspend register 5"]
pub mod susp5r;
#[doc = "SUSP6R (rw) register accessor: AES suspend register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp6r`]
module"]
pub type SUSP6R = crate::Reg<susp6r::SUSP6Rrs>;
#[doc = "AES suspend register 6"]
pub mod susp6r;
#[doc = "SUSP7R (rw) register accessor: AES suspend register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp7r`]
module"]
pub type SUSP7R = crate::Reg<susp7r::SUSP7Rrs>;
#[doc = "AES suspend register 7"]
pub mod susp7r;
