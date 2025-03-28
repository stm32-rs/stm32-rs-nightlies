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
    bkpr: [BKPR; 32],
}
impl RegisterBlock {
    ///0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    ///0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x08 - RTC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - This register is write protected (except for RTC_ISR\[13:8\] bits). The write access procedure is described in RTC register write protection on page9.
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    ///0x14 - This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.
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
    ///0x24 - RTC write protection register
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    ///0x28 - RTC sub second register
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    ///0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page9.
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    ///0x30 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    ///0x34 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    ///0x38 - The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset.
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    ///0x3c - This register is write protected. The write access procedure is described in RTC register write protection on page9.
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    ///0x40 - RTC tamper and alternate function configuration register
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
    ///0x4c - RTC option register
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
    ///0x50..0xd0 - RTC backup registers
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    ///Iterator for array of:
    ///0x50..0xd0 - RTC backup registers
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
    ///0x64 - RTC backup registers
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKPR {
        self.bkpr(5)
    }
    ///0x68 - RTC backup registers
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKPR {
        self.bkpr(6)
    }
    ///0x6c - RTC backup registers
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKPR {
        self.bkpr(7)
    }
    ///0x70 - RTC backup registers
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKPR {
        self.bkpr(8)
    }
    ///0x74 - RTC backup registers
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKPR {
        self.bkpr(9)
    }
    ///0x78 - RTC backup registers
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKPR {
        self.bkpr(10)
    }
    ///0x7c - RTC backup registers
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKPR {
        self.bkpr(11)
    }
    ///0x80 - RTC backup registers
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKPR {
        self.bkpr(12)
    }
    ///0x84 - RTC backup registers
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKPR {
        self.bkpr(13)
    }
    ///0x88 - RTC backup registers
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKPR {
        self.bkpr(14)
    }
    ///0x8c - RTC backup registers
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKPR {
        self.bkpr(15)
    }
    ///0x90 - RTC backup registers
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKPR {
        self.bkpr(16)
    }
    ///0x94 - RTC backup registers
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKPR {
        self.bkpr(17)
    }
    ///0x98 - RTC backup registers
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKPR {
        self.bkpr(18)
    }
    ///0x9c - RTC backup registers
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKPR {
        self.bkpr(19)
    }
    ///0xa0 - RTC backup registers
    #[inline(always)]
    pub const fn bkp20r(&self) -> &BKPR {
        self.bkpr(20)
    }
    ///0xa4 - RTC backup registers
    #[inline(always)]
    pub const fn bkp21r(&self) -> &BKPR {
        self.bkpr(21)
    }
    ///0xa8 - RTC backup registers
    #[inline(always)]
    pub const fn bkp22r(&self) -> &BKPR {
        self.bkpr(22)
    }
    ///0xac - RTC backup registers
    #[inline(always)]
    pub const fn bkp23r(&self) -> &BKPR {
        self.bkpr(23)
    }
    ///0xb0 - RTC backup registers
    #[inline(always)]
    pub const fn bkp24r(&self) -> &BKPR {
        self.bkpr(24)
    }
    ///0xb4 - RTC backup registers
    #[inline(always)]
    pub const fn bkp25r(&self) -> &BKPR {
        self.bkpr(25)
    }
    ///0xb8 - RTC backup registers
    #[inline(always)]
    pub const fn bkp26r(&self) -> &BKPR {
        self.bkpr(26)
    }
    ///0xbc - RTC backup registers
    #[inline(always)]
    pub const fn bkp27r(&self) -> &BKPR {
        self.bkpr(27)
    }
    ///0xc0 - RTC backup registers
    #[inline(always)]
    pub const fn bkp28r(&self) -> &BKPR {
        self.bkpr(28)
    }
    ///0xc4 - RTC backup registers
    #[inline(always)]
    pub const fn bkp29r(&self) -> &BKPR {
        self.bkpr(29)
    }
    ///0xc8 - RTC backup registers
    #[inline(always)]
    pub const fn bkp30r(&self) -> &BKPR {
        self.bkpr(30)
    }
    ///0xcc - RTC backup registers
    #[inline(always)]
    pub const fn bkp31r(&self) -> &BKPR {
        self.bkpr(31)
    }
}
/**TR (rw) register accessor: The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.

You can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:TR)

For information about available fields see [`mod@tr`] module*/
pub type TR = crate::Reg<tr::TRrs>;
///The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.
pub mod tr;
/**DR (rw) register accessor: The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.
pub mod dr;
/**CR (rw) register accessor: RTC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RTC control register
pub mod cr;
/**ISR (rw) register accessor: This register is write protected (except for RTC_ISR\[13:8\] bits). The write access procedure is described in RTC register write protection on page9.

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///This register is write protected (except for RTC_ISR\[13:8\] bits). The write access procedure is described in RTC register write protection on page9.
pub mod isr;
/**PRER (rw) register accessor: This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.

You can [`read`](crate::Reg::read) this register and get [`prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:PRER)

For information about available fields see [`mod@prer`] module*/
pub type PRER = crate::Reg<prer::PRERrs>;
///This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.
pub mod prer;
/**WUTR (rw) register accessor: This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.

You can [`read`](crate::Reg::read) this register and get [`wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:WUTR)

For information about available fields see [`mod@wutr`] module*/
pub type WUTR = crate::Reg<wutr::WUTRrs>;
///This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.
pub mod wutr;
/**ALRMR (rw) register accessor: Alarm %s register

You can [`read`](crate::Reg::read) this register and get [`alrmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:ALRM[A]R)

For information about available fields see [`mod@alrmr`] module*/
pub type ALRMR = crate::Reg<alrmr::ALRMRrs>;
///Alarm %s register
pub mod alrmr;
/**WPR (w) register accessor: RTC write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:WPR)

For information about available fields see [`mod@wpr`] module*/
pub type WPR = crate::Reg<wpr::WPRrs>;
///RTC write protection register
pub mod wpr;
/**SSR (r) register accessor: RTC sub second register

You can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:SSR)

For information about available fields see [`mod@ssr`] module*/
pub type SSR = crate::Reg<ssr::SSRrs>;
///RTC sub second register
pub mod ssr;
/**SHIFTR (w) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:SHIFTR)

For information about available fields see [`mod@shiftr`] module*/
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
///This register is write protected. The write access procedure is described in RTC register write protection on page9.
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
/**CALR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.

You can [`read`](crate::Reg::read) this register and get [`calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:CALR)

For information about available fields see [`mod@calr`] module*/
pub type CALR = crate::Reg<calr::CALRrs>;
///This register is write protected. The write access procedure is described in RTC register write protection on page9.
pub mod calr;
/**TAMPCR (rw) register accessor: RTC tamper and alternate function configuration register

You can [`read`](crate::Reg::read) this register and get [`tampcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:TAMPCR)

For information about available fields see [`mod@tampcr`] module*/
pub type TAMPCR = crate::Reg<tampcr::TAMPCRrs>;
///RTC tamper and alternate function configuration register
pub mod tampcr;
/**ALRMSSR (rw) register accessor: Alarm %s sub-second register

You can [`read`](crate::Reg::read) this register and get [`alrmssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:ALRM[A]SSR)

For information about available fields see [`mod@alrmssr`] module*/
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSRrs>;
///Alarm %s sub-second register
pub mod alrmssr;
/**BKPR (rw) register accessor: RTC backup registers

You can [`read`](crate::Reg::read) this register and get [`bkpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:BKP[0]R)

For information about available fields see [`mod@bkpr`] module*/
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
///RTC backup registers
pub mod bkpr;
/**OR (rw) register accessor: RTC option register

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RTC:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///RTC option register
pub mod or;
