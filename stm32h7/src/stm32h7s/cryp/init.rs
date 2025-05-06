#[repr(C)]
#[derive(Debug)]
///Cluster INIT%s, containing IV?LR, IV?RR
pub struct INIT {
    ivlr: IVLR,
    ivrr: IVRR,
}
impl INIT {
    ///0x00 - CRYP initialization vector register 0L
    #[inline(always)]
    pub const fn ivlr(&self) -> &IVLR {
        &self.ivlr
    }
    ///0x04 - CRYP initialization vector register 0R
    #[inline(always)]
    pub const fn ivrr(&self) -> &IVRR {
        &self.ivrr
    }
}
/**IVLR (rw) register accessor: CRYP initialization vector register 0L

You can [`read`](crate::Reg::read) this register and get [`ivlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ivlr`] module*/
pub type IVLR = crate::Reg<ivlr::IVLRrs>;
///CRYP initialization vector register 0L
pub mod ivlr;
/**IVRR (rw) register accessor: CRYP initialization vector register 0R

You can [`read`](crate::Reg::read) this register and get [`ivrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ivrr`] module*/
pub type IVRR = crate::Reg<ivrr::IVRRrs>;
///CRYP initialization vector register 0R
pub mod ivrr;
