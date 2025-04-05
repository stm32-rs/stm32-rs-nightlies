#[repr(C)]
#[derive(Debug)]
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub struct CH {
    cr1: CR1,
    cr2: CR2,
    frcr: FRCR,
    slotr: SLOTR,
    im: IM,
    sr: SR,
    clrfr: CLRFR,
    dr: DR,
}
impl CH {
    ///0x00 - AConfiguration register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - AConfiguration register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - AFRCR
    #[inline(always)]
    pub const fn frcr(&self) -> &FRCR {
        &self.frcr
    }
    ///0x0c - ASlot register
    #[inline(always)]
    pub const fn slotr(&self) -> &SLOTR {
        &self.slotr
    }
    ///0x10 - AInterrupt mask register2
    #[inline(always)]
    pub const fn im(&self) -> &IM {
        &self.im
    }
    ///0x14 - AStatus register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - AClear flag register
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
    ///0x1c - AData register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
/**CR1 (rw) register accessor: AConfiguration register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///AConfiguration register 1
pub mod cr1;
/**CR2 (rw) register accessor: AConfiguration register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///AConfiguration register 2
pub mod cr2;
/**FRCR (rw) register accessor: AFRCR

You can [`read`](crate::Reg::read) this register and get [`frcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frcr`] module*/
pub type FRCR = crate::Reg<frcr::FRCRrs>;
///AFRCR
pub mod frcr;
/**SLOTR (rw) register accessor: ASlot register

You can [`read`](crate::Reg::read) this register and get [`slotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slotr`] module*/
pub type SLOTR = crate::Reg<slotr::SLOTRrs>;
///ASlot register
pub mod slotr;
/**IM (rw) register accessor: AInterrupt mask register2

You can [`read`](crate::Reg::read) this register and get [`im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@im`] module*/
pub type IM = crate::Reg<im::IMrs>;
///AInterrupt mask register2
pub mod im;
/**SR (r) register accessor: AStatus register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///AStatus register
pub mod sr;
/**CLRFR (w) register accessor: AClear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clrfr`] module*/
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
///AClear flag register
pub mod clrfr;
/**DR (rw) register accessor: AData register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///AData register
pub mod dr;
