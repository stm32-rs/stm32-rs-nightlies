#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcache_cr: DCACHE_CR,
    dcache_sr: DCACHE_SR,
    dcache_ier: DCACHE_IER,
    dcache_fcr: DCACHE_FCR,
    dcache_rhmonr: DCACHE_RHMONR,
    dcache_rmmonr: DCACHE_RMMONR,
    _reserved6: [u8; 0x08],
    dcache_whmonr: DCACHE_WHMONR,
    dcache_wmmonr: DCACHE_WMMONR,
    dcache_cmdrsaddrr: DCACHE_CMDRSADDRR,
    dcache_cmdreaddrr: DCACHE_CMDREADDRR,
}
impl RegisterBlock {
    #[doc = "0x00 - DCACHE control register"]
    #[inline(always)]
    pub const fn dcache_cr(&self) -> &DCACHE_CR {
        &self.dcache_cr
    }
    #[doc = "0x04 - DCACHE status register"]
    #[inline(always)]
    pub const fn dcache_sr(&self) -> &DCACHE_SR {
        &self.dcache_sr
    }
    #[doc = "0x08 - DCACHE interrupt enable register"]
    #[inline(always)]
    pub const fn dcache_ier(&self) -> &DCACHE_IER {
        &self.dcache_ier
    }
    #[doc = "0x0c - DCACHE flag clear register"]
    #[inline(always)]
    pub const fn dcache_fcr(&self) -> &DCACHE_FCR {
        &self.dcache_fcr
    }
    #[doc = "0x10 - DCACHE read-hit monitor register"]
    #[inline(always)]
    pub const fn dcache_rhmonr(&self) -> &DCACHE_RHMONR {
        &self.dcache_rhmonr
    }
    #[doc = "0x14 - DCACHE read-miss monitor register"]
    #[inline(always)]
    pub const fn dcache_rmmonr(&self) -> &DCACHE_RMMONR {
        &self.dcache_rmmonr
    }
    #[doc = "0x20 - write-hit monitor register"]
    #[inline(always)]
    pub const fn dcache_whmonr(&self) -> &DCACHE_WHMONR {
        &self.dcache_whmonr
    }
    #[doc = "0x24 - write-miss monitor register"]
    #[inline(always)]
    pub const fn dcache_wmmonr(&self) -> &DCACHE_WMMONR {
        &self.dcache_wmmonr
    }
    #[doc = "0x28 - command range start address register"]
    #[inline(always)]
    pub const fn dcache_cmdrsaddrr(&self) -> &DCACHE_CMDRSADDRR {
        &self.dcache_cmdrsaddrr
    }
    #[doc = "0x2c - command range start address register"]
    #[inline(always)]
    pub const fn dcache_cmdreaddrr(&self) -> &DCACHE_CMDREADDRR {
        &self.dcache_cmdreaddrr
    }
}
#[doc = "DCACHE_CR (rw) register accessor: DCACHE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_cr`]
module"]
pub type DCACHE_CR = crate::Reg<dcache_cr::DCACHE_CRrs>;
#[doc = "DCACHE control register"]
pub mod dcache_cr;
#[doc = "DCACHE_SR (r) register accessor: DCACHE status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_sr`]
module"]
pub type DCACHE_SR = crate::Reg<dcache_sr::DCACHE_SRrs>;
#[doc = "DCACHE status register"]
pub mod dcache_sr;
#[doc = "DCACHE_IER (rw) register accessor: DCACHE interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_ier`]
module"]
pub type DCACHE_IER = crate::Reg<dcache_ier::DCACHE_IERrs>;
#[doc = "DCACHE interrupt enable register"]
pub mod dcache_ier;
#[doc = "DCACHE_FCR (w) register accessor: DCACHE flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_fcr`]
module"]
pub type DCACHE_FCR = crate::Reg<dcache_fcr::DCACHE_FCRrs>;
#[doc = "DCACHE flag clear register"]
pub mod dcache_fcr;
#[doc = "DCACHE_RHMONR (r) register accessor: DCACHE read-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_rhmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_rhmonr`]
module"]
pub type DCACHE_RHMONR = crate::Reg<dcache_rhmonr::DCACHE_RHMONRrs>;
#[doc = "DCACHE read-hit monitor register"]
pub mod dcache_rhmonr;
#[doc = "DCACHE_RMMONR (r) register accessor: DCACHE read-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_rmmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_rmmonr`]
module"]
pub type DCACHE_RMMONR = crate::Reg<dcache_rmmonr::DCACHE_RMMONRrs>;
#[doc = "DCACHE read-miss monitor register"]
pub mod dcache_rmmonr;
#[doc = "DCACHE_WHMONR (r) register accessor: write-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_whmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_whmonr`]
module"]
pub type DCACHE_WHMONR = crate::Reg<dcache_whmonr::DCACHE_WHMONRrs>;
#[doc = "write-hit monitor register"]
pub mod dcache_whmonr;
#[doc = "DCACHE_WMMONR (r) register accessor: write-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_wmmonr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_wmmonr`]
module"]
pub type DCACHE_WMMONR = crate::Reg<dcache_wmmonr::DCACHE_WMMONRrs>;
#[doc = "write-miss monitor register"]
pub mod dcache_wmmonr;
#[doc = "DCACHE_CMDRSADDRR (rw) register accessor: command range start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_cmdrsaddrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_cmdrsaddrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_cmdrsaddrr`]
module"]
pub type DCACHE_CMDRSADDRR = crate::Reg<dcache_cmdrsaddrr::DCACHE_CMDRSADDRRrs>;
#[doc = "command range start address register"]
pub mod dcache_cmdrsaddrr;
#[doc = "DCACHE_CMDREADDRR (rw) register accessor: command range start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_cmdreaddrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_cmdreaddrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_cmdreaddrr`]
module"]
pub type DCACHE_CMDREADDRR = crate::Reg<dcache_cmdreaddrr::DCACHE_CMDREADDRRrs>;
#[doc = "command range start address register"]
pub mod dcache_cmdreaddrr;
