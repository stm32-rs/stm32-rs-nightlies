#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dbg_idcode: DBG_IDCODE,
    dbg_cr: DBG_CR,
    dbg_apb_fz1: DBG_APB_FZ1,
    dbg_apb_fz2: DBG_APB_FZ2,
}
impl RegisterBlock {
    #[doc = "0x00 - DBG device ID code register"]
    #[inline(always)]
    pub const fn dbg_idcode(&self) -> &DBG_IDCODE {
        &self.dbg_idcode
    }
    #[doc = "0x04 - DBG configuration register"]
    #[inline(always)]
    pub const fn dbg_cr(&self) -> &DBG_CR {
        &self.dbg_cr
    }
    #[doc = "0x08 - DBG APB freeze register 1"]
    #[inline(always)]
    pub const fn dbg_apb_fz1(&self) -> &DBG_APB_FZ1 {
        &self.dbg_apb_fz1
    }
    #[doc = "0x0c - DBG APB freeze register 2"]
    #[inline(always)]
    pub const fn dbg_apb_fz2(&self) -> &DBG_APB_FZ2 {
        &self.dbg_apb_fz2
    }
}
#[doc = "DBG_IDCODE (r) register accessor: DBG device ID code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_idcode`]
module"]
pub type DBG_IDCODE = crate::Reg<dbg_idcode::DBG_IDCODErs>;
#[doc = "DBG device ID code register"]
pub mod dbg_idcode;
#[doc = "DBG_CR (rw) register accessor: DBG configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_cr`]
module"]
pub type DBG_CR = crate::Reg<dbg_cr::DBG_CRrs>;
#[doc = "DBG configuration register"]
pub mod dbg_cr;
#[doc = "DBG_APB_FZ1 (rw) register accessor: DBG APB freeze register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_apb_fz1`]
module"]
pub type DBG_APB_FZ1 = crate::Reg<dbg_apb_fz1::DBG_APB_FZ1rs>;
#[doc = "DBG APB freeze register 1"]
pub mod dbg_apb_fz1;
#[doc = "DBG_APB_FZ2 (rw) register accessor: DBG APB freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_apb_fz2`]
module"]
pub type DBG_APB_FZ2 = crate::Reg<dbg_apb_fz2::DBG_APB_FZ2rs>;
#[doc = "DBG APB freeze register 2"]
pub mod dbg_apb_fz2;
