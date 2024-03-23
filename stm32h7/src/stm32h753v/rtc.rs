#[repr(C)]
#[doc = "Register block"]
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
    bkpr: [BKPR; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x08 - RTC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn wutr(&self) -> &WUTR {
        &self.wutr
    }
    #[doc = "0x1c..0x24 - Alarm %s register"]
    #[inline(always)]
    pub const fn alrmr(&self, n: usize) -> &ALRMR {
        &self.alrmr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - Alarm %s register"]
    #[inline(always)]
    pub fn alrmr_iter(&self) -> impl Iterator<Item = &ALRMR> {
        self.alrmr.iter()
    }
    #[doc = "0x1c - Alarm A register"]
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMR {
        self.alrmr(0)
    }
    #[doc = "0x20 - Alarm B register"]
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMR {
        self.alrmr(1)
    }
    #[doc = "0x24 - RTC write protection register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    #[doc = "0x28 - RTC sub second register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    #[doc = "0x38 - The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    #[doc = "0x3c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    #[doc = "0x40 - RTC tamper and alternate function configuration register"]
    #[inline(always)]
    pub const fn tampcr(&self) -> &TAMPCR {
        &self.tampcr
    }
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub const fn alrmssr(&self, n: usize) -> &ALRMSSR {
        &self.alrmssr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub fn alrmssr_iter(&self) -> impl Iterator<Item = &ALRMSSR> {
        self.alrmssr.iter()
    }
    #[doc = "0x44 - Alarm A sub-second register"]
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMSSR {
        self.alrmssr(0)
    }
    #[doc = "0x48 - Alarm B sub-second register"]
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMSSR {
        self.alrmssr(1)
    }
    #[doc = "0x4c - RTC option register"]
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
    #[doc = "0x50..0xd0 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0xd0 - RTC backup registers"]
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    #[doc = "0x50 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    #[doc = "0x54 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    #[doc = "0x58 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    #[doc = "0x5c - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    #[doc = "0x60 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
    }
    #[doc = "0x64 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKPR {
        self.bkpr(5)
    }
    #[doc = "0x68 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKPR {
        self.bkpr(6)
    }
    #[doc = "0x6c - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKPR {
        self.bkpr(7)
    }
    #[doc = "0x70 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKPR {
        self.bkpr(8)
    }
    #[doc = "0x74 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKPR {
        self.bkpr(9)
    }
    #[doc = "0x78 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKPR {
        self.bkpr(10)
    }
    #[doc = "0x7c - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKPR {
        self.bkpr(11)
    }
    #[doc = "0x80 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKPR {
        self.bkpr(12)
    }
    #[doc = "0x84 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKPR {
        self.bkpr(13)
    }
    #[doc = "0x88 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKPR {
        self.bkpr(14)
    }
    #[doc = "0x8c - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKPR {
        self.bkpr(15)
    }
    #[doc = "0x90 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKPR {
        self.bkpr(16)
    }
    #[doc = "0x94 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKPR {
        self.bkpr(17)
    }
    #[doc = "0x98 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKPR {
        self.bkpr(18)
    }
    #[doc = "0x9c - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKPR {
        self.bkpr(19)
    }
    #[doc = "0xa0 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp20r(&self) -> &BKPR {
        self.bkpr(20)
    }
    #[doc = "0xa4 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp21r(&self) -> &BKPR {
        self.bkpr(21)
    }
    #[doc = "0xa8 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp22r(&self) -> &BKPR {
        self.bkpr(22)
    }
    #[doc = "0xac - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp23r(&self) -> &BKPR {
        self.bkpr(23)
    }
    #[doc = "0xb0 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp24r(&self) -> &BKPR {
        self.bkpr(24)
    }
    #[doc = "0xb4 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp25r(&self) -> &BKPR {
        self.bkpr(25)
    }
    #[doc = "0xb8 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp26r(&self) -> &BKPR {
        self.bkpr(26)
    }
    #[doc = "0xbc - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp27r(&self) -> &BKPR {
        self.bkpr(27)
    }
    #[doc = "0xc0 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp28r(&self) -> &BKPR {
        self.bkpr(28)
    }
    #[doc = "0xc4 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp29r(&self) -> &BKPR {
        self.bkpr(29)
    }
    #[doc = "0xc8 - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp30r(&self) -> &BKPR {
        self.bkpr(30)
    }
    #[doc = "0xcc - RTC backup registers"]
    #[inline(always)]
    pub const fn bkp31r(&self) -> &BKPR {
        self.bkpr(31)
    }
}
#[doc = "TR (rw) register accessor: The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TRrs>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod tr;
#[doc = "DR (rw) register accessor: The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod dr;
#[doc = "CR (rw) register accessor: RTC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "RTC control register"]
pub mod cr;
#[doc = "ISR (rw) register accessor: This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
pub mod isr;
#[doc = "PRER (rw) register accessor: This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer`]
module"]
pub type PRER = crate::Reg<prer::PRERrs>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod prer;
#[doc = "WUTR (rw) register accessor: This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutr`]
module"]
pub type WUTR = crate::Reg<wutr::WUTRrs>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod wutr;
#[doc = "ALRMR (rw) register accessor: Alarm %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmr`]
module"]
pub type ALRMR = crate::Reg<alrmr::ALRMRrs>;
#[doc = "Alarm %s register"]
pub mod alrmr;
#[doc = "WPR (w) register accessor: RTC write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPRrs>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSRrs>;
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "SHIFTR (w) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftr`]
module"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
#[doc = "CALR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
pub type CALR = crate::Reg<calr::CALRrs>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod calr;
#[doc = "TAMPCR (rw) register accessor: RTC tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tampcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tampcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampcr`]
module"]
pub type TAMPCR = crate::Reg<tampcr::TAMPCRrs>;
#[doc = "RTC tamper and alternate function configuration register"]
pub mod tampcr;
#[doc = "ALRMSSR (rw) register accessor: Alarm %s sub-second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmssr`]
module"]
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSRrs>;
#[doc = "Alarm %s sub-second register"]
pub mod alrmssr;
#[doc = "BKPR (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkpr`]
module"]
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
#[doc = "RTC backup registers"]
pub mod bkpr;
#[doc = "OR (rw) register accessor: RTC option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`]
module"]
pub type OR = crate::Reg<or::ORrs>;
#[doc = "RTC option register"]
pub mod or;
