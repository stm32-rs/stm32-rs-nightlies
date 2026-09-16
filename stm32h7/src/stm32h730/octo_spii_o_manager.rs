#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    p1cr: P1CR,
    p2cr: P2CR,
}
impl RegisterBlock {
    ///0x00 - OctoSPI IO Manager Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - OctoSPI IO Manager Port 1 configuration register
    #[inline(always)]
    pub const fn p1cr(&self) -> &P1CR {
        &self.p1cr
    }
    ///0x08 - OctoSPI IO Manager Port 2 configuration register
    #[inline(always)]
    pub const fn p2cr(&self) -> &P2CR {
        &self.p2cr
    }
}
/**CR (rw) register accessor: OctoSPI IO Manager Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#OctoSPII_O_Manager:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///OctoSPI IO Manager Control Register
pub mod cr;
/**P1CR (rw) register accessor: OctoSPI IO Manager Port 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#OctoSPII_O_Manager:P1CR)

For information about available fields see [`mod@p1cr`] module*/
pub type P1CR = crate::Reg<p1cr::P1CRrs>;
///OctoSPI IO Manager Port 1 configuration register
pub mod p1cr;
/**P2CR (rw) register accessor: OctoSPI IO Manager Port 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#OctoSPII_O_Manager:P2CR)

For information about available fields see [`mod@p2cr`] module*/
pub type P2CR = crate::Reg<p2cr::P2CRrs>;
///OctoSPI IO Manager Port 2 configuration register
pub mod p2cr;
