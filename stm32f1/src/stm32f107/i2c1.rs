#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    _reserved2: [u8; 0x02],
    oar1: OAR1,
    _reserved3: [u8; 0x02],
    oar2: OAR2,
    _reserved4: [u8; 0x02],
    dr: DR,
    _reserved5: [u8; 0x02],
    sr1: SR1,
    _reserved6: [u8; 0x02],
    sr2: SR2,
    _reserved7: [u8; 0x02],
    ccr: CCR,
    _reserved8: [u8; 0x02],
    trise: TRISE,
}
impl RegisterBlock {
    ///0x00 - Control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - Control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - Own address register 1
    #[inline(always)]
    pub const fn oar1(&self) -> &OAR1 {
        &self.oar1
    }
    ///0x0c - Own address register 2
    #[inline(always)]
    pub const fn oar2(&self) -> &OAR2 {
        &self.oar2
    }
    ///0x10 - Data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x14 - Status register 1
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    ///0x18 - Status register 2
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    ///0x1c - Clock control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x20 - TRISE register
    #[inline(always)]
    pub const fn trise(&self) -> &TRISE {
        &self.trise
    }
}
/**CR1 (rw) register accessor: Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///Control register 1
pub mod cr1;
/**CR2 (rw) register accessor: Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///Control register 2
pub mod cr2;
/**OAR1 (rw) register accessor: Own address register 1

You can [`read`](crate::Reg::read) this register and get [`oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:OAR1)

For information about available fields see [`mod@oar1`] module*/
pub type OAR1 = crate::Reg<oar1::OAR1rs>;
///Own address register 1
pub mod oar1;
/**OAR2 (rw) register accessor: Own address register 2

You can [`read`](crate::Reg::read) this register and get [`oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:OAR2)

For information about available fields see [`mod@oar2`] module*/
pub type OAR2 = crate::Reg<oar2::OAR2rs>;
///Own address register 2
pub mod oar2;
/**DR (rw) register accessor: Data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///Data register
pub mod dr;
/**SR1 (rw) register accessor: Status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:SR1)

For information about available fields see [`mod@sr1`] module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///Status register 1
pub mod sr1;
/**SR2 (r) register accessor: Status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:SR2)

For information about available fields see [`mod@sr2`] module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///Status register 2
pub mod sr2;
/**CCR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///Clock control register
pub mod ccr;
/**TRISE (rw) register accessor: TRISE register

You can [`read`](crate::Reg::read) this register and get [`trise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#I2C1:TRISE)

For information about available fields see [`mod@trise`] module*/
pub type TRISE = crate::Reg<trise::TRISErs>;
///TRISE register
pub mod trise;
