#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    iasr: IASR,
    iacr: IACR,
    iaier: IAIER,
    _reserved5: [u8; 0x10],
    iaddr: IADDR,
    _reserved6: [u8; 0x18],
    regcr1: REGCR1,
    saddr1: SADDR1,
    eaddr1: EADDR1,
    _reserved9: [u8; 0x04],
    regcr2: REGCR2,
    saddr2: SADDR2,
    eaddr2: EADDR2,
    _reserved12: [u8; 0x04],
    regcr3: REGCR3,
    saddr3: SADDR3,
    eaddr3: EADDR3,
    _reserved15: [u8; 0x04],
    regcr4: REGCR4,
    saddr4: SADDR4,
    eaddr4: EADDR4,
    _reserved18: [u8; 0x0184],
    mkeyr0: MKEYR0,
    mkeyr1: MKEYR1,
    mkeyr2: MKEYR2,
    mkeyr3: MKEYR3,
    mkeyr4: MKEYR4,
    mkeyr5: MKEYR5,
    mkeyr6: MKEYR6,
    mkeyr7: MKEYR7,
    fmkeyr0: FMKEYR0,
    fmkeyr1: FMKEYR1,
    fmkeyr2: FMKEYR2,
    fmkeyr3: FMKEYR3,
    fmkeyr4: FMKEYR4,
    fmkeyr5: FMKEYR5,
    fmkeyr6: FMKEYR6,
    fmkeyr7: FMKEYR7,
    cc1cfgr: CC1CFGR,
    cc1nr0: CC1NR0,
    cc1nr1: CC1NR1,
    cc1keyr0: CC1KEYR0,
    cc1keyr1: CC1KEYR1,
    cc1keyr2: CC1KEYR2,
    cc1keyr3: CC1KEYR3,
    _reserved41: [u8; 0x14],
    cc2cfgr: CC2CFGR,
    cc2nr0: CC2NR0,
    cc2nr1: CC2NR1,
    cc2keyr0: CC2KEYR0,
    cc2keyr1: CC2KEYR1,
    cc2keyr2: CC2KEYR2,
    cc2keyr3: CC2KEYR3,
}
impl RegisterBlock {
    ///0x00 - MCE configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - MCE status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - MCE illegal access status register
    #[inline(always)]
    pub const fn iasr(&self) -> &IASR {
        &self.iasr
    }
    ///0x0c - MCE illegal access clear register
    #[inline(always)]
    pub const fn iacr(&self) -> &IACR {
        &self.iacr
    }
    ///0x10 - MCE illegal access interrupt enable register
    #[inline(always)]
    pub const fn iaier(&self) -> &IAIER {
        &self.iaier
    }
    ///0x24 - MCE illegal address register
    #[inline(always)]
    pub const fn iaddr(&self) -> &IADDR {
        &self.iaddr
    }
    ///0x40 - MCE region 1 configuration register
    #[inline(always)]
    pub const fn regcr1(&self) -> &REGCR1 {
        &self.regcr1
    }
    ///0x44 - MCE start address for region 1 register
    #[inline(always)]
    pub const fn saddr1(&self) -> &SADDR1 {
        &self.saddr1
    }
    ///0x48 - MCE end address for region 1 register
    #[inline(always)]
    pub const fn eaddr1(&self) -> &EADDR1 {
        &self.eaddr1
    }
    ///0x50 - MCE region 2 configuration register
    #[inline(always)]
    pub const fn regcr2(&self) -> &REGCR2 {
        &self.regcr2
    }
    ///0x54 - MCE start address for region 2 register
    #[inline(always)]
    pub const fn saddr2(&self) -> &SADDR2 {
        &self.saddr2
    }
    ///0x58 - MCE end address for region 2 register
    #[inline(always)]
    pub const fn eaddr2(&self) -> &EADDR2 {
        &self.eaddr2
    }
    ///0x60 - MCE region 3 configuration register
    #[inline(always)]
    pub const fn regcr3(&self) -> &REGCR3 {
        &self.regcr3
    }
    ///0x64 - MCE start address for region 3 register
    #[inline(always)]
    pub const fn saddr3(&self) -> &SADDR3 {
        &self.saddr3
    }
    ///0x68 - MCE end address for region 3 register
    #[inline(always)]
    pub const fn eaddr3(&self) -> &EADDR3 {
        &self.eaddr3
    }
    ///0x70 - MCE region 4 configuration register
    #[inline(always)]
    pub const fn regcr4(&self) -> &REGCR4 {
        &self.regcr4
    }
    ///0x74 - MCE start address for region 4 register
    #[inline(always)]
    pub const fn saddr4(&self) -> &SADDR4 {
        &self.saddr4
    }
    ///0x78 - MCE end address for region 4 register
    #[inline(always)]
    pub const fn eaddr4(&self) -> &EADDR4 {
        &self.eaddr4
    }
    ///0x200 - .MCE master key 0
    #[inline(always)]
    pub const fn mkeyr0(&self) -> &MKEYR0 {
        &self.mkeyr0
    }
    ///0x204 - .MCE master key 1
    #[inline(always)]
    pub const fn mkeyr1(&self) -> &MKEYR1 {
        &self.mkeyr1
    }
    ///0x208 - .MCE master key 2
    #[inline(always)]
    pub const fn mkeyr2(&self) -> &MKEYR2 {
        &self.mkeyr2
    }
    ///0x20c - .MCE master key 3
    #[inline(always)]
    pub const fn mkeyr3(&self) -> &MKEYR3 {
        &self.mkeyr3
    }
    ///0x210 - .MCE master key 4
    #[inline(always)]
    pub const fn mkeyr4(&self) -> &MKEYR4 {
        &self.mkeyr4
    }
    ///0x214 - .MCE master key 5
    #[inline(always)]
    pub const fn mkeyr5(&self) -> &MKEYR5 {
        &self.mkeyr5
    }
    ///0x218 - .MCE master key 6
    #[inline(always)]
    pub const fn mkeyr6(&self) -> &MKEYR6 {
        &self.mkeyr6
    }
    ///0x21c - .MCE master key 7
    #[inline(always)]
    pub const fn mkeyr7(&self) -> &MKEYR7 {
        &self.mkeyr7
    }
    ///0x220 - MCE fast master key 0
    #[inline(always)]
    pub const fn fmkeyr0(&self) -> &FMKEYR0 {
        &self.fmkeyr0
    }
    ///0x224 - MCE fast master key 1
    #[inline(always)]
    pub const fn fmkeyr1(&self) -> &FMKEYR1 {
        &self.fmkeyr1
    }
    ///0x228 - MCE fast master key 2
    #[inline(always)]
    pub const fn fmkeyr2(&self) -> &FMKEYR2 {
        &self.fmkeyr2
    }
    ///0x22c - MCE fast master key 3
    #[inline(always)]
    pub const fn fmkeyr3(&self) -> &FMKEYR3 {
        &self.fmkeyr3
    }
    ///0x230 - MCE fast master key 4
    #[inline(always)]
    pub const fn fmkeyr4(&self) -> &FMKEYR4 {
        &self.fmkeyr4
    }
    ///0x234 - MCE fast master key 5
    #[inline(always)]
    pub const fn fmkeyr5(&self) -> &FMKEYR5 {
        &self.fmkeyr5
    }
    ///0x238 - MCE fast master key 6
    #[inline(always)]
    pub const fn fmkeyr6(&self) -> &FMKEYR6 {
        &self.fmkeyr6
    }
    ///0x23c - MCE fast master key 7
    #[inline(always)]
    pub const fn fmkeyr7(&self) -> &FMKEYR7 {
        &self.fmkeyr7
    }
    ///0x240 - MCE cipher context 1 configuration register
    #[inline(always)]
    pub const fn cc1cfgr(&self) -> &CC1CFGR {
        &self.cc1cfgr
    }
    ///0x244 - MCE cipher context 1 nonce register 0
    #[inline(always)]
    pub const fn cc1nr0(&self) -> &CC1NR0 {
        &self.cc1nr0
    }
    ///0x248 - MCE cipher context 1 nonce register 1
    #[inline(always)]
    pub const fn cc1nr1(&self) -> &CC1NR1 {
        &self.cc1nr1
    }
    ///0x24c - MCE cipher context 1 key register 0
    #[inline(always)]
    pub const fn cc1keyr0(&self) -> &CC1KEYR0 {
        &self.cc1keyr0
    }
    ///0x250 - MCE cipher context 1 key register 1
    #[inline(always)]
    pub const fn cc1keyr1(&self) -> &CC1KEYR1 {
        &self.cc1keyr1
    }
    ///0x254 - MCE cipher context 1 key register 2
    #[inline(always)]
    pub const fn cc1keyr2(&self) -> &CC1KEYR2 {
        &self.cc1keyr2
    }
    ///0x258 - MCE cipher context 1 key register 3
    #[inline(always)]
    pub const fn cc1keyr3(&self) -> &CC1KEYR3 {
        &self.cc1keyr3
    }
    ///0x270 - MCE cipher context 2 configuration register
    #[inline(always)]
    pub const fn cc2cfgr(&self) -> &CC2CFGR {
        &self.cc2cfgr
    }
    ///0x274 - MCE cipher context 2 nonce register 0
    #[inline(always)]
    pub const fn cc2nr0(&self) -> &CC2NR0 {
        &self.cc2nr0
    }
    ///0x278 - MCE cipher context 2 nonce register 1
    #[inline(always)]
    pub const fn cc2nr1(&self) -> &CC2NR1 {
        &self.cc2nr1
    }
    ///0x27c - MCE cipher context 2 key register 0
    #[inline(always)]
    pub const fn cc2keyr0(&self) -> &CC2KEYR0 {
        &self.cc2keyr0
    }
    ///0x280 - MCE cipher context 2 key register 1
    #[inline(always)]
    pub const fn cc2keyr1(&self) -> &CC2KEYR1 {
        &self.cc2keyr1
    }
    ///0x284 - MCE cipher context 2 key register 2
    #[inline(always)]
    pub const fn cc2keyr2(&self) -> &CC2KEYR2 {
        &self.cc2keyr2
    }
    ///0x288 - MCE cipher context 2 key register 3
    #[inline(always)]
    pub const fn cc2keyr3(&self) -> &CC2KEYR3 {
        &self.cc2keyr3
    }
}
/**CR (rw) register accessor: MCE configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MCE configuration register
pub mod cr;
/**SR (r) register accessor: MCE status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///MCE status register
pub mod sr;
/**IASR (r) register accessor: MCE illegal access status register

You can [`read`](crate::Reg::read) this register and get [`iasr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:IASR)

For information about available fields see [`mod@iasr`] module*/
pub type IASR = crate::Reg<iasr::IASRrs>;
///MCE illegal access status register
pub mod iasr;
/**IACR (w) register accessor: MCE illegal access clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iacr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:IACR)

For information about available fields see [`mod@iacr`] module*/
pub type IACR = crate::Reg<iacr::IACRrs>;
///MCE illegal access clear register
pub mod iacr;
/**IAIER (r) register accessor: MCE illegal access interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`iaier::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:IAIER)

For information about available fields see [`mod@iaier`] module*/
pub type IAIER = crate::Reg<iaier::IAIERrs>;
///MCE illegal access interrupt enable register
pub mod iaier;
/**IADDR (r) register accessor: MCE illegal address register

You can [`read`](crate::Reg::read) this register and get [`iaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:IADDR)

For information about available fields see [`mod@iaddr`] module*/
pub type IADDR = crate::Reg<iaddr::IADDRrs>;
///MCE illegal address register
pub mod iaddr;
/**REGCR1 (rw) register accessor: MCE region 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`regcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:REGCR1)

For information about available fields see [`mod@regcr1`] module*/
pub type REGCR1 = crate::Reg<regcr1::REGCR1rs>;
///MCE region 1 configuration register
pub mod regcr1;
/**SADDR1 (rw) register accessor: MCE start address for region 1 register

You can [`read`](crate::Reg::read) this register and get [`saddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:SADDR1)

For information about available fields see [`mod@saddr1`] module*/
pub type SADDR1 = crate::Reg<saddr1::SADDR1rs>;
///MCE start address for region 1 register
pub mod saddr1;
/**EADDR1 (rw) register accessor: MCE end address for region 1 register

You can [`read`](crate::Reg::read) this register and get [`eaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:EADDR1)

For information about available fields see [`mod@eaddr1`] module*/
pub type EADDR1 = crate::Reg<eaddr1::EADDR1rs>;
///MCE end address for region 1 register
pub mod eaddr1;
/**REGCR2 (rw) register accessor: MCE region 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`regcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:REGCR2)

For information about available fields see [`mod@regcr2`] module*/
pub type REGCR2 = crate::Reg<regcr2::REGCR2rs>;
///MCE region 2 configuration register
pub mod regcr2;
/**SADDR2 (rw) register accessor: MCE start address for region 2 register

You can [`read`](crate::Reg::read) this register and get [`saddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:SADDR2)

For information about available fields see [`mod@saddr2`] module*/
pub type SADDR2 = crate::Reg<saddr2::SADDR2rs>;
///MCE start address for region 2 register
pub mod saddr2;
/**EADDR2 (rw) register accessor: MCE end address for region 2 register

You can [`read`](crate::Reg::read) this register and get [`eaddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:EADDR2)

For information about available fields see [`mod@eaddr2`] module*/
pub type EADDR2 = crate::Reg<eaddr2::EADDR2rs>;
///MCE end address for region 2 register
pub mod eaddr2;
/**REGCR3 (rw) register accessor: MCE region 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`regcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:REGCR3)

For information about available fields see [`mod@regcr3`] module*/
pub type REGCR3 = crate::Reg<regcr3::REGCR3rs>;
///MCE region 3 configuration register
pub mod regcr3;
/**SADDR3 (rw) register accessor: MCE start address for region 3 register

You can [`read`](crate::Reg::read) this register and get [`saddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:SADDR3)

For information about available fields see [`mod@saddr3`] module*/
pub type SADDR3 = crate::Reg<saddr3::SADDR3rs>;
///MCE start address for region 3 register
pub mod saddr3;
/**EADDR3 (rw) register accessor: MCE end address for region 3 register

You can [`read`](crate::Reg::read) this register and get [`eaddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:EADDR3)

For information about available fields see [`mod@eaddr3`] module*/
pub type EADDR3 = crate::Reg<eaddr3::EADDR3rs>;
///MCE end address for region 3 register
pub mod eaddr3;
/**REGCR4 (rw) register accessor: MCE region 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`regcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:REGCR4)

For information about available fields see [`mod@regcr4`] module*/
pub type REGCR4 = crate::Reg<regcr4::REGCR4rs>;
///MCE region 4 configuration register
pub mod regcr4;
/**SADDR4 (rw) register accessor: MCE start address for region 4 register

You can [`read`](crate::Reg::read) this register and get [`saddr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:SADDR4)

For information about available fields see [`mod@saddr4`] module*/
pub type SADDR4 = crate::Reg<saddr4::SADDR4rs>;
///MCE start address for region 4 register
pub mod saddr4;
/**EADDR4 (rw) register accessor: MCE end address for region 4 register

You can [`read`](crate::Reg::read) this register and get [`eaddr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:EADDR4)

For information about available fields see [`mod@eaddr4`] module*/
pub type EADDR4 = crate::Reg<eaddr4::EADDR4rs>;
///MCE end address for region 4 register
pub mod eaddr4;
/**MKEYR0 (w) register accessor: .MCE master key 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR0)

For information about available fields see [`mod@mkeyr0`] module*/
pub type MKEYR0 = crate::Reg<mkeyr0::MKEYR0rs>;
///.MCE master key 0
pub mod mkeyr0;
/**MKEYR1 (w) register accessor: .MCE master key 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR1)

For information about available fields see [`mod@mkeyr1`] module*/
pub type MKEYR1 = crate::Reg<mkeyr1::MKEYR1rs>;
///.MCE master key 1
pub mod mkeyr1;
/**MKEYR2 (w) register accessor: .MCE master key 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR2)

For information about available fields see [`mod@mkeyr2`] module*/
pub type MKEYR2 = crate::Reg<mkeyr2::MKEYR2rs>;
///.MCE master key 2
pub mod mkeyr2;
/**MKEYR3 (w) register accessor: .MCE master key 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR3)

For information about available fields see [`mod@mkeyr3`] module*/
pub type MKEYR3 = crate::Reg<mkeyr3::MKEYR3rs>;
///.MCE master key 3
pub mod mkeyr3;
/**MKEYR4 (w) register accessor: .MCE master key 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR4)

For information about available fields see [`mod@mkeyr4`] module*/
pub type MKEYR4 = crate::Reg<mkeyr4::MKEYR4rs>;
///.MCE master key 4
pub mod mkeyr4;
/**MKEYR5 (w) register accessor: .MCE master key 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR5)

For information about available fields see [`mod@mkeyr5`] module*/
pub type MKEYR5 = crate::Reg<mkeyr5::MKEYR5rs>;
///.MCE master key 5
pub mod mkeyr5;
/**MKEYR6 (w) register accessor: .MCE master key 6

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR6)

For information about available fields see [`mod@mkeyr6`] module*/
pub type MKEYR6 = crate::Reg<mkeyr6::MKEYR6rs>;
///.MCE master key 6
pub mod mkeyr6;
/**MKEYR7 (w) register accessor: .MCE master key 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR7)

For information about available fields see [`mod@mkeyr7`] module*/
pub type MKEYR7 = crate::Reg<mkeyr7::MKEYR7rs>;
///.MCE master key 7
pub mod mkeyr7;
/**FMKEYR0 (w) register accessor: MCE fast master key 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR0)

For information about available fields see [`mod@fmkeyr0`] module*/
pub type FMKEYR0 = crate::Reg<fmkeyr0::FMKEYR0rs>;
///MCE fast master key 0
pub mod fmkeyr0;
/**FMKEYR1 (w) register accessor: MCE fast master key 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR1)

For information about available fields see [`mod@fmkeyr1`] module*/
pub type FMKEYR1 = crate::Reg<fmkeyr1::FMKEYR1rs>;
///MCE fast master key 1
pub mod fmkeyr1;
/**FMKEYR2 (w) register accessor: MCE fast master key 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR2)

For information about available fields see [`mod@fmkeyr2`] module*/
pub type FMKEYR2 = crate::Reg<fmkeyr2::FMKEYR2rs>;
///MCE fast master key 2
pub mod fmkeyr2;
/**FMKEYR3 (w) register accessor: MCE fast master key 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR3)

For information about available fields see [`mod@fmkeyr3`] module*/
pub type FMKEYR3 = crate::Reg<fmkeyr3::FMKEYR3rs>;
///MCE fast master key 3
pub mod fmkeyr3;
/**FMKEYR4 (w) register accessor: MCE fast master key 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR4)

For information about available fields see [`mod@fmkeyr4`] module*/
pub type FMKEYR4 = crate::Reg<fmkeyr4::FMKEYR4rs>;
///MCE fast master key 4
pub mod fmkeyr4;
/**FMKEYR5 (w) register accessor: MCE fast master key 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR5)

For information about available fields see [`mod@fmkeyr5`] module*/
pub type FMKEYR5 = crate::Reg<fmkeyr5::FMKEYR5rs>;
///MCE fast master key 5
pub mod fmkeyr5;
/**FMKEYR6 (w) register accessor: MCE fast master key 6

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR6)

For information about available fields see [`mod@fmkeyr6`] module*/
pub type FMKEYR6 = crate::Reg<fmkeyr6::FMKEYR6rs>;
///MCE fast master key 6
pub mod fmkeyr6;
/**FMKEYR7 (w) register accessor: MCE fast master key 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR7)

For information about available fields see [`mod@fmkeyr7`] module*/
pub type FMKEYR7 = crate::Reg<fmkeyr7::FMKEYR7rs>;
///MCE fast master key 7
pub mod fmkeyr7;
/**CC1CFGR (rw) register accessor: MCE cipher context 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`cc1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1CFGR)

For information about available fields see [`mod@cc1cfgr`] module*/
pub type CC1CFGR = crate::Reg<cc1cfgr::CC1CFGRrs>;
///MCE cipher context 1 configuration register
pub mod cc1cfgr;
/**CC1NR0 (rw) register accessor: MCE cipher context 1 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`cc1nr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1nr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1NR0)

For information about available fields see [`mod@cc1nr0`] module*/
pub type CC1NR0 = crate::Reg<cc1nr0::CC1NR0rs>;
///MCE cipher context 1 nonce register 0
pub mod cc1nr0;
/**CC1NR1 (rw) register accessor: MCE cipher context 1 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`cc1nr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1nr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1NR1)

For information about available fields see [`mod@cc1nr1`] module*/
pub type CC1NR1 = crate::Reg<cc1nr1::CC1NR1rs>;
///MCE cipher context 1 nonce register 1
pub mod cc1nr1;
/**CC1KEYR0 (w) register accessor: MCE cipher context 1 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1KEYR0)

For information about available fields see [`mod@cc1keyr0`] module*/
pub type CC1KEYR0 = crate::Reg<cc1keyr0::CC1KEYR0rs>;
///MCE cipher context 1 key register 0
pub mod cc1keyr0;
/**CC1KEYR1 (w) register accessor: MCE cipher context 1 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1KEYR1)

For information about available fields see [`mod@cc1keyr1`] module*/
pub type CC1KEYR1 = crate::Reg<cc1keyr1::CC1KEYR1rs>;
///MCE cipher context 1 key register 1
pub mod cc1keyr1;
/**CC1KEYR2 (w) register accessor: MCE cipher context 1 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1KEYR2)

For information about available fields see [`mod@cc1keyr2`] module*/
pub type CC1KEYR2 = crate::Reg<cc1keyr2::CC1KEYR2rs>;
///MCE cipher context 1 key register 2
pub mod cc1keyr2;
/**CC1KEYR3 (w) register accessor: MCE cipher context 1 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC1KEYR3)

For information about available fields see [`mod@cc1keyr3`] module*/
pub type CC1KEYR3 = crate::Reg<cc1keyr3::CC1KEYR3rs>;
///MCE cipher context 1 key register 3
pub mod cc1keyr3;
/**CC2CFGR (rw) register accessor: MCE cipher context 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`cc2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2CFGR)

For information about available fields see [`mod@cc2cfgr`] module*/
pub type CC2CFGR = crate::Reg<cc2cfgr::CC2CFGRrs>;
///MCE cipher context 2 configuration register
pub mod cc2cfgr;
/**CC2NR0 (rw) register accessor: MCE cipher context 2 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`cc2nr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2nr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2NR0)

For information about available fields see [`mod@cc2nr0`] module*/
pub type CC2NR0 = crate::Reg<cc2nr0::CC2NR0rs>;
///MCE cipher context 2 nonce register 0
pub mod cc2nr0;
/**CC2NR1 (rw) register accessor: MCE cipher context 2 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`cc2nr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2nr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2NR1)

For information about available fields see [`mod@cc2nr1`] module*/
pub type CC2NR1 = crate::Reg<cc2nr1::CC2NR1rs>;
///MCE cipher context 2 nonce register 1
pub mod cc2nr1;
/**CC2KEYR0 (w) register accessor: MCE cipher context 2 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2KEYR0)

For information about available fields see [`mod@cc2keyr0`] module*/
pub type CC2KEYR0 = crate::Reg<cc2keyr0::CC2KEYR0rs>;
///MCE cipher context 2 key register 0
pub mod cc2keyr0;
/**CC2KEYR1 (w) register accessor: MCE cipher context 2 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2KEYR1)

For information about available fields see [`mod@cc2keyr1`] module*/
pub type CC2KEYR1 = crate::Reg<cc2keyr1::CC2KEYR1rs>;
///MCE cipher context 2 key register 1
pub mod cc2keyr1;
/**CC2KEYR2 (w) register accessor: MCE cipher context 2 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2KEYR2)

For information about available fields see [`mod@cc2keyr2`] module*/
pub type CC2KEYR2 = crate::Reg<cc2keyr2::CC2KEYR2rs>;
///MCE cipher context 2 key register 2
pub mod cc2keyr2;
/**CC2KEYR3 (w) register accessor: MCE cipher context 2 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2KEYR3)

For information about available fields see [`mod@cc2keyr3`] module*/
pub type CC2KEYR3 = crate::Reg<cc2keyr3::CC2KEYR3rs>;
///MCE cipher context 2 key register 3
pub mod cc2keyr3;
