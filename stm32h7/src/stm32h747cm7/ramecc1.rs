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
    _reserved13: [u8; 0x08],
    m3cr: M3CR,
    m3sr: M3SR,
    m3far: M3FAR,
    m3fdrl: M3FDRL,
    m3fdrh: M3FDRH,
    m3fecr: M3FECR,
    _reserved19: [u8; 0x08],
    m4cr: M4CR,
    m4sr: M4SR,
    m4far: M4FAR,
    m4fdrl: M4FDRL,
    m4fdrh: M4FDRH,
    m4fecr: M4FECR,
    _reserved25: [u8; 0x08],
    m5cr: M5CR,
    m5sr: M5SR,
    m5far: M5FAR,
    m5fdrl: M5FDRL,
    m5fdrh: M5FDRH,
    m5fecr: M5FECR,
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
    ///0x60 - RAMECC monitor 3 configuration register
    #[inline(always)]
    pub const fn m3cr(&self) -> &M3CR {
        &self.m3cr
    }
    ///0x64 - RAMECC monitor 3 status register
    #[inline(always)]
    pub const fn m3sr(&self) -> &M3SR {
        &self.m3sr
    }
    ///0x68 - RAMECC monitor 3 failing address register
    #[inline(always)]
    pub const fn m3far(&self) -> &M3FAR {
        &self.m3far
    }
    ///0x6c - RAMECC monitor 3 failing data low register
    #[inline(always)]
    pub const fn m3fdrl(&self) -> &M3FDRL {
        &self.m3fdrl
    }
    ///0x70 - RAMECC monitor 3 failing data high register
    #[inline(always)]
    pub const fn m3fdrh(&self) -> &M3FDRH {
        &self.m3fdrh
    }
    ///0x74 - RAMECC monitor 3 failing error code register
    #[inline(always)]
    pub const fn m3fecr(&self) -> &M3FECR {
        &self.m3fecr
    }
    ///0x80 - RAMECC monitor 4 configuration register
    #[inline(always)]
    pub const fn m4cr(&self) -> &M4CR {
        &self.m4cr
    }
    ///0x84 - RAMECC monitor 4 status register
    #[inline(always)]
    pub const fn m4sr(&self) -> &M4SR {
        &self.m4sr
    }
    ///0x88 - RAMECC monitor 4 failing address register
    #[inline(always)]
    pub const fn m4far(&self) -> &M4FAR {
        &self.m4far
    }
    ///0x8c - RAMECC monitor 4 failing data low register
    #[inline(always)]
    pub const fn m4fdrl(&self) -> &M4FDRL {
        &self.m4fdrl
    }
    ///0x90 - RAMECC monitor 4 failing data high register
    #[inline(always)]
    pub const fn m4fdrh(&self) -> &M4FDRH {
        &self.m4fdrh
    }
    ///0x94 - RAMECC monitor 4 failing error code register
    #[inline(always)]
    pub const fn m4fecr(&self) -> &M4FECR {
        &self.m4fecr
    }
    ///0xa0 - RAMECC monitor 5 configuration register
    #[inline(always)]
    pub const fn m5cr(&self) -> &M5CR {
        &self.m5cr
    }
    ///0xa4 - RAMECC monitor 5 status register
    #[inline(always)]
    pub const fn m5sr(&self) -> &M5SR {
        &self.m5sr
    }
    ///0xa8 - RAMECC monitor 5 failing address register
    #[inline(always)]
    pub const fn m5far(&self) -> &M5FAR {
        &self.m5far
    }
    ///0xac - RAMECC monitor 5 failing data low register
    #[inline(always)]
    pub const fn m5fdrl(&self) -> &M5FDRL {
        &self.m5fdrl
    }
    ///0xb0 - RAMECC monitor 5 failing data high register
    #[inline(always)]
    pub const fn m5fdrh(&self) -> &M5FDRH {
        &self.m5fdrh
    }
    ///0xb4 - RAMECC monitor 5 failing error code register
    #[inline(always)]
    pub const fn m5fecr(&self) -> &M5FECR {
        &self.m5fecr
    }
}
/**IER (rw) register accessor: RAMECC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///RAMECC interrupt enable register
pub mod ier;
/**M1CR (rw) register accessor: RAMECC monitor 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`m1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M1CR)

For information about available fields see [`mod@m1cr`] module*/
pub type M1CR = crate::Reg<m1cr::M1CRrs>;
///RAMECC monitor 1 configuration register
pub mod m1cr;
/**M1SR (rw) register accessor: RAMECC monitor 1 status register

You can [`read`](crate::Reg::read) this register and get [`m1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M1SR)

For information about available fields see [`mod@m1sr`] module*/
pub type M1SR = crate::Reg<m1sr::M1SRrs>;
///RAMECC monitor 1 status register
pub mod m1sr;
/**M1FAR (rw) register accessor: RAMECC monitor 1 failing address register

You can [`read`](crate::Reg::read) this register and get [`m1far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M1FAR)

For information about available fields see [`mod@m1far`] module*/
pub type M1FAR = crate::Reg<m1far::M1FARrs>;
///RAMECC monitor 1 failing address register
pub mod m1far;
/**M1FDRL (rw) register accessor: RAMECC monitor 1 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m1fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M1FDRL)

For information about available fields see [`mod@m1fdrl`] module*/
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRLrs>;
///RAMECC monitor 1 failing data low register
pub mod m1fdrl;
/**M1FDRH (rw) register accessor: RAMECC monitor 1 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m1fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M1FDRH)

For information about available fields see [`mod@m1fdrh`] module*/
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRHrs>;
///RAMECC monitor 1 failing data high register
pub mod m1fdrh;
/**M1FECR (rw) register accessor: RAMECC monitor 1 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m1fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M1FECR)

For information about available fields see [`mod@m1fecr`] module*/
pub type M1FECR = crate::Reg<m1fecr::M1FECRrs>;
///RAMECC monitor 1 failing error code register
pub mod m1fecr;
/**M2CR (rw) register accessor: RAMECC monitor 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M2CR)

For information about available fields see [`mod@m2cr`] module*/
pub type M2CR = crate::Reg<m2cr::M2CRrs>;
///RAMECC monitor 2 configuration register
pub mod m2cr;
/**M2SR (rw) register accessor: RAMECC monitor 2 status register

You can [`read`](crate::Reg::read) this register and get [`m2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M2SR)

For information about available fields see [`mod@m2sr`] module*/
pub type M2SR = crate::Reg<m2sr::M2SRrs>;
///RAMECC monitor 2 status register
pub mod m2sr;
/**M2FAR (rw) register accessor: RAMECC monitor 2 failing address register

You can [`read`](crate::Reg::read) this register and get [`m2far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M2FAR)

For information about available fields see [`mod@m2far`] module*/
pub type M2FAR = crate::Reg<m2far::M2FARrs>;
///RAMECC monitor 2 failing address register
pub mod m2far;
/**M2FDRL (rw) register accessor: RAMECC monitor 2 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m2fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M2FDRL)

For information about available fields see [`mod@m2fdrl`] module*/
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRLrs>;
///RAMECC monitor 2 failing data low register
pub mod m2fdrl;
/**M2FDRH (rw) register accessor: RAMECC monitor 2 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m2fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M2FDRH)

For information about available fields see [`mod@m2fdrh`] module*/
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRHrs>;
///RAMECC monitor 2 failing data high register
pub mod m2fdrh;
/**M2FECR (rw) register accessor: RAMECC monitor 2 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m2fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M2FECR)

For information about available fields see [`mod@m2fecr`] module*/
pub type M2FECR = crate::Reg<m2fecr::M2FECRrs>;
///RAMECC monitor 2 failing error code register
pub mod m2fecr;
/**M3CR (rw) register accessor: RAMECC monitor 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`m3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M3CR)

For information about available fields see [`mod@m3cr`] module*/
pub type M3CR = crate::Reg<m3cr::M3CRrs>;
///RAMECC monitor 3 configuration register
pub mod m3cr;
/**M3SR (rw) register accessor: RAMECC monitor 3 status register

You can [`read`](crate::Reg::read) this register and get [`m3sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M3SR)

For information about available fields see [`mod@m3sr`] module*/
pub type M3SR = crate::Reg<m3sr::M3SRrs>;
///RAMECC monitor 3 status register
pub mod m3sr;
/**M3FAR (rw) register accessor: RAMECC monitor 3 failing address register

You can [`read`](crate::Reg::read) this register and get [`m3far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M3FAR)

For information about available fields see [`mod@m3far`] module*/
pub type M3FAR = crate::Reg<m3far::M3FARrs>;
///RAMECC monitor 3 failing address register
pub mod m3far;
/**M3FDRL (rw) register accessor: RAMECC monitor 3 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m3fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M3FDRL)

For information about available fields see [`mod@m3fdrl`] module*/
pub type M3FDRL = crate::Reg<m3fdrl::M3FDRLrs>;
///RAMECC monitor 3 failing data low register
pub mod m3fdrl;
/**M3FDRH (rw) register accessor: RAMECC monitor 3 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m3fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M3FDRH)

For information about available fields see [`mod@m3fdrh`] module*/
pub type M3FDRH = crate::Reg<m3fdrh::M3FDRHrs>;
///RAMECC monitor 3 failing data high register
pub mod m3fdrh;
/**M3FECR (rw) register accessor: RAMECC monitor 3 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m3fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M3FECR)

For information about available fields see [`mod@m3fecr`] module*/
pub type M3FECR = crate::Reg<m3fecr::M3FECRrs>;
///RAMECC monitor 3 failing error code register
pub mod m3fecr;
/**M4CR (rw) register accessor: RAMECC monitor 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`m4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M4CR)

For information about available fields see [`mod@m4cr`] module*/
pub type M4CR = crate::Reg<m4cr::M4CRrs>;
///RAMECC monitor 4 configuration register
pub mod m4cr;
/**M4SR (rw) register accessor: RAMECC monitor 4 status register

You can [`read`](crate::Reg::read) this register and get [`m4sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M4SR)

For information about available fields see [`mod@m4sr`] module*/
pub type M4SR = crate::Reg<m4sr::M4SRrs>;
///RAMECC monitor 4 status register
pub mod m4sr;
/**M4FAR (rw) register accessor: RAMECC monitor 4 failing address register

You can [`read`](crate::Reg::read) this register and get [`m4far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M4FAR)

For information about available fields see [`mod@m4far`] module*/
pub type M4FAR = crate::Reg<m4far::M4FARrs>;
///RAMECC monitor 4 failing address register
pub mod m4far;
/**M4FDRL (rw) register accessor: RAMECC monitor 4 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m4fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M4FDRL)

For information about available fields see [`mod@m4fdrl`] module*/
pub type M4FDRL = crate::Reg<m4fdrl::M4FDRLrs>;
///RAMECC monitor 4 failing data low register
pub mod m4fdrl;
/**M4FDRH (rw) register accessor: RAMECC monitor 4 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m4fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M4FDRH)

For information about available fields see [`mod@m4fdrh`] module*/
pub type M4FDRH = crate::Reg<m4fdrh::M4FDRHrs>;
///RAMECC monitor 4 failing data high register
pub mod m4fdrh;
/**M4FECR (rw) register accessor: RAMECC monitor 4 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m4fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M4FECR)

For information about available fields see [`mod@m4fecr`] module*/
pub type M4FECR = crate::Reg<m4fecr::M4FECRrs>;
///RAMECC monitor 4 failing error code register
pub mod m4fecr;
/**M5CR (rw) register accessor: RAMECC monitor 5 configuration register

You can [`read`](crate::Reg::read) this register and get [`m5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M5CR)

For information about available fields see [`mod@m5cr`] module*/
pub type M5CR = crate::Reg<m5cr::M5CRrs>;
///RAMECC monitor 5 configuration register
pub mod m5cr;
/**M5SR (rw) register accessor: RAMECC monitor 5 status register

You can [`read`](crate::Reg::read) this register and get [`m5sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M5SR)

For information about available fields see [`mod@m5sr`] module*/
pub type M5SR = crate::Reg<m5sr::M5SRrs>;
///RAMECC monitor 5 status register
pub mod m5sr;
/**M5FAR (rw) register accessor: RAMECC monitor 5 failing address register

You can [`read`](crate::Reg::read) this register and get [`m5far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M5FAR)

For information about available fields see [`mod@m5far`] module*/
pub type M5FAR = crate::Reg<m5far::M5FARrs>;
///RAMECC monitor 5 failing address register
pub mod m5far;
/**M5FDRL (rw) register accessor: RAMECC monitor 5 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m5fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M5FDRL)

For information about available fields see [`mod@m5fdrl`] module*/
pub type M5FDRL = crate::Reg<m5fdrl::M5FDRLrs>;
///RAMECC monitor 5 failing data low register
pub mod m5fdrl;
/**M5FDRH (rw) register accessor: RAMECC monitor 5 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m5fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M5FDRH)

For information about available fields see [`mod@m5fdrh`] module*/
pub type M5FDRH = crate::Reg<m5fdrh::M5FDRHrs>;
///RAMECC monitor 5 failing data high register
pub mod m5fdrh;
/**M5FECR (rw) register accessor: RAMECC monitor 5 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m5fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M5FECR)

For information about available fields see [`mod@m5fecr`] module*/
pub type M5FECR = crate::Reg<m5fecr::M5FECRrs>;
///RAMECC monitor 5 failing error code register
pub mod m5fecr;
