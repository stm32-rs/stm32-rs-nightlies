#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    actrl: ACTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Auxiliary control register"]
    #[inline(always)]
    pub const fn actrl(&self) -> &ACTRL {
        &self.actrl
    }
}
#[doc = "ACTRL (rw) register accessor: Auxiliary control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actrl`]
module"]
pub type ACTRL = crate::Reg<actrl::ACTRLrs>;
#[doc = "Auxiliary control register"]
pub mod actrl;
