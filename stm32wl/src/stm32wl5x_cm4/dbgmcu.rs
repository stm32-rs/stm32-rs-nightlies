#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcoder: IDCODER,
    cr: CR,
    _reserved2: [u8; 0x34],
    apb1fzr1: APB1FZR1,
    c2apb1fzr1: C2APB1FZR1,
    apb1fzr2: APB1FZR2,
    c2apb1fzr2: C2APB1FZR2,
    apb2fzr: APB2FZR,
    c2apb2fzr: C2APB2FZR,
}
impl RegisterBlock {
    #[doc = "0x00 - DBGMCU Identity Code Register"]
    #[inline(always)]
    pub const fn idcoder(&self) -> &IDCODER {
        &self.idcoder
    }
    #[doc = "0x04 - DBGMCU Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x3c - DBGMCU CPU1 APB1 Peripheral Freeze Register 1"]
    #[inline(always)]
    pub const fn apb1fzr1(&self) -> &APB1FZR1 {
        &self.apb1fzr1
    }
    #[doc = "0x40 - DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \\[dual core device"]
    #[inline(always)]
    pub const fn c2apb1fzr1(&self) -> &C2APB1FZR1 {
        &self.c2apb1fzr1
    }
    #[doc = "0x44 - DBGMCU CPU1 APB1 Peripheral Freeze Register 2"]
    #[inline(always)]
    pub const fn apb1fzr2(&self) -> &APB1FZR2 {
        &self.apb1fzr2
    }
    #[doc = "0x48 - DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \\[dual core device"]
    #[inline(always)]
    pub const fn c2apb1fzr2(&self) -> &C2APB1FZR2 {
        &self.c2apb1fzr2
    }
    #[doc = "0x4c - DBGMCU CPU1 APB2 Peripheral Freeze Register"]
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
    }
    #[doc = "0x50 - DBGMCU CPU2 APB2 Peripheral Freeze Register \\[dual core device"]
    #[inline(always)]
    pub const fn c2apb2fzr(&self) -> &C2APB2FZR {
        &self.c2apb2fzr
    }
}
#[doc = "IDCODER (r) register accessor: DBGMCU Identity Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcoder::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcoder`]
module"]
pub type IDCODER = crate::Reg<idcoder::IDCODERrs>;
#[doc = "DBGMCU Identity Code Register"]
pub mod idcoder;
#[doc = "CR (rw) register accessor: DBGMCU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DBGMCU Configuration Register"]
pub mod cr;
#[doc = "APB1FZR1 (rw) register accessor: DBGMCU CPU1 APB1 Peripheral Freeze Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1fzr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1fzr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1fzr1`]
module"]
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1rs>;
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 1"]
pub mod apb1fzr1;
#[doc = "C2APB1FZR1 (rw) register accessor: DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \\[dual core device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb1fzr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb1fzr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2apb1fzr1`]
module"]
pub type C2APB1FZR1 = crate::Reg<c2apb1fzr1::C2APB1FZR1rs>;
#[doc = "DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \\[dual core device"]
pub mod c2apb1fzr1;
#[doc = "APB1FZR2 (rw) register accessor: DBGMCU CPU1 APB1 Peripheral Freeze Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1fzr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1fzr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1fzr2`]
module"]
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2rs>;
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 2"]
pub mod apb1fzr2;
#[doc = "C2APB1FZR2 (rw) register accessor: DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \\[dual core device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb1fzr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb1fzr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2apb1fzr2`]
module"]
pub type C2APB1FZR2 = crate::Reg<c2apb1fzr2::C2APB1FZR2rs>;
#[doc = "DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \\[dual core device"]
pub mod c2apb1fzr2;
#[doc = "APB2FZR (rw) register accessor: DBGMCU CPU1 APB2 Peripheral Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2fzr`]
module"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
#[doc = "DBGMCU CPU1 APB2 Peripheral Freeze Register"]
pub mod apb2fzr;
#[doc = "C2APB2FZR (rw) register accessor: DBGMCU CPU2 APB2 Peripheral Freeze Register \\[dual core device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb2fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb2fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2apb2fzr`]
module"]
pub type C2APB2FZR = crate::Reg<c2apb2fzr::C2APB2FZRrs>;
#[doc = "DBGMCU CPU2 APB2 Peripheral Freeze Register \\[dual core device"]
pub mod c2apb2fzr;
