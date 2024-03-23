#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r: [R; 32],
    rlr: [RLR; 32],
    ier: IER,
    icr: ICR,
    isr: ISR,
    misr: MISR,
    _reserved6: [u8; 0x30],
    cr: CR,
    keyr: KEYR,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - HSEM register HSEM_R%s"]
    #[inline(always)]
    pub const fn r(&self, n: usize) -> &R {
        &self.r[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - HSEM register HSEM_R%s"]
    #[inline(always)]
    pub fn r_iter(&self) -> impl Iterator<Item = &R> {
        self.r.iter()
    }
    #[doc = "0x80..0x100 - Semaphore %s read lock register"]
    #[inline(always)]
    pub const fn rlr(&self, n: usize) -> &RLR {
        &self.rlr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Semaphore %s read lock register"]
    #[inline(always)]
    pub fn rlr_iter(&self) -> impl Iterator<Item = &RLR> {
        self.rlr.iter()
    }
    #[doc = "0x100 - HSEM Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x104 - HSEM Interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x108 - HSEM Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    #[doc = "0x140 - HSEM Clear register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x144 - HSEM Interrupt clear register"]
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
}
#[doc = "R (rw) register accessor: HSEM register HSEM_R%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r`]
module"]
pub type R = crate::Reg<r::Rrs>;
#[doc = "HSEM register HSEM_R%s"]
pub mod r;
#[doc = "RLR (r) register accessor: Semaphore %s read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
pub type RLR = crate::Reg<rlr::RLRrs>;
#[doc = "Semaphore %s read lock register"]
pub mod rlr;
#[doc = "IER (rw) register accessor: HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "HSEM Interrupt enable register"]
pub mod ier;
#[doc = "ICR (r) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "HSEM Interrupt clear register"]
pub mod icr;
#[doc = "ISR (r) register accessor: HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "HSEM Interrupt status register"]
pub mod isr;
#[doc = "MISR (r) register accessor: HSEM Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
pub type MISR = crate::Reg<misr::MISRrs>;
#[doc = "HSEM Masked interrupt status register"]
pub mod misr;
#[doc = "CR (rw) register accessor: HSEM Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "HSEM Clear register"]
pub mod cr;
#[doc = "KEYR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYRrs>;
#[doc = "HSEM Interrupt clear register"]
pub mod keyr;
