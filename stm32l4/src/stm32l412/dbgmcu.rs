#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb1fzr1: APB1FZR1,
    apb1fzr2: APB1FZR2,
    apb2fzr: APB2FZR,
}
impl RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    #[doc = "0x04 - Debug MCU configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - Debug MCU APB1 freeze register1"]
    #[inline(always)]
    pub const fn apb1fzr1(&self) -> &APB1FZR1 {
        &self.apb1fzr1
    }
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    #[inline(always)]
    pub const fn apb1fzr2(&self) -> &APB1FZR2 {
        &self.apb1fzr2
    }
    #[doc = "0x10 - Debug MCU APB2 freeze register"]
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
    }
}
#[doc = "IDCODE (r) register accessor: DBGMCU_IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`]
module"]
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "CR (rw) register accessor: Debug MCU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Debug MCU configuration register"]
pub mod cr;
#[doc = "APB1FZR1 (rw) register accessor: Debug MCU APB1 freeze register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1fzr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1fzr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1fzr1`]
module"]
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1rs>;
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb1fzr1;
#[doc = "APB1FZR2 (rw) register accessor: Debug MCU APB1 freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1fzr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1fzr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1fzr2`]
module"]
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2rs>;
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb1fzr2;
#[doc = "APB2FZR (rw) register accessor: Debug MCU APB2 freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2fzr`]
module"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
#[doc = "Debug MCU APB2 freeze register"]
pub mod apb2fzr;
