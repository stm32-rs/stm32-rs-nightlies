#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rtc_tr: RTC_TR,
    rtc_dr: RTC_DR,
    rtc_ssr: RTC_SSR,
    rtc_icsr: RTC_ICSR,
    rtc_prer: RTC_PRER,
    rtc_wutr: RTC_WUTR,
    rtc_cr: RTC_CR,
    _reserved7: [u8; 0x08],
    rtc_wpr: RTC_WPR,
    rtc_calr: RTC_CALR,
    rtc_shiftr: RTC_SHIFTR,
    rtc_tstr: RTC_TSTR,
    rtc_tsdr: RTC_TSDR,
    rtc_tsssr: RTC_TSSSR,
    _reserved13: [u8; 0x04],
    rtc_alrmar: RTC_ALRMAR,
    rtc_alrmassr: RTC_ALRMASSR,
    rtc_alrmbr: RTC_ALRMBR,
    rtc_alrmbssr: RTC_ALRMBSSR,
    rtc_sr: RTC_SR,
    rtc_misr: RTC_MISR,
    _reserved19: [u8; 0x04],
    rtc_scr: RTC_SCR,
    _reserved20: [u8; 0x10],
    rtc_alrabinr: RTC_ALRABINR,
    rtc_alrbbinr: RTC_ALRBBINR,
}
impl RegisterBlock {
    ///0x00 - RTC time register
    #[inline(always)]
    pub const fn rtc_tr(&self) -> &RTC_TR {
        &self.rtc_tr
    }
    ///0x04 - RTC date register
    #[inline(always)]
    pub const fn rtc_dr(&self) -> &RTC_DR {
        &self.rtc_dr
    }
    ///0x08 - RTC subsecond register
    #[inline(always)]
    pub const fn rtc_ssr(&self) -> &RTC_SSR {
        &self.rtc_ssr
    }
    ///0x0c - RTC initialization control and status register
    #[inline(always)]
    pub const fn rtc_icsr(&self) -> &RTC_ICSR {
        &self.rtc_icsr
    }
    ///0x10 - RTC prescaler register
    #[inline(always)]
    pub const fn rtc_prer(&self) -> &RTC_PRER {
        &self.rtc_prer
    }
    ///0x14 - RTC wake-up timer register
    #[inline(always)]
    pub const fn rtc_wutr(&self) -> &RTC_WUTR {
        &self.rtc_wutr
    }
    ///0x18 - RTC control register
    #[inline(always)]
    pub const fn rtc_cr(&self) -> &RTC_CR {
        &self.rtc_cr
    }
    ///0x24 - RTC write protection register
    #[inline(always)]
    pub const fn rtc_wpr(&self) -> &RTC_WPR {
        &self.rtc_wpr
    }
    ///0x28 - RTC calibration register
    #[inline(always)]
    pub const fn rtc_calr(&self) -> &RTC_CALR {
        &self.rtc_calr
    }
    ///0x2c - RTC shift control register
    #[inline(always)]
    pub const fn rtc_shiftr(&self) -> &RTC_SHIFTR {
        &self.rtc_shiftr
    }
    ///0x30 - RTC timestamp time register
    #[inline(always)]
    pub const fn rtc_tstr(&self) -> &RTC_TSTR {
        &self.rtc_tstr
    }
    ///0x34 - RTC timestamp date register
    #[inline(always)]
    pub const fn rtc_tsdr(&self) -> &RTC_TSDR {
        &self.rtc_tsdr
    }
    ///0x38 - RTC timestamp subsecond register
    #[inline(always)]
    pub const fn rtc_tsssr(&self) -> &RTC_TSSSR {
        &self.rtc_tsssr
    }
    ///0x40 - RTC alarm A register
    #[inline(always)]
    pub const fn rtc_alrmar(&self) -> &RTC_ALRMAR {
        &self.rtc_alrmar
    }
    ///0x44 - RTC alarm A subsecond register
    #[inline(always)]
    pub const fn rtc_alrmassr(&self) -> &RTC_ALRMASSR {
        &self.rtc_alrmassr
    }
    ///0x48 - RTC alarm B register
    #[inline(always)]
    pub const fn rtc_alrmbr(&self) -> &RTC_ALRMBR {
        &self.rtc_alrmbr
    }
    ///0x4c - RTC alarm B subsecond register
    #[inline(always)]
    pub const fn rtc_alrmbssr(&self) -> &RTC_ALRMBSSR {
        &self.rtc_alrmbssr
    }
    ///0x50 - RTC status register
    #[inline(always)]
    pub const fn rtc_sr(&self) -> &RTC_SR {
        &self.rtc_sr
    }
    ///0x54 - RTC masked interrupt status register
    #[inline(always)]
    pub const fn rtc_misr(&self) -> &RTC_MISR {
        &self.rtc_misr
    }
    ///0x5c - RTC status clear register
    #[inline(always)]
    pub const fn rtc_scr(&self) -> &RTC_SCR {
        &self.rtc_scr
    }
    ///0x70 - RTC alarm A binary mode register
    #[inline(always)]
    pub const fn rtc_alrabinr(&self) -> &RTC_ALRABINR {
        &self.rtc_alrabinr
    }
    ///0x74 - RTC alarm B binary mode register
    #[inline(always)]
    pub const fn rtc_alrbbinr(&self) -> &RTC_ALRBBINR {
        &self.rtc_alrbbinr
    }
}
/**RTC_TR (rw) register accessor: RTC time register

You can [`read`](crate::Reg::read) this register and get [`rtc_tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_TR)

For information about available fields see [`mod@rtc_tr`]
module*/
pub type RTC_TR = crate::Reg<rtc_tr::RTC_TRrs>;
///RTC time register
pub mod rtc_tr;
/**RTC_DR (rw) register accessor: RTC date register

You can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_DR)

For information about available fields see [`mod@rtc_dr`]
module*/
pub type RTC_DR = crate::Reg<rtc_dr::RTC_DRrs>;
///RTC date register
pub mod rtc_dr;
/**RTC_SSR (r) register accessor: RTC subsecond register

You can [`read`](crate::Reg::read) this register and get [`rtc_ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_SSR)

For information about available fields see [`mod@rtc_ssr`]
module*/
pub type RTC_SSR = crate::Reg<rtc_ssr::RTC_SSRrs>;
///RTC subsecond register
pub mod rtc_ssr;
/**RTC_ICSR (rw) register accessor: RTC initialization control and status register

You can [`read`](crate::Reg::read) this register and get [`rtc_icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_ICSR)

For information about available fields see [`mod@rtc_icsr`]
module*/
pub type RTC_ICSR = crate::Reg<rtc_icsr::RTC_ICSRrs>;
///RTC initialization control and status register
pub mod rtc_icsr;
/**RTC_PRER (rw) register accessor: RTC prescaler register

You can [`read`](crate::Reg::read) this register and get [`rtc_prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_PRER)

For information about available fields see [`mod@rtc_prer`]
module*/
pub type RTC_PRER = crate::Reg<rtc_prer::RTC_PRERrs>;
///RTC prescaler register
pub mod rtc_prer;
/**RTC_WUTR (rw) register accessor: RTC wake-up timer register

You can [`read`](crate::Reg::read) this register and get [`rtc_wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_WUTR)

For information about available fields see [`mod@rtc_wutr`]
module*/
pub type RTC_WUTR = crate::Reg<rtc_wutr::RTC_WUTRrs>;
///RTC wake-up timer register
pub mod rtc_wutr;
/**RTC_CR (rw) register accessor: RTC control register

You can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_CR)

For information about available fields see [`mod@rtc_cr`]
module*/
pub type RTC_CR = crate::Reg<rtc_cr::RTC_CRrs>;
///RTC control register
pub mod rtc_cr;
/**RTC_WPR (w) register accessor: RTC write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_WPR)

For information about available fields see [`mod@rtc_wpr`]
module*/
pub type RTC_WPR = crate::Reg<rtc_wpr::RTC_WPRrs>;
///RTC write protection register
pub mod rtc_wpr;
/**RTC_CALR (rw) register accessor: RTC calibration register

You can [`read`](crate::Reg::read) this register and get [`rtc_calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_CALR)

For information about available fields see [`mod@rtc_calr`]
module*/
pub type RTC_CALR = crate::Reg<rtc_calr::RTC_CALRrs>;
///RTC calibration register
pub mod rtc_calr;
/**RTC_SHIFTR (w) register accessor: RTC shift control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_SHIFTR)

For information about available fields see [`mod@rtc_shiftr`]
module*/
pub type RTC_SHIFTR = crate::Reg<rtc_shiftr::RTC_SHIFTRrs>;
///RTC shift control register
pub mod rtc_shiftr;
/**RTC_TSTR (r) register accessor: RTC timestamp time register

You can [`read`](crate::Reg::read) this register and get [`rtc_tstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_TSTR)

For information about available fields see [`mod@rtc_tstr`]
module*/
pub type RTC_TSTR = crate::Reg<rtc_tstr::RTC_TSTRrs>;
///RTC timestamp time register
pub mod rtc_tstr;
/**RTC_TSDR (r) register accessor: RTC timestamp date register

You can [`read`](crate::Reg::read) this register and get [`rtc_tsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_TSDR)

For information about available fields see [`mod@rtc_tsdr`]
module*/
pub type RTC_TSDR = crate::Reg<rtc_tsdr::RTC_TSDRrs>;
///RTC timestamp date register
pub mod rtc_tsdr;
/**RTC_TSSSR (r) register accessor: RTC timestamp subsecond register

You can [`read`](crate::Reg::read) this register and get [`rtc_tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_TSSSR)

For information about available fields see [`mod@rtc_tsssr`]
module*/
pub type RTC_TSSSR = crate::Reg<rtc_tsssr::RTC_TSSSRrs>;
///RTC timestamp subsecond register
pub mod rtc_tsssr;
/**RTC_ALRMAR (rw) register accessor: RTC alarm A register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_ALRMAR)

For information about available fields see [`mod@rtc_alrmar`]
module*/
pub type RTC_ALRMAR = crate::Reg<rtc_alrmar::RTC_ALRMARrs>;
///RTC alarm A register
pub mod rtc_alrmar;
/**RTC_ALRMASSR (rw) register accessor: RTC alarm A subsecond register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrmassr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmassr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_ALRMASSR)

For information about available fields see [`mod@rtc_alrmassr`]
module*/
pub type RTC_ALRMASSR = crate::Reg<rtc_alrmassr::RTC_ALRMASSRrs>;
///RTC alarm A subsecond register
pub mod rtc_alrmassr;
/**RTC_ALRMBR (rw) register accessor: RTC alarm B register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrmbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_ALRMBR)

For information about available fields see [`mod@rtc_alrmbr`]
module*/
pub type RTC_ALRMBR = crate::Reg<rtc_alrmbr::RTC_ALRMBRrs>;
///RTC alarm B register
pub mod rtc_alrmbr;
/**RTC_ALRMBSSR (rw) register accessor: RTC alarm B subsecond register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrmbssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_ALRMBSSR)

For information about available fields see [`mod@rtc_alrmbssr`]
module*/
pub type RTC_ALRMBSSR = crate::Reg<rtc_alrmbssr::RTC_ALRMBSSRrs>;
///RTC alarm B subsecond register
pub mod rtc_alrmbssr;
/**RTC_SR (r) register accessor: RTC status register

You can [`read`](crate::Reg::read) this register and get [`rtc_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_SR)

For information about available fields see [`mod@rtc_sr`]
module*/
pub type RTC_SR = crate::Reg<rtc_sr::RTC_SRrs>;
///RTC status register
pub mod rtc_sr;
/**RTC_MISR (r) register accessor: RTC masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`rtc_misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_MISR)

For information about available fields see [`mod@rtc_misr`]
module*/
pub type RTC_MISR = crate::Reg<rtc_misr::RTC_MISRrs>;
///RTC masked interrupt status register
pub mod rtc_misr;
/**RTC_SCR (w) register accessor: RTC status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_SCR)

For information about available fields see [`mod@rtc_scr`]
module*/
pub type RTC_SCR = crate::Reg<rtc_scr::RTC_SCRrs>;
///RTC status clear register
pub mod rtc_scr;
/**RTC_ALRABINR (rw) register accessor: RTC alarm A binary mode register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrabinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrabinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_ALRABINR)

For information about available fields see [`mod@rtc_alrabinr`]
module*/
pub type RTC_ALRABINR = crate::Reg<rtc_alrabinr::RTC_ALRABINRrs>;
///RTC alarm A binary mode register
pub mod rtc_alrabinr;
/**RTC_ALRBBINR (rw) register accessor: RTC alarm B binary mode register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrbbinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrbbinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RTC:RTC_ALRBBINR)

For information about available fields see [`mod@rtc_alrbbinr`]
module*/
pub type RTC_ALRBBINR = crate::Reg<rtc_alrbbinr::RTC_ALRBBINRrs>;
///RTC alarm B binary mode register
pub mod rtc_alrbbinr;
