#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lisr: LISR,
    hisr: HISR,
    lifcr: LIFCR,
    hifcr: HIFCR,
    st: [ST; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    #[inline(always)]
    pub const fn lisr(&self) -> &LISR {
        &self.lisr
    }
    #[doc = "0x04 - high interrupt status register"]
    #[inline(always)]
    pub const fn hisr(&self) -> &HISR {
        &self.hisr
    }
    #[doc = "0x08 - low interrupt flag clear register"]
    #[inline(always)]
    pub const fn lifcr(&self) -> &LIFCR {
        &self.lifcr
    }
    #[doc = "0x0c - high interrupt flag clear register"]
    #[inline(always)]
    pub const fn hifcr(&self) -> &HIFCR {
        &self.hifcr
    }
    #[doc = "0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    #[inline(always)]
    pub const fn st(&self, n: usize) -> &ST {
        &self.st[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    #[inline(always)]
    pub fn st_iter(&self) -> impl Iterator<Item = &ST> {
        self.st.iter()
    }
}
#[doc = "LISR (r) register accessor: low interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lisr`]
module"]
pub type LISR = crate::Reg<lisr::LISRrs>;
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "HISR (r) register accessor: high interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hisr`]
module"]
pub type HISR = crate::Reg<hisr::HISRrs>;
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "LIFCR (w) register accessor: low interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lifcr`]
module"]
pub type LIFCR = crate::Reg<lifcr::LIFCRrs>;
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "HIFCR (w) register accessor: high interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hifcr`]
module"]
pub type HIFCR = crate::Reg<hifcr::HIFCRrs>;
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub use self::st::ST;
#[doc = r"Cluster"]
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub mod st;
