#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mmccr: MMCCR,
    mmcrir: MMCRIR,
    mmctir: MMCTIR,
    mmcrimr: MMCRIMR,
    mmctimr: MMCTIMR,
    _reserved5: [u8; 0x38],
    mmctgfsccr: MMCTGFSCCR,
    mmctgfmsccr: MMCTGFMSCCR,
    _reserved7: [u8; 0x14],
    mmctgfcr: MMCTGFCR,
    _reserved8: [u8; 0x28],
    mmcrfcecr: MMCRFCECR,
    mmcrfaecr: MMCRFAECR,
    _reserved10: [u8; 0x28],
    mmcrgufcr: MMCRGUFCR,
}
impl RegisterBlock {
    ///0x00 - Ethernet MMC control register
    #[inline(always)]
    pub const fn mmccr(&self) -> &MMCCR {
        &self.mmccr
    }
    ///0x04 - Ethernet MMC receive interrupt register
    #[inline(always)]
    pub const fn mmcrir(&self) -> &MMCRIR {
        &self.mmcrir
    }
    ///0x08 - Ethernet MMC transmit interrupt register
    #[inline(always)]
    pub const fn mmctir(&self) -> &MMCTIR {
        &self.mmctir
    }
    ///0x0c - Ethernet MMC receive interrupt mask register
    #[inline(always)]
    pub const fn mmcrimr(&self) -> &MMCRIMR {
        &self.mmcrimr
    }
    ///0x10 - Ethernet MMC transmit interrupt mask register
    #[inline(always)]
    pub const fn mmctimr(&self) -> &MMCTIMR {
        &self.mmctimr
    }
    ///0x4c - Ethernet MMC transmitted good frames after a single collision counter
    #[inline(always)]
    pub const fn mmctgfsccr(&self) -> &MMCTGFSCCR {
        &self.mmctgfsccr
    }
    ///0x50 - Ethernet MMC transmitted good frames after more than a single collision
    #[inline(always)]
    pub const fn mmctgfmsccr(&self) -> &MMCTGFMSCCR {
        &self.mmctgfmsccr
    }
    ///0x68 - Ethernet MMC transmitted good frames counter register
    #[inline(always)]
    pub const fn mmctgfcr(&self) -> &MMCTGFCR {
        &self.mmctgfcr
    }
    ///0x94 - Ethernet MMC received frames with CRC error counter register
    #[inline(always)]
    pub const fn mmcrfcecr(&self) -> &MMCRFCECR {
        &self.mmcrfcecr
    }
    ///0x98 - Ethernet MMC received frames with alignment error counter register
    #[inline(always)]
    pub const fn mmcrfaecr(&self) -> &MMCRFAECR {
        &self.mmcrfaecr
    }
    ///0xc4 - MMC received good unicast frames counter register
    #[inline(always)]
    pub const fn mmcrgufcr(&self) -> &MMCRGUFCR {
        &self.mmcrgufcr
    }
}
/**MMCCR (rw) register accessor: Ethernet MMC control register

You can [`read`](crate::Reg::read) this register and get [`mmccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCCR)

For information about available fields see [`mod@mmccr`] module*/
pub type MMCCR = crate::Reg<mmccr::MMCCRrs>;
///Ethernet MMC control register
pub mod mmccr;
/**MMCRIR (rw) register accessor: Ethernet MMC receive interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmcrir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCRIR)

For information about available fields see [`mod@mmcrir`] module*/
pub type MMCRIR = crate::Reg<mmcrir::MMCRIRrs>;
///Ethernet MMC receive interrupt register
pub mod mmcrir;
/**MMCTIR (r) register accessor: Ethernet MMC transmit interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmctir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCTIR)

For information about available fields see [`mod@mmctir`] module*/
pub type MMCTIR = crate::Reg<mmctir::MMCTIRrs>;
///Ethernet MMC transmit interrupt register
pub mod mmctir;
/**MMCRIMR (rw) register accessor: Ethernet MMC receive interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmcrimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCRIMR)

For information about available fields see [`mod@mmcrimr`] module*/
pub type MMCRIMR = crate::Reg<mmcrimr::MMCRIMRrs>;
///Ethernet MMC receive interrupt mask register
pub mod mmcrimr;
/**MMCTIMR (rw) register accessor: Ethernet MMC transmit interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmctimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCTIMR)

For information about available fields see [`mod@mmctimr`] module*/
pub type MMCTIMR = crate::Reg<mmctimr::MMCTIMRrs>;
///Ethernet MMC transmit interrupt mask register
pub mod mmctimr;
/**MMCTGFSCCR (r) register accessor: Ethernet MMC transmitted good frames after a single collision counter

You can [`read`](crate::Reg::read) this register and get [`mmctgfsccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCTGFSCCR)

For information about available fields see [`mod@mmctgfsccr`] module*/
pub type MMCTGFSCCR = crate::Reg<mmctgfsccr::MMCTGFSCCRrs>;
///Ethernet MMC transmitted good frames after a single collision counter
pub mod mmctgfsccr;
/**MMCTGFMSCCR (r) register accessor: Ethernet MMC transmitted good frames after more than a single collision

You can [`read`](crate::Reg::read) this register and get [`mmctgfmsccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCTGFMSCCR)

For information about available fields see [`mod@mmctgfmsccr`] module*/
pub type MMCTGFMSCCR = crate::Reg<mmctgfmsccr::MMCTGFMSCCRrs>;
///Ethernet MMC transmitted good frames after more than a single collision
pub mod mmctgfmsccr;
/**MMCTGFCR (r) register accessor: Ethernet MMC transmitted good frames counter register

You can [`read`](crate::Reg::read) this register and get [`mmctgfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCTGFCR)

For information about available fields see [`mod@mmctgfcr`] module*/
pub type MMCTGFCR = crate::Reg<mmctgfcr::MMCTGFCRrs>;
///Ethernet MMC transmitted good frames counter register
pub mod mmctgfcr;
/**MMCRFCECR (r) register accessor: Ethernet MMC received frames with CRC error counter register

You can [`read`](crate::Reg::read) this register and get [`mmcrfcecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCRFCECR)

For information about available fields see [`mod@mmcrfcecr`] module*/
pub type MMCRFCECR = crate::Reg<mmcrfcecr::MMCRFCECRrs>;
///Ethernet MMC received frames with CRC error counter register
pub mod mmcrfcecr;
/**MMCRFAECR (r) register accessor: Ethernet MMC received frames with alignment error counter register

You can [`read`](crate::Reg::read) this register and get [`mmcrfaecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCRFAECR)

For information about available fields see [`mod@mmcrfaecr`] module*/
pub type MMCRFAECR = crate::Reg<mmcrfaecr::MMCRFAECRrs>;
///Ethernet MMC received frames with alignment error counter register
pub mod mmcrfaecr;
/**MMCRGUFCR (r) register accessor: MMC received good unicast frames counter register

You can [`read`](crate::Reg::read) this register and get [`mmcrgufcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_MMC:MMCRGUFCR)

For information about available fields see [`mod@mmcrgufcr`] module*/
pub type MMCRGUFCR = crate::Reg<mmcrgufcr::MMCRGUFCRrs>;
///MMC received good unicast frames counter register
pub mod mmcrgufcr;
