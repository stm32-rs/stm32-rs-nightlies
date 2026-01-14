#[repr(C)]
#[derive(Debug)]
///Register block
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
    bmtrgr: BMTRGR,
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
    bdmupr: BDMUPR,
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
    ///0x00 - Control Register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - Control Register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - Interrupt Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x0c - Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x10 - Interrupt Enable Register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x14 - Output Enable Register
    #[inline(always)]
    pub const fn oenr(&self) -> &OENR {
        &self.oenr
    }
    ///0x18 - ODISR
    #[inline(always)]
    pub const fn odisr(&self) -> &ODISR {
        &self.odisr
    }
    ///0x1c - Output Disable Status Register
    #[inline(always)]
    pub const fn odsr(&self) -> &ODSR {
        &self.odsr
    }
    ///0x20 - Burst Mode Control Register
    #[inline(always)]
    pub const fn bmcr(&self) -> &BMCR {
        &self.bmcr
    }
    ///0x24 - BMTRG
    #[inline(always)]
    pub const fn bmtrgr(&self) -> &BMTRGR {
        &self.bmtrgr
    }
    ///0x28 - BMCMPR
    #[inline(always)]
    pub const fn bmcmpr(&self) -> &BMCMPR {
        &self.bmcmpr
    }
    ///0x2c - Burst Mode Period Register
    #[inline(always)]
    pub const fn bmper(&self) -> &BMPER {
        &self.bmper
    }
    ///0x30 - Timer External Event Control Register 1
    #[inline(always)]
    pub const fn eecr1(&self) -> &EECR1 {
        &self.eecr1
    }
    ///0x34 - Timer External Event Control Register 2
    #[inline(always)]
    pub const fn eecr2(&self) -> &EECR2 {
        &self.eecr2
    }
    ///0x38 - Timer External Event Control Register 3
    #[inline(always)]
    pub const fn eecr3(&self) -> &EECR3 {
        &self.eecr3
    }
    ///0x3c - ADC Trigger 1 Register
    #[inline(always)]
    pub const fn adc1r(&self) -> &ADC1R {
        &self.adc1r
    }
    ///0x40 - ADC Trigger 2 Register
    #[inline(always)]
    pub const fn adc2r(&self) -> &ADC2R {
        &self.adc2r
    }
    ///0x44 - ADC Trigger 3 Register
    #[inline(always)]
    pub const fn adc3r(&self) -> &ADC3R {
        &self.adc3r
    }
    ///0x48 - ADC Trigger 4 Register
    #[inline(always)]
    pub const fn adc4r(&self) -> &ADC4R {
        &self.adc4r
    }
    ///0x4c - DLL Control Register
    #[inline(always)]
    pub const fn dllcr(&self) -> &DLLCR {
        &self.dllcr
    }
    ///0x50 - HRTIM Fault Input Register 1
    #[inline(always)]
    pub const fn fltinr1(&self) -> &FLTINR1 {
        &self.fltinr1
    }
    ///0x54 - HRTIM Fault Input Register 2
    #[inline(always)]
    pub const fn fltinr2(&self) -> &FLTINR2 {
        &self.fltinr2
    }
    ///0x58 - BDMUPDR
    #[inline(always)]
    pub const fn bdmupr(&self) -> &BDMUPR {
        &self.bdmupr
    }
    ///0x5c - Burst DMA Timerx update Register
    #[inline(always)]
    pub const fn bdtaupr(&self) -> &BDTAUPR {
        &self.bdtaupr
    }
    ///0x60 - Burst DMA Timerx update Register
    #[inline(always)]
    pub const fn bdtbupr(&self) -> &BDTBUPR {
        &self.bdtbupr
    }
    ///0x64 - Burst DMA Timerx update Register
    #[inline(always)]
    pub const fn bdtcupr(&self) -> &BDTCUPR {
        &self.bdtcupr
    }
    ///0x68 - Burst DMA Timerx update Register
    #[inline(always)]
    pub const fn bdtdupr(&self) -> &BDTDUPR {
        &self.bdtdupr
    }
    ///0x6c - Burst DMA Timerx update Register
    #[inline(always)]
    pub const fn bdteupr(&self) -> &BDTEUPR {
        &self.bdteupr
    }
    ///0x70 - Burst DMA Data Register
    #[inline(always)]
    pub const fn bdmadr(&self) -> &BDMADR {
        &self.bdmadr
    }
    ///0x74 - Burst DMA Timerx update Register
    #[inline(always)]
    pub const fn bdtfupr(&self) -> &BDTFUPR {
        &self.bdtfupr
    }
    ///0x78 - HRTIM ADC Extended Trigger Register
    #[inline(always)]
    pub const fn adcer(&self) -> &ADCER {
        &self.adcer
    }
    ///0x7c - HRTIM ADC Trigger Update Register
    #[inline(always)]
    pub const fn adcur(&self) -> &ADCUR {
        &self.adcur
    }
    ///0x80 - HRTIM ADC Post Scaler Register 1
    #[inline(always)]
    pub const fn adcps1(&self) -> &ADCPS1 {
        &self.adcps1
    }
    ///0x84 - HRTIM ADC Post Scaler Register 2
    #[inline(always)]
    pub const fn adcps2(&self) -> &ADCPS2 {
        &self.adcps2
    }
    ///0x88 - HRTIM Fault Input Register 3
    #[inline(always)]
    pub const fn fltinr3(&self) -> &FLTINR3 {
        &self.fltinr3
    }
    ///0x8c - HRTIM Fault Input Register 4
    #[inline(always)]
    pub const fn fltinr4(&self) -> &FLTINR4 {
        &self.fltinr4
    }
}
/**CR1 (rw) register accessor: Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///Control Register 1
pub mod cr1;
/**CR2 (rw) register accessor: Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///Control Register 2
pub mod cr2;
/**ISR (r) register accessor: Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///Interrupt Status Register
pub mod isr;
/**ICR (w) register accessor: Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///Interrupt Clear Register
pub mod icr;
/**IER (rw) register accessor: Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///Interrupt Enable Register
pub mod ier;
/**OENR (rw) register accessor: Output Enable Register

You can [`read`](crate::Reg::read) this register and get [`oenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:OENR)

For information about available fields see [`mod@oenr`] module*/
pub type OENR = crate::Reg<oenr::OENRrs>;
///Output Enable Register
pub mod oenr;
/**ODISR (w) register accessor: ODISR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odisr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ODISR)

For information about available fields see [`mod@odisr`] module*/
pub type ODISR = crate::Reg<odisr::ODISRrs>;
///ODISR
pub mod odisr;
/**ODSR (r) register accessor: Output Disable Status Register

You can [`read`](crate::Reg::read) this register and get [`odsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ODSR)

For information about available fields see [`mod@odsr`] module*/
pub type ODSR = crate::Reg<odsr::ODSRrs>;
///Output Disable Status Register
pub mod odsr;
/**BMCR (rw) register accessor: Burst Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`bmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BMCR)

For information about available fields see [`mod@bmcr`] module*/
pub type BMCR = crate::Reg<bmcr::BMCRrs>;
///Burst Mode Control Register
pub mod bmcr;
/**BMTRGR (rw) register accessor: BMTRG

You can [`read`](crate::Reg::read) this register and get [`bmtrgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmtrgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BMTRGR)

For information about available fields see [`mod@bmtrgr`] module*/
pub type BMTRGR = crate::Reg<bmtrgr::BMTRGRrs>;
///BMTRG
pub mod bmtrgr;
/**BMCMPR (rw) register accessor: BMCMPR

You can [`read`](crate::Reg::read) this register and get [`bmcmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BMCMPR)

For information about available fields see [`mod@bmcmpr`] module*/
pub type BMCMPR = crate::Reg<bmcmpr::BMCMPRrs>;
///BMCMPR
pub mod bmcmpr;
/**BMPER (rw) register accessor: Burst Mode Period Register

You can [`read`](crate::Reg::read) this register and get [`bmper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BMPER)

For information about available fields see [`mod@bmper`] module*/
pub type BMPER = crate::Reg<bmper::BMPERrs>;
///Burst Mode Period Register
pub mod bmper;
/**EECR1 (rw) register accessor: Timer External Event Control Register 1

You can [`read`](crate::Reg::read) this register and get [`eecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:EECR1)

For information about available fields see [`mod@eecr1`] module*/
pub type EECR1 = crate::Reg<eecr1::EECR1rs>;
///Timer External Event Control Register 1
pub mod eecr1;
/**EECR2 (rw) register accessor: Timer External Event Control Register 2

You can [`read`](crate::Reg::read) this register and get [`eecr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:EECR2)

For information about available fields see [`mod@eecr2`] module*/
pub type EECR2 = crate::Reg<eecr2::EECR2rs>;
///Timer External Event Control Register 2
pub mod eecr2;
/**EECR3 (rw) register accessor: Timer External Event Control Register 3

You can [`read`](crate::Reg::read) this register and get [`eecr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:EECR3)

For information about available fields see [`mod@eecr3`] module*/
pub type EECR3 = crate::Reg<eecr3::EECR3rs>;
///Timer External Event Control Register 3
pub mod eecr3;
/**ADC1R (rw) register accessor: ADC Trigger 1 Register

You can [`read`](crate::Reg::read) this register and get [`adc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADC1R)

For information about available fields see [`mod@adc1r`] module*/
pub type ADC1R = crate::Reg<adc1r::ADC1Rrs>;
///ADC Trigger 1 Register
pub mod adc1r;
/**ADC2R (rw) register accessor: ADC Trigger 2 Register

You can [`read`](crate::Reg::read) this register and get [`adc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADC2R)

For information about available fields see [`mod@adc2r`] module*/
pub type ADC2R = crate::Reg<adc2r::ADC2Rrs>;
///ADC Trigger 2 Register
pub mod adc2r;
pub use adc1r as adc3r;
pub use adc2r as adc4r;
pub use ADC1R as ADC3R;
pub use ADC2R as ADC4R;
/**DLLCR (rw) register accessor: DLL Control Register

You can [`read`](crate::Reg::read) this register and get [`dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:DLLCR)

For information about available fields see [`mod@dllcr`] module*/
pub type DLLCR = crate::Reg<dllcr::DLLCRrs>;
///DLL Control Register
pub mod dllcr;
/**FLTINR1 (rw) register accessor: HRTIM Fault Input Register 1

You can [`read`](crate::Reg::read) this register and get [`fltinr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:FLTINR1)

For information about available fields see [`mod@fltinr1`] module*/
pub type FLTINR1 = crate::Reg<fltinr1::FLTINR1rs>;
///HRTIM Fault Input Register 1
pub mod fltinr1;
/**FLTINR2 (rw) register accessor: HRTIM Fault Input Register 2

You can [`read`](crate::Reg::read) this register and get [`fltinr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:FLTINR2)

For information about available fields see [`mod@fltinr2`] module*/
pub type FLTINR2 = crate::Reg<fltinr2::FLTINR2rs>;
///HRTIM Fault Input Register 2
pub mod fltinr2;
/**BDMUPR (rw) register accessor: BDMUPDR

You can [`read`](crate::Reg::read) this register and get [`bdmupr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmupr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BDMUPR)

For information about available fields see [`mod@bdmupr`] module*/
pub type BDMUPR = crate::Reg<bdmupr::BDMUPRrs>;
///BDMUPDR
pub mod bdmupr;
/**BDTAUPR (rw) register accessor: Burst DMA Timerx update Register

You can [`read`](crate::Reg::read) this register and get [`bdtaupr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtaupr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BDTAUPR)

For information about available fields see [`mod@bdtaupr`] module*/
pub type BDTAUPR = crate::Reg<bdtaupr::BDTAUPRrs>;
///Burst DMA Timerx update Register
pub mod bdtaupr;
pub use bdtaupr as bdtbupr;
pub use bdtaupr as bdtcupr;
pub use bdtaupr as bdtdupr;
pub use bdtaupr as bdteupr;
pub use bdtaupr as bdtfupr;
pub use BDTAUPR as BDTBUPR;
pub use BDTAUPR as BDTCUPR;
pub use BDTAUPR as BDTDUPR;
pub use BDTAUPR as BDTEUPR;
pub use BDTAUPR as BDTFUPR;
/**BDMADR (w) register accessor: Burst DMA Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmadr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BDMADR)

For information about available fields see [`mod@bdmadr`] module*/
pub type BDMADR = crate::Reg<bdmadr::BDMADRrs>;
///Burst DMA Data Register
pub mod bdmadr;
/**ADCER (rw) register accessor: HRTIM ADC Extended Trigger Register

You can [`read`](crate::Reg::read) this register and get [`adcer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADCER)

For information about available fields see [`mod@adcer`] module*/
pub type ADCER = crate::Reg<adcer::ADCERrs>;
///HRTIM ADC Extended Trigger Register
pub mod adcer;
/**ADCUR (rw) register accessor: HRTIM ADC Trigger Update Register

You can [`read`](crate::Reg::read) this register and get [`adcur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADCUR)

For information about available fields see [`mod@adcur`] module*/
pub type ADCUR = crate::Reg<adcur::ADCURrs>;
///HRTIM ADC Trigger Update Register
pub mod adcur;
/**ADCPS1 (rw) register accessor: HRTIM ADC Post Scaler Register 1

You can [`read`](crate::Reg::read) this register and get [`adcps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADCPS1)

For information about available fields see [`mod@adcps1`] module*/
pub type ADCPS1 = crate::Reg<adcps1::ADCPS1rs>;
///HRTIM ADC Post Scaler Register 1
pub mod adcps1;
/**ADCPS2 (rw) register accessor: HRTIM ADC Post Scaler Register 2

You can [`read`](crate::Reg::read) this register and get [`adcps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADCPS2)

For information about available fields see [`mod@adcps2`] module*/
pub type ADCPS2 = crate::Reg<adcps2::ADCPS2rs>;
///HRTIM ADC Post Scaler Register 2
pub mod adcps2;
/**FLTINR3 (rw) register accessor: HRTIM Fault Input Register 3

You can [`read`](crate::Reg::read) this register and get [`fltinr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:FLTINR3)

For information about available fields see [`mod@fltinr3`] module*/
pub type FLTINR3 = crate::Reg<fltinr3::FLTINR3rs>;
///HRTIM Fault Input Register 3
pub mod fltinr3;
/**FLTINR4 (rw) register accessor: HRTIM Fault Input Register 4

You can [`read`](crate::Reg::read) this register and get [`fltinr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:FLTINR4)

For information about available fields see [`mod@fltinr4`] module*/
pub type FLTINR4 = crate::Reg<fltinr4::FLTINR4rs>;
///HRTIM Fault Input Register 4
pub mod fltinr4;
