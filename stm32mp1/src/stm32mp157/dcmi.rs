#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcmi_cr: DCMI_CR,
    dcmi_sr: DCMI_SR,
    dcmi_ris: DCMI_RIS,
    dcmi_ier: DCMI_IER,
    dcmi_mis: DCMI_MIS,
    dcmi_icr: DCMI_ICR,
    dcmi_escr: DCMI_ESCR,
    dcmi_esur: DCMI_ESUR,
    dcmi_cwstrt: DCMI_CWSTRT,
    dcmi_cwsize: DCMI_CWSIZE,
    dcmi_dr: DCMI_DR,
}
impl RegisterBlock {
    #[doc = "0x00 - DCMI control register"]
    #[inline(always)]
    pub const fn dcmi_cr(&self) -> &DCMI_CR {
        &self.dcmi_cr
    }
    #[doc = "0x04 - DCMI status register"]
    #[inline(always)]
    pub const fn dcmi_sr(&self) -> &DCMI_SR {
        &self.dcmi_sr
    }
    #[doc = "0x08 - DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value."]
    #[inline(always)]
    pub const fn dcmi_ris(&self) -> &DCMI_RIS {
        &self.dcmi_ris
    }
    #[doc = "0x0c - The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write."]
    #[inline(always)]
    pub const fn dcmi_ier(&self) -> &DCMI_IER {
        &self.dcmi_ier
    }
    #[doc = "0x10 - This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set."]
    #[inline(always)]
    pub const fn dcmi_mis(&self) -> &DCMI_MIS {
        &self.dcmi_mis
    }
    #[doc = "0x14 - The DCMI_ICR register is write-only."]
    #[inline(always)]
    pub const fn dcmi_icr(&self) -> &DCMI_ICR {
        &self.dcmi_icr
    }
    #[doc = "0x18 - DCMI embedded synchronization code register"]
    #[inline(always)]
    pub const fn dcmi_escr(&self) -> &DCMI_ESCR {
        &self.dcmi_escr
    }
    #[doc = "0x1c - DCMI embedded synchronization unmask register"]
    #[inline(always)]
    pub const fn dcmi_esur(&self) -> &DCMI_ESUR {
        &self.dcmi_esur
    }
    #[doc = "0x20 - DCMI crop window start"]
    #[inline(always)]
    pub const fn dcmi_cwstrt(&self) -> &DCMI_CWSTRT {
        &self.dcmi_cwstrt
    }
    #[doc = "0x24 - DCMI crop window size"]
    #[inline(always)]
    pub const fn dcmi_cwsize(&self) -> &DCMI_CWSIZE {
        &self.dcmi_cwsize
    }
    #[doc = "0x28 - DCMI data register"]
    #[inline(always)]
    pub const fn dcmi_dr(&self) -> &DCMI_DR {
        &self.dcmi_dr
    }
}
#[doc = "DCMI_CR (rw) register accessor: DCMI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_cr`]
module"]
pub type DCMI_CR = crate::Reg<dcmi_cr::DCMI_CRrs>;
#[doc = "DCMI control register"]
pub mod dcmi_cr;
#[doc = "DCMI_SR (r) register accessor: DCMI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_sr`]
module"]
pub type DCMI_SR = crate::Reg<dcmi_sr::DCMI_SRrs>;
#[doc = "DCMI status register"]
pub mod dcmi_sr;
#[doc = "DCMI_RIS (r) register accessor: DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_ris`]
module"]
pub type DCMI_RIS = crate::Reg<dcmi_ris::DCMI_RISrs>;
#[doc = "DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value."]
pub mod dcmi_ris;
#[doc = "DCMI_IER (rw) register accessor: The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_ier`]
module"]
pub type DCMI_IER = crate::Reg<dcmi_ier::DCMI_IERrs>;
#[doc = "The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write."]
pub mod dcmi_ier;
#[doc = "DCMI_MIS (r) register accessor: This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_mis`]
module"]
pub type DCMI_MIS = crate::Reg<dcmi_mis::DCMI_MISrs>;
#[doc = "This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set."]
pub mod dcmi_mis;
#[doc = "DCMI_ICR (w) register accessor: The DCMI_ICR register is write-only.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_icr`]
module"]
pub type DCMI_ICR = crate::Reg<dcmi_icr::DCMI_ICRrs>;
#[doc = "The DCMI_ICR register is write-only."]
pub mod dcmi_icr;
#[doc = "DCMI_ESCR (rw) register accessor: DCMI embedded synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_escr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_escr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_escr`]
module"]
pub type DCMI_ESCR = crate::Reg<dcmi_escr::DCMI_ESCRrs>;
#[doc = "DCMI embedded synchronization code register"]
pub mod dcmi_escr;
#[doc = "DCMI_ESUR (rw) register accessor: DCMI embedded synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_esur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_esur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_esur`]
module"]
pub type DCMI_ESUR = crate::Reg<dcmi_esur::DCMI_ESURrs>;
#[doc = "DCMI embedded synchronization unmask register"]
pub mod dcmi_esur;
#[doc = "DCMI_CWSTRT (rw) register accessor: DCMI crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_cwstrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_cwstrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_cwstrt`]
module"]
pub type DCMI_CWSTRT = crate::Reg<dcmi_cwstrt::DCMI_CWSTRTrs>;
#[doc = "DCMI crop window start"]
pub mod dcmi_cwstrt;
#[doc = "DCMI_CWSIZE (rw) register accessor: DCMI crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_cwsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_cwsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_cwsize`]
module"]
pub type DCMI_CWSIZE = crate::Reg<dcmi_cwsize::DCMI_CWSIZErs>;
#[doc = "DCMI crop window size"]
pub mod dcmi_cwsize;
#[doc = "DCMI_DR (r) register accessor: DCMI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmi_dr`]
module"]
pub type DCMI_DR = crate::Reg<dcmi_dr::DCMI_DRrs>;
#[doc = "DCMI data register"]
pub mod dcmi_dr;
