#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ier: IER,
    _reserved1: [u8; 0x1c],
    m1cr: M1CR,
    m1sr: M1SR,
    m1far: M1FAR,
    m1fdrl: M1FDRL,
    m1fdrh: M1FDRH,
    m1fecr: M1FECR,
    _reserved7: [u8; 0x08],
    m2cr: M2CR,
    m2sr: M2SR,
    m2far: M2FAR,
    m2fdrl: M2FDRL,
    m2fdrh: M2FDRH,
    m2fecr: M2FECR,
}
impl RegisterBlock {
    ///0x00 - RAMECC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x20 - RAMECC monitor 1 configuration register
    #[inline(always)]
    pub const fn m1cr(&self) -> &M1CR {
        &self.m1cr
    }
    ///0x24 - RAMECC monitor 1 status register
    #[inline(always)]
    pub const fn m1sr(&self) -> &M1SR {
        &self.m1sr
    }
    ///0x28 - RAMECC monitor 1 failing address register
    #[inline(always)]
    pub const fn m1far(&self) -> &M1FAR {
        &self.m1far
    }
    ///0x2c - RAMECC monitor 1 failing data low register
    #[inline(always)]
    pub const fn m1fdrl(&self) -> &M1FDRL {
        &self.m1fdrl
    }
    ///0x30 - RAMECC monitor 1 failing data high register
    #[inline(always)]
    pub const fn m1fdrh(&self) -> &M1FDRH {
        &self.m1fdrh
    }
    ///0x34 - RAMECC monitor 1 failing error code register
    #[inline(always)]
    pub const fn m1fecr(&self) -> &M1FECR {
        &self.m1fecr
    }
    ///0x40 - RAMECC monitor 2 configuration register
    #[inline(always)]
    pub const fn m2cr(&self) -> &M2CR {
        &self.m2cr
    }
    ///0x44 - RAMECC monitor 2 status register
    #[inline(always)]
    pub const fn m2sr(&self) -> &M2SR {
        &self.m2sr
    }
    ///0x48 - RAMECC monitor 2 failing address register
    #[inline(always)]
    pub const fn m2far(&self) -> &M2FAR {
        &self.m2far
    }
    ///0x4c - RAMECC monitor 2 failing data low register
    #[inline(always)]
    pub const fn m2fdrl(&self) -> &M2FDRL {
        &self.m2fdrl
    }
    ///0x50 - RAMECC monitor 2 failing data high register
    #[inline(always)]
    pub const fn m2fdrh(&self) -> &M2FDRH {
        &self.m2fdrh
    }
    ///0x54 - RAMECC monitor 2 failing error code register
    #[inline(always)]
    pub const fn m2fecr(&self) -> &M2FECR {
        &self.m2fecr
    }
}
/**IER (rw) register accessor: RAMECC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///RAMECC interrupt enable register
pub mod ier;
/**M1CR (rw) register accessor: RAMECC monitor 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`m1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M1CR)

For information about available fields see [`mod@m1cr`] module*/
pub type M1CR = crate::Reg<m1cr::M1CRrs>;
///RAMECC monitor 1 configuration register
pub mod m1cr;
/**M1SR (rw) register accessor: RAMECC monitor 1 status register

You can [`read`](crate::Reg::read) this register and get [`m1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M1SR)

For information about available fields see [`mod@m1sr`] module*/
pub type M1SR = crate::Reg<m1sr::M1SRrs>;
///RAMECC monitor 1 status register
pub mod m1sr;
/**M1FAR (rw) register accessor: RAMECC monitor 1 failing address register

You can [`read`](crate::Reg::read) this register and get [`m1far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M1FAR)

For information about available fields see [`mod@m1far`] module*/
pub type M1FAR = crate::Reg<m1far::M1FARrs>;
///RAMECC monitor 1 failing address register
pub mod m1far;
/**M1FDRL (rw) register accessor: RAMECC monitor 1 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m1fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M1FDRL)

For information about available fields see [`mod@m1fdrl`] module*/
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRLrs>;
///RAMECC monitor 1 failing data low register
pub mod m1fdrl;
/**M1FDRH (rw) register accessor: RAMECC monitor 1 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m1fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M1FDRH)

For information about available fields see [`mod@m1fdrh`] module*/
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRHrs>;
///RAMECC monitor 1 failing data high register
pub mod m1fdrh;
/**M1FECR (rw) register accessor: RAMECC monitor 1 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m1fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M1FECR)

For information about available fields see [`mod@m1fecr`] module*/
pub type M1FECR = crate::Reg<m1fecr::M1FECRrs>;
///RAMECC monitor 1 failing error code register
pub mod m1fecr;
/**M2CR (rw) register accessor: RAMECC monitor 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M2CR)

For information about available fields see [`mod@m2cr`] module*/
pub type M2CR = crate::Reg<m2cr::M2CRrs>;
///RAMECC monitor 2 configuration register
pub mod m2cr;
/**M2SR (rw) register accessor: RAMECC monitor 2 status register

You can [`read`](crate::Reg::read) this register and get [`m2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M2SR)

For information about available fields see [`mod@m2sr`] module*/
pub type M2SR = crate::Reg<m2sr::M2SRrs>;
///RAMECC monitor 2 status register
pub mod m2sr;
/**M2FAR (rw) register accessor: RAMECC monitor 2 failing address register

You can [`read`](crate::Reg::read) this register and get [`m2far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M2FAR)

For information about available fields see [`mod@m2far`] module*/
pub type M2FAR = crate::Reg<m2far::M2FARrs>;
///RAMECC monitor 2 failing address register
pub mod m2far;
/**M2FDRL (rw) register accessor: RAMECC monitor 2 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m2fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M2FDRL)

For information about available fields see [`mod@m2fdrl`] module*/
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRLrs>;
///RAMECC monitor 2 failing data low register
pub mod m2fdrl;
/**M2FDRH (rw) register accessor: RAMECC monitor 2 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m2fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M2FDRH)

For information about available fields see [`mod@m2fdrh`] module*/
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRHrs>;
///RAMECC monitor 2 failing data high register
pub mod m2fdrh;
/**M2FECR (rw) register accessor: RAMECC monitor 2 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m2fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M2FECR)

For information about available fields see [`mod@m2fecr`] module*/
pub type M2FECR = crate::Reg<m2fecr::M2FECRrs>;
///RAMECC monitor 2 failing error code register
pub mod m2fecr;
