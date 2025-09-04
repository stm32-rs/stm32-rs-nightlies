#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    fvr: [FVR; 376],
    _reserved1: [u8; 0x0220],
    splock: [SPLOCK; 12],
    _reserved2: [u8; 0x10],
    swlock: [SWLOCK; 12],
    _reserved3: [u8; 0x10],
    srlock: [SRLOCK; 12],
    _reserved4: [u8; 0x10],
    otpvldr: [OTPVLDR; 12],
    _reserved5: [u8; 0x50],
    sfsr0: SFSR0,
    sfsr1: SFSR1,
    sfsr2: SFSR2,
    sfsr3: SFSR3,
    sfsr4: SFSR4,
    sfsr5: SFSR5,
    sfsr6: SFSR6,
    sfsr7: SFSR7,
    sfsr8: SFSR8,
    sfsr9: SFSR9,
    sfsr10: SFSR10,
    sfsr11: SFSR11,
    _reserved17: [u8; 0x0294],
    otpcr: OTPCR,
    wdr: WDR,
    _reserved19: [u8; 0x01f4],
    scratchr: [SCRATCHR; 4],
    lockr: LOCKR,
    jtaginr: JTAGINR,
    jtagoutr: JTAGOUTR,
    _reserved23: [u8; 0x08],
    unmapr: UNMAPR,
    _reserved24: [u8; 0x18],
    sr: SR,
    otpsr: OTPSR,
    _reserved26: [u8; 0x38],
    epochr: [EPOCHR; 2],
    epoch_selr: EPOCH_SELR,
    dbgcr: DBGCR,
    ap_unlock: AP_UNLOCK,
    hdplsr: HDPLSR,
    hdplcr: HDPLCR,
    nextlr: NEXTLR,
    _reserved33: [u8; 0xa0],
    woscr: [WOSCR; 8],
    _reserved34: [u8; 0x88],
    hrcr: HRCR,
    wrcr: WRCR,
}
impl RegisterBlock {
    ///0x00..0x5e0 - BSEC fuse word %s value register
    #[inline(always)]
    pub const fn fvr(&self, n: usize) -> &FVR {
        &self.fvr[n]
    }
    ///Iterator for array of:
    ///0x00..0x5e0 - BSEC fuse word %s value register
    #[inline(always)]
    pub fn fvr_iter(&self) -> impl Iterator<Item = &FVR> {
        self.fvr.iter()
    }
    ///0x800..0x830 - BSEC sticky programming lock register %s
    #[inline(always)]
    pub const fn splock(&self, n: usize) -> &SPLOCK {
        &self.splock[n]
    }
    ///Iterator for array of:
    ///0x800..0x830 - BSEC sticky programming lock register %s
    #[inline(always)]
    pub fn splock_iter(&self) -> impl Iterator<Item = &SPLOCK> {
        self.splock.iter()
    }
    ///0x840..0x870 - BSEC sticky write lock register %s
    #[inline(always)]
    pub const fn swlock(&self, n: usize) -> &SWLOCK {
        &self.swlock[n]
    }
    ///Iterator for array of:
    ///0x840..0x870 - BSEC sticky write lock register %s
    #[inline(always)]
    pub fn swlock_iter(&self) -> impl Iterator<Item = &SWLOCK> {
        self.swlock.iter()
    }
    ///0x880..0x8b0 - BSEC sticky reload lock register %s
    #[inline(always)]
    pub const fn srlock(&self, n: usize) -> &SRLOCK {
        &self.srlock[n]
    }
    ///Iterator for array of:
    ///0x880..0x8b0 - BSEC sticky reload lock register %s
    #[inline(always)]
    pub fn srlock_iter(&self) -> impl Iterator<Item = &SRLOCK> {
        self.srlock.iter()
    }
    ///0x8c0..0x8f0 - BSEC OTP valid register %s
    #[inline(always)]
    pub const fn otpvldr(&self, n: usize) -> &OTPVLDR {
        &self.otpvldr[n]
    }
    ///Iterator for array of:
    ///0x8c0..0x8f0 - BSEC OTP valid register %s
    #[inline(always)]
    pub fn otpvldr_iter(&self) -> impl Iterator<Item = &OTPVLDR> {
        self.otpvldr.iter()
    }
    ///0x940 - BSEC shadowed fuses status register 0
    #[inline(always)]
    pub const fn sfsr0(&self) -> &SFSR0 {
        &self.sfsr0
    }
    ///0x944 - BSEC shadowed fuses status register 1
    #[inline(always)]
    pub const fn sfsr1(&self) -> &SFSR1 {
        &self.sfsr1
    }
    ///0x948 - BSEC shadowed fuses status register 2
    #[inline(always)]
    pub const fn sfsr2(&self) -> &SFSR2 {
        &self.sfsr2
    }
    ///0x94c - BSEC shadowed fuses status register 3
    #[inline(always)]
    pub const fn sfsr3(&self) -> &SFSR3 {
        &self.sfsr3
    }
    ///0x950 - BSEC shadowed fuses status register 4
    #[inline(always)]
    pub const fn sfsr4(&self) -> &SFSR4 {
        &self.sfsr4
    }
    ///0x954 - BSEC shadowed fuses status register 5
    #[inline(always)]
    pub const fn sfsr5(&self) -> &SFSR5 {
        &self.sfsr5
    }
    ///0x958 - BSEC shadowed fuses status register 6
    #[inline(always)]
    pub const fn sfsr6(&self) -> &SFSR6 {
        &self.sfsr6
    }
    ///0x95c - BSEC shadowed fuses status register 7
    #[inline(always)]
    pub const fn sfsr7(&self) -> &SFSR7 {
        &self.sfsr7
    }
    ///0x960 - BSEC shadowed fuses status register 8
    #[inline(always)]
    pub const fn sfsr8(&self) -> &SFSR8 {
        &self.sfsr8
    }
    ///0x964 - BSEC shadowed fuses status register 9
    #[inline(always)]
    pub const fn sfsr9(&self) -> &SFSR9 {
        &self.sfsr9
    }
    ///0x968 - BSEC shadowed fuses status register 10
    #[inline(always)]
    pub const fn sfsr10(&self) -> &SFSR10 {
        &self.sfsr10
    }
    ///0x96c - BSEC shadowed fuses status register 11
    #[inline(always)]
    pub const fn sfsr11(&self) -> &SFSR11 {
        &self.sfsr11
    }
    ///0xc04 - BSEC OTP control register
    #[inline(always)]
    pub const fn otpcr(&self) -> &OTPCR {
        &self.otpcr
    }
    ///0xc08 - BSEC write data register
    #[inline(always)]
    pub const fn wdr(&self) -> &WDR {
        &self.wdr
    }
    ///0xe00..0xe10 - BSEC scratch register %s
    #[inline(always)]
    pub const fn scratchr(&self, n: usize) -> &SCRATCHR {
        &self.scratchr[n]
    }
    ///Iterator for array of:
    ///0xe00..0xe10 - BSEC scratch register %s
    #[inline(always)]
    pub fn scratchr_iter(&self) -> impl Iterator<Item = &SCRATCHR> {
        self.scratchr.iter()
    }
    ///0xe10 - BSEC lock register
    #[inline(always)]
    pub const fn lockr(&self) -> &LOCKR {
        &self.lockr
    }
    ///0xe14 - BSEC JTAG input register
    #[inline(always)]
    pub const fn jtaginr(&self) -> &JTAGINR {
        &self.jtaginr
    }
    ///0xe18 - BSEC JTAG output register
    #[inline(always)]
    pub const fn jtagoutr(&self) -> &JTAGOUTR {
        &self.jtagoutr
    }
    ///0xe24 - BSEC unmap register
    #[inline(always)]
    pub const fn unmapr(&self) -> &UNMAPR {
        &self.unmapr
    }
    ///0xe40 - BSEC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0xe44 - BSEC OTP status register
    #[inline(always)]
    pub const fn otpsr(&self) -> &OTPSR {
        &self.otpsr
    }
    ///0xe80..0xe88 - BSEC epoch register
    #[inline(always)]
    pub const fn epochr(&self, n: usize) -> &EPOCHR {
        &self.epochr[n]
    }
    ///Iterator for array of:
    ///0xe80..0xe88 - BSEC epoch register
    #[inline(always)]
    pub fn epochr_iter(&self) -> impl Iterator<Item = &EPOCHR> {
        self.epochr.iter()
    }
    ///0xe88 - BSEC epoch select register
    #[inline(always)]
    pub const fn epoch_selr(&self) -> &EPOCH_SELR {
        &self.epoch_selr
    }
    ///0xe8c - BSEC Debug
    #[inline(always)]
    pub const fn dbgcr(&self) -> &DBGCR {
        &self.dbgcr
    }
    ///0xe90 - BSEC AP Unlock
    #[inline(always)]
    pub const fn ap_unlock(&self) -> &AP_UNLOCK {
        &self.ap_unlock
    }
    ///0xe94 - BSEC HDPL
    #[inline(always)]
    pub const fn hdplsr(&self) -> &HDPLSR {
        &self.hdplsr
    }
    ///0xe98 - BSEC HDPL control
    #[inline(always)]
    pub const fn hdplcr(&self) -> &HDPLCR {
        &self.hdplcr
    }
    ///0xe9c - BSEC Next HDPL
    #[inline(always)]
    pub const fn nextlr(&self) -> &NEXTLR {
        &self.nextlr
    }
    ///0xf40..0xf60 - BSEC write once scratch register %s
    #[inline(always)]
    pub const fn woscr(&self, n: usize) -> &WOSCR {
        &self.woscr[n]
    }
    ///Iterator for array of:
    ///0xf40..0xf60 - BSEC write once scratch register %s
    #[inline(always)]
    pub fn woscr_iter(&self) -> impl Iterator<Item = &WOSCR> {
        self.woscr.iter()
    }
    ///0xfe8 - BSEC hot reset count register
    #[inline(always)]
    pub const fn hrcr(&self) -> &HRCR {
        &self.hrcr
    }
    ///0xfec - BSEC warm reset count register
    #[inline(always)]
    pub const fn wrcr(&self) -> &WRCR {
        &self.wrcr
    }
}
/**FVR (rw) register accessor: BSEC fuse word %s value register

You can [`read`](crate::Reg::read) this register and get [`fvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:FVR[0])

For information about available fields see [`mod@fvr`] module*/
pub type FVR = crate::Reg<fvr::FVRrs>;
///BSEC fuse word %s value register
pub mod fvr;
/**SPLOCK (rw) register accessor: BSEC sticky programming lock register %s

You can [`read`](crate::Reg::read) this register and get [`splock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`splock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SPLOCK[0])

For information about available fields see [`mod@splock`] module*/
pub type SPLOCK = crate::Reg<splock::SPLOCKrs>;
///BSEC sticky programming lock register %s
pub mod splock;
/**SWLOCK (rw) register accessor: BSEC sticky write lock register %s

You can [`read`](crate::Reg::read) this register and get [`swlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SWLOCK[0])

For information about available fields see [`mod@swlock`] module*/
pub type SWLOCK = crate::Reg<swlock::SWLOCKrs>;
///BSEC sticky write lock register %s
pub mod swlock;
/**SRLOCK (rw) register accessor: BSEC sticky reload lock register %s

You can [`read`](crate::Reg::read) this register and get [`srlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SRLOCK[0])

For information about available fields see [`mod@srlock`] module*/
pub type SRLOCK = crate::Reg<srlock::SRLOCKrs>;
///BSEC sticky reload lock register %s
pub mod srlock;
/**OTPVLDR (r) register accessor: BSEC OTP valid register %s

You can [`read`](crate::Reg::read) this register and get [`otpvldr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:OTPVLDR[0])

For information about available fields see [`mod@otpvldr`] module*/
pub type OTPVLDR = crate::Reg<otpvldr::OTPVLDRrs>;
///BSEC OTP valid register %s
pub mod otpvldr;
/**SFSR0 (r) register accessor: BSEC shadowed fuses status register 0

You can [`read`](crate::Reg::read) this register and get [`sfsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR0)

For information about available fields see [`mod@sfsr0`] module*/
pub type SFSR0 = crate::Reg<sfsr0::SFSR0rs>;
///BSEC shadowed fuses status register 0
pub mod sfsr0;
/**SFSR1 (r) register accessor: BSEC shadowed fuses status register 1

You can [`read`](crate::Reg::read) this register and get [`sfsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR1)

For information about available fields see [`mod@sfsr1`] module*/
pub type SFSR1 = crate::Reg<sfsr1::SFSR1rs>;
///BSEC shadowed fuses status register 1
pub mod sfsr1;
/**SFSR2 (r) register accessor: BSEC shadowed fuses status register 2

You can [`read`](crate::Reg::read) this register and get [`sfsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR2)

For information about available fields see [`mod@sfsr2`] module*/
pub type SFSR2 = crate::Reg<sfsr2::SFSR2rs>;
///BSEC shadowed fuses status register 2
pub mod sfsr2;
/**SFSR3 (r) register accessor: BSEC shadowed fuses status register 3

You can [`read`](crate::Reg::read) this register and get [`sfsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR3)

For information about available fields see [`mod@sfsr3`] module*/
pub type SFSR3 = crate::Reg<sfsr3::SFSR3rs>;
///BSEC shadowed fuses status register 3
pub mod sfsr3;
/**SFSR4 (r) register accessor: BSEC shadowed fuses status register 4

You can [`read`](crate::Reg::read) this register and get [`sfsr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR4)

For information about available fields see [`mod@sfsr4`] module*/
pub type SFSR4 = crate::Reg<sfsr4::SFSR4rs>;
///BSEC shadowed fuses status register 4
pub mod sfsr4;
/**SFSR5 (r) register accessor: BSEC shadowed fuses status register 5

You can [`read`](crate::Reg::read) this register and get [`sfsr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR5)

For information about available fields see [`mod@sfsr5`] module*/
pub type SFSR5 = crate::Reg<sfsr5::SFSR5rs>;
///BSEC shadowed fuses status register 5
pub mod sfsr5;
/**SFSR6 (r) register accessor: BSEC shadowed fuses status register 6

You can [`read`](crate::Reg::read) this register and get [`sfsr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR6)

For information about available fields see [`mod@sfsr6`] module*/
pub type SFSR6 = crate::Reg<sfsr6::SFSR6rs>;
///BSEC shadowed fuses status register 6
pub mod sfsr6;
/**SFSR7 (r) register accessor: BSEC shadowed fuses status register 7

You can [`read`](crate::Reg::read) this register and get [`sfsr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR7)

For information about available fields see [`mod@sfsr7`] module*/
pub type SFSR7 = crate::Reg<sfsr7::SFSR7rs>;
///BSEC shadowed fuses status register 7
pub mod sfsr7;
/**SFSR8 (r) register accessor: BSEC shadowed fuses status register 8

You can [`read`](crate::Reg::read) this register and get [`sfsr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR8)

For information about available fields see [`mod@sfsr8`] module*/
pub type SFSR8 = crate::Reg<sfsr8::SFSR8rs>;
///BSEC shadowed fuses status register 8
pub mod sfsr8;
/**SFSR9 (r) register accessor: BSEC shadowed fuses status register 9

You can [`read`](crate::Reg::read) this register and get [`sfsr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR9)

For information about available fields see [`mod@sfsr9`] module*/
pub type SFSR9 = crate::Reg<sfsr9::SFSR9rs>;
///BSEC shadowed fuses status register 9
pub mod sfsr9;
/**SFSR10 (r) register accessor: BSEC shadowed fuses status register 10

You can [`read`](crate::Reg::read) this register and get [`sfsr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR10)

For information about available fields see [`mod@sfsr10`] module*/
pub type SFSR10 = crate::Reg<sfsr10::SFSR10rs>;
///BSEC shadowed fuses status register 10
pub mod sfsr10;
/**SFSR11 (r) register accessor: BSEC shadowed fuses status register 11

You can [`read`](crate::Reg::read) this register and get [`sfsr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR11)

For information about available fields see [`mod@sfsr11`] module*/
pub type SFSR11 = crate::Reg<sfsr11::SFSR11rs>;
///BSEC shadowed fuses status register 11
pub mod sfsr11;
/**OTPCR (rw) register accessor: BSEC OTP control register

You can [`read`](crate::Reg::read) this register and get [`otpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:OTPCR)

For information about available fields see [`mod@otpcr`] module*/
pub type OTPCR = crate::Reg<otpcr::OTPCRrs>;
///BSEC OTP control register
pub mod otpcr;
/**WDR (w) register accessor: BSEC write data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:WDR)

For information about available fields see [`mod@wdr`] module*/
pub type WDR = crate::Reg<wdr::WDRrs>;
///BSEC write data register
pub mod wdr;
/**SCRATCHR (rw) register accessor: BSEC scratch register %s

You can [`read`](crate::Reg::read) this register and get [`scratchr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratchr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SCRATCHR[0])

For information about available fields see [`mod@scratchr`] module*/
pub type SCRATCHR = crate::Reg<scratchr::SCRATCHRrs>;
///BSEC scratch register %s
pub mod scratchr;
/**LOCKR (rw) register accessor: BSEC lock register

You can [`read`](crate::Reg::read) this register and get [`lockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:LOCKR)

For information about available fields see [`mod@lockr`] module*/
pub type LOCKR = crate::Reg<lockr::LOCKRrs>;
///BSEC lock register
pub mod lockr;
/**JTAGINR (r) register accessor: BSEC JTAG input register

You can [`read`](crate::Reg::read) this register and get [`jtaginr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:JTAGINR)

For information about available fields see [`mod@jtaginr`] module*/
pub type JTAGINR = crate::Reg<jtaginr::JTAGINRrs>;
///BSEC JTAG input register
pub mod jtaginr;
/**JTAGOUTR (w) register accessor: BSEC JTAG output register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtagoutr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:JTAGOUTR)

For information about available fields see [`mod@jtagoutr`] module*/
pub type JTAGOUTR = crate::Reg<jtagoutr::JTAGOUTRrs>;
///BSEC JTAG output register
pub mod jtagoutr;
/**UNMAPR (rw) register accessor: BSEC unmap register

You can [`read`](crate::Reg::read) this register and get [`unmapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unmapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:UNMAPR)

For information about available fields see [`mod@unmapr`] module*/
pub type UNMAPR = crate::Reg<unmapr::UNMAPRrs>;
///BSEC unmap register
pub mod unmapr;
/**SR (r) register accessor: BSEC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///BSEC status register
pub mod sr;
/**OTPSR (r) register accessor: BSEC OTP status register

You can [`read`](crate::Reg::read) this register and get [`otpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:OTPSR)

For information about available fields see [`mod@otpsr`] module*/
pub type OTPSR = crate::Reg<otpsr::OTPSRrs>;
///BSEC OTP status register
pub mod otpsr;
/**EPOCHR (rw) register accessor: BSEC epoch register

You can [`read`](crate::Reg::read) this register and get [`epochr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epochr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:EPOCHR[0])

For information about available fields see [`mod@epochr`] module*/
pub type EPOCHR = crate::Reg<epochr::EPOCHRrs>;
///BSEC epoch register
pub mod epochr;
/**EPOCH_SELR (rw) register accessor: BSEC epoch select register

You can [`read`](crate::Reg::read) this register and get [`epoch_selr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epoch_selr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:EPOCH_SELR)

For information about available fields see [`mod@epoch_selr`] module*/
pub type EPOCH_SELR = crate::Reg<epoch_selr::EPOCH_SELRrs>;
///BSEC epoch select register
pub mod epoch_selr;
/**DBGCR (rw) register accessor: BSEC Debug

You can [`read`](crate::Reg::read) this register and get [`dbgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:DBGCR)

For information about available fields see [`mod@dbgcr`] module*/
pub type DBGCR = crate::Reg<dbgcr::DBGCRrs>;
///BSEC Debug
pub mod dbgcr;
/**AP_UNLOCK (rw) register accessor: BSEC AP Unlock

You can [`read`](crate::Reg::read) this register and get [`ap_unlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap_unlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:AP_UNLOCK)

For information about available fields see [`mod@ap_unlock`] module*/
pub type AP_UNLOCK = crate::Reg<ap_unlock::AP_UNLOCKrs>;
///BSEC AP Unlock
pub mod ap_unlock;
/**HDPLSR (r) register accessor: BSEC HDPL

You can [`read`](crate::Reg::read) this register and get [`hdplsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:HDPLSR)

For information about available fields see [`mod@hdplsr`] module*/
pub type HDPLSR = crate::Reg<hdplsr::HDPLSRrs>;
///BSEC HDPL
pub mod hdplsr;
/**HDPLCR (w) register accessor: BSEC HDPL control

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:HDPLCR)

For information about available fields see [`mod@hdplcr`] module*/
pub type HDPLCR = crate::Reg<hdplcr::HDPLCRrs>;
///BSEC HDPL control
pub mod hdplcr;
/**NEXTLR (rw) register accessor: BSEC Next HDPL

You can [`read`](crate::Reg::read) this register and get [`nextlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nextlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:NEXTLR)

For information about available fields see [`mod@nextlr`] module*/
pub type NEXTLR = crate::Reg<nextlr::NEXTLRrs>;
///BSEC Next HDPL
pub mod nextlr;
/**WOSCR (rw) register accessor: BSEC write once scratch register %s

You can [`read`](crate::Reg::read) this register and get [`woscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`woscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:WOSCR[0])

For information about available fields see [`mod@woscr`] module*/
pub type WOSCR = crate::Reg<woscr::WOSCRrs>;
///BSEC write once scratch register %s
pub mod woscr;
/**HRCR (r) register accessor: BSEC hot reset count register

You can [`read`](crate::Reg::read) this register and get [`hrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:HRCR)

For information about available fields see [`mod@hrcr`] module*/
pub type HRCR = crate::Reg<hrcr::HRCRrs>;
///BSEC hot reset count register
pub mod hrcr;
/**WRCR (r) register accessor: BSEC warm reset count register

You can [`read`](crate::Reg::read) this register and get [`wrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:WRCR)

For information about available fields see [`mod@wrcr`] module*/
pub type WRCR = crate::Reg<wrcr::WRCRrs>;
///BSEC warm reset count register
pub mod wrcr;
