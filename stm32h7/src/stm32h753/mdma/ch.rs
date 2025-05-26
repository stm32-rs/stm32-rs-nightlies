#[repr(C)]
#[derive(Debug)]
///Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
pub struct CH {
    isr: ISR,
    ifcr: IFCR,
    esr: ESR,
    cr: CR,
    tcr: TCR,
    bndtr: BNDTR,
    sar: SAR,
    dar: DAR,
    brur: BRUR,
    lar: LAR,
    tbr: TBR,
    _reserved11: [u8; 0x04],
    mar: MAR,
    mdr: MDR,
    _reserved_end: [u8; 0x08],
}
impl CH {
    ///0x00 - MDMA channel x interrupt/status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - MDMA channel x interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x08 - MDMA Channel x error status register
    #[inline(always)]
    pub const fn esr(&self) -> &ESR {
        &self.esr
    }
    ///0x0c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - This register is used to configure the concerned channel.
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    ///0x14 - MDMA Channel x block number of data register
    #[inline(always)]
    pub const fn bndtr(&self) -> &BNDTR {
        &self.bndtr
    }
    ///0x18 - MDMA channel x source address register
    #[inline(always)]
    pub const fn sar(&self) -> &SAR {
        &self.sar
    }
    ///0x1c - MDMA channel x destination address register
    #[inline(always)]
    pub const fn dar(&self) -> &DAR {
        &self.dar
    }
    ///0x20 - MDMA channel x Block Repeat address Update register
    #[inline(always)]
    pub const fn brur(&self) -> &BRUR {
        &self.brur
    }
    ///0x24 - MDMA channel x Link Address register
    #[inline(always)]
    pub const fn lar(&self) -> &LAR {
        &self.lar
    }
    ///0x28 - MDMA channel x Trigger and Bus selection Register
    #[inline(always)]
    pub const fn tbr(&self) -> &TBR {
        &self.tbr
    }
    ///0x30 - MDMA channel x Mask address register
    #[inline(always)]
    pub const fn mar(&self) -> &MAR {
        &self.mar
    }
    ///0x34 - MDMA channel x Mask Data register
    #[inline(always)]
    pub const fn mdr(&self) -> &MDR {
        &self.mdr
    }
}
/**ISR (r) register accessor: MDMA channel x interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///MDMA channel x interrupt/status register
pub mod isr;
/**IFCR (w) register accessor: MDMA channel x interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///MDMA channel x interrupt flag clear register
pub mod ifcr;
/**ESR (r) register accessor: MDMA Channel x error status register

You can [`read`](crate::Reg::read) this register and get [`esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@esr`] module*/
pub type ESR = crate::Reg<esr::ESRrs>;
///MDMA Channel x error status register
pub mod esr;
/**CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///This register is used to control the concerned channel.
pub mod cr;
/**TCR (rw) register accessor: This register is used to configure the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcr`] module*/
pub type TCR = crate::Reg<tcr::TCRrs>;
///This register is used to configure the concerned channel.
pub mod tcr;
/**BNDTR (rw) register accessor: MDMA Channel x block number of data register

You can [`read`](crate::Reg::read) this register and get [`bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bndtr`] module*/
pub type BNDTR = crate::Reg<bndtr::BNDTRrs>;
///MDMA Channel x block number of data register
pub mod bndtr;
/**SAR (rw) register accessor: MDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar`] module*/
pub type SAR = crate::Reg<sar::SARrs>;
///MDMA channel x source address register
pub mod sar;
/**DAR (rw) register accessor: MDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dar`] module*/
pub type DAR = crate::Reg<dar::DARrs>;
///MDMA channel x destination address register
pub mod dar;
/**BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register

You can [`read`](crate::Reg::read) this register and get [`brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brur`] module*/
pub type BRUR = crate::Reg<brur::BRURrs>;
///MDMA channel x Block Repeat address Update register
pub mod brur;
/**LAR (rw) register accessor: MDMA channel x Link Address register

You can [`read`](crate::Reg::read) this register and get [`lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lar`] module*/
pub type LAR = crate::Reg<lar::LARrs>;
///MDMA channel x Link Address register
pub mod lar;
/**TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register

You can [`read`](crate::Reg::read) this register and get [`tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tbr`] module*/
pub type TBR = crate::Reg<tbr::TBRrs>;
///MDMA channel x Trigger and Bus selection Register
pub mod tbr;
/**MAR (rw) register accessor: MDMA channel x Mask address register

You can [`read`](crate::Reg::read) this register and get [`mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mar`] module*/
pub type MAR = crate::Reg<mar::MARrs>;
///MDMA channel x Mask address register
pub mod mar;
/**MDR (rw) register accessor: MDMA channel x Mask Data register

You can [`read`](crate::Reg::read) this register and get [`mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mdr`] module*/
pub type MDR = crate::Reg<mdr::MDRrs>;
///MDMA channel x Mask Data register
pub mod mdr;
