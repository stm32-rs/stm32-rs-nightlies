#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cordic_csr: CORDIC_CSR,
    cordic_wdata: CORDIC_WDATA,
    cordic_rdata: CORDIC_RDATA,
}
impl RegisterBlock {
    #[doc = "0x00 - CORDIC control/status register"]
    #[inline(always)]
    pub const fn cordic_csr(&self) -> &CORDIC_CSR {
        &self.cordic_csr
    }
    #[doc = "0x04 - CORDIC argument register"]
    #[inline(always)]
    pub const fn cordic_wdata(&self) -> &CORDIC_WDATA {
        &self.cordic_wdata
    }
    #[doc = "0x08 - CORDIC result register"]
    #[inline(always)]
    pub const fn cordic_rdata(&self) -> &CORDIC_RDATA {
        &self.cordic_rdata
    }
}
#[doc = "CORDIC_CSR (rw) register accessor: CORDIC control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cordic_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cordic_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_csr`]
module"]
pub type CORDIC_CSR = crate::Reg<cordic_csr::CORDIC_CSRrs>;
#[doc = "CORDIC control/status register"]
pub mod cordic_csr;
#[doc = "CORDIC_WDATA (w) register accessor: CORDIC argument register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cordic_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_wdata`]
module"]
pub type CORDIC_WDATA = crate::Reg<cordic_wdata::CORDIC_WDATArs>;
#[doc = "CORDIC argument register"]
pub mod cordic_wdata;
#[doc = "CORDIC_RDATA (r) register accessor: CORDIC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cordic_rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_rdata`]
module"]
pub type CORDIC_RDATA = crate::Reg<cordic_rdata::CORDIC_RDATArs>;
#[doc = "CORDIC result register"]
pub mod cordic_rdata;
