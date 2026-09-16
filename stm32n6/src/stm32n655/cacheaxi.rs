#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    sr: SR,
    ier: IER,
    fcr: FCR,
    rhmonr: RHMONR,
    rmmonr: RMMONR,
    rammonr: RAMMONR,
    evimonr: EVIMONR,
    whmonr: WHMONR,
    wmmonr: WMMONR,
    wammonr: WAMMONR,
    wtmonr: WTMONR,
    _reserved12: [u8; 0xd0],
    cr2: CR2,
    cmdrsaddrr: CMDRSADDRR,
    cmdreaddrr: CMDREADDRR,
}
impl RegisterBlock {
    ///0x00 - CACHEAXI control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - CACHEAXI status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - CACHEAXI interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x0c - CACHEAXI flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - CACHEAXI read-hit monitor register
    #[inline(always)]
    pub const fn rhmonr(&self) -> &RHMONR {
        &self.rhmonr
    }
    ///0x14 - CACHEAXI read-miss monitor register
    #[inline(always)]
    pub const fn rmmonr(&self) -> &RMMONR {
        &self.rmmonr
    }
    ///0x18 - CACHEAXI read-allocate miss monitor register
    #[inline(always)]
    pub const fn rammonr(&self) -> &RAMMONR {
        &self.rammonr
    }
    ///0x1c - CACHEAXI eviction monitor register
    #[inline(always)]
    pub const fn evimonr(&self) -> &EVIMONR {
        &self.evimonr
    }
    ///0x20 - CACHEAXI write-hit monitor register
    #[inline(always)]
    pub const fn whmonr(&self) -> &WHMONR {
        &self.whmonr
    }
    ///0x24 - CACHEAXI write-miss monitor register
    #[inline(always)]
    pub const fn wmmonr(&self) -> &WMMONR {
        &self.wmmonr
    }
    ///0x28 - CACHEAXI write-allocate miss monitor register
    #[inline(always)]
    pub const fn wammonr(&self) -> &WAMMONR {
        &self.wammonr
    }
    ///0x2c - CACHEAXI write-through monitor register
    #[inline(always)]
    pub const fn wtmonr(&self) -> &WTMONR {
        &self.wtmonr
    }
    ///0x100 - CACHEAXI control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x104 - CACHEAXI command range start address register
    #[inline(always)]
    pub const fn cmdrsaddrr(&self) -> &CMDRSADDRR {
        &self.cmdrsaddrr
    }
    ///0x108 - CACHEAXI command range end address register
    #[inline(always)]
    pub const fn cmdreaddrr(&self) -> &CMDREADDRR {
        &self.cmdreaddrr
    }
}
/**CR1 (rw) register accessor: CACHEAXI control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///CACHEAXI control register 1
pub mod cr1;
/**SR (r) register accessor: CACHEAXI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///CACHEAXI status register
pub mod sr;
/**IER (rw) register accessor: CACHEAXI interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///CACHEAXI interrupt enable register
pub mod ier;
/**FCR (w) register accessor: CACHEAXI flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///CACHEAXI flag clear register
pub mod fcr;
/**RHMONR (r) register accessor: CACHEAXI read-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`rhmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:RHMONR)

For information about available fields see [`mod@rhmonr`] module*/
pub type RHMONR = crate::Reg<rhmonr::RHMONRrs>;
///CACHEAXI read-hit monitor register
pub mod rhmonr;
/**RMMONR (r) register accessor: CACHEAXI read-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`rmmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:RMMONR)

For information about available fields see [`mod@rmmonr`] module*/
pub type RMMONR = crate::Reg<rmmonr::RMMONRrs>;
///CACHEAXI read-miss monitor register
pub mod rmmonr;
/**RAMMONR (r) register accessor: CACHEAXI read-allocate miss monitor register

You can [`read`](crate::Reg::read) this register and get [`rammonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:RAMMONR)

For information about available fields see [`mod@rammonr`] module*/
pub type RAMMONR = crate::Reg<rammonr::RAMMONRrs>;
///CACHEAXI read-allocate miss monitor register
pub mod rammonr;
/**EVIMONR (r) register accessor: CACHEAXI eviction monitor register

You can [`read`](crate::Reg::read) this register and get [`evimonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:EVIMONR)

For information about available fields see [`mod@evimonr`] module*/
pub type EVIMONR = crate::Reg<evimonr::EVIMONRrs>;
///CACHEAXI eviction monitor register
pub mod evimonr;
/**WHMONR (r) register accessor: CACHEAXI write-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`whmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:WHMONR)

For information about available fields see [`mod@whmonr`] module*/
pub type WHMONR = crate::Reg<whmonr::WHMONRrs>;
///CACHEAXI write-hit monitor register
pub mod whmonr;
/**WMMONR (r) register accessor: CACHEAXI write-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`wmmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:WMMONR)

For information about available fields see [`mod@wmmonr`] module*/
pub type WMMONR = crate::Reg<wmmonr::WMMONRrs>;
///CACHEAXI write-miss monitor register
pub mod wmmonr;
/**WAMMONR (r) register accessor: CACHEAXI write-allocate miss monitor register

You can [`read`](crate::Reg::read) this register and get [`wammonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:WAMMONR)

For information about available fields see [`mod@wammonr`] module*/
pub type WAMMONR = crate::Reg<wammonr::WAMMONRrs>;
///CACHEAXI write-allocate miss monitor register
pub mod wammonr;
/**WTMONR (r) register accessor: CACHEAXI write-through monitor register

You can [`read`](crate::Reg::read) this register and get [`wtmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:WTMONR)

For information about available fields see [`mod@wtmonr`] module*/
pub type WTMONR = crate::Reg<wtmonr::WTMONRrs>;
///CACHEAXI write-through monitor register
pub mod wtmonr;
/**CR2 (rw) register accessor: CACHEAXI control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///CACHEAXI control register 2
pub mod cr2;
/**CMDRSADDRR (rw) register accessor: CACHEAXI command range start address register

You can [`read`](crate::Reg::read) this register and get [`cmdrsaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdrsaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:CMDRSADDRR)

For information about available fields see [`mod@cmdrsaddrr`] module*/
pub type CMDRSADDRR = crate::Reg<cmdrsaddrr::CMDRSADDRRrs>;
///CACHEAXI command range start address register
pub mod cmdrsaddrr;
/**CMDREADDRR (rw) register accessor: CACHEAXI command range end address register

You can [`read`](crate::Reg::read) this register and get [`cmdreaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdreaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:CMDREADDRR)

For information about available fields see [`mod@cmdreaddrr`] module*/
pub type CMDREADDRR = crate::Reg<cmdreaddrr::CMDREADDRRrs>;
///CACHEAXI command range end address register
pub mod cmdreaddrr;
