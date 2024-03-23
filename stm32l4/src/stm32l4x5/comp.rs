#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp1_csr: COMP1_CSR,
    comp2_csr: COMP2_CSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator 1 control and status register"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &COMP1_CSR {
        &self.comp1_csr
    }
    #[doc = "0x04 - Comparator 2 control and status register"]
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &COMP2_CSR {
        &self.comp2_csr
    }
}
#[doc = "COMP1_CSR (rw) register accessor: Comparator 1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`]
module"]
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSRrs>;
#[doc = "Comparator 1 control and status register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: Comparator 2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_csr`]
module"]
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSRrs>;
#[doc = "Comparator 2 control and status register"]
pub mod comp2_csr;
