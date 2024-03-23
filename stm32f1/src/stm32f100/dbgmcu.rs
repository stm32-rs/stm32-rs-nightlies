#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
}
impl RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    #[doc = "0x04 - DBGMCU_CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
}
#[doc = "IDCODE (r) register accessor: DBGMCU_IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`]
module"]
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "CR (rw) register accessor: DBGMCU_CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DBGMCU_CR"]
pub mod cr;
