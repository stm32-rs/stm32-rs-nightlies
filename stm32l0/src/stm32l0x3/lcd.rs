#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    fcr: FCR,
    sr: SR,
    clr: CLR,
    _reserved4: [u8; 0x04],
    ram_com0: RAM_COM0,
    ram_com1: RAM_COM1,
    ram_com2: RAM_COM2,
    ram_com3: RAM_COM3,
    ram_com4: RAM_COM4,
    ram_com5: RAM_COM5,
    ram_com6: RAM_COM6,
    ram_com7: RAM_COM7,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - frame control register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x08 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - clear register
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    ///0x14..0x1c - LCD display memory
    #[inline(always)]
    pub const fn ram_com0(&self) -> &RAM_COM0 {
        &self.ram_com0
    }
    ///0x1c..0x24 - LCD display memory
    #[inline(always)]
    pub const fn ram_com1(&self) -> &RAM_COM1 {
        &self.ram_com1
    }
    ///0x24..0x2c - LCD display memory
    #[inline(always)]
    pub const fn ram_com2(&self) -> &RAM_COM2 {
        &self.ram_com2
    }
    ///0x2c..0x34 - LCD display memory
    #[inline(always)]
    pub const fn ram_com3(&self) -> &RAM_COM3 {
        &self.ram_com3
    }
    ///0x34..0x3c - LCD display memory
    #[inline(always)]
    pub const fn ram_com4(&self) -> &RAM_COM4 {
        &self.ram_com4
    }
    ///0x3c..0x44 - LCD display memory
    #[inline(always)]
    pub const fn ram_com5(&self) -> &RAM_COM5 {
        &self.ram_com5
    }
    ///0x44..0x4c - LCD display memory
    #[inline(always)]
    pub const fn ram_com6(&self) -> &RAM_COM6 {
        &self.ram_com6
    }
    ///0x4c..0x54 - LCD display memory
    #[inline(always)]
    pub const fn ram_com7(&self) -> &RAM_COM7 {
        &self.ram_com7
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#LCD:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**FCR (rw) register accessor: frame control register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#LCD:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///frame control register
pub mod fcr;
/**SR (rw) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#LCD:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**CLR (w) register accessor: clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#LCD:CLR)

For information about available fields see [`mod@clr`] module*/
pub type CLR = crate::Reg<clr::CLRrs>;
///clear register
pub mod clr;
/**RAM_COM0 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`ram_com0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#LCD:RAM_COM0)

For information about available fields see [`mod@ram_com0`] module*/
pub type RAM_COM0 = crate::Reg<ram_com0::RAM_COM0rs>;
///LCD display memory
pub mod ram_com0;
/**RAM_COM4 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`ram_com4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#LCD:RAM_COM4)

For information about available fields see [`mod@ram_com4`] module*/
pub type RAM_COM4 = crate::Reg<ram_com4::RAM_COM4rs>;
///LCD display memory
pub mod ram_com4;
pub use ram_com0 as ram_com1;
pub use ram_com0 as ram_com2;
pub use ram_com0 as ram_com3;
pub use ram_com4 as ram_com5;
pub use ram_com4 as ram_com6;
pub use ram_com4 as ram_com7;
pub use RAM_COM0 as RAM_COM1;
pub use RAM_COM0 as RAM_COM2;
pub use RAM_COM0 as RAM_COM3;
pub use RAM_COM4 as RAM_COM5;
pub use RAM_COM4 as RAM_COM6;
pub use RAM_COM4 as RAM_COM7;
