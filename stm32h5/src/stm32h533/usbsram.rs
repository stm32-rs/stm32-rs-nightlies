#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_txrxbd_: [u8; 0x04],
    _reserved_1_rxtxbd_: [u8; 0x04],
    _reserved_2_txrxbd_: [u8; 0x04],
    _reserved_3_rxtxbd_: [u8; 0x04],
    _reserved_4_txrxbd_: [u8; 0x04],
    _reserved_5_rxtxbd_: [u8; 0x04],
    _reserved_6_txrxbd_: [u8; 0x04],
    _reserved_7_rxtxbd_: [u8; 0x04],
    _reserved_8_txrxbd_: [u8; 0x04],
    _reserved_9_rxtxbd_: [u8; 0x04],
    _reserved_10_txrxbd_: [u8; 0x04],
    _reserved_11_rxtxbd_: [u8; 0x04],
    _reserved_12_txrxbd_: [u8; 0x04],
    _reserved_13_rxtxbd_: [u8; 0x04],
    _reserved_14_txrxbd_: [u8; 0x04],
    _reserved_15_rxtxbd_: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - Channel/endpoint receive buffer descriptor 0
    #[inline(always)]
    pub const fn txrxbd_0_alternate1(&self) -> &TXRXBD_0_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Channel/endpoint transmit buffer descriptor 0
    #[inline(always)]
    pub const fn txrxbd_0(&self) -> &TXRXBD_0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - Channel/endpoint transmit buffer descriptor 0
    #[inline(always)]
    pub const fn rxtxbd_0_alternate1(&self) -> &RXTXBD_0_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Channel/endpoint receive buffer descriptor 0
    #[inline(always)]
    pub const fn rxtxbd_0(&self) -> &RXTXBD_0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - Channel/endpoint receive buffer descriptor 1
    #[inline(always)]
    pub const fn txrxbd_1_alternate1(&self) -> &TXRXBD_1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - Channel/endpoint transmit buffer descriptor 1
    #[inline(always)]
    pub const fn txrxbd_1(&self) -> &TXRXBD_1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c - Channel/endpoint transmit buffer descriptor 1
    #[inline(always)]
    pub const fn rxtxbd_1_alternate1(&self) -> &RXTXBD_1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x0c - Channel/endpoint receive buffer descriptor 1
    #[inline(always)]
    pub const fn rxtxbd_1(&self) -> &RXTXBD_1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x10 - Channel/endpoint receive buffer descriptor 2
    #[inline(always)]
    pub const fn txrxbd_2_alternate1(&self) -> &TXRXBD_2_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - Channel/endpoint transmit buffer descriptor 2
    #[inline(always)]
    pub const fn txrxbd_2(&self) -> &TXRXBD_2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x14 - Channel/endpoint transmit buffer descriptor 2
    #[inline(always)]
    pub const fn rxtxbd_2_alternate1(&self) -> &RXTXBD_2_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - Channel/endpoint receive buffer descriptor 2
    #[inline(always)]
    pub const fn rxtxbd_2(&self) -> &RXTXBD_2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x18 - Channel/endpoint receive buffer descriptor 3
    #[inline(always)]
    pub const fn txrxbd_3_alternate1(&self) -> &TXRXBD_3_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - Channel/endpoint transmit buffer descriptor 3
    #[inline(always)]
    pub const fn txrxbd_3(&self) -> &TXRXBD_3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - Channel/endpoint transmit buffer descriptor 3
    #[inline(always)]
    pub const fn rxtxbd_3_alternate1(&self) -> &RXTXBD_3_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - Channel/endpoint receive buffer descriptor 3
    #[inline(always)]
    pub const fn rxtxbd_3(&self) -> &RXTXBD_3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - Channel/endpoint receive buffer descriptor 4
    #[inline(always)]
    pub const fn txrxbd_4_alternate1(&self) -> &TXRXBD_4_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - Channel/endpoint transmit buffer descriptor 4
    #[inline(always)]
    pub const fn txrxbd_4(&self) -> &TXRXBD_4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - Channel/endpoint transmit buffer descriptor 4
    #[inline(always)]
    pub const fn rxtxbd_4_alternate1(&self) -> &RXTXBD_4_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - Channel/endpoint receive buffer descriptor 4
    #[inline(always)]
    pub const fn rxtxbd_4(&self) -> &RXTXBD_4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x28 - Channel/endpoint receive buffer descriptor 5
    #[inline(always)]
    pub const fn txrxbd_5_alternate1(&self) -> &TXRXBD_5_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - Channel/endpoint transmit buffer descriptor 5
    #[inline(always)]
    pub const fn txrxbd_5(&self) -> &TXRXBD_5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - Channel/endpoint transmit buffer descriptor 5
    #[inline(always)]
    pub const fn rxtxbd_5_alternate1(&self) -> &RXTXBD_5_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    ///0x2c - Channel/endpoint receive buffer descriptor 5
    #[inline(always)]
    pub const fn rxtxbd_5(&self) -> &RXTXBD_5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    ///0x30 - Channel/endpoint receive buffer descriptor 6
    #[inline(always)]
    pub const fn txrxbd_6_alternate1(&self) -> &TXRXBD_6_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x30 - Channel/endpoint transmit buffer descriptor 6
    #[inline(always)]
    pub const fn txrxbd_6(&self) -> &TXRXBD_6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x34 - Channel/endpoint transmit buffer descriptor 6
    #[inline(always)]
    pub const fn rxtxbd_6_alternate1(&self) -> &RXTXBD_6_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    ///0x34 - Channel/endpoint receive buffer descriptor 6
    #[inline(always)]
    pub const fn rxtxbd_6(&self) -> &RXTXBD_6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    ///0x38 - Channel/endpoint receive buffer descriptor 7
    #[inline(always)]
    pub const fn txrxbd_7_alternate1(&self) -> &TXRXBD_7_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x38 - Channel/endpoint transmit buffer descriptor 7
    #[inline(always)]
    pub const fn txrxbd_7(&self) -> &TXRXBD_7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x3c - Channel/endpoint transmit buffer descriptor 7
    #[inline(always)]
    pub const fn rxtxbd_7_alternate1(&self) -> &RXTXBD_7_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    ///0x3c - Channel/endpoint receive buffer descriptor 7
    #[inline(always)]
    pub const fn rxtxbd_7(&self) -> &RXTXBD_7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
}
/**TXRXBD_0 (rw) register accessor: Channel/endpoint transmit buffer descriptor 0

You can [`read`](crate::Reg::read) this register and get [`txrxbd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_0)

For information about available fields see [`mod@txrxbd_0`] module*/
pub type TXRXBD_0 = crate::Reg<txrxbd_0::TXRXBD_0rs>;
///Channel/endpoint transmit buffer descriptor 0
pub mod txrxbd_0;
/**TXRXBD_0_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 0

You can [`read`](crate::Reg::read) this register and get [`txrxbd_0_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_0_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_0_ALTERNATE1)

For information about available fields see [`mod@txrxbd_0_alternate1`] module*/
pub type TXRXBD_0_ALTERNATE1 = crate::Reg<txrxbd_0_alternate1::TXRXBD_0_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 0
pub mod txrxbd_0_alternate1;
/**RXTXBD_0 (rw) register accessor: Channel/endpoint receive buffer descriptor 0

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_0)

For information about available fields see [`mod@rxtxbd_0`] module*/
pub type RXTXBD_0 = crate::Reg<rxtxbd_0::RXTXBD_0rs>;
///Channel/endpoint receive buffer descriptor 0
pub mod rxtxbd_0;
/**RXTXBD_0_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 0

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_0_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_0_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_0_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_0_alternate1`] module*/
pub type RXTXBD_0_ALTERNATE1 = crate::Reg<rxtxbd_0_alternate1::RXTXBD_0_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 0
pub mod rxtxbd_0_alternate1;
/**TXRXBD_1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 1

You can [`read`](crate::Reg::read) this register and get [`txrxbd_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_1)

For information about available fields see [`mod@txrxbd_1`] module*/
pub type TXRXBD_1 = crate::Reg<txrxbd_1::TXRXBD_1rs>;
///Channel/endpoint transmit buffer descriptor 1
pub mod txrxbd_1;
/**TXRXBD_1_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 1

You can [`read`](crate::Reg::read) this register and get [`txrxbd_1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_1_ALTERNATE1)

For information about available fields see [`mod@txrxbd_1_alternate1`] module*/
pub type TXRXBD_1_ALTERNATE1 = crate::Reg<txrxbd_1_alternate1::TXRXBD_1_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 1
pub mod txrxbd_1_alternate1;
/**RXTXBD_1 (rw) register accessor: Channel/endpoint receive buffer descriptor 1

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_1)

For information about available fields see [`mod@rxtxbd_1`] module*/
pub type RXTXBD_1 = crate::Reg<rxtxbd_1::RXTXBD_1rs>;
///Channel/endpoint receive buffer descriptor 1
pub mod rxtxbd_1;
/**RXTXBD_1_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 1

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_1_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_1_alternate1`] module*/
pub type RXTXBD_1_ALTERNATE1 = crate::Reg<rxtxbd_1_alternate1::RXTXBD_1_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 1
pub mod rxtxbd_1_alternate1;
/**TXRXBD_2 (rw) register accessor: Channel/endpoint transmit buffer descriptor 2

You can [`read`](crate::Reg::read) this register and get [`txrxbd_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_2)

For information about available fields see [`mod@txrxbd_2`] module*/
pub type TXRXBD_2 = crate::Reg<txrxbd_2::TXRXBD_2rs>;
///Channel/endpoint transmit buffer descriptor 2
pub mod txrxbd_2;
/**TXRXBD_2_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 2

You can [`read`](crate::Reg::read) this register and get [`txrxbd_2_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_2_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_2_ALTERNATE1)

For information about available fields see [`mod@txrxbd_2_alternate1`] module*/
pub type TXRXBD_2_ALTERNATE1 = crate::Reg<txrxbd_2_alternate1::TXRXBD_2_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 2
pub mod txrxbd_2_alternate1;
/**RXTXBD_2 (rw) register accessor: Channel/endpoint receive buffer descriptor 2

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_2)

For information about available fields see [`mod@rxtxbd_2`] module*/
pub type RXTXBD_2 = crate::Reg<rxtxbd_2::RXTXBD_2rs>;
///Channel/endpoint receive buffer descriptor 2
pub mod rxtxbd_2;
/**RXTXBD_2_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 2

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_2_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_2_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_2_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_2_alternate1`] module*/
pub type RXTXBD_2_ALTERNATE1 = crate::Reg<rxtxbd_2_alternate1::RXTXBD_2_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 2
pub mod rxtxbd_2_alternate1;
/**TXRXBD_3 (rw) register accessor: Channel/endpoint transmit buffer descriptor 3

You can [`read`](crate::Reg::read) this register and get [`txrxbd_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_3)

For information about available fields see [`mod@txrxbd_3`] module*/
pub type TXRXBD_3 = crate::Reg<txrxbd_3::TXRXBD_3rs>;
///Channel/endpoint transmit buffer descriptor 3
pub mod txrxbd_3;
/**TXRXBD_3_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 3

You can [`read`](crate::Reg::read) this register and get [`txrxbd_3_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_3_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_3_ALTERNATE1)

For information about available fields see [`mod@txrxbd_3_alternate1`] module*/
pub type TXRXBD_3_ALTERNATE1 = crate::Reg<txrxbd_3_alternate1::TXRXBD_3_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 3
pub mod txrxbd_3_alternate1;
/**RXTXBD_3 (rw) register accessor: Channel/endpoint receive buffer descriptor 3

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_3)

For information about available fields see [`mod@rxtxbd_3`] module*/
pub type RXTXBD_3 = crate::Reg<rxtxbd_3::RXTXBD_3rs>;
///Channel/endpoint receive buffer descriptor 3
pub mod rxtxbd_3;
/**RXTXBD_3_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 3

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_3_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_3_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_3_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_3_alternate1`] module*/
pub type RXTXBD_3_ALTERNATE1 = crate::Reg<rxtxbd_3_alternate1::RXTXBD_3_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 3
pub mod rxtxbd_3_alternate1;
/**TXRXBD_4 (rw) register accessor: Channel/endpoint transmit buffer descriptor 4

You can [`read`](crate::Reg::read) this register and get [`txrxbd_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_4)

For information about available fields see [`mod@txrxbd_4`] module*/
pub type TXRXBD_4 = crate::Reg<txrxbd_4::TXRXBD_4rs>;
///Channel/endpoint transmit buffer descriptor 4
pub mod txrxbd_4;
/**TXRXBD_4_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 4

You can [`read`](crate::Reg::read) this register and get [`txrxbd_4_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_4_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_4_ALTERNATE1)

For information about available fields see [`mod@txrxbd_4_alternate1`] module*/
pub type TXRXBD_4_ALTERNATE1 = crate::Reg<txrxbd_4_alternate1::TXRXBD_4_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 4
pub mod txrxbd_4_alternate1;
/**RXTXBD_4 (rw) register accessor: Channel/endpoint receive buffer descriptor 4

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_4)

For information about available fields see [`mod@rxtxbd_4`] module*/
pub type RXTXBD_4 = crate::Reg<rxtxbd_4::RXTXBD_4rs>;
///Channel/endpoint receive buffer descriptor 4
pub mod rxtxbd_4;
/**RXTXBD_4_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 4

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_4_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_4_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_4_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_4_alternate1`] module*/
pub type RXTXBD_4_ALTERNATE1 = crate::Reg<rxtxbd_4_alternate1::RXTXBD_4_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 4
pub mod rxtxbd_4_alternate1;
/**TXRXBD_5 (rw) register accessor: Channel/endpoint transmit buffer descriptor 5

You can [`read`](crate::Reg::read) this register and get [`txrxbd_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_5)

For information about available fields see [`mod@txrxbd_5`] module*/
pub type TXRXBD_5 = crate::Reg<txrxbd_5::TXRXBD_5rs>;
///Channel/endpoint transmit buffer descriptor 5
pub mod txrxbd_5;
/**TXRXBD_5_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 5

You can [`read`](crate::Reg::read) this register and get [`txrxbd_5_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_5_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_5_ALTERNATE1)

For information about available fields see [`mod@txrxbd_5_alternate1`] module*/
pub type TXRXBD_5_ALTERNATE1 = crate::Reg<txrxbd_5_alternate1::TXRXBD_5_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 5
pub mod txrxbd_5_alternate1;
/**RXTXBD_5 (rw) register accessor: Channel/endpoint receive buffer descriptor 5

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_5)

For information about available fields see [`mod@rxtxbd_5`] module*/
pub type RXTXBD_5 = crate::Reg<rxtxbd_5::RXTXBD_5rs>;
///Channel/endpoint receive buffer descriptor 5
pub mod rxtxbd_5;
/**RXTXBD_5_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 5

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_5_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_5_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_5_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_5_alternate1`] module*/
pub type RXTXBD_5_ALTERNATE1 = crate::Reg<rxtxbd_5_alternate1::RXTXBD_5_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 5
pub mod rxtxbd_5_alternate1;
/**TXRXBD_6 (rw) register accessor: Channel/endpoint transmit buffer descriptor 6

You can [`read`](crate::Reg::read) this register and get [`txrxbd_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_6)

For information about available fields see [`mod@txrxbd_6`] module*/
pub type TXRXBD_6 = crate::Reg<txrxbd_6::TXRXBD_6rs>;
///Channel/endpoint transmit buffer descriptor 6
pub mod txrxbd_6;
/**TXRXBD_6_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 6

You can [`read`](crate::Reg::read) this register and get [`txrxbd_6_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_6_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_6_ALTERNATE1)

For information about available fields see [`mod@txrxbd_6_alternate1`] module*/
pub type TXRXBD_6_ALTERNATE1 = crate::Reg<txrxbd_6_alternate1::TXRXBD_6_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 6
pub mod txrxbd_6_alternate1;
/**RXTXBD_6 (rw) register accessor: Channel/endpoint receive buffer descriptor 6

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_6)

For information about available fields see [`mod@rxtxbd_6`] module*/
pub type RXTXBD_6 = crate::Reg<rxtxbd_6::RXTXBD_6rs>;
///Channel/endpoint receive buffer descriptor 6
pub mod rxtxbd_6;
/**RXTXBD_6_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 6

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_6_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_6_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_6_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_6_alternate1`] module*/
pub type RXTXBD_6_ALTERNATE1 = crate::Reg<rxtxbd_6_alternate1::RXTXBD_6_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 6
pub mod rxtxbd_6_alternate1;
/**TXRXBD_7 (rw) register accessor: Channel/endpoint transmit buffer descriptor 7

You can [`read`](crate::Reg::read) this register and get [`txrxbd_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_7)

For information about available fields see [`mod@txrxbd_7`] module*/
pub type TXRXBD_7 = crate::Reg<txrxbd_7::TXRXBD_7rs>;
///Channel/endpoint transmit buffer descriptor 7
pub mod txrxbd_7;
/**TXRXBD_7_ALTERNATE1 (rw) register accessor: Channel/endpoint receive buffer descriptor 7

You can [`read`](crate::Reg::read) this register and get [`txrxbd_7_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_7_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_7_ALTERNATE1)

For information about available fields see [`mod@txrxbd_7_alternate1`] module*/
pub type TXRXBD_7_ALTERNATE1 = crate::Reg<txrxbd_7_alternate1::TXRXBD_7_ALTERNATE1rs>;
///Channel/endpoint receive buffer descriptor 7
pub mod txrxbd_7_alternate1;
/**RXTXBD_7 (rw) register accessor: Channel/endpoint receive buffer descriptor 7

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_7)

For information about available fields see [`mod@rxtxbd_7`] module*/
pub type RXTXBD_7 = crate::Reg<rxtxbd_7::RXTXBD_7rs>;
///Channel/endpoint receive buffer descriptor 7
pub mod rxtxbd_7;
/**RXTXBD_7_ALTERNATE1 (rw) register accessor: Channel/endpoint transmit buffer descriptor 7

You can [`read`](crate::Reg::read) this register and get [`rxtxbd_7_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtxbd_7_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:RXTXBD_7_ALTERNATE1)

For information about available fields see [`mod@rxtxbd_7_alternate1`] module*/
pub type RXTXBD_7_ALTERNATE1 = crate::Reg<rxtxbd_7_alternate1::RXTXBD_7_ALTERNATE1rs>;
///Channel/endpoint transmit buffer descriptor 7
pub mod rxtxbd_7_alternate1;
