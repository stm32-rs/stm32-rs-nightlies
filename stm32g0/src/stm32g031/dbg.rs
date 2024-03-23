#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb_fz1: APB_FZ1,
    apb_fz2: APB_FZ2,
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
    pub const fn apb_fz1(&self) -> &APB_FZ1 {
        &self.apb_fz1
    }
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    #[inline(always)]
    pub const fn apb_fz2(&self) -> &APB_FZ2 {
        &self.apb_fz2
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
#[doc = "APB_FZ1 (rw) register accessor: Debug MCU APB1 freeze register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_fz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_fz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_fz1`]
module"]
pub type APB_FZ1 = crate::Reg<apb_fz1::APB_FZ1rs>;
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb_fz1;
#[doc = "APB_FZ2 (rw) register accessor: Debug MCU APB1 freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_fz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_fz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_fz2`]
module"]
pub type APB_FZ2 = crate::Reg<apb_fz2::APB_FZ2rs>;
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb_fz2;
