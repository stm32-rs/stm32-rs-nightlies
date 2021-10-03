#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCMI control register"]
    pub dcmi_cr: crate::Reg<dcmi_cr::DCMI_CR_SPEC>,
    #[doc = "0x04 - DCMI status register"]
    pub dcmi_sr: crate::Reg<dcmi_sr::DCMI_SR_SPEC>,
    #[doc = "0x08 - DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value."]
    pub dcmi_ris: crate::Reg<dcmi_ris::DCMI_RIS_SPEC>,
    #[doc = "0x0c - The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write."]
    pub dcmi_ier: crate::Reg<dcmi_ier::DCMI_IER_SPEC>,
    #[doc = "0x10 - This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set."]
    pub dcmi_mis: crate::Reg<dcmi_mis::DCMI_MIS_SPEC>,
    #[doc = "0x14 - The DCMI_ICR register is write-only."]
    pub dcmi_icr: crate::Reg<dcmi_icr::DCMI_ICR_SPEC>,
    #[doc = "0x18 - DCMI embedded synchronization code register"]
    pub dcmi_escr: crate::Reg<dcmi_escr::DCMI_ESCR_SPEC>,
    #[doc = "0x1c - DCMI embedded synchronization unmask register"]
    pub dcmi_esur: crate::Reg<dcmi_esur::DCMI_ESUR_SPEC>,
    #[doc = "0x20 - DCMI crop window start"]
    pub dcmi_cwstrt: crate::Reg<dcmi_cwstrt::DCMI_CWSTRT_SPEC>,
    #[doc = "0x24 - DCMI crop window size"]
    pub dcmi_cwsize: crate::Reg<dcmi_cwsize::DCMI_CWSIZE_SPEC>,
    #[doc = "0x28 - DCMI data register"]
    pub dcmi_dr: crate::Reg<dcmi_dr::DCMI_DR_SPEC>,
}
#[doc = "DCMI_CR register accessor: an alias for `Reg<DCMI_CR_SPEC>`"]
pub type DCMI_CR = crate::Reg<dcmi_cr::DCMI_CR_SPEC>;
#[doc = "DCMI control register"]
pub mod dcmi_cr;
#[doc = "DCMI_SR register accessor: an alias for `Reg<DCMI_SR_SPEC>`"]
pub type DCMI_SR = crate::Reg<dcmi_sr::DCMI_SR_SPEC>;
#[doc = "DCMI status register"]
pub mod dcmi_sr;
#[doc = "DCMI_RIS register accessor: an alias for `Reg<DCMI_RIS_SPEC>`"]
pub type DCMI_RIS = crate::Reg<dcmi_ris::DCMI_RIS_SPEC>;
#[doc = "DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value."]
pub mod dcmi_ris;
#[doc = "DCMI_IER register accessor: an alias for `Reg<DCMI_IER_SPEC>`"]
pub type DCMI_IER = crate::Reg<dcmi_ier::DCMI_IER_SPEC>;
#[doc = "The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write."]
pub mod dcmi_ier;
#[doc = "DCMI_MIS register accessor: an alias for `Reg<DCMI_MIS_SPEC>`"]
pub type DCMI_MIS = crate::Reg<dcmi_mis::DCMI_MIS_SPEC>;
#[doc = "This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set."]
pub mod dcmi_mis;
#[doc = "DCMI_ICR register accessor: an alias for `Reg<DCMI_ICR_SPEC>`"]
pub type DCMI_ICR = crate::Reg<dcmi_icr::DCMI_ICR_SPEC>;
#[doc = "The DCMI_ICR register is write-only."]
pub mod dcmi_icr;
#[doc = "DCMI_ESCR register accessor: an alias for `Reg<DCMI_ESCR_SPEC>`"]
pub type DCMI_ESCR = crate::Reg<dcmi_escr::DCMI_ESCR_SPEC>;
#[doc = "DCMI embedded synchronization code register"]
pub mod dcmi_escr;
#[doc = "DCMI_ESUR register accessor: an alias for `Reg<DCMI_ESUR_SPEC>`"]
pub type DCMI_ESUR = crate::Reg<dcmi_esur::DCMI_ESUR_SPEC>;
#[doc = "DCMI embedded synchronization unmask register"]
pub mod dcmi_esur;
#[doc = "DCMI_CWSTRT register accessor: an alias for `Reg<DCMI_CWSTRT_SPEC>`"]
pub type DCMI_CWSTRT = crate::Reg<dcmi_cwstrt::DCMI_CWSTRT_SPEC>;
#[doc = "DCMI crop window start"]
pub mod dcmi_cwstrt;
#[doc = "DCMI_CWSIZE register accessor: an alias for `Reg<DCMI_CWSIZE_SPEC>`"]
pub type DCMI_CWSIZE = crate::Reg<dcmi_cwsize::DCMI_CWSIZE_SPEC>;
#[doc = "DCMI crop window size"]
pub mod dcmi_cwsize;
#[doc = "DCMI_DR register accessor: an alias for `Reg<DCMI_DR_SPEC>`"]
pub type DCMI_DR = crate::Reg<dcmi_dr::DCMI_DR_SPEC>;
#[doc = "DCMI data register"]
pub mod dcmi_dr;
