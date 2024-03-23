#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb1_fz: APB1_FZ,
    apb2_fz: APB2_FZ,
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
    pub const fn apb1_fz(&self) -> &APB1_FZ {
        &self.apb1_fz
    }
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    #[inline(always)]
    pub const fn apb2_fz(&self) -> &APB2_FZ {
        &self.apb2_fz
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
#[doc = "APB1_FZ (rw) register accessor: Debug MCU APB1 freeze register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1_fz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1_fz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1_fz`]
module"]
pub type APB1_FZ = crate::Reg<apb1_fz::APB1_FZrs>;
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb1_fz;
#[doc = "APB2_FZ (rw) register accessor: Debug MCU APB1 freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2_fz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2_fz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2_fz`]
module"]
pub type APB2_FZ = crate::Reg<apb2_fz::APB2_FZrs>;
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb2_fz;
