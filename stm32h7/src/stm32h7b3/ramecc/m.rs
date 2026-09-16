#[repr(C)]
#[derive(Debug)]
///Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
pub struct M {
    cr: CR,
    sr: SR,
    far: FAR,
    fdrl: FDRL,
    fdrh: FDRH,
    fecr: FECR,
    _reserved_end: [u8; 0x08],
}
impl M {
    ///0x00 - RAMECC monitor x configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RAMECC monitor x status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - RAMECC monitor x failing address register
    #[inline(always)]
    pub const fn far(&self) -> &FAR {
        &self.far
    }
    ///0x0c - RAMECC monitor x failing data low register
    #[inline(always)]
    pub const fn fdrl(&self) -> &FDRL {
        &self.fdrl
    }
    ///0x10 - RAMECC monitor x failing data high register
    #[inline(always)]
    pub const fn fdrh(&self) -> &FDRH {
        &self.fdrh
    }
    ///0x14 - RAMECC monitor x failing ECC error code register
    #[inline(always)]
    pub const fn fecr(&self) -> &FECR {
        &self.fecr
    }
}
/**CR (rw) register accessor: RAMECC monitor x configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RAMECC monitor x configuration register
pub mod cr;
/**SR (rw) register accessor: RAMECC monitor x status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///RAMECC monitor x status register
pub mod sr;
/**FAR (r) register accessor: RAMECC monitor x failing address register

You can [`read`](crate::Reg::read) this register and get [`far::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@far`] module*/
pub type FAR = crate::Reg<far::FARrs>;
///RAMECC monitor x failing address register
pub mod far;
/**FDRL (r) register accessor: RAMECC monitor x failing data low register

You can [`read`](crate::Reg::read) this register and get [`fdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fdrl`] module*/
pub type FDRL = crate::Reg<fdrl::FDRLrs>;
///RAMECC monitor x failing data low register
pub mod fdrl;
/**FDRH (r) register accessor: RAMECC monitor x failing data high register

You can [`read`](crate::Reg::read) this register and get [`fdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fdrh`] module*/
pub type FDRH = crate::Reg<fdrh::FDRHrs>;
///RAMECC monitor x failing data high register
pub mod fdrh;
/**FECR (r) register accessor: RAMECC monitor x failing ECC error code register

You can [`read`](crate::Reg::read) this register and get [`fecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fecr`] module*/
pub type FECR = crate::Reg<fecr::FECRrs>;
///RAMECC monitor x failing ECC error code register
pub mod fecr;
