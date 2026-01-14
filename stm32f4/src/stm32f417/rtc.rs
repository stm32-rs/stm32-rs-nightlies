#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tr: TR,
    dr: DR,
    cr: CR,
    isr: ISR,
    prer: PRER,
    wutr: WUTR,
    calibr: CALIBR,
    alrmar: ALRMAR,
    alrmbr: ALRMBR,
    wpr: WPR,
    ssr: SSR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    calr: CALR,
    tafcr: TAFCR,
    alrmassr: ALRMASSR,
    alrmbssr: ALRMBSSR,
    _reserved19: [u8; 0x04],
    bkp0r: BKP0R,
    bkp1r: BKP1R,
    bkp2r: BKP2R,
    bkp3r: BKP3R,
    bkp4r: BKP4R,
    bkp5r: BKP5R,
    bkp6r: BKP6R,
    bkp7r: BKP7R,
    bkp8r: BKP8R,
    bkp9r: BKP9R,
    bkp10r: BKP10R,
    bkp11r: BKP11R,
    bkp12r: BKP12R,
    bkp13r: BKP13R,
    bkp14r: BKP14R,
    bkp15r: BKP15R,
    bkp16r: BKP16R,
    bkp17r: BKP17R,
    bkp18r: BKP18R,
    bkp19r: BKP19R,
}
impl RegisterBlock {
    ///0x00 - time register
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    ///0x04 - date register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x08 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - initialization and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10 - prescaler register
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    ///0x14 - wakeup timer register
    #[inline(always)]
    pub const fn wutr(&self) -> &WUTR {
        &self.wutr
    }
    ///0x18 - calibration register
    #[inline(always)]
    pub const fn calibr(&self) -> &CALIBR {
        &self.calibr
    }
    ///0x1c - alarm A register
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMAR {
        &self.alrmar
    }
    ///0x20 - alarm B register
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMBR {
        &self.alrmbr
    }
    ///0x24 - write protection register
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    ///0x28 - sub second register
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    ///0x2c - shift control register
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    ///0x30 - time stamp time register
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    ///0x34 - time stamp date register
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    ///0x38 - timestamp sub second register
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    ///0x3c - calibration register
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    ///0x40 - tamper and alternate function configuration register
    #[inline(always)]
    pub const fn tafcr(&self) -> &TAFCR {
        &self.tafcr
    }
    ///0x44 - alarm A sub second register
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMASSR {
        &self.alrmassr
    }
    ///0x48 - alarm B sub second register
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMBSSR {
        &self.alrmbssr
    }
    ///0x50 - backup register
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKP0R {
        &self.bkp0r
    }
    ///0x54 - backup register
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKP1R {
        &self.bkp1r
    }
    ///0x58 - backup register
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKP2R {
        &self.bkp2r
    }
    ///0x5c - backup register
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKP3R {
        &self.bkp3r
    }
    ///0x60 - backup register
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKP4R {
        &self.bkp4r
    }
    ///0x64 - backup register
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKP5R {
        &self.bkp5r
    }
    ///0x68 - backup register
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKP6R {
        &self.bkp6r
    }
    ///0x6c - backup register
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKP7R {
        &self.bkp7r
    }
    ///0x70 - backup register
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKP8R {
        &self.bkp8r
    }
    ///0x74 - backup register
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKP9R {
        &self.bkp9r
    }
    ///0x78 - backup register
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKP10R {
        &self.bkp10r
    }
    ///0x7c - backup register
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKP11R {
        &self.bkp11r
    }
    ///0x80 - backup register
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKP12R {
        &self.bkp12r
    }
    ///0x84 - backup register
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKP13R {
        &self.bkp13r
    }
    ///0x88 - backup register
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKP14R {
        &self.bkp14r
    }
    ///0x8c - backup register
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKP15R {
        &self.bkp15r
    }
    ///0x90 - backup register
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKP16R {
        &self.bkp16r
    }
    ///0x94 - backup register
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKP17R {
        &self.bkp17r
    }
    ///0x98 - backup register
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKP18R {
        &self.bkp18r
    }
    ///0x9c - backup register
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKP19R {
        &self.bkp19r
    }
}
/**TR (rw) register accessor: time register

You can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:TR)

For information about available fields see [`mod@tr`] module*/
pub type TR = crate::Reg<tr::TRrs>;
///time register
pub mod tr;
/**DR (rw) register accessor: date register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///date register
pub mod dr;
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**ISR (rw) register accessor: initialization and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///initialization and status register
pub mod isr;
/**PRER (rw) register accessor: prescaler register

You can [`read`](crate::Reg::read) this register and get [`prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:PRER)

For information about available fields see [`mod@prer`] module*/
pub type PRER = crate::Reg<prer::PRERrs>;
///prescaler register
pub mod prer;
/**WUTR (rw) register accessor: wakeup timer register

You can [`read`](crate::Reg::read) this register and get [`wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:WUTR)

For information about available fields see [`mod@wutr`] module*/
pub type WUTR = crate::Reg<wutr::WUTRrs>;
///wakeup timer register
pub mod wutr;
/**CALIBR (rw) register accessor: calibration register

You can [`read`](crate::Reg::read) this register and get [`calibr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calibr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:CALIBR)

For information about available fields see [`mod@calibr`] module*/
pub type CALIBR = crate::Reg<calibr::CALIBRrs>;
///calibration register
pub mod calibr;
/**ALRMAR (rw) register accessor: alarm A register

You can [`read`](crate::Reg::read) this register and get [`alrmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:ALRMAR)

For information about available fields see [`mod@alrmar`] module*/
pub type ALRMAR = crate::Reg<alrmar::ALRMARrs>;
///alarm A register
pub mod alrmar;
/**ALRMBR (rw) register accessor: alarm B register

You can [`read`](crate::Reg::read) this register and get [`alrmbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:ALRMBR)

For information about available fields see [`mod@alrmbr`] module*/
pub type ALRMBR = crate::Reg<alrmbr::ALRMBRrs>;
///alarm B register
pub mod alrmbr;
/**WPR (w) register accessor: write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:WPR)

For information about available fields see [`mod@wpr`] module*/
pub type WPR = crate::Reg<wpr::WPRrs>;
///write protection register
pub mod wpr;
/**SSR (r) register accessor: sub second register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:SSR)

For information about available fields see [`mod@ssr`] module*/
pub type SSR = crate::Reg<ssr::SSRrs>;
///sub second register
pub mod ssr;
/**SHIFTR (w) register accessor: shift control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:SHIFTR)

For information about available fields see [`mod@shiftr`] module*/
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
///shift control register
pub mod shiftr;
/**TSTR (r) register accessor: time stamp time register

You can [`read`](crate::Reg::read) this register and get [`tstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:TSTR)

For information about available fields see [`mod@tstr`] module*/
pub type TSTR = crate::Reg<tstr::TSTRrs>;
///time stamp time register
pub mod tstr;
/**TSDR (r) register accessor: time stamp date register

You can [`read`](crate::Reg::read) this register and get [`tsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:TSDR)

For information about available fields see [`mod@tsdr`] module*/
pub type TSDR = crate::Reg<tsdr::TSDRrs>;
///time stamp date register
pub mod tsdr;
/**TSSSR (r) register accessor: timestamp sub second register

You can [`read`](crate::Reg::read) this register and get [`tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:TSSSR)

For information about available fields see [`mod@tsssr`] module*/
pub type TSSSR = crate::Reg<tsssr::TSSSRrs>;
///timestamp sub second register
pub mod tsssr;
/**CALR (rw) register accessor: calibration register

You can [`read`](crate::Reg::read) this register and get [`calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:CALR)

For information about available fields see [`mod@calr`] module*/
pub type CALR = crate::Reg<calr::CALRrs>;
///calibration register
pub mod calr;
/**TAFCR (rw) register accessor: tamper and alternate function configuration register

You can [`read`](crate::Reg::read) this register and get [`tafcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tafcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:TAFCR)

For information about available fields see [`mod@tafcr`] module*/
pub type TAFCR = crate::Reg<tafcr::TAFCRrs>;
///tamper and alternate function configuration register
pub mod tafcr;
/**ALRMASSR (rw) register accessor: alarm A sub second register

You can [`read`](crate::Reg::read) this register and get [`alrmassr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmassr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:ALRMASSR)

For information about available fields see [`mod@alrmassr`] module*/
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSRrs>;
///alarm A sub second register
pub mod alrmassr;
/**ALRMBSSR (rw) register accessor: alarm B sub second register

You can [`read`](crate::Reg::read) this register and get [`alrmbssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:ALRMBSSR)

For information about available fields see [`mod@alrmbssr`] module*/
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSRrs>;
///alarm B sub second register
pub mod alrmbssr;
/**BKP0R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP0R)

For information about available fields see [`mod@bkp0r`] module*/
pub type BKP0R = crate::Reg<bkp0r::BKP0Rrs>;
///backup register
pub mod bkp0r;
/**BKP1R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP1R)

For information about available fields see [`mod@bkp1r`] module*/
pub type BKP1R = crate::Reg<bkp1r::BKP1Rrs>;
///backup register
pub mod bkp1r;
/**BKP2R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP2R)

For information about available fields see [`mod@bkp2r`] module*/
pub type BKP2R = crate::Reg<bkp2r::BKP2Rrs>;
///backup register
pub mod bkp2r;
/**BKP3R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP3R)

For information about available fields see [`mod@bkp3r`] module*/
pub type BKP3R = crate::Reg<bkp3r::BKP3Rrs>;
///backup register
pub mod bkp3r;
/**BKP4R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP4R)

For information about available fields see [`mod@bkp4r`] module*/
pub type BKP4R = crate::Reg<bkp4r::BKP4Rrs>;
///backup register
pub mod bkp4r;
/**BKP5R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP5R)

For information about available fields see [`mod@bkp5r`] module*/
pub type BKP5R = crate::Reg<bkp5r::BKP5Rrs>;
///backup register
pub mod bkp5r;
/**BKP6R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP6R)

For information about available fields see [`mod@bkp6r`] module*/
pub type BKP6R = crate::Reg<bkp6r::BKP6Rrs>;
///backup register
pub mod bkp6r;
/**BKP7R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP7R)

For information about available fields see [`mod@bkp7r`] module*/
pub type BKP7R = crate::Reg<bkp7r::BKP7Rrs>;
///backup register
pub mod bkp7r;
/**BKP8R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP8R)

For information about available fields see [`mod@bkp8r`] module*/
pub type BKP8R = crate::Reg<bkp8r::BKP8Rrs>;
///backup register
pub mod bkp8r;
/**BKP9R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp9r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp9r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP9R)

For information about available fields see [`mod@bkp9r`] module*/
pub type BKP9R = crate::Reg<bkp9r::BKP9Rrs>;
///backup register
pub mod bkp9r;
/**BKP10R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP10R)

For information about available fields see [`mod@bkp10r`] module*/
pub type BKP10R = crate::Reg<bkp10r::BKP10Rrs>;
///backup register
pub mod bkp10r;
/**BKP11R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP11R)

For information about available fields see [`mod@bkp11r`] module*/
pub type BKP11R = crate::Reg<bkp11r::BKP11Rrs>;
///backup register
pub mod bkp11r;
/**BKP12R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP12R)

For information about available fields see [`mod@bkp12r`] module*/
pub type BKP12R = crate::Reg<bkp12r::BKP12Rrs>;
///backup register
pub mod bkp12r;
/**BKP13R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp13r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp13r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP13R)

For information about available fields see [`mod@bkp13r`] module*/
pub type BKP13R = crate::Reg<bkp13r::BKP13Rrs>;
///backup register
pub mod bkp13r;
/**BKP14R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp14r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp14r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP14R)

For information about available fields see [`mod@bkp14r`] module*/
pub type BKP14R = crate::Reg<bkp14r::BKP14Rrs>;
///backup register
pub mod bkp14r;
/**BKP15R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp15r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp15r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP15R)

For information about available fields see [`mod@bkp15r`] module*/
pub type BKP15R = crate::Reg<bkp15r::BKP15Rrs>;
///backup register
pub mod bkp15r;
/**BKP16R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp16r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp16r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP16R)

For information about available fields see [`mod@bkp16r`] module*/
pub type BKP16R = crate::Reg<bkp16r::BKP16Rrs>;
///backup register
pub mod bkp16r;
/**BKP17R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp17r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp17r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP17R)

For information about available fields see [`mod@bkp17r`] module*/
pub type BKP17R = crate::Reg<bkp17r::BKP17Rrs>;
///backup register
pub mod bkp17r;
/**BKP18R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp18r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp18r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP18R)

For information about available fields see [`mod@bkp18r`] module*/
pub type BKP18R = crate::Reg<bkp18r::BKP18Rrs>;
///backup register
pub mod bkp18r;
/**BKP19R (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkp19r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp19r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RTC:BKP19R)

For information about available fields see [`mod@bkp19r`] module*/
pub type BKP19R = crate::Reg<bkp19r::BKP19Rrs>;
///backup register
pub mod bkp19r;
