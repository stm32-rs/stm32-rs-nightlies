#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    command: COMMAND,
    config: CONFIG,
    irqstat: IRQSTAT,
    irqmask: IRQMASK,
    irqraw: IRQRAW,
    size: SIZE,
    address: ADDRESS,
    _reserved7: [u8; 0x08],
    lfsrval: LFSRVAL,
    _reserved8: [u8; 0x0c],
    pageprot0: PAGEPROT0,
    pageprot1: PAGEPROT1,
    _reserved10: [u8; 0x04],
    data0: DATA0,
    data1: DATA1,
    data2: DATA2,
    data3: DATA3,
}
impl RegisterBlock {
    ///0x00 - COMMAND register
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
    ///0x04 - CONFIG register
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    ///0x08 - IRQSTAT register
    #[inline(always)]
    pub const fn irqstat(&self) -> &IRQSTAT {
        &self.irqstat
    }
    ///0x0c - IRQMASK register
    #[inline(always)]
    pub const fn irqmask(&self) -> &IRQMASK {
        &self.irqmask
    }
    ///0x10 - IRQRAW register
    #[inline(always)]
    pub const fn irqraw(&self) -> &IRQRAW {
        &self.irqraw
    }
    ///0x14 - SIZE register
    #[inline(always)]
    pub const fn size(&self) -> &SIZE {
        &self.size
    }
    ///0x18 - ADDRESS register
    #[inline(always)]
    pub const fn address(&self) -> &ADDRESS {
        &self.address
    }
    ///0x24 - LFSRVAL register
    #[inline(always)]
    pub const fn lfsrval(&self) -> &LFSRVAL {
        &self.lfsrval
    }
    ///0x34 - PAGEPROT0 register
    #[inline(always)]
    pub const fn pageprot0(&self) -> &PAGEPROT0 {
        &self.pageprot0
    }
    ///0x38 - PAGEPROT1 register
    #[inline(always)]
    pub const fn pageprot1(&self) -> &PAGEPROT1 {
        &self.pageprot1
    }
    ///0x40 - DATA0 register
    #[inline(always)]
    pub const fn data0(&self) -> &DATA0 {
        &self.data0
    }
    ///0x44 - DATA1 register
    #[inline(always)]
    pub const fn data1(&self) -> &DATA1 {
        &self.data1
    }
    ///0x48 - DATA2 register
    #[inline(always)]
    pub const fn data2(&self) -> &DATA2 {
        &self.data2
    }
    ///0x4c - DATA3 register
    #[inline(always)]
    pub const fn data3(&self) -> &DATA3 {
        &self.data3
    }
}
/**COMMAND (rw) register accessor: COMMAND register

You can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:COMMAND)

For information about available fields see [`mod@command`] module*/
pub type COMMAND = crate::Reg<command::COMMANDrs>;
///COMMAND register
pub mod command;
/**CONFIG (rw) register accessor: CONFIG register

You can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:CONFIG)

For information about available fields see [`mod@config`] module*/
pub type CONFIG = crate::Reg<config::CONFIGrs>;
///CONFIG register
pub mod config;
/**IRQSTAT (rw) register accessor: IRQSTAT register

You can [`read`](crate::Reg::read) this register and get [`irqstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:IRQSTAT)

For information about available fields see [`mod@irqstat`] module*/
pub type IRQSTAT = crate::Reg<irqstat::IRQSTATrs>;
///IRQSTAT register
pub mod irqstat;
/**IRQMASK (rw) register accessor: IRQMASK register

You can [`read`](crate::Reg::read) this register and get [`irqmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:IRQMASK)

For information about available fields see [`mod@irqmask`] module*/
pub type IRQMASK = crate::Reg<irqmask::IRQMASKrs>;
///IRQMASK register
pub mod irqmask;
/**IRQRAW (rw) register accessor: IRQRAW register

You can [`read`](crate::Reg::read) this register and get [`irqraw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqraw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:IRQRAW)

For information about available fields see [`mod@irqraw`] module*/
pub type IRQRAW = crate::Reg<irqraw::IRQRAWrs>;
///IRQRAW register
pub mod irqraw;
/**SIZE (r) register accessor: SIZE register

You can [`read`](crate::Reg::read) this register and get [`size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:SIZE)

For information about available fields see [`mod@size`] module*/
pub type SIZE = crate::Reg<size::SIZErs>;
///SIZE register
pub mod size;
/**ADDRESS (rw) register accessor: ADDRESS register

You can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:ADDRESS)

For information about available fields see [`mod@address`] module*/
pub type ADDRESS = crate::Reg<address::ADDRESSrs>;
///ADDRESS register
pub mod address;
/**LFSRVAL (r) register accessor: LFSRVAL register

You can [`read`](crate::Reg::read) this register and get [`lfsrval::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:LFSRVAL)

For information about available fields see [`mod@lfsrval`] module*/
pub type LFSRVAL = crate::Reg<lfsrval::LFSRVALrs>;
///LFSRVAL register
pub mod lfsrval;
/**PAGEPROT0 (rw) register accessor: PAGEPROT0 register

You can [`read`](crate::Reg::read) this register and get [`pageprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:PAGEPROT0)

For information about available fields see [`mod@pageprot0`] module*/
pub type PAGEPROT0 = crate::Reg<pageprot0::PAGEPROT0rs>;
///PAGEPROT0 register
pub mod pageprot0;
/**PAGEPROT1 (rw) register accessor: PAGEPROT1 register

You can [`read`](crate::Reg::read) this register and get [`pageprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:PAGEPROT1)

For information about available fields see [`mod@pageprot1`] module*/
pub type PAGEPROT1 = crate::Reg<pageprot1::PAGEPROT1rs>;
///PAGEPROT1 register
pub mod pageprot1;
/**DATA0 (rw) register accessor: DATA0 register

You can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:DATA0)

For information about available fields see [`mod@data0`] module*/
pub type DATA0 = crate::Reg<data0::DATA0rs>;
///DATA0 register
pub mod data0;
/**DATA1 (rw) register accessor: DATA1 register

You can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:DATA1)

For information about available fields see [`mod@data1`] module*/
pub type DATA1 = crate::Reg<data1::DATA1rs>;
///DATA1 register
pub mod data1;
/**DATA2 (rw) register accessor: DATA2 register

You can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:DATA2)

For information about available fields see [`mod@data2`] module*/
pub type DATA2 = crate::Reg<data2::DATA2rs>;
///DATA2 register
pub mod data2;
/**DATA3 (rw) register accessor: DATA3 register

You can [`read`](crate::Reg::read) this register and get [`data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:DATA3)

For information about available fields see [`mod@data3`] module*/
pub type DATA3 = crate::Reg<data3::DATA3rs>;
///DATA3 register
pub mod data3;
