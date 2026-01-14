#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tr: TR,
    dr: DR,
    ssr: SSR,
    icsr: ICSR,
    prer: PRER,
    wutr: WUTR,
    cr: CR,
    _reserved7: [u8; 0x04],
    smcr: SMCR,
    wpr: WPR,
    calr: CALR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    _reserved14: [u8; 0x04],
    alrmr: (),
    _reserved15: [u8; 0x04],
    alrmssr: (),
    _reserved16: [u8; 0x0c],
    sr: SR,
    misr: MISR,
    smisr: SMISR,
    scr: SCR,
    cfgr: CFGR,
    _reserved21: [u8; 0x038c],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    ///0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x08 - RTC sub second register
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    ///0x0c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn icsr(&self) -> &ICSR {
        &self.icsr
    }
    ///0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    ///0x14 - This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn wutr(&self) -> &WUTR {
        &self.wutr
    }
    ///0x18 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x20 - This register can be written only when the APB access is secure.
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x24 - RTC write protection register
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    ///0x28 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    ///0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    ///0x30 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    ///0x34 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    ///0x38 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    ///0x40..0x48 - Alarm %s register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ALRMAR` register.</div>
    #[inline(always)]
    pub const fn alrmr(&self, n: usize) -> &ALRMR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x40..0x48 - Alarm %s register
    #[inline(always)]
    pub fn alrmr_iter(&self) -> impl Iterator<Item = &ALRMR> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(8 * n)
                .cast()
        })
    }
    ///0x40 - Alarm A register
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMR {
        self.alrmr(0)
    }
    ///0x48 - Alarm B register
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMR {
        self.alrmr(1)
    }
    ///0x44..0x4c - Alarm %s sub-second register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `ALRMASSR` register.</div>
    #[inline(always)]
    pub const fn alrmssr(&self, n: usize) -> &ALRMSSR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x44..0x4c - Alarm %s sub-second register
    #[inline(always)]
    pub fn alrmssr_iter(&self) -> impl Iterator<Item = &ALRMSSR> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(8 * n)
                .cast()
        })
    }
    ///0x44 - Alarm A sub-second register
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMSSR {
        self.alrmssr(0)
    }
    ///0x4c - Alarm B sub-second register
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMSSR {
        self.alrmssr(1)
    }
    ///0x50 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x54 - RTC non-secure masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x58 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    ///0x5c - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    ///0x60 - RTC configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x3f0 - RTC hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - RTC version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - RTC identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - RTC size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**TR (rw) register accessor: The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:TR)

For information about available fields see [`mod@tr`] module*/
pub type TR = crate::Reg<tr::TRrs>;
///The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod tr;
/**DR (rw) register accessor: The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod dr;
/**SSR (r) register accessor: RTC sub second register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SSR)

For information about available fields see [`mod@ssr`] module*/
pub type SSR = crate::Reg<ssr::SSRrs>;
///RTC sub second register
pub mod ssr;
/**ICSR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:ICSR)

For information about available fields see [`mod@icsr`] module*/
pub type ICSR = crate::Reg<icsr::ICSRrs>;
///This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod icsr;
/**PRER (rw) register accessor: This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:PRER)

For information about available fields see [`mod@prer`] module*/
pub type PRER = crate::Reg<prer::PRERrs>;
///This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod prer;
/**WUTR (rw) register accessor: This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:WUTR)

For information about available fields see [`mod@wutr`] module*/
pub type WUTR = crate::Reg<wutr::WUTRrs>;
///This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod wutr;
/**CR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod cr;
/**SMCR (rw) register accessor: This register can be written only when the APB access is secure.

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///This register can be written only when the APB access is secure.
pub mod smcr;
/**WPR (w) register accessor: RTC write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:WPR)

For information about available fields see [`mod@wpr`] module*/
pub type WPR = crate::Reg<wpr::WPRrs>;
///RTC write protection register
pub mod wpr;
/**CALR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:CALR)

For information about available fields see [`mod@calr`] module*/
pub type CALR = crate::Reg<calr::CALRrs>;
///This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod calr;
/**SHIFTR (w) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SHIFTR)

For information about available fields see [`mod@shiftr`] module*/
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
///This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
/**ALRMR (rw) register accessor: Alarm %s register

You can [`read`](crate::Reg::read) this register and get [`alrmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:ALRM[A]R)

For information about available fields see [`mod@alrmr`] module*/
pub type ALRMR = crate::Reg<alrmr::ALRMRrs>;
///Alarm %s register
pub mod alrmr;
/**ALRMSSR (rw) register accessor: Alarm %s sub-second register

You can [`read`](crate::Reg::read) this register and get [`alrmssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:ALRM[A]SSR)

For information about available fields see [`mod@alrmssr`] module*/
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSRrs>;
///Alarm %s sub-second register
pub mod alrmssr;
/**SR (r) register accessor: This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod sr;
/**MISR (r) register accessor: RTC non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///RTC non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SMISR)

For information about available fields see [`mod@smisr`] module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod smisr;
/**SCR (w) register accessor: This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SCR)

For information about available fields see [`mod@scr`] module*/
pub type SCR = crate::Reg<scr::SCRrs>;
///This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.
pub mod scr;
/**CFGR (rw) register accessor: RTC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///RTC configuration register
pub mod cfgr;
/**HWCFGR (r) register accessor: RTC hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///RTC hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: RTC version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///RTC version register
pub mod verr;
/**IPIDR (r) register accessor: RTC identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///RTC identification register
pub mod ipidr;
/**SIDR (r) register accessor: RTC size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///RTC size identification register
pub mod sidr;
