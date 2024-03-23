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
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - Data input register"]
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    #[doc = "0x0c - Data output register"]
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    #[doc = "0x10 - AES Key register 0"]
    #[inline(always)]
    pub const fn keyr0(&self) -> &KEYR0 {
        &self.keyr0
    }
    #[doc = "0x14 - AES Key register 1"]
    #[inline(always)]
    pub const fn keyr1(&self) -> &KEYR1 {
        &self.keyr1
    }
    #[doc = "0x18 - AES Key register 2"]
    #[inline(always)]
    pub const fn keyr2(&self) -> &KEYR2 {
        &self.keyr2
    }
    #[doc = "0x1c - AES Key register 3"]
    #[inline(always)]
    pub const fn keyr3(&self) -> &KEYR3 {
        &self.keyr3
    }
    #[doc = "0x20 - Initialization Vector Register 0"]
    #[inline(always)]
    pub const fn ivr0(&self) -> &IVR0 {
        &self.ivr0
    }
    #[doc = "0x24 - Initialization Vector Register 1"]
    #[inline(always)]
    pub const fn ivr1(&self) -> &IVR1 {
        &self.ivr1
    }
    #[doc = "0x28 - Initialization Vector Register 2"]
    #[inline(always)]
    pub const fn ivr2(&self) -> &IVR2 {
        &self.ivr2
    }
    #[doc = "0x2c - Initialization Vector Register 3"]
    #[inline(always)]
    pub const fn ivr3(&self) -> &IVR3 {
        &self.ivr3
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status register"]
pub mod sr;
#[doc = "DINR (rw) register accessor: Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr`]
module"]
pub type DINR = crate::Reg<dinr::DINRrs>;
#[doc = "Data input register"]
pub mod dinr;
#[doc = "DOUTR (r) register accessor: Data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr`]
module"]
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
#[doc = "Data output register"]
pub mod doutr;
#[doc = "KEYR0 (rw) register accessor: AES Key register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr0`]
module"]
pub type KEYR0 = crate::Reg<keyr0::KEYR0rs>;
#[doc = "AES Key register 0"]
pub mod keyr0;
#[doc = "KEYR1 (rw) register accessor: AES Key register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr1`]
module"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1rs>;
#[doc = "AES Key register 1"]
pub mod keyr1;
#[doc = "KEYR2 (rw) register accessor: AES Key register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr2`]
module"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2rs>;
#[doc = "AES Key register 2"]
pub mod keyr2;
#[doc = "KEYR3 (rw) register accessor: AES Key register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr3`]
module"]
pub type KEYR3 = crate::Reg<keyr3::KEYR3rs>;
#[doc = "AES Key register 3"]
pub mod keyr3;
#[doc = "IVR0 (rw) register accessor: Initialization Vector Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr0`]
module"]
pub type IVR0 = crate::Reg<ivr0::IVR0rs>;
#[doc = "Initialization Vector Register 0"]
pub mod ivr0;
#[doc = "IVR1 (rw) register accessor: Initialization Vector Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr1`]
module"]
pub type IVR1 = crate::Reg<ivr1::IVR1rs>;
#[doc = "Initialization Vector Register 1"]
pub mod ivr1;
#[doc = "IVR2 (rw) register accessor: Initialization Vector Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr2`]
module"]
pub type IVR2 = crate::Reg<ivr2::IVR2rs>;
#[doc = "Initialization Vector Register 2"]
pub mod ivr2;
#[doc = "IVR3 (rw) register accessor: Initialization Vector Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr3`]
module"]
pub type IVR3 = crate::Reg<ivr3::IVR3rs>;
#[doc = "Initialization Vector Register 3"]
pub mod ivr3;
