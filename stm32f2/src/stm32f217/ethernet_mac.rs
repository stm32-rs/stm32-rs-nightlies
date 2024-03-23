#[repr(C)]
#[doc = "Register block"]
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
    _reserved10: [u8; 0x04],
    macdbgr: MACDBGR,
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
    #[doc = "0x00 - Ethernet MAC configuration register"]
    #[inline(always)]
    pub const fn maccr(&self) -> &MACCR {
        &self.maccr
    }
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    #[inline(always)]
    pub const fn macffr(&self) -> &MACFFR {
        &self.macffr
    }
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    #[inline(always)]
    pub const fn machthr(&self) -> &MACHTHR {
        &self.machthr
    }
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    #[inline(always)]
    pub const fn machtlr(&self) -> &MACHTLR {
        &self.machtlr
    }
    #[doc = "0x10 - Ethernet MAC MII address register"]
    #[inline(always)]
    pub const fn macmiiar(&self) -> &MACMIIAR {
        &self.macmiiar
    }
    #[doc = "0x14 - Ethernet MAC MII data register"]
    #[inline(always)]
    pub const fn macmiidr(&self) -> &MACMIIDR {
        &self.macmiidr
    }
    #[doc = "0x18 - Ethernet MAC flow control register"]
    #[inline(always)]
    pub const fn macfcr(&self) -> &MACFCR {
        &self.macfcr
    }
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    #[inline(always)]
    pub const fn macvlantr(&self) -> &MACVLANTR {
        &self.macvlantr
    }
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register"]
    #[inline(always)]
    pub const fn macrwuffr(&self) -> &MACRWUFFR {
        &self.macrwuffr
    }
    #[doc = "0x2c - Ethernet MAC PMT control and status register"]
    #[inline(always)]
    pub const fn macpmtcsr(&self) -> &MACPMTCSR {
        &self.macpmtcsr
    }
    #[doc = "0x34 - Ethernet MAC debug register"]
    #[inline(always)]
    pub const fn macdbgr(&self) -> &MACDBGR {
        &self.macdbgr
    }
    #[doc = "0x38 - Ethernet MAC interrupt status register"]
    #[inline(always)]
    pub const fn macsr(&self) -> &MACSR {
        &self.macsr
    }
    #[doc = "0x3c - Ethernet MAC interrupt mask register"]
    #[inline(always)]
    pub const fn macimr(&self) -> &MACIMR {
        &self.macimr
    }
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    #[inline(always)]
    pub const fn maca0hr(&self) -> &MACA0HR {
        &self.maca0hr
    }
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    #[inline(always)]
    pub const fn maca0lr(&self) -> &MACA0LR {
        &self.maca0lr
    }
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    #[inline(always)]
    pub const fn maca1hr(&self) -> &MACA1HR {
        &self.maca1hr
    }
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    #[inline(always)]
    pub const fn maca1lr(&self) -> &MACA1LR {
        &self.maca1lr
    }
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub const fn maca2hr(&self) -> &MACA2HR {
        &self.maca2hr
    }
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    #[inline(always)]
    pub const fn maca2lr(&self) -> &MACA2LR {
        &self.maca2lr
    }
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    #[inline(always)]
    pub const fn maca3hr(&self) -> &MACA3HR {
        &self.maca3hr
    }
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    #[inline(always)]
    pub const fn maca3lr(&self) -> &MACA3LR {
        &self.maca3lr
    }
}
#[doc = "MACCR (rw) register accessor: Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maccr`]
module"]
pub type MACCR = crate::Reg<maccr::MACCRrs>;
#[doc = "Ethernet MAC configuration register"]
pub mod maccr;
#[doc = "MACFFR (rw) register accessor: Ethernet MAC frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macffr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macffr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macffr`]
module"]
pub type MACFFR = crate::Reg<macffr::MACFFRrs>;
#[doc = "Ethernet MAC frame filter register"]
pub mod macffr;
#[doc = "MACHTHR (rw) register accessor: Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machthr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machthr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machthr`]
module"]
pub type MACHTHR = crate::Reg<machthr::MACHTHRrs>;
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "MACHTLR (rw) register accessor: Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machtlr`]
module"]
pub type MACHTLR = crate::Reg<machtlr::MACHTLRrs>;
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "MACMIIAR (rw) register accessor: Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiiar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiiar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiiar`]
module"]
pub type MACMIIAR = crate::Reg<macmiiar::MACMIIARrs>;
#[doc = "Ethernet MAC MII address register"]
pub mod macmiiar;
#[doc = "MACMIIDR (rw) register accessor: Ethernet MAC MII data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiidr`]
module"]
pub type MACMIIDR = crate::Reg<macmiidr::MACMIIDRrs>;
#[doc = "Ethernet MAC MII data register"]
pub mod macmiidr;
#[doc = "MACFCR (rw) register accessor: Ethernet MAC flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macfcr`]
module"]
pub type MACFCR = crate::Reg<macfcr::MACFCRrs>;
#[doc = "Ethernet MAC flow control register"]
pub mod macfcr;
#[doc = "MACVLANTR (rw) register accessor: Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvlantr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvlantr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvlantr`]
module"]
pub type MACVLANTR = crate::Reg<macvlantr::MACVLANTRrs>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod macvlantr;
#[doc = "MACPMTCSR (rw) register accessor: Ethernet MAC PMT control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macpmtcsr`]
module"]
pub type MACPMTCSR = crate::Reg<macpmtcsr::MACPMTCSRrs>;
#[doc = "Ethernet MAC PMT control and status register"]
pub mod macpmtcsr;
#[doc = "MACDBGR (r) register accessor: Ethernet MAC debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macdbgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macdbgr`]
module"]
pub type MACDBGR = crate::Reg<macdbgr::MACDBGRrs>;
#[doc = "Ethernet MAC debug register"]
pub mod macdbgr;
#[doc = "MACSR (rw) register accessor: Ethernet MAC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macsr`]
module"]
pub type MACSR = crate::Reg<macsr::MACSRrs>;
#[doc = "Ethernet MAC interrupt status register"]
pub mod macsr;
#[doc = "MACIMR (rw) register accessor: Ethernet MAC interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macimr`]
module"]
pub type MACIMR = crate::Reg<macimr::MACIMRrs>;
#[doc = "Ethernet MAC interrupt mask register"]
pub mod macimr;
#[doc = "MACA0HR (rw) register accessor: Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0hr`]
module"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HRrs>;
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0lr`]
module"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LRrs>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1HR (rw) register accessor: Ethernet MAC address 1 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1hr`]
module"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HRrs>;
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1hr;
#[doc = "MACA1LR (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1lr`]
module"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LRrs>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "MACA2HR (rw) register accessor: Ethernet MAC address 2 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2hr`]
module"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HRrs>;
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2hr;
#[doc = "MACA2LR (rw) register accessor: Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2lr`]
module"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LRrs>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "MACA3HR (rw) register accessor: Ethernet MAC address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3hr`]
module"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HRrs>;
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3hr;
#[doc = "MACA3LR (rw) register accessor: Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3lr`]
module"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LRrs>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
#[doc = "MACRWUFFR (rw) register accessor: Ethernet MAC remote wakeup frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwuffr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwuffr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macrwuffr`]
module"]
pub type MACRWUFFR = crate::Reg<macrwuffr::MACRWUFFRrs>;
#[doc = "Ethernet MAC remote wakeup frame filter register"]
pub mod macrwuffr;
