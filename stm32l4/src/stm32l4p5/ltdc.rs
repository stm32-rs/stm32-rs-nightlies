#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    sscr: SSCR,
    bpcr: BPCR,
    awcr: AWCR,
    twcr: TWCR,
    gcr: GCR,
    _reserved5: [u8; 0x08],
    srcr: SRCR,
    _reserved6: [u8; 0x04],
    bccr: BCCR,
    _reserved7: [u8; 0x04],
    ier: IER,
    isr: ISR,
    icr: ICR,
    lipcr: LIPCR,
    cpsr: CPSR,
    cdsr: CDSR,
    _reserved13: [u8; 0x38],
    layer: [LAYER; 2],
}
impl RegisterBlock {
    ///0x08 - LTDC Synchronization Size Configuration Register
    #[inline(always)]
    pub const fn sscr(&self) -> &SSCR {
        &self.sscr
    }
    ///0x0c - LTDC Back Porch Configuration Register
    #[inline(always)]
    pub const fn bpcr(&self) -> &BPCR {
        &self.bpcr
    }
    ///0x10 - LTDC Active Width Configuration Register
    #[inline(always)]
    pub const fn awcr(&self) -> &AWCR {
        &self.awcr
    }
    ///0x14 - LTDC Total Width Configuration Register
    #[inline(always)]
    pub const fn twcr(&self) -> &TWCR {
        &self.twcr
    }
    ///0x18 - LTDC Global Control Register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x24 - LTDC Shadow Reload Configuration Register
    #[inline(always)]
    pub const fn srcr(&self) -> &SRCR {
        &self.srcr
    }
    ///0x2c - LTDC Background Color Configuration Register
    #[inline(always)]
    pub const fn bccr(&self) -> &BCCR {
        &self.bccr
    }
    ///0x34 - LTDC Interrupt Enable Register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x38 - LTDC Interrupt Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x3c - LTDC Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x40 - LTDC Line Interrupt Position Configuration Register
    #[inline(always)]
    pub const fn lipcr(&self) -> &LIPCR {
        &self.lipcr
    }
    ///0x44 - LTDC Current Position Status Register
    #[inline(always)]
    pub const fn cpsr(&self) -> &CPSR {
        &self.cpsr
    }
    ///0x48 - LTDC Current Display Status Register
    #[inline(always)]
    pub const fn cdsr(&self) -> &CDSR {
        &self.cdsr
    }
    ///0x84..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `LAYER1` cluster.</div>
    #[inline(always)]
    pub const fn layer(&self, n: usize) -> &LAYER {
        &self.layer[n]
    }
    ///Iterator for array of:
    ///0x84..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    #[inline(always)]
    pub fn layer_iter(&self) -> impl Iterator<Item = &LAYER> {
        self.layer.iter()
    }
    ///0x84..0x104 - Cluster LAYER1, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    #[inline(always)]
    pub const fn layer1(&self) -> &LAYER {
        self.layer(0)
    }
    ///0x104..0x184 - Cluster LAYER2, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    #[inline(always)]
    pub const fn layer2(&self) -> &LAYER {
        self.layer(1)
    }
}
/**SSCR (rw) register accessor: LTDC Synchronization Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:SSCR)

For information about available fields see [`mod@sscr`] module*/
pub type SSCR = crate::Reg<sscr::SSCRrs>;
///LTDC Synchronization Size Configuration Register
pub mod sscr;
/**BPCR (rw) register accessor: LTDC Back Porch Configuration Register

You can [`read`](crate::Reg::read) this register and get [`bpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:BPCR)

For information about available fields see [`mod@bpcr`] module*/
pub type BPCR = crate::Reg<bpcr::BPCRrs>;
///LTDC Back Porch Configuration Register
pub mod bpcr;
/**AWCR (rw) register accessor: LTDC Active Width Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:AWCR)

For information about available fields see [`mod@awcr`] module*/
pub type AWCR = crate::Reg<awcr::AWCRrs>;
///LTDC Active Width Configuration Register
pub mod awcr;
/**TWCR (rw) register accessor: LTDC Total Width Configuration Register

You can [`read`](crate::Reg::read) this register and get [`twcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:TWCR)

For information about available fields see [`mod@twcr`] module*/
pub type TWCR = crate::Reg<twcr::TWCRrs>;
///LTDC Total Width Configuration Register
pub mod twcr;
/**GCR (rw) register accessor: LTDC Global Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///LTDC Global Control Register
pub mod gcr;
/**SRCR (rw) register accessor: LTDC Shadow Reload Configuration Register

You can [`read`](crate::Reg::read) this register and get [`srcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:SRCR)

For information about available fields see [`mod@srcr`] module*/
pub type SRCR = crate::Reg<srcr::SRCRrs>;
///LTDC Shadow Reload Configuration Register
pub mod srcr;
/**BCCR (rw) register accessor: LTDC Background Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`bccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:BCCR)

For information about available fields see [`mod@bccr`] module*/
pub type BCCR = crate::Reg<bccr::BCCRrs>;
///LTDC Background Color Configuration Register
pub mod bccr;
/**IER (rw) register accessor: LTDC Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///LTDC Interrupt Enable Register
pub mod ier;
/**ISR (r) register accessor: LTDC Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///LTDC Interrupt Status Register
pub mod isr;
/**ICR (w) register accessor: LTDC Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///LTDC Interrupt Clear Register
pub mod icr;
/**LIPCR (rw) register accessor: LTDC Line Interrupt Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`lipcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:LIPCR)

For information about available fields see [`mod@lipcr`] module*/
pub type LIPCR = crate::Reg<lipcr::LIPCRrs>;
///LTDC Line Interrupt Position Configuration Register
pub mod lipcr;
/**CPSR (r) register accessor: LTDC Current Position Status Register

You can [`read`](crate::Reg::read) this register and get [`cpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:CPSR)

For information about available fields see [`mod@cpsr`] module*/
pub type CPSR = crate::Reg<cpsr::CPSRrs>;
///LTDC Current Position Status Register
pub mod cpsr;
/**CDSR (r) register accessor: LTDC Current Display Status Register

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:CDSR)

For information about available fields see [`mod@cdsr`] module*/
pub type CDSR = crate::Reg<cdsr::CDSRrs>;
///LTDC Current Display Status Register
pub mod cdsr;
///Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
pub use self::layer::LAYER;
///Cluster
///Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
pub mod layer;
