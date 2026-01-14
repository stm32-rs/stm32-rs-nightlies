#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tr: TR,
    dr: DR,
    cr: CR,
    isr: ISR,
    prer: PRER,
    _reserved5: [u8; 0x08],
    alrmar: ALRMAR,
    _reserved6: [u8; 0x04],
    wpr: WPR,
    ssr: SSR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    calr: CALR,
    tafcr: TAFCR,
    alrmassr: ALRMASSR,
    _reserved15: [u8; 0x08],
    bkpr: [BKPR; 5],
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
    ///0x1c - alarm A register
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMAR {
        &self.alrmar
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
    ///0x30 - timestamp time register
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    ///0x34 - timestamp date register
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    ///0x38 - time-stamp sub second register
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
    ///0x50..0x64 - backup register
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    ///Iterator for array of:
    ///0x50..0x64 - backup register
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    ///0x50 - backup register
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    ///0x54 - backup register
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    ///0x58 - backup register
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    ///0x5c - backup register
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    ///0x60 - backup register
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
    }
}
/**TR (rw) register accessor: time register

You can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:TR)

For information about available fields see [`mod@tr`] module*/
pub type TR = crate::Reg<tr::TRrs>;
///time register
pub mod tr;
/**DR (rw) register accessor: date register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///date register
pub mod dr;
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**ISR (rw) register accessor: initialization and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///initialization and status register
pub mod isr;
/**PRER (rw) register accessor: prescaler register

You can [`read`](crate::Reg::read) this register and get [`prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:PRER)

For information about available fields see [`mod@prer`] module*/
pub type PRER = crate::Reg<prer::PRERrs>;
///prescaler register
pub mod prer;
/**ALRMAR (rw) register accessor: alarm A register

You can [`read`](crate::Reg::read) this register and get [`alrmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:ALRMAR)

For information about available fields see [`mod@alrmar`] module*/
pub type ALRMAR = crate::Reg<alrmar::ALRMARrs>;
///alarm A register
pub mod alrmar;
/**WPR (w) register accessor: write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:WPR)

For information about available fields see [`mod@wpr`] module*/
pub type WPR = crate::Reg<wpr::WPRrs>;
///write protection register
pub mod wpr;
/**SSR (r) register accessor: sub second register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:SSR)

For information about available fields see [`mod@ssr`] module*/
pub type SSR = crate::Reg<ssr::SSRrs>;
///sub second register
pub mod ssr;
/**SHIFTR (w) register accessor: shift control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:SHIFTR)

For information about available fields see [`mod@shiftr`] module*/
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
///shift control register
pub mod shiftr;
pub use dr as tsdr;
pub use tr as tstr;
pub use DR as TSDR;
pub use TR as TSTR;
/**TSSSR (r) register accessor: time-stamp sub second register

You can [`read`](crate::Reg::read) this register and get [`tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:TSSSR)

For information about available fields see [`mod@tsssr`] module*/
pub type TSSSR = crate::Reg<tsssr::TSSSRrs>;
///time-stamp sub second register
pub mod tsssr;
/**CALR (rw) register accessor: calibration register

You can [`read`](crate::Reg::read) this register and get [`calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:CALR)

For information about available fields see [`mod@calr`] module*/
pub type CALR = crate::Reg<calr::CALRrs>;
///calibration register
pub mod calr;
/**TAFCR (rw) register accessor: tamper and alternate function configuration register

You can [`read`](crate::Reg::read) this register and get [`tafcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tafcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:TAFCR)

For information about available fields see [`mod@tafcr`] module*/
pub type TAFCR = crate::Reg<tafcr::TAFCRrs>;
///tamper and alternate function configuration register
pub mod tafcr;
/**ALRMASSR (rw) register accessor: alarm A sub second register

You can [`read`](crate::Reg::read) this register and get [`alrmassr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmassr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:ALRMASSR)

For information about available fields see [`mod@alrmassr`] module*/
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSRrs>;
///alarm A sub second register
pub mod alrmassr;
/**BKPR (rw) register accessor: backup register

You can [`read`](crate::Reg::read) this register and get [`bkpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#RTC:BKP[0]R)

For information about available fields see [`mod@bkpr`] module*/
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
///backup register
pub mod bkpr;
