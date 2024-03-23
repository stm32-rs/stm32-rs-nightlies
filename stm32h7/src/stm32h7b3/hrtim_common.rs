#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    isr: ISR,
    icr: ICR,
    ier: IER,
    oenr: OENR,
    disr: DISR,
    odsr: ODSR,
    bmcr: BMCR,
    bmtrg: BMTRG,
    bmcmpr6: BMCMPR6,
    bmper: BMPER,
    eecr1: EECR1,
    eecr2: EECR2,
    eecr3: EECR3,
    adc1r: ADC1R,
    adc2r: ADC2R,
    adc3r: ADC3R,
    adc4r: ADC4R,
    dllcr: DLLCR,
    fltinr1: FLTINR1,
    fltinr2: FLTINR2,
    bdmupdr: BDMUPDR,
    bdtx_upr: BDTX_UPR,
    _reserved24: [u8; 0x10],
    bdmadr: BDMADR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - Control Register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x0c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x14 - Output Enable Register"]
    #[inline(always)]
    pub const fn oenr(&self) -> &OENR {
        &self.oenr
    }
    #[doc = "0x18 - DISR"]
    #[inline(always)]
    pub const fn disr(&self) -> &DISR {
        &self.disr
    }
    #[doc = "0x1c - Output Disable Status Register"]
    #[inline(always)]
    pub const fn odsr(&self) -> &ODSR {
        &self.odsr
    }
    #[doc = "0x20 - Burst Mode Control Register"]
    #[inline(always)]
    pub const fn bmcr(&self) -> &BMCR {
        &self.bmcr
    }
    #[doc = "0x24 - BMTRG"]
    #[inline(always)]
    pub const fn bmtrg(&self) -> &BMTRG {
        &self.bmtrg
    }
    #[doc = "0x28 - BMCMPR6"]
    #[inline(always)]
    pub const fn bmcmpr6(&self) -> &BMCMPR6 {
        &self.bmcmpr6
    }
    #[doc = "0x2c - Burst Mode Period Register"]
    #[inline(always)]
    pub const fn bmper(&self) -> &BMPER {
        &self.bmper
    }
    #[doc = "0x30 - Timer External Event Control Register 1"]
    #[inline(always)]
    pub const fn eecr1(&self) -> &EECR1 {
        &self.eecr1
    }
    #[doc = "0x34 - Timer External Event Control Register 2"]
    #[inline(always)]
    pub const fn eecr2(&self) -> &EECR2 {
        &self.eecr2
    }
    #[doc = "0x38 - Timer External Event Control Register 3"]
    #[inline(always)]
    pub const fn eecr3(&self) -> &EECR3 {
        &self.eecr3
    }
    #[doc = "0x3c - ADC Trigger 1 Register"]
    #[inline(always)]
    pub const fn adc1r(&self) -> &ADC1R {
        &self.adc1r
    }
    #[doc = "0x40 - ADC Trigger 2 Register"]
    #[inline(always)]
    pub const fn adc2r(&self) -> &ADC2R {
        &self.adc2r
    }
    #[doc = "0x44 - ADC Trigger 3 Register"]
    #[inline(always)]
    pub const fn adc3r(&self) -> &ADC3R {
        &self.adc3r
    }
    #[doc = "0x48 - ADC Trigger 4 Register"]
    #[inline(always)]
    pub const fn adc4r(&self) -> &ADC4R {
        &self.adc4r
    }
    #[doc = "0x4c - DLL Control Register"]
    #[inline(always)]
    pub const fn dllcr(&self) -> &DLLCR {
        &self.dllcr
    }
    #[doc = "0x50 - HRTIM Fault Input Register 1"]
    #[inline(always)]
    pub const fn fltinr1(&self) -> &FLTINR1 {
        &self.fltinr1
    }
    #[doc = "0x54 - HRTIM Fault Input Register 2"]
    #[inline(always)]
    pub const fn fltinr2(&self) -> &FLTINR2 {
        &self.fltinr2
    }
    #[doc = "0x58 - BDMUPDR"]
    #[inline(always)]
    pub const fn bdmupdr(&self) -> &BDMUPDR {
        &self.bdmupdr
    }
    #[doc = "0x5c - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtx_upr(&self) -> &BDTX_UPR {
        &self.bdtx_upr
    }
    #[doc = "0x70 - Burst DMA Data register"]
    #[inline(always)]
    pub const fn bdmadr(&self) -> &BDMADR {
        &self.bdmadr
    }
}
#[doc = "CR1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "ISR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "OENR (w) register accessor: Output Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oenr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oenr`]
module"]
pub type OENR = crate::Reg<oenr::OENRrs>;
#[doc = "Output Enable Register"]
pub mod oenr;
#[doc = "DISR (rw) register accessor: DISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disr`]
module"]
pub type DISR = crate::Reg<disr::DISRrs>;
#[doc = "DISR"]
pub mod disr;
#[doc = "ODSR (r) register accessor: Output Disable Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odsr`]
module"]
pub type ODSR = crate::Reg<odsr::ODSRrs>;
#[doc = "Output Disable Status Register"]
pub mod odsr;
#[doc = "BMCR (rw) register accessor: Burst Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmcr`]
module"]
pub type BMCR = crate::Reg<bmcr::BMCRrs>;
#[doc = "Burst Mode Control Register"]
pub mod bmcr;
#[doc = "BMTRG (rw) register accessor: BMTRG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmtrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmtrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmtrg`]
module"]
pub type BMTRG = crate::Reg<bmtrg::BMTRGrs>;
#[doc = "BMTRG"]
pub mod bmtrg;
#[doc = "BMCMPR6 (rw) register accessor: BMCMPR6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmcmpr6`]
module"]
pub type BMCMPR6 = crate::Reg<bmcmpr6::BMCMPR6rs>;
#[doc = "BMCMPR6"]
pub mod bmcmpr6;
#[doc = "BMPER (rw) register accessor: Burst Mode Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmper`]
module"]
pub type BMPER = crate::Reg<bmper::BMPERrs>;
#[doc = "Burst Mode Period Register"]
pub mod bmper;
#[doc = "EECR1 (rw) register accessor: Timer External Event Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr1`]
module"]
pub type EECR1 = crate::Reg<eecr1::EECR1rs>;
#[doc = "Timer External Event Control Register 1"]
pub mod eecr1;
#[doc = "EECR2 (rw) register accessor: Timer External Event Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr2`]
module"]
pub type EECR2 = crate::Reg<eecr2::EECR2rs>;
#[doc = "Timer External Event Control Register 2"]
pub mod eecr2;
#[doc = "EECR3 (rw) register accessor: Timer External Event Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr3`]
module"]
pub type EECR3 = crate::Reg<eecr3::EECR3rs>;
#[doc = "Timer External Event Control Register 3"]
pub mod eecr3;
#[doc = "ADC1R (rw) register accessor: ADC Trigger 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1r`]
module"]
pub type ADC1R = crate::Reg<adc1r::ADC1Rrs>;
#[doc = "ADC Trigger 1 Register"]
pub mod adc1r;
#[doc = "ADC2R (rw) register accessor: ADC Trigger 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc2r`]
module"]
pub type ADC2R = crate::Reg<adc2r::ADC2Rrs>;
#[doc = "ADC Trigger 2 Register"]
pub mod adc2r;
#[doc = "ADC3R (rw) register accessor: ADC Trigger 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc3r`]
module"]
pub type ADC3R = crate::Reg<adc3r::ADC3Rrs>;
#[doc = "ADC Trigger 3 Register"]
pub mod adc3r;
#[doc = "ADC4R (rw) register accessor: ADC Trigger 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc4r`]
module"]
pub type ADC4R = crate::Reg<adc4r::ADC4Rrs>;
#[doc = "ADC Trigger 4 Register"]
pub mod adc4r;
#[doc = "DLLCR (rw) register accessor: DLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dllcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dllcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dllcr`]
module"]
pub type DLLCR = crate::Reg<dllcr::DLLCRrs>;
#[doc = "DLL Control Register"]
pub mod dllcr;
#[doc = "FLTINR1 (rw) register accessor: HRTIM Fault Input Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr1`]
module"]
pub type FLTINR1 = crate::Reg<fltinr1::FLTINR1rs>;
#[doc = "HRTIM Fault Input Register 1"]
pub mod fltinr1;
#[doc = "FLTINR2 (rw) register accessor: HRTIM Fault Input Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr2`]
module"]
pub type FLTINR2 = crate::Reg<fltinr2::FLTINR2rs>;
#[doc = "HRTIM Fault Input Register 2"]
pub mod fltinr2;
#[doc = "BDMUPDR (rw) register accessor: BDMUPDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdmupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdmupdr`]
module"]
pub type BDMUPDR = crate::Reg<bdmupdr::BDMUPDRrs>;
#[doc = "BDMUPDR"]
pub mod bdmupdr;
#[doc = "BDTxUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtx_upr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtx_upr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtx_upr`]
module"]
#[doc(alias = "BDTxUPR")]
pub type BDTX_UPR = crate::Reg<bdtx_upr::BDTX_UPRrs>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtx_upr;
#[doc = "BDMADR (rw) register accessor: Burst DMA Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdmadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdmadr`]
module"]
pub type BDMADR = crate::Reg<bdmadr::BDMADRrs>;
#[doc = "Burst DMA Data register"]
pub mod bdmadr;
