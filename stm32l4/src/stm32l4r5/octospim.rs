#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    p1cr: P1CR,
    p2cr: P2CR,
}
impl RegisterBlock {
    ///0x00 - configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - OctoSPI IO Manager Port 1 Configuration Register
    #[inline(always)]
    pub const fn p1cr(&self) -> &P1CR {
        &self.p1cr
    }
    ///0x08 - OctoSPI IO Manager Port 2 Configuration Register
    #[inline(always)]
    pub const fn p2cr(&self) -> &P2CR {
        &self.p2cr
    }
}
/**CR (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#OCTOSPIM:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///configuration register
pub mod cr;
/**P1CR (rw) register accessor: OctoSPI IO Manager Port 1 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`p1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#OCTOSPIM:P1CR)

For information about available fields see [`mod@p1cr`] module*/
pub type P1CR = crate::Reg<p1cr::P1CRrs>;
///OctoSPI IO Manager Port 1 Configuration Register
pub mod p1cr;
/**P2CR (rw) register accessor: OctoSPI IO Manager Port 2 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`p2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#OCTOSPIM:P2CR)

For information about available fields see [`mod@p2cr`] module*/
pub type P2CR = crate::Reg<p2cr::P2CRrs>;
///OctoSPI IO Manager Port 2 Configuration Register
pub mod p2cr;
