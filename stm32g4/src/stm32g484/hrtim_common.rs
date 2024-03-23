#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    isr: ISR,
    icr: ICR,
    ier: IER,
    oenr: OENR,
    odisr: ODISR,
    odsr: ODSR,
    bmcr: BMCR,
    bmtrg: BMTRG,
    bmcmpr: BMCMPR,
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
    bdtaupr: BDTAUPR,
    bdtbupr: BDTBUPR,
    bdtcupr: BDTCUPR,
    bdtdupr: BDTDUPR,
    bdteupr: BDTEUPR,
    bdmadr: BDMADR,
    bdtfupr: BDTFUPR,
    adcer: ADCER,
    adcur: ADCUR,
    adcps1: ADCPS1,
    adcps2: ADCPS2,
    fltinr3: FLTINR3,
    fltinr4: FLTINR4,
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
    #[doc = "0x18 - ODISR"]
    #[inline(always)]
    pub const fn odisr(&self) -> &ODISR {
        &self.odisr
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
    #[doc = "0x28 - BMCMPR"]
    #[inline(always)]
    pub const fn bmcmpr(&self) -> &BMCMPR {
        &self.bmcmpr
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
    pub const fn bdtaupr(&self) -> &BDTAUPR {
        &self.bdtaupr
    }
    #[doc = "0x60 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtbupr(&self) -> &BDTBUPR {
        &self.bdtbupr
    }
    #[doc = "0x64 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtcupr(&self) -> &BDTCUPR {
        &self.bdtcupr
    }
    #[doc = "0x68 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtdupr(&self) -> &BDTDUPR {
        &self.bdtdupr
    }
    #[doc = "0x6c - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdteupr(&self) -> &BDTEUPR {
        &self.bdteupr
    }
    #[doc = "0x70 - Burst DMA Data Register"]
    #[inline(always)]
    pub const fn bdmadr(&self) -> &BDMADR {
        &self.bdmadr
    }
    #[doc = "0x74 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtfupr(&self) -> &BDTFUPR {
        &self.bdtfupr
    }
    #[doc = "0x78 - HRTIM ADC Extended Trigger Register"]
    #[inline(always)]
    pub const fn adcer(&self) -> &ADCER {
        &self.adcer
    }
    #[doc = "0x7c - HRTIM ADC Trigger Update Register"]
    #[inline(always)]
    pub const fn adcur(&self) -> &ADCUR {
        &self.adcur
    }
    #[doc = "0x80 - HRTIM ADC Post Scaler Register 1"]
    #[inline(always)]
    pub const fn adcps1(&self) -> &ADCPS1 {
        &self.adcps1
    }
    #[doc = "0x84 - HRTIM ADC Post Scaler Register 2"]
    #[inline(always)]
    pub const fn adcps2(&self) -> &ADCPS2 {
        &self.adcps2
    }
    #[doc = "0x88 - HRTIM Fault Input Register 3"]
    #[inline(always)]
    pub const fn fltinr3(&self) -> &FLTINR3 {
        &self.fltinr3
    }
    #[doc = "0x8c - HRTIM Fault Input Register 4"]
    #[inline(always)]
    pub const fn fltinr4(&self) -> &FLTINR4 {
        &self.fltinr4
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
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "OENR (rw) register accessor: Output Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oenr`]
module"]
pub type OENR = crate::Reg<oenr::OENRrs>;
#[doc = "Output Enable Register"]
pub mod oenr;
#[doc = "ODISR (w) register accessor: ODISR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odisr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odisr`]
module"]
pub type ODISR = crate::Reg<odisr::ODISRrs>;
#[doc = "ODISR"]
pub mod odisr;
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
#[doc = "BMCMPR (rw) register accessor: BMCMPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmcmpr`]
module"]
pub type BMCMPR = crate::Reg<bmcmpr::BMCMPRrs>;
#[doc = "BMCMPR"]
pub mod bmcmpr;
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
#[doc = "BDTAUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtaupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtaupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtaupr`]
module"]
pub type BDTAUPR = crate::Reg<bdtaupr::BDTAUPRrs>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtaupr;
#[doc = "BDTBUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtbupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtbupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtbupr`]
module"]
pub type BDTBUPR = crate::Reg<bdtbupr::BDTBUPRrs>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtbupr;
#[doc = "BDTCUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtcupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtcupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtcupr`]
module"]
pub type BDTCUPR = crate::Reg<bdtcupr::BDTCUPRrs>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtcupr;
#[doc = "BDTDUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtdupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtdupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtdupr`]
module"]
pub type BDTDUPR = crate::Reg<bdtdupr::BDTDUPRrs>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtdupr;
#[doc = "BDTEUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdteupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdteupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdteupr`]
module"]
pub type BDTEUPR = crate::Reg<bdteupr::BDTEUPRrs>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdteupr;
#[doc = "BDTFUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtfupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtfupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtfupr`]
module"]
pub type BDTFUPR = crate::Reg<bdtfupr::BDTFUPRrs>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtfupr;
#[doc = "BDMADR (w) register accessor: Burst DMA Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmadr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdmadr`]
module"]
pub type BDMADR = crate::Reg<bdmadr::BDMADRrs>;
#[doc = "Burst DMA Data Register"]
pub mod bdmadr;
#[doc = "ADCER (rw) register accessor: HRTIM ADC Extended Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcer`]
module"]
pub type ADCER = crate::Reg<adcer::ADCERrs>;
#[doc = "HRTIM ADC Extended Trigger Register"]
pub mod adcer;
#[doc = "ADCUR (rw) register accessor: HRTIM ADC Trigger Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcur`]
module"]
pub type ADCUR = crate::Reg<adcur::ADCURrs>;
#[doc = "HRTIM ADC Trigger Update Register"]
pub mod adcur;
#[doc = "ADCPS1 (rw) register accessor: HRTIM ADC Post Scaler Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcps1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcps1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcps1`]
module"]
pub type ADCPS1 = crate::Reg<adcps1::ADCPS1rs>;
#[doc = "HRTIM ADC Post Scaler Register 1"]
pub mod adcps1;
#[doc = "ADCPS2 (rw) register accessor: HRTIM ADC Post Scaler Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcps2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcps2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcps2`]
module"]
pub type ADCPS2 = crate::Reg<adcps2::ADCPS2rs>;
#[doc = "HRTIM ADC Post Scaler Register 2"]
pub mod adcps2;
#[doc = "FLTINR3 (rw) register accessor: HRTIM Fault Input Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr3`]
module"]
pub type FLTINR3 = crate::Reg<fltinr3::FLTINR3rs>;
#[doc = "HRTIM Fault Input Register 3"]
pub mod fltinr3;
#[doc = "FLTINR4 (rw) register accessor: HRTIM Fault Input Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr4`]
module"]
pub type FLTINR4 = crate::Reg<fltinr4::FLTINR4rs>;
#[doc = "HRTIM Fault Input Register 4"]
pub mod fltinr4;
