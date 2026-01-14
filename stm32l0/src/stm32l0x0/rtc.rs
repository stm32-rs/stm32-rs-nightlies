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
    _reserved6: [u8; 0x04],
    alrmr: [ALRMR; 2],
    wpr: WPR,
    ssr: SSR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    calr: CALR,
    tampcr: TAMPCR,
    alrmssr: [ALRMSSR; 2],
    or: OR,
    bkpr: [BKPR; 5],
}
impl RegisterBlock {
    ///0x00 - RTC time register
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    ///0x04 - RTC date register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x08 - RTC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - RTC initialization and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10 - RTC prescaler register
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    ///0x14 - RTC wakeup timer register
    #[inline(always)]
    pub const fn wutr(&self) -> &WUTR {
        &self.wutr
    }
    ///0x1c..0x24 - Alarm %s register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ALRMAR` register.</div>
    #[inline(always)]
    pub const fn alrmr(&self, n: usize) -> &ALRMR {
        &self.alrmr[n]
    }
    ///Iterator for array of:
    ///0x1c..0x24 - Alarm %s register
    #[inline(always)]
    pub fn alrmr_iter(&self) -> impl Iterator<Item = &ALRMR> {
        self.alrmr.iter()
    }
    ///0x1c - Alarm A register
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMR {
        self.alrmr(0)
    }
    ///0x20 - Alarm B register
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMR {
        self.alrmr(1)
    }
    ///0x24 - write protection register
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    ///0x28 - RTC sub second register
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    ///0x2c - RTC shift control register
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    ///0x30 - RTC timestamp time register
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    ///0x34 - RTC timestamp date register
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    ///0x38 - RTC time-stamp sub second register
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    ///0x3c - RTC calibration register
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    ///0x40 - RTC tamper configuration register
    #[inline(always)]
    pub const fn tampcr(&self) -> &TAMPCR {
        &self.tampcr
    }
    ///0x44..0x4c - Alarm %s sub-second register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ALRMASSR` register.</div>
    #[inline(always)]
    pub const fn alrmssr(&self, n: usize) -> &ALRMSSR {
        &self.alrmssr[n]
    }
    ///Iterator for array of:
    ///0x44..0x4c - Alarm %s sub-second register
    #[inline(always)]
    pub fn alrmssr_iter(&self) -> impl Iterator<Item = &ALRMSSR> {
        self.alrmssr.iter()
    }
    ///0x44 - Alarm A sub-second register
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMSSR {
        self.alrmssr(0)
    }
    ///0x48 - Alarm B sub-second register
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMSSR {
        self.alrmssr(1)
    }
    ///0x4c - option register
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
    ///0x50..0x64 - RTC backup registers
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    ///Iterator for array of:
    ///0x50..0x64 - RTC backup registers
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    ///0x50 - RTC backup registers
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    ///0x54 - RTC backup registers
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    ///0x58 - RTC backup registers
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    ///0x5c - RTC backup registers
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    ///0x60 - RTC backup registers
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
    }
}
/**TR (rw) register accessor: RTC time register

You can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:TR)

For information about available fields see [`mod@tr`] module*/
pub type TR = crate::Reg<tr::TRrs>;
///RTC time register
pub mod tr;
/**DR (rw) register accessor: RTC date register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///RTC date register
pub mod dr;
/**CR (rw) register accessor: RTC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RTC control register
pub mod cr;
/**ISR (rw) register accessor: RTC initialization and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///RTC initialization and status register
pub mod isr;
/**PRER (rw) register accessor: RTC prescaler register

You can [`read`](crate::Reg::read) this register and get [`prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:PRER)

For information about available fields see [`mod@prer`] module*/
pub type PRER = crate::Reg<prer::PRERrs>;
///RTC prescaler register
pub mod prer;
/**WUTR (rw) register accessor: RTC wakeup timer register

You can [`read`](crate::Reg::read) this register and get [`wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:WUTR)

For information about available fields see [`mod@wutr`] module*/
pub type WUTR = crate::Reg<wutr::WUTRrs>;
///RTC wakeup timer register
pub mod wutr;
/**ALRMR (rw) register accessor: Alarm %s register

You can [`read`](crate::Reg::read) this register and get [`alrmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:ALRM[A]R)

For information about available fields see [`mod@alrmr`] module*/
pub type ALRMR = crate::Reg<alrmr::ALRMRrs>;
///Alarm %s register
pub mod alrmr;
/**WPR (w) register accessor: write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:WPR)

For information about available fields see [`mod@wpr`] module*/
pub type WPR = crate::Reg<wpr::WPRrs>;
///write protection register
pub mod wpr;
/**SSR (r) register accessor: RTC sub second register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:SSR)

For information about available fields see [`mod@ssr`] module*/
pub type SSR = crate::Reg<ssr::SSRrs>;
///RTC sub second register
pub mod ssr;
/**SHIFTR (w) register accessor: RTC shift control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:SHIFTR)

For information about available fields see [`mod@shiftr`] module*/
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
///RTC shift control register
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
/**CALR (rw) register accessor: RTC calibration register

You can [`read`](crate::Reg::read) this register and get [`calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:CALR)

For information about available fields see [`mod@calr`] module*/
pub type CALR = crate::Reg<calr::CALRrs>;
///RTC calibration register
pub mod calr;
/**TAMPCR (rw) register accessor: RTC tamper configuration register

You can [`read`](crate::Reg::read) this register and get [`tampcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:TAMPCR)

For information about available fields see [`mod@tampcr`] module*/
pub type TAMPCR = crate::Reg<tampcr::TAMPCRrs>;
///RTC tamper configuration register
pub mod tampcr;
/**ALRMSSR (rw) register accessor: Alarm %s sub-second register

You can [`read`](crate::Reg::read) this register and get [`alrmssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:ALRM[A]SSR)

For information about available fields see [`mod@alrmssr`] module*/
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSRrs>;
///Alarm %s sub-second register
pub mod alrmssr;
/**OR (rw) register accessor: option register

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///option register
pub mod or;
/**BKPR (rw) register accessor: RTC backup registers

You can [`read`](crate::Reg::read) this register and get [`bkpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RTC:BKP[0]R)

For information about available fields see [`mod@bkpr`] module*/
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
///RTC backup registers
pub mod bkpr;
