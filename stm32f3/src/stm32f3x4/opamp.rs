#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x3c],
    opamp2_csr: OPAMP2_CSR,
}
impl RegisterBlock {
    #[doc = "0x3c - OPAMP2 control register"]
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &OPAMP2_CSR {
        &self.opamp2_csr
    }
}
#[doc = "OPAMP2_CSR (rw) register accessor: OPAMP2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_csr`]
module"]
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSRrs>;
#[doc = "OPAMP2 control register"]
pub mod opamp2_csr;
