#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    pvtreg_lockr: PVTREG_LOCKR,
    pvtlock_sr: PVTLOCK_SR,
    _reserved2: [u8; 0x08],
    pvttmr_cr: PVTTMR_CR,
    pvttmr_sr: PVTTMR_SR,
    _reserved4: [u8; 0x18],
    pvt_ier: PVT_IER,
    _reserved5: [u8; 0x0c],
    pvtirqtrmaskr: PVTIRQTRMASKR,
    ts_mr: TS_MR,
    _reserved7: [u8; 0x08],
    pvttr_sr: PVTTR_SR,
    ts_isr: TS_ISR,
    _reserved9: [u8; 0x08],
    pvttmrraw_isr: PVTTMRRAW_ISR,
    tsraw_isr: TSRAW_ISR,
    _reserved11: [u8; 0x08],
    tscclksynthr: TSCCLKSYNTHR,
    tscsdifdisabler: TSCSDIFDISABLER,
    tscsdif_sr: TSCSDIF_SR,
    tscsdif_cr: TSCSDIF_CR,
    tscsdifhaltr: TSCSDIFHALTR,
    tscsdif_cfgr: TSCSDIF_CFGR,
    _reserved17: [u8; 0x08],
    tscsmpl_cr: TSCSMPL_CR,
    tscsdifsmplclrr: TSCSDIFSMPLCLRR,
    tscsmplcntr: TSCSMPLCNTR,
    _reserved20: [u8; 0x14],
    ts0_ier: TS0_IER,
    ts0_isr: TS0_ISR,
    ts0_icr: TS0_ICR,
    ts0irqtestr: TS0IRQTESTR,
    ts0sdifrdatar: TS0SDIFRDATAR,
    ts0sdifdoner: TS0SDIFDONER,
    ts0sdifdatar: TS0SDIFDATAR,
    _reserved27: [u8; 0x04],
    ts0alarma_cfgr: TS0ALARMA_CFGR,
    ts0alarmb_cfgr: TS0ALARMB_CFGR,
    ts0hlsampler: TS0HLSAMPLER,
    ts0hiloresetr: TS0HILORESETR,
    _reserved31: [u8; 0x10],
    ts1_ier: TS1_IER,
    ts1_isr: TS1_ISR,
    ts1_icr: TS1_ICR,
    ts1irqtestr: TS1IRQTESTR,
    ts1sdifrdatar: TS1SDIFRDATAR,
    ts1sdifdoner: TS1SDIFDONER,
    ts1sdifdatar: TS1SDIFDATAR,
    _reserved38: [u8; 0x04],
    ts1alarma_cfgr: TS1ALARMA_CFGR,
    ts1alarmb_cfgr: TS1ALARMB_CFGR,
    ts1hlsampler: TS1HLSAMPLER,
    ts1hiloresetr: TS1HILORESETR,
}
impl RegisterBlock {
    ///0x10 - DTS PVT register lock register
    #[inline(always)]
    pub const fn pvtreg_lockr(&self) -> &PVTREG_LOCKR {
        &self.pvtreg_lockr
    }
    ///0x14 - DTS PVT lock status register
    #[inline(always)]
    pub const fn pvtlock_sr(&self) -> &PVTLOCK_SR {
        &self.pvtlock_sr
    }
    ///0x20 - DTS PVT timer control register
    #[inline(always)]
    pub const fn pvttmr_cr(&self) -> &PVTTMR_CR {
        &self.pvttmr_cr
    }
    ///0x24 - DTS PVT timer status register
    #[inline(always)]
    pub const fn pvttmr_sr(&self) -> &PVTTMR_SR {
        &self.pvttmr_sr
    }
    ///0x40 - DTS PVT IRQ enable register
    #[inline(always)]
    pub const fn pvt_ier(&self) -> &PVT_IER {
        &self.pvt_ier
    }
    ///0x50 - DTS PVT IRQ timer mask register
    #[inline(always)]
    pub const fn pvtirqtrmaskr(&self) -> &PVTIRQTRMASKR {
        &self.pvtirqtrmaskr
    }
    ///0x54 - DTS PVT IRQ TS mask register
    #[inline(always)]
    pub const fn ts_mr(&self) -> &TS_MR {
        &self.ts_mr
    }
    ///0x60 - DTS PVT IRQ timer status register
    #[inline(always)]
    pub const fn pvttr_sr(&self) -> &PVTTR_SR {
        &self.pvttr_sr
    }
    ///0x64 - DTS PVT IRQ TS status register
    #[inline(always)]
    pub const fn ts_isr(&self) -> &TS_ISR {
        &self.ts_isr
    }
    ///0x70 - DTS PVT IRQ timer raw status register
    #[inline(always)]
    pub const fn pvttmrraw_isr(&self) -> &PVTTMRRAW_ISR {
        &self.pvttmrraw_isr
    }
    ///0x74 - DTS PVT IRQ TS raw status register
    #[inline(always)]
    pub const fn tsraw_isr(&self) -> &TSRAW_ISR {
        &self.tsraw_isr
    }
    ///0x80 - DTS TSC clock synthesizer register
    #[inline(always)]
    pub const fn tscclksynthr(&self) -> &TSCCLKSYNTHR {
        &self.tscclksynthr
    }
    ///0x84 - DTS TSC SDIF interface disable register
    #[inline(always)]
    pub const fn tscsdifdisabler(&self) -> &TSCSDIFDISABLER {
        &self.tscsdifdisabler
    }
    ///0x88 - DTS TSC SDIF status register
    #[inline(always)]
    pub const fn tscsdif_sr(&self) -> &TSCSDIF_SR {
        &self.tscsdif_sr
    }
    ///0x8c - DTS TSC SDIF register
    #[inline(always)]
    pub const fn tscsdif_cr(&self) -> &TSCSDIF_CR {
        &self.tscsdif_cr
    }
    ///0x90 - DTS TSC SDIF halt register
    #[inline(always)]
    pub const fn tscsdifhaltr(&self) -> &TSCSDIFHALTR {
        &self.tscsdifhaltr
    }
    ///0x94 - DTS TSC SDIF control register
    #[inline(always)]
    pub const fn tscsdif_cfgr(&self) -> &TSCSDIF_CFGR {
        &self.tscsdif_cfgr
    }
    ///0xa0 - DTS TSC sample control register
    #[inline(always)]
    pub const fn tscsmpl_cr(&self) -> &TSCSMPL_CR {
        &self.tscsmpl_cr
    }
    ///0xa4 - DTS TSC sample clear register
    #[inline(always)]
    pub const fn tscsdifsmplclrr(&self) -> &TSCSDIFSMPLCLRR {
        &self.tscsdifsmplclrr
    }
    ///0xa8 - DTS TSC sample count register
    #[inline(always)]
    pub const fn tscsmplcntr(&self) -> &TSCSMPLCNTR {
        &self.tscsmplcntr
    }
    ///0xc0 - DTS TS0 IRQ enable register
    #[inline(always)]
    pub const fn ts0_ier(&self) -> &TS0_IER {
        &self.ts0_ier
    }
    ///0xc4 - DTS TS0 IRQ status register
    #[inline(always)]
    pub const fn ts0_isr(&self) -> &TS0_ISR {
        &self.ts0_isr
    }
    ///0xc8 - DTS TS0 IRQ clear register
    #[inline(always)]
    pub const fn ts0_icr(&self) -> &TS0_ICR {
        &self.ts0_icr
    }
    ///0xcc - DTS TS0 IRQ test register
    #[inline(always)]
    pub const fn ts0irqtestr(&self) -> &TS0IRQTESTR {
        &self.ts0irqtestr
    }
    ///0xd0 - DTS TS0 SDIF RDATA register
    #[inline(always)]
    pub const fn ts0sdifrdatar(&self) -> &TS0SDIFRDATAR {
        &self.ts0sdifrdatar
    }
    ///0xd4 - DTS TS0 SDIF done register
    #[inline(always)]
    pub const fn ts0sdifdoner(&self) -> &TS0SDIFDONER {
        &self.ts0sdifdoner
    }
    ///0xd8 - DTS TS0 SDIF data register
    #[inline(always)]
    pub const fn ts0sdifdatar(&self) -> &TS0SDIFDATAR {
        &self.ts0sdifdatar
    }
    ///0xe0 - DTS TS0 alarm A configuration register
    #[inline(always)]
    pub const fn ts0alarma_cfgr(&self) -> &TS0ALARMA_CFGR {
        &self.ts0alarma_cfgr
    }
    ///0xe4 - DTS TS0 alarm B configuration register
    #[inline(always)]
    pub const fn ts0alarmb_cfgr(&self) -> &TS0ALARMB_CFGR {
        &self.ts0alarmb_cfgr
    }
    ///0xe8 - DTS TS0 high/low sample register
    #[inline(always)]
    pub const fn ts0hlsampler(&self) -> &TS0HLSAMPLER {
        &self.ts0hlsampler
    }
    ///0xec - DTS TS0 high/low reset register
    #[inline(always)]
    pub const fn ts0hiloresetr(&self) -> &TS0HILORESETR {
        &self.ts0hiloresetr
    }
    ///0x100 - DTS TS1 IRQ enable register
    #[inline(always)]
    pub const fn ts1_ier(&self) -> &TS1_IER {
        &self.ts1_ier
    }
    ///0x104 - DTS TS1 IRQ status register
    #[inline(always)]
    pub const fn ts1_isr(&self) -> &TS1_ISR {
        &self.ts1_isr
    }
    ///0x108 - DTS TS1 IRQ clear register
    #[inline(always)]
    pub const fn ts1_icr(&self) -> &TS1_ICR {
        &self.ts1_icr
    }
    ///0x10c - DTS TS1 IRQ test register
    #[inline(always)]
    pub const fn ts1irqtestr(&self) -> &TS1IRQTESTR {
        &self.ts1irqtestr
    }
    ///0x110 - DTS TS1 SDIF RDATA register
    #[inline(always)]
    pub const fn ts1sdifrdatar(&self) -> &TS1SDIFRDATAR {
        &self.ts1sdifrdatar
    }
    ///0x114 - DTS TS1 SDIF done register
    #[inline(always)]
    pub const fn ts1sdifdoner(&self) -> &TS1SDIFDONER {
        &self.ts1sdifdoner
    }
    ///0x118 - DTS TS1 SDIF data register
    #[inline(always)]
    pub const fn ts1sdifdatar(&self) -> &TS1SDIFDATAR {
        &self.ts1sdifdatar
    }
    ///0x120 - DTS TS1 alarm A configuration register
    #[inline(always)]
    pub const fn ts1alarma_cfgr(&self) -> &TS1ALARMA_CFGR {
        &self.ts1alarma_cfgr
    }
    ///0x124 - DTS TS1 alarm B configuration register
    #[inline(always)]
    pub const fn ts1alarmb_cfgr(&self) -> &TS1ALARMB_CFGR {
        &self.ts1alarmb_cfgr
    }
    ///0x128 - DTS TS1 high/low sample register
    #[inline(always)]
    pub const fn ts1hlsampler(&self) -> &TS1HLSAMPLER {
        &self.ts1hlsampler
    }
    ///0x12c - DTS TS1 high/low reset register
    #[inline(always)]
    pub const fn ts1hiloresetr(&self) -> &TS1HILORESETR {
        &self.ts1hiloresetr
    }
}
/**PVTREG_LOCKR (rw) register accessor: DTS PVT register lock register

You can [`read`](crate::Reg::read) this register and get [`pvtreg_lockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvtreg_lockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTREG_LOCKR)

For information about available fields see [`mod@pvtreg_lockr`] module*/
pub type PVTREG_LOCKR = crate::Reg<pvtreg_lockr::PVTREG_LOCKRrs>;
///DTS PVT register lock register
pub mod pvtreg_lockr;
/**PVTLOCK_SR (r) register accessor: DTS PVT lock status register

You can [`read`](crate::Reg::read) this register and get [`pvtlock_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTLOCK_SR)

For information about available fields see [`mod@pvtlock_sr`] module*/
pub type PVTLOCK_SR = crate::Reg<pvtlock_sr::PVTLOCK_SRrs>;
///DTS PVT lock status register
pub mod pvtlock_sr;
/**PVTTMR_CR (rw) register accessor: DTS PVT timer control register

You can [`read`](crate::Reg::read) this register and get [`pvttmr_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvttmr_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTTMR_CR)

For information about available fields see [`mod@pvttmr_cr`] module*/
pub type PVTTMR_CR = crate::Reg<pvttmr_cr::PVTTMR_CRrs>;
///DTS PVT timer control register
pub mod pvttmr_cr;
/**PVTTMR_SR (r) register accessor: DTS PVT timer status register

You can [`read`](crate::Reg::read) this register and get [`pvttmr_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTTMR_SR)

For information about available fields see [`mod@pvttmr_sr`] module*/
pub type PVTTMR_SR = crate::Reg<pvttmr_sr::PVTTMR_SRrs>;
///DTS PVT timer status register
pub mod pvttmr_sr;
/**PVT_IER (rw) register accessor: DTS PVT IRQ enable register

You can [`read`](crate::Reg::read) this register and get [`pvt_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVT_IER)

For information about available fields see [`mod@pvt_ier`] module*/
pub type PVT_IER = crate::Reg<pvt_ier::PVT_IERrs>;
///DTS PVT IRQ enable register
pub mod pvt_ier;
/**PVTIRQTRMASKR (rw) register accessor: DTS PVT IRQ timer mask register

You can [`read`](crate::Reg::read) this register and get [`pvtirqtrmaskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvtirqtrmaskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTIRQTRMASKR)

For information about available fields see [`mod@pvtirqtrmaskr`] module*/
pub type PVTIRQTRMASKR = crate::Reg<pvtirqtrmaskr::PVTIRQTRMASKRrs>;
///DTS PVT IRQ timer mask register
pub mod pvtirqtrmaskr;
/**TS_MR (rw) register accessor: DTS PVT IRQ TS mask register

You can [`read`](crate::Reg::read) this register and get [`ts_mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS_MR)

For information about available fields see [`mod@ts_mr`] module*/
pub type TS_MR = crate::Reg<ts_mr::TS_MRrs>;
///DTS PVT IRQ TS mask register
pub mod ts_mr;
/**PVTTR_SR (r) register accessor: DTS PVT IRQ timer status register

You can [`read`](crate::Reg::read) this register and get [`pvttr_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTTR_SR)

For information about available fields see [`mod@pvttr_sr`] module*/
pub type PVTTR_SR = crate::Reg<pvttr_sr::PVTTR_SRrs>;
///DTS PVT IRQ timer status register
pub mod pvttr_sr;
/**TS_ISR (r) register accessor: DTS PVT IRQ TS status register

You can [`read`](crate::Reg::read) this register and get [`ts_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS_ISR)

For information about available fields see [`mod@ts_isr`] module*/
pub type TS_ISR = crate::Reg<ts_isr::TS_ISRrs>;
///DTS PVT IRQ TS status register
pub mod ts_isr;
/**PVTTMRRAW_ISR (r) register accessor: DTS PVT IRQ timer raw status register

You can [`read`](crate::Reg::read) this register and get [`pvttmrraw_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTTMRRAW_ISR)

For information about available fields see [`mod@pvttmrraw_isr`] module*/
pub type PVTTMRRAW_ISR = crate::Reg<pvttmrraw_isr::PVTTMRRAW_ISRrs>;
///DTS PVT IRQ timer raw status register
pub mod pvttmrraw_isr;
/**TSRAW_ISR (r) register accessor: DTS PVT IRQ TS raw status register

You can [`read`](crate::Reg::read) this register and get [`tsraw_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSRAW_ISR)

For information about available fields see [`mod@tsraw_isr`] module*/
pub type TSRAW_ISR = crate::Reg<tsraw_isr::TSRAW_ISRrs>;
///DTS PVT IRQ TS raw status register
pub mod tsraw_isr;
/**TSCCLKSYNTHR (rw) register accessor: DTS TSC clock synthesizer register

You can [`read`](crate::Reg::read) this register and get [`tscclksynthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscclksynthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCCLKSYNTHR)

For information about available fields see [`mod@tscclksynthr`] module*/
pub type TSCCLKSYNTHR = crate::Reg<tscclksynthr::TSCCLKSYNTHRrs>;
///DTS TSC clock synthesizer register
pub mod tscclksynthr;
/**TSCSDIFDISABLER (rw) register accessor: DTS TSC SDIF interface disable register

You can [`read`](crate::Reg::read) this register and get [`tscsdifdisabler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdifdisabler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIFDISABLER)

For information about available fields see [`mod@tscsdifdisabler`] module*/
pub type TSCSDIFDISABLER = crate::Reg<tscsdifdisabler::TSCSDIFDISABLERrs>;
///DTS TSC SDIF interface disable register
pub mod tscsdifdisabler;
/**TSCSDIF_SR (r) register accessor: DTS TSC SDIF status register

You can [`read`](crate::Reg::read) this register and get [`tscsdif_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIF_SR)

For information about available fields see [`mod@tscsdif_sr`] module*/
pub type TSCSDIF_SR = crate::Reg<tscsdif_sr::TSCSDIF_SRrs>;
///DTS TSC SDIF status register
pub mod tscsdif_sr;
/**TSCSDIF_CR (rw) register accessor: DTS TSC SDIF register

You can [`read`](crate::Reg::read) this register and get [`tscsdif_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdif_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIF_CR)

For information about available fields see [`mod@tscsdif_cr`] module*/
pub type TSCSDIF_CR = crate::Reg<tscsdif_cr::TSCSDIF_CRrs>;
///DTS TSC SDIF register
pub mod tscsdif_cr;
/**TSCSDIFHALTR (w) register accessor: DTS TSC SDIF halt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdifhaltr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIFHALTR)

For information about available fields see [`mod@tscsdifhaltr`] module*/
pub type TSCSDIFHALTR = crate::Reg<tscsdifhaltr::TSCSDIFHALTRrs>;
///DTS TSC SDIF halt register
pub mod tscsdifhaltr;
/**TSCSDIF_CFGR (rw) register accessor: DTS TSC SDIF control register

You can [`read`](crate::Reg::read) this register and get [`tscsdif_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdif_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIF_CFGR)

For information about available fields see [`mod@tscsdif_cfgr`] module*/
pub type TSCSDIF_CFGR = crate::Reg<tscsdif_cfgr::TSCSDIF_CFGRrs>;
///DTS TSC SDIF control register
pub mod tscsdif_cfgr;
/**TSCSMPL_CR (rw) register accessor: DTS TSC sample control register

You can [`read`](crate::Reg::read) this register and get [`tscsmpl_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsmpl_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSMPL_CR)

For information about available fields see [`mod@tscsmpl_cr`] module*/
pub type TSCSMPL_CR = crate::Reg<tscsmpl_cr::TSCSMPL_CRrs>;
///DTS TSC sample control register
pub mod tscsmpl_cr;
/**TSCSDIFSMPLCLRR (w) register accessor: DTS TSC sample clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdifsmplclrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIFSMPLCLRR)

For information about available fields see [`mod@tscsdifsmplclrr`] module*/
pub type TSCSDIFSMPLCLRR = crate::Reg<tscsdifsmplclrr::TSCSDIFSMPLCLRRrs>;
///DTS TSC sample clear register
pub mod tscsdifsmplclrr;
/**TSCSMPLCNTR (r) register accessor: DTS TSC sample count register

You can [`read`](crate::Reg::read) this register and get [`tscsmplcntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSMPLCNTR)

For information about available fields see [`mod@tscsmplcntr`] module*/
pub type TSCSMPLCNTR = crate::Reg<tscsmplcntr::TSCSMPLCNTRrs>;
///DTS TSC sample count register
pub mod tscsmplcntr;
/**TS0_IER (rw) register accessor: DTS TS0 IRQ enable register

You can [`read`](crate::Reg::read) this register and get [`ts0_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0_IER)

For information about available fields see [`mod@ts0_ier`] module*/
pub type TS0_IER = crate::Reg<ts0_ier::TS0_IERrs>;
///DTS TS0 IRQ enable register
pub mod ts0_ier;
/**TS0_ISR (r) register accessor: DTS TS0 IRQ status register

You can [`read`](crate::Reg::read) this register and get [`ts0_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0_ISR)

For information about available fields see [`mod@ts0_isr`] module*/
pub type TS0_ISR = crate::Reg<ts0_isr::TS0_ISRrs>;
///DTS TS0 IRQ status register
pub mod ts0_isr;
/**TS0_ICR (w) register accessor: DTS TS0 IRQ clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0_ICR)

For information about available fields see [`mod@ts0_icr`] module*/
pub type TS0_ICR = crate::Reg<ts0_icr::TS0_ICRrs>;
///DTS TS0 IRQ clear register
pub mod ts0_icr;
/**TS0IRQTESTR (rw) register accessor: DTS TS0 IRQ test register

You can [`read`](crate::Reg::read) this register and get [`ts0irqtestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0irqtestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0IRQTESTR)

For information about available fields see [`mod@ts0irqtestr`] module*/
pub type TS0IRQTESTR = crate::Reg<ts0irqtestr::TS0IRQTESTRrs>;
///DTS TS0 IRQ test register
pub mod ts0irqtestr;
/**TS0SDIFRDATAR (r) register accessor: DTS TS0 SDIF RDATA register

You can [`read`](crate::Reg::read) this register and get [`ts0sdifrdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0SDIFRDATAR)

For information about available fields see [`mod@ts0sdifrdatar`] module*/
pub type TS0SDIFRDATAR = crate::Reg<ts0sdifrdatar::TS0SDIFRDATARrs>;
///DTS TS0 SDIF RDATA register
pub mod ts0sdifrdatar;
/**TS0SDIFDONER (r) register accessor: DTS TS0 SDIF done register

You can [`read`](crate::Reg::read) this register and get [`ts0sdifdoner::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0SDIFDONER)

For information about available fields see [`mod@ts0sdifdoner`] module*/
pub type TS0SDIFDONER = crate::Reg<ts0sdifdoner::TS0SDIFDONERrs>;
///DTS TS0 SDIF done register
pub mod ts0sdifdoner;
/**TS0SDIFDATAR (r) register accessor: DTS TS0 SDIF data register

You can [`read`](crate::Reg::read) this register and get [`ts0sdifdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0SDIFDATAR)

For information about available fields see [`mod@ts0sdifdatar`] module*/
pub type TS0SDIFDATAR = crate::Reg<ts0sdifdatar::TS0SDIFDATARrs>;
///DTS TS0 SDIF data register
pub mod ts0sdifdatar;
/**TS0ALARMA_CFGR (rw) register accessor: DTS TS0 alarm A configuration register

You can [`read`](crate::Reg::read) this register and get [`ts0alarma_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0alarma_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0ALARMA_CFGR)

For information about available fields see [`mod@ts0alarma_cfgr`] module*/
pub type TS0ALARMA_CFGR = crate::Reg<ts0alarma_cfgr::TS0ALARMA_CFGRrs>;
///DTS TS0 alarm A configuration register
pub mod ts0alarma_cfgr;
/**TS0ALARMB_CFGR (rw) register accessor: DTS TS0 alarm B configuration register

You can [`read`](crate::Reg::read) this register and get [`ts0alarmb_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0alarmb_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0ALARMB_CFGR)

For information about available fields see [`mod@ts0alarmb_cfgr`] module*/
pub type TS0ALARMB_CFGR = crate::Reg<ts0alarmb_cfgr::TS0ALARMB_CFGRrs>;
///DTS TS0 alarm B configuration register
pub mod ts0alarmb_cfgr;
/**TS0HLSAMPLER (r) register accessor: DTS TS0 high/low sample register

You can [`read`](crate::Reg::read) this register and get [`ts0hlsampler::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0HLSAMPLER)

For information about available fields see [`mod@ts0hlsampler`] module*/
pub type TS0HLSAMPLER = crate::Reg<ts0hlsampler::TS0HLSAMPLERrs>;
///DTS TS0 high/low sample register
pub mod ts0hlsampler;
/**TS0HILORESETR (w) register accessor: DTS TS0 high/low reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0hiloresetr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0HILORESETR)

For information about available fields see [`mod@ts0hiloresetr`] module*/
pub type TS0HILORESETR = crate::Reg<ts0hiloresetr::TS0HILORESETRrs>;
///DTS TS0 high/low reset register
pub mod ts0hiloresetr;
/**TS1_IER (rw) register accessor: DTS TS1 IRQ enable register

You can [`read`](crate::Reg::read) this register and get [`ts1_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1_IER)

For information about available fields see [`mod@ts1_ier`] module*/
pub type TS1_IER = crate::Reg<ts1_ier::TS1_IERrs>;
///DTS TS1 IRQ enable register
pub mod ts1_ier;
/**TS1_ISR (r) register accessor: DTS TS1 IRQ status register

You can [`read`](crate::Reg::read) this register and get [`ts1_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1_ISR)

For information about available fields see [`mod@ts1_isr`] module*/
pub type TS1_ISR = crate::Reg<ts1_isr::TS1_ISRrs>;
///DTS TS1 IRQ status register
pub mod ts1_isr;
/**TS1_ICR (w) register accessor: DTS TS1 IRQ clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1_ICR)

For information about available fields see [`mod@ts1_icr`] module*/
pub type TS1_ICR = crate::Reg<ts1_icr::TS1_ICRrs>;
///DTS TS1 IRQ clear register
pub mod ts1_icr;
/**TS1IRQTESTR (rw) register accessor: DTS TS1 IRQ test register

You can [`read`](crate::Reg::read) this register and get [`ts1irqtestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1irqtestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1IRQTESTR)

For information about available fields see [`mod@ts1irqtestr`] module*/
pub type TS1IRQTESTR = crate::Reg<ts1irqtestr::TS1IRQTESTRrs>;
///DTS TS1 IRQ test register
pub mod ts1irqtestr;
/**TS1SDIFRDATAR (r) register accessor: DTS TS1 SDIF RDATA register

You can [`read`](crate::Reg::read) this register and get [`ts1sdifrdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1SDIFRDATAR)

For information about available fields see [`mod@ts1sdifrdatar`] module*/
pub type TS1SDIFRDATAR = crate::Reg<ts1sdifrdatar::TS1SDIFRDATARrs>;
///DTS TS1 SDIF RDATA register
pub mod ts1sdifrdatar;
/**TS1SDIFDONER (r) register accessor: DTS TS1 SDIF done register

You can [`read`](crate::Reg::read) this register and get [`ts1sdifdoner::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1SDIFDONER)

For information about available fields see [`mod@ts1sdifdoner`] module*/
pub type TS1SDIFDONER = crate::Reg<ts1sdifdoner::TS1SDIFDONERrs>;
///DTS TS1 SDIF done register
pub mod ts1sdifdoner;
/**TS1SDIFDATAR (r) register accessor: DTS TS1 SDIF data register

You can [`read`](crate::Reg::read) this register and get [`ts1sdifdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1SDIFDATAR)

For information about available fields see [`mod@ts1sdifdatar`] module*/
pub type TS1SDIFDATAR = crate::Reg<ts1sdifdatar::TS1SDIFDATARrs>;
///DTS TS1 SDIF data register
pub mod ts1sdifdatar;
/**TS1ALARMA_CFGR (rw) register accessor: DTS TS1 alarm A configuration register

You can [`read`](crate::Reg::read) this register and get [`ts1alarma_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1alarma_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1ALARMA_CFGR)

For information about available fields see [`mod@ts1alarma_cfgr`] module*/
pub type TS1ALARMA_CFGR = crate::Reg<ts1alarma_cfgr::TS1ALARMA_CFGRrs>;
///DTS TS1 alarm A configuration register
pub mod ts1alarma_cfgr;
/**TS1ALARMB_CFGR (rw) register accessor: DTS TS1 alarm B configuration register

You can [`read`](crate::Reg::read) this register and get [`ts1alarmb_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1alarmb_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1ALARMB_CFGR)

For information about available fields see [`mod@ts1alarmb_cfgr`] module*/
pub type TS1ALARMB_CFGR = crate::Reg<ts1alarmb_cfgr::TS1ALARMB_CFGRrs>;
///DTS TS1 alarm B configuration register
pub mod ts1alarmb_cfgr;
/**TS1HLSAMPLER (r) register accessor: DTS TS1 high/low sample register

You can [`read`](crate::Reg::read) this register and get [`ts1hlsampler::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1HLSAMPLER)

For information about available fields see [`mod@ts1hlsampler`] module*/
pub type TS1HLSAMPLER = crate::Reg<ts1hlsampler::TS1HLSAMPLERrs>;
///DTS TS1 high/low sample register
pub mod ts1hlsampler;
/**TS1HILORESETR (w) register accessor: DTS TS1 high/low reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1hiloresetr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1HILORESETR)

For information about available fields see [`mod@ts1hiloresetr`] module*/
pub type TS1HILORESETR = crate::Reg<ts1hiloresetr::TS1HILORESETRrs>;
///DTS TS1 high/low reset register
pub mod ts1hiloresetr;
