#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    _reserved2: [u8; 0x08],
    apb1lfz1: APB1LFZ1,
    apb1hfz1: APB1HFZ1,
    apb2fz1: APB2FZ1,
    apb4fz1: APB4FZ1,
    apb5fz1: APB5FZ1,
    ahb1fz1: AHB1FZ1,
    ahb5fz1: AHB5FZ1,
    _reserved9: [u8; 0xd0],
    sr: SR,
    dbg_auth_host: DBG_AUTH_HOST,
    dbg_auth_dev: DBG_AUTH_DEV,
    dbg_auth_ack: DBG_AUTH_ACK,
}
impl RegisterBlock {
    ///0x00 - DBGMCU identity code register
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    ///0x04 - DBGMCU configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - DBGMCU APB1L peripheral freeze register
    #[inline(always)]
    pub const fn apb1lfz1(&self) -> &APB1LFZ1 {
        &self.apb1lfz1
    }
    ///0x14 - DBGMCU APB1H peripheral freeze register
    #[inline(always)]
    pub const fn apb1hfz1(&self) -> &APB1HFZ1 {
        &self.apb1hfz1
    }
    ///0x18 - DBGMCU APB2 peripheral freeze register
    #[inline(always)]
    pub const fn apb2fz1(&self) -> &APB2FZ1 {
        &self.apb2fz1
    }
    ///0x1c - DBGMCU APB4 peripheral freeze register
    #[inline(always)]
    pub const fn apb4fz1(&self) -> &APB4FZ1 {
        &self.apb4fz1
    }
    ///0x20 - DBGMCU APB5 peripheral freeze register
    #[inline(always)]
    pub const fn apb5fz1(&self) -> &APB5FZ1 {
        &self.apb5fz1
    }
    ///0x24 - DBGMCU AHB1 peripheral freeze register
    #[inline(always)]
    pub const fn ahb1fz1(&self) -> &AHB1FZ1 {
        &self.ahb1fz1
    }
    ///0x28 - DBGMCU AHB5 peripheral freeze register
    #[inline(always)]
    pub const fn ahb5fz1(&self) -> &AHB5FZ1 {
        &self.ahb5fz1
    }
    ///0xfc - DBGMCU status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x100 - DBGMCU host authentication register
    #[inline(always)]
    pub const fn dbg_auth_host(&self) -> &DBG_AUTH_HOST {
        &self.dbg_auth_host
    }
    ///0x104 - DBGMCU device authentication register
    #[inline(always)]
    pub const fn dbg_auth_dev(&self) -> &DBG_AUTH_DEV {
        &self.dbg_auth_dev
    }
    ///0x108 - DBGMCU message read acknowledge authentication register
    #[inline(always)]
    pub const fn dbg_auth_ack(&self) -> &DBG_AUTH_ACK {
        &self.dbg_auth_ack
    }
}
/**IDCODE (r) register accessor: DBGMCU identity code register

You can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:IDCODE)

For information about available fields see [`mod@idcode`] module*/
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
///DBGMCU identity code register
pub mod idcode;
/**CR (rw) register accessor: DBGMCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DBGMCU configuration register
pub mod cr;
/**APB1LFZ1 (rw) register accessor: DBGMCU APB1L peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1lfz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:APB1LFZ1)

For information about available fields see [`mod@apb1lfz1`] module*/
pub type APB1LFZ1 = crate::Reg<apb1lfz1::APB1LFZ1rs>;
///DBGMCU APB1L peripheral freeze register
pub mod apb1lfz1;
/**APB1HFZ1 (rw) register accessor: DBGMCU APB1H peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1hfz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:APB1HFZ1)

For information about available fields see [`mod@apb1hfz1`] module*/
pub type APB1HFZ1 = crate::Reg<apb1hfz1::APB1HFZ1rs>;
///DBGMCU APB1H peripheral freeze register
pub mod apb1hfz1;
/**APB2FZ1 (rw) register accessor: DBGMCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:APB2FZ1)

For information about available fields see [`mod@apb2fz1`] module*/
pub type APB2FZ1 = crate::Reg<apb2fz1::APB2FZ1rs>;
///DBGMCU APB2 peripheral freeze register
pub mod apb2fz1;
/**APB4FZ1 (rw) register accessor: DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:APB4FZ1)

For information about available fields see [`mod@apb4fz1`] module*/
pub type APB4FZ1 = crate::Reg<apb4fz1::APB4FZ1rs>;
///DBGMCU APB4 peripheral freeze register
pub mod apb4fz1;
/**APB5FZ1 (rw) register accessor: DBGMCU APB5 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb5fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:APB5FZ1)

For information about available fields see [`mod@apb5fz1`] module*/
pub type APB5FZ1 = crate::Reg<apb5fz1::APB5FZ1rs>;
///DBGMCU APB5 peripheral freeze register
pub mod apb5fz1;
/**AHB1FZ1 (rw) register accessor: DBGMCU AHB1 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb1fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:AHB1FZ1)

For information about available fields see [`mod@ahb1fz1`] module*/
pub type AHB1FZ1 = crate::Reg<ahb1fz1::AHB1FZ1rs>;
///DBGMCU AHB1 peripheral freeze register
pub mod ahb1fz1;
/**AHB5FZ1 (rw) register accessor: DBGMCU AHB5 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb5fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:AHB5FZ1)

For information about available fields see [`mod@ahb5fz1`] module*/
pub type AHB5FZ1 = crate::Reg<ahb5fz1::AHB5FZ1rs>;
///DBGMCU AHB5 peripheral freeze register
pub mod ahb5fz1;
/**SR (r) register accessor: DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DBGMCU status register
pub mod sr;
/**DBG_AUTH_HOST (rw) register accessor: DBGMCU host authentication register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:DBG_AUTH_HOST)

For information about available fields see [`mod@dbg_auth_host`] module*/
pub type DBG_AUTH_HOST = crate::Reg<dbg_auth_host::DBG_AUTH_HOSTrs>;
///DBGMCU host authentication register
pub mod dbg_auth_host;
/**DBG_AUTH_DEV (rw) register accessor: DBGMCU device authentication register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_dev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_dev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:DBG_AUTH_DEV)

For information about available fields see [`mod@dbg_auth_dev`] module*/
pub type DBG_AUTH_DEV = crate::Reg<dbg_auth_dev::DBG_AUTH_DEVrs>;
///DBGMCU device authentication register
pub mod dbg_auth_dev;
/**DBG_AUTH_ACK (r) register accessor: DBGMCU message read acknowledge authentication register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_ack::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DBGMCU:DBG_AUTH_ACK)

For information about available fields see [`mod@dbg_auth_ack`] module*/
pub type DBG_AUTH_ACK = crate::Reg<dbg_auth_ack::DBG_AUTH_ACKrs>;
///DBGMCU message read acknowledge authentication register
pub mod dbg_auth_ack;
