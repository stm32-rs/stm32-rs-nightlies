#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    maccr: MACCR,
    macffr: MACFFR,
    machthr: MACHTHR,
    machtlr: MACHTLR,
    macmiiar: MACMIIAR,
    macmiidr: MACMIIDR,
    macfcr: MACFCR,
    macvlantr: MACVLANTR,
    _reserved8: [u8; 0x08],
    macrwuffr: MACRWUFFR,
    macpmtcsr: MACPMTCSR,
    _reserved10: [u8; 0x08],
    macsr: MACSR,
    macimr: MACIMR,
    maca0hr: MACA0HR,
    maca0lr: MACA0LR,
    maca1hr: MACA1HR,
    maca1lr: MACA1LR,
    maca2hr: MACA2HR,
    maca2lr: MACA2LR,
    maca3hr: MACA3HR,
    maca3lr: MACA3LR,
}
impl RegisterBlock {
    ///0x00 - Ethernet MAC configuration register (ETH_MACCR)
    #[inline(always)]
    pub const fn maccr(&self) -> &MACCR {
        &self.maccr
    }
    ///0x04 - Ethernet MAC frame filter register (ETH_MACCFFR)
    #[inline(always)]
    pub const fn macffr(&self) -> &MACFFR {
        &self.macffr
    }
    ///0x08 - Ethernet MAC hash table high register
    #[inline(always)]
    pub const fn machthr(&self) -> &MACHTHR {
        &self.machthr
    }
    ///0x0c - Ethernet MAC hash table low register
    #[inline(always)]
    pub const fn machtlr(&self) -> &MACHTLR {
        &self.machtlr
    }
    ///0x10 - Ethernet MAC MII address register (ETH_MACMIIAR)
    #[inline(always)]
    pub const fn macmiiar(&self) -> &MACMIIAR {
        &self.macmiiar
    }
    ///0x14 - Ethernet MAC MII data register (ETH_MACMIIDR)
    #[inline(always)]
    pub const fn macmiidr(&self) -> &MACMIIDR {
        &self.macmiidr
    }
    ///0x18 - Ethernet MAC flow control register (ETH_MACFCR)
    #[inline(always)]
    pub const fn macfcr(&self) -> &MACFCR {
        &self.macfcr
    }
    ///0x1c - Ethernet MAC VLAN tag register (ETH_MACVLANTR)
    #[inline(always)]
    pub const fn macvlantr(&self) -> &MACVLANTR {
        &self.macvlantr
    }
    ///0x28 - Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)
    #[inline(always)]
    pub const fn macrwuffr(&self) -> &MACRWUFFR {
        &self.macrwuffr
    }
    ///0x2c - Ethernet MAC PMT control and status register (ETH_MACPMTCSR)
    #[inline(always)]
    pub const fn macpmtcsr(&self) -> &MACPMTCSR {
        &self.macpmtcsr
    }
    ///0x38 - Ethernet MAC interrupt status register (ETH_MACSR)
    #[inline(always)]
    pub const fn macsr(&self) -> &MACSR {
        &self.macsr
    }
    ///0x3c - Ethernet MAC interrupt mask register (ETH_MACIMR)
    #[inline(always)]
    pub const fn macimr(&self) -> &MACIMR {
        &self.macimr
    }
    ///0x40 - Ethernet MAC address 0 high register (ETH_MACA0HR)
    #[inline(always)]
    pub const fn maca0hr(&self) -> &MACA0HR {
        &self.maca0hr
    }
    ///0x44 - Ethernet MAC address 0 low register
    #[inline(always)]
    pub const fn maca0lr(&self) -> &MACA0LR {
        &self.maca0lr
    }
    ///0x48 - Ethernet MAC address 1 high register (ETH_MACA1HR)
    #[inline(always)]
    pub const fn maca1hr(&self) -> &MACA1HR {
        &self.maca1hr
    }
    ///0x4c - Ethernet MAC address1 low register
    #[inline(always)]
    pub const fn maca1lr(&self) -> &MACA1LR {
        &self.maca1lr
    }
    ///0x50 - Ethernet MAC address 2 high register (ETH_MACA2HR)
    #[inline(always)]
    pub const fn maca2hr(&self) -> &MACA2HR {
        &self.maca2hr
    }
    ///0x54 - Ethernet MAC address 2 low register
    #[inline(always)]
    pub const fn maca2lr(&self) -> &MACA2LR {
        &self.maca2lr
    }
    ///0x58 - Ethernet MAC address 3 high register (ETH_MACA3HR)
    #[inline(always)]
    pub const fn maca3hr(&self) -> &MACA3HR {
        &self.maca3hr
    }
    ///0x5c - Ethernet MAC address 3 low register
    #[inline(always)]
    pub const fn maca3lr(&self) -> &MACA3LR {
        &self.maca3lr
    }
}
/**MACCR (rw) register accessor: Ethernet MAC configuration register (ETH_MACCR)

You can [`read`](crate::Reg::read) this register and get [`maccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACCR)

For information about available fields see [`mod@maccr`] module*/
pub type MACCR = crate::Reg<maccr::MACCRrs>;
///Ethernet MAC configuration register (ETH_MACCR)
pub mod maccr;
/**MACFFR (rw) register accessor: Ethernet MAC frame filter register (ETH_MACCFFR)

You can [`read`](crate::Reg::read) this register and get [`macffr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macffr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACFFR)

For information about available fields see [`mod@macffr`] module*/
pub type MACFFR = crate::Reg<macffr::MACFFRrs>;
///Ethernet MAC frame filter register (ETH_MACCFFR)
pub mod macffr;
/**MACHTHR (rw) register accessor: Ethernet MAC hash table high register

You can [`read`](crate::Reg::read) this register and get [`machthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACHTHR)

For information about available fields see [`mod@machthr`] module*/
pub type MACHTHR = crate::Reg<machthr::MACHTHRrs>;
///Ethernet MAC hash table high register
pub mod machthr;
/**MACHTLR (rw) register accessor: Ethernet MAC hash table low register

You can [`read`](crate::Reg::read) this register and get [`machtlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machtlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACHTLR)

For information about available fields see [`mod@machtlr`] module*/
pub type MACHTLR = crate::Reg<machtlr::MACHTLRrs>;
///Ethernet MAC hash table low register
pub mod machtlr;
/**MACMIIAR (rw) register accessor: Ethernet MAC MII address register (ETH_MACMIIAR)

You can [`read`](crate::Reg::read) this register and get [`macmiiar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiiar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACMIIAR)

For information about available fields see [`mod@macmiiar`] module*/
pub type MACMIIAR = crate::Reg<macmiiar::MACMIIARrs>;
///Ethernet MAC MII address register (ETH_MACMIIAR)
pub mod macmiiar;
/**MACMIIDR (rw) register accessor: Ethernet MAC MII data register (ETH_MACMIIDR)

You can [`read`](crate::Reg::read) this register and get [`macmiidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACMIIDR)

For information about available fields see [`mod@macmiidr`] module*/
pub type MACMIIDR = crate::Reg<macmiidr::MACMIIDRrs>;
///Ethernet MAC MII data register (ETH_MACMIIDR)
pub mod macmiidr;
/**MACFCR (rw) register accessor: Ethernet MAC flow control register (ETH_MACFCR)

You can [`read`](crate::Reg::read) this register and get [`macfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACFCR)

For information about available fields see [`mod@macfcr`] module*/
pub type MACFCR = crate::Reg<macfcr::MACFCRrs>;
///Ethernet MAC flow control register (ETH_MACFCR)
pub mod macfcr;
/**MACVLANTR (rw) register accessor: Ethernet MAC VLAN tag register (ETH_MACVLANTR)

You can [`read`](crate::Reg::read) this register and get [`macvlantr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvlantr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACVLANTR)

For information about available fields see [`mod@macvlantr`] module*/
pub type MACVLANTR = crate::Reg<macvlantr::MACVLANTRrs>;
///Ethernet MAC VLAN tag register (ETH_MACVLANTR)
pub mod macvlantr;
/**MACRWUFFR (rw) register accessor: Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)

You can [`read`](crate::Reg::read) this register and get [`macrwuffr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwuffr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACRWUFFR)

For information about available fields see [`mod@macrwuffr`] module*/
pub type MACRWUFFR = crate::Reg<macrwuffr::MACRWUFFRrs>;
///Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)
pub mod macrwuffr;
/**MACPMTCSR (rw) register accessor: Ethernet MAC PMT control and status register (ETH_MACPMTCSR)

You can [`read`](crate::Reg::read) this register and get [`macpmtcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpmtcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACPMTCSR)

For information about available fields see [`mod@macpmtcsr`] module*/
pub type MACPMTCSR = crate::Reg<macpmtcsr::MACPMTCSRrs>;
///Ethernet MAC PMT control and status register (ETH_MACPMTCSR)
pub mod macpmtcsr;
/**MACSR (rw) register accessor: Ethernet MAC interrupt status register (ETH_MACSR)

You can [`read`](crate::Reg::read) this register and get [`macsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACSR)

For information about available fields see [`mod@macsr`] module*/
pub type MACSR = crate::Reg<macsr::MACSRrs>;
///Ethernet MAC interrupt status register (ETH_MACSR)
pub mod macsr;
/**MACIMR (rw) register accessor: Ethernet MAC interrupt mask register (ETH_MACIMR)

You can [`read`](crate::Reg::read) this register and get [`macimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACIMR)

For information about available fields see [`mod@macimr`] module*/
pub type MACIMR = crate::Reg<macimr::MACIMRrs>;
///Ethernet MAC interrupt mask register (ETH_MACIMR)
pub mod macimr;
/**MACA0HR (rw) register accessor: Ethernet MAC address 0 high register (ETH_MACA0HR)

You can [`read`](crate::Reg::read) this register and get [`maca0hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA0HR)

For information about available fields see [`mod@maca0hr`] module*/
pub type MACA0HR = crate::Reg<maca0hr::MACA0HRrs>;
///Ethernet MAC address 0 high register (ETH_MACA0HR)
pub mod maca0hr;
/**MACA0LR (rw) register accessor: Ethernet MAC address 0 low register

You can [`read`](crate::Reg::read) this register and get [`maca0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA0LR)

For information about available fields see [`mod@maca0lr`] module*/
pub type MACA0LR = crate::Reg<maca0lr::MACA0LRrs>;
///Ethernet MAC address 0 low register
pub mod maca0lr;
/**MACA1HR (rw) register accessor: Ethernet MAC address 1 high register (ETH_MACA1HR)

You can [`read`](crate::Reg::read) this register and get [`maca1hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA1HR)

For information about available fields see [`mod@maca1hr`] module*/
pub type MACA1HR = crate::Reg<maca1hr::MACA1HRrs>;
///Ethernet MAC address 1 high register (ETH_MACA1HR)
pub mod maca1hr;
/**MACA1LR (rw) register accessor: Ethernet MAC address1 low register

You can [`read`](crate::Reg::read) this register and get [`maca1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA1LR)

For information about available fields see [`mod@maca1lr`] module*/
pub type MACA1LR = crate::Reg<maca1lr::MACA1LRrs>;
///Ethernet MAC address1 low register
pub mod maca1lr;
/**MACA2HR (rw) register accessor: Ethernet MAC address 2 high register (ETH_MACA2HR)

You can [`read`](crate::Reg::read) this register and get [`maca2hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA2HR)

For information about available fields see [`mod@maca2hr`] module*/
pub type MACA2HR = crate::Reg<maca2hr::MACA2HRrs>;
///Ethernet MAC address 2 high register (ETH_MACA2HR)
pub mod maca2hr;
/**MACA2LR (rw) register accessor: Ethernet MAC address 2 low register

You can [`read`](crate::Reg::read) this register and get [`maca2lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA2LR)

For information about available fields see [`mod@maca2lr`] module*/
pub type MACA2LR = crate::Reg<maca2lr::MACA2LRrs>;
///Ethernet MAC address 2 low register
pub mod maca2lr;
/**MACA3HR (rw) register accessor: Ethernet MAC address 3 high register (ETH_MACA3HR)

You can [`read`](crate::Reg::read) this register and get [`maca3hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA3HR)

For information about available fields see [`mod@maca3hr`] module*/
pub type MACA3HR = crate::Reg<maca3hr::MACA3HRrs>;
///Ethernet MAC address 3 high register (ETH_MACA3HR)
pub mod maca3hr;
/**MACA3LR (rw) register accessor: Ethernet MAC address 3 low register

You can [`read`](crate::Reg::read) this register and get [`maca3lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA3LR)

For information about available fields see [`mod@maca3lr`] module*/
pub type MACA3LR = crate::Reg<maca3lr::MACA3LRrs>;
///Ethernet MAC address 3 low register
pub mod maca3lr;
