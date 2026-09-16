#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    axisram1cr: AXISRAM1CR,
    _reserved1: [u8; 0x04],
    axisram1isr: AXISRAM1ISR,
    _reserved2: [u8; 0x1c],
    axisram1erkeyr: AXISRAM1ERKEYR,
    _reserved3: [u8; 0x54],
    axisram2cr: AXISRAM2CR,
    _reserved4: [u8; 0x04],
    axisram2isr: AXISRAM2ISR,
    _reserved5: [u8; 0x1c],
    axisram2erkeyr: AXISRAM2ERKEYR,
    _reserved6: [u8; 0x54],
    axisram3cr: AXISRAM3CR,
    _reserved7: [u8; 0x04],
    axisram3isr: AXISRAM3ISR,
    _reserved8: [u8; 0x1c],
    axisram3erkeyr: AXISRAM3ERKEYR,
    _reserved9: [u8; 0x54],
    axisram4cr: AXISRAM4CR,
    _reserved10: [u8; 0x04],
    axisram4isr: AXISRAM4ISR,
    _reserved11: [u8; 0x1c],
    axisram4erkeyr: AXISRAM4ERKEYR,
    _reserved12: [u8; 0x54],
    axisram5cr: AXISRAM5CR,
    _reserved13: [u8; 0x04],
    axisram5isr: AXISRAM5ISR,
    _reserved14: [u8; 0x1c],
    axisram5erkeyr: AXISRAM5ERKEYR,
    _reserved15: [u8; 0x54],
    axisram6cr: AXISRAM6CR,
    _reserved16: [u8; 0x04],
    axisram6isr: AXISRAM6ISR,
    _reserved17: [u8; 0x1c],
    axisram6erkeyr: AXISRAM6ERKEYR,
    _reserved18: [u8; 0x54],
    ahbsram1cr: AHBSRAM1CR,
    _reserved19: [u8; 0x04],
    ahbsram1isr: AHBSRAM1ISR,
    _reserved20: [u8; 0x1c],
    ahbsram1erkeyr: AHBSRAM1ERKEYR,
    _reserved21: [u8; 0x54],
    ahbsram2cr: AHBSRAM2CR,
    _reserved22: [u8; 0x04],
    ahbsram2isr: AHBSRAM2ISR,
    _reserved23: [u8; 0x1c],
    ahbsram2erkeyr: AHBSRAM2ERKEYR,
    _reserved24: [u8; 0x54],
    vencramcr: VENCRAMCR,
    _reserved25: [u8; 0x04],
    vencramisr: VENCRAMISR,
    _reserved26: [u8; 0x1c],
    vencramerkeyr: VENCRAMERKEYR,
    _reserved27: [u8; 0x54],
    bkpsramcr: BKPSRAMCR,
    bkpsramier: BKPSRAMIER,
    bkpsramisr: BKPSRAMISR,
    bkpsramesear: BKPSRAMESEAR,
    bkpsramedear: BKPSRAMEDEAR,
    bkpsramicr: BKPSRAMICR,
    _reserved33: [u8; 0x0c],
    bkpsramecckeyr: BKPSRAMECCKEYR,
    bkpsramerkeyr: BKPSRAMERKEYR,
    _reserved35: [u8; 0x54],
    flexramcr: FLEXRAMCR,
    _reserved36: [u8; 0x04],
    flexramisr: FLEXRAMISR,
    _reserved37: [u8; 0x1c],
    flexramerkeyr: FLEXRAMERKEYR,
}
impl RegisterBlock {
    ///0x00 - RAMCFG AXISRAM1 control register
    #[inline(always)]
    pub const fn axisram1cr(&self) -> &AXISRAM1CR {
        &self.axisram1cr
    }
    ///0x08 - RAMCFG AXISRAM1 interrupt status register
    #[inline(always)]
    pub const fn axisram1isr(&self) -> &AXISRAM1ISR {
        &self.axisram1isr
    }
    ///0x28 - RAMCFG AXISRAM1 erase key register
    #[inline(always)]
    pub const fn axisram1erkeyr(&self) -> &AXISRAM1ERKEYR {
        &self.axisram1erkeyr
    }
    ///0x80 - RAMCFG AXISRAM2 control register
    #[inline(always)]
    pub const fn axisram2cr(&self) -> &AXISRAM2CR {
        &self.axisram2cr
    }
    ///0x88 - RAMCFG AXISRAM2 interrupt status register
    #[inline(always)]
    pub const fn axisram2isr(&self) -> &AXISRAM2ISR {
        &self.axisram2isr
    }
    ///0xa8 - RAMCFG AXISRAM2 erase key register
    #[inline(always)]
    pub const fn axisram2erkeyr(&self) -> &AXISRAM2ERKEYR {
        &self.axisram2erkeyr
    }
    ///0x100 - RAMCFG AXISRAM3 control register
    #[inline(always)]
    pub const fn axisram3cr(&self) -> &AXISRAM3CR {
        &self.axisram3cr
    }
    ///0x108 - RAMCFG AXISRAM3 interrupt status register
    #[inline(always)]
    pub const fn axisram3isr(&self) -> &AXISRAM3ISR {
        &self.axisram3isr
    }
    ///0x128 - RAMCFG AXISRAM3 erase key register
    #[inline(always)]
    pub const fn axisram3erkeyr(&self) -> &AXISRAM3ERKEYR {
        &self.axisram3erkeyr
    }
    ///0x180 - RAMCFG AXISRAM4 control register
    #[inline(always)]
    pub const fn axisram4cr(&self) -> &AXISRAM4CR {
        &self.axisram4cr
    }
    ///0x188 - RAMCFG AXISRAM4 interrupt status register
    #[inline(always)]
    pub const fn axisram4isr(&self) -> &AXISRAM4ISR {
        &self.axisram4isr
    }
    ///0x1a8 - RAMCFG AXISRAM4 erase key register
    #[inline(always)]
    pub const fn axisram4erkeyr(&self) -> &AXISRAM4ERKEYR {
        &self.axisram4erkeyr
    }
    ///0x200 - RAMCFG AXISRAM5 control register
    #[inline(always)]
    pub const fn axisram5cr(&self) -> &AXISRAM5CR {
        &self.axisram5cr
    }
    ///0x208 - RAMCFG AXISRAM5 interrupt status register
    #[inline(always)]
    pub const fn axisram5isr(&self) -> &AXISRAM5ISR {
        &self.axisram5isr
    }
    ///0x228 - RAMCFG AXISRAM5 erase key register
    #[inline(always)]
    pub const fn axisram5erkeyr(&self) -> &AXISRAM5ERKEYR {
        &self.axisram5erkeyr
    }
    ///0x280 - RAMCFG AXISRAM6 control register
    #[inline(always)]
    pub const fn axisram6cr(&self) -> &AXISRAM6CR {
        &self.axisram6cr
    }
    ///0x288 - RAMCFG AXISRAM6 interrupt status register
    #[inline(always)]
    pub const fn axisram6isr(&self) -> &AXISRAM6ISR {
        &self.axisram6isr
    }
    ///0x2a8 - RAMCFG AXISRAM6 erase key register
    #[inline(always)]
    pub const fn axisram6erkeyr(&self) -> &AXISRAM6ERKEYR {
        &self.axisram6erkeyr
    }
    ///0x300 - RAMCFG AHBSRAM1 control register
    #[inline(always)]
    pub const fn ahbsram1cr(&self) -> &AHBSRAM1CR {
        &self.ahbsram1cr
    }
    ///0x308 - RAMCFG AHBSRAM1 interrupt status register
    #[inline(always)]
    pub const fn ahbsram1isr(&self) -> &AHBSRAM1ISR {
        &self.ahbsram1isr
    }
    ///0x328 - RAMCFG AHBSRAM1 erase key register
    #[inline(always)]
    pub const fn ahbsram1erkeyr(&self) -> &AHBSRAM1ERKEYR {
        &self.ahbsram1erkeyr
    }
    ///0x380 - RAMCFG AHBSRAM2 control register
    #[inline(always)]
    pub const fn ahbsram2cr(&self) -> &AHBSRAM2CR {
        &self.ahbsram2cr
    }
    ///0x388 - RAMCFG AHBSRAM2 interrupt status register
    #[inline(always)]
    pub const fn ahbsram2isr(&self) -> &AHBSRAM2ISR {
        &self.ahbsram2isr
    }
    ///0x3a8 - RAMCFG AHBSRAM2 erase key register
    #[inline(always)]
    pub const fn ahbsram2erkeyr(&self) -> &AHBSRAM2ERKEYR {
        &self.ahbsram2erkeyr
    }
    ///0x400 - RAMCFG VENCRAM control register
    #[inline(always)]
    pub const fn vencramcr(&self) -> &VENCRAMCR {
        &self.vencramcr
    }
    ///0x408 - RAMCFG VENCRAM interrupt status register
    #[inline(always)]
    pub const fn vencramisr(&self) -> &VENCRAMISR {
        &self.vencramisr
    }
    ///0x428 - RAMCFG VENCRAM erase key register
    #[inline(always)]
    pub const fn vencramerkeyr(&self) -> &VENCRAMERKEYR {
        &self.vencramerkeyr
    }
    ///0x480 - RAMCFG BKPSRAM control register
    #[inline(always)]
    pub const fn bkpsramcr(&self) -> &BKPSRAMCR {
        &self.bkpsramcr
    }
    ///0x484 - RAMCFG BKPSRAM interrupt enable register
    #[inline(always)]
    pub const fn bkpsramier(&self) -> &BKPSRAMIER {
        &self.bkpsramier
    }
    ///0x488 - RAMCFG BKPSRAM interrupt status register
    #[inline(always)]
    pub const fn bkpsramisr(&self) -> &BKPSRAMISR {
        &self.bkpsramisr
    }
    ///0x48c - RAMCFG BKPSRAM single error address register
    #[inline(always)]
    pub const fn bkpsramesear(&self) -> &BKPSRAMESEAR {
        &self.bkpsramesear
    }
    ///0x490 - RAMCFG BKPSRAM double error address register
    #[inline(always)]
    pub const fn bkpsramedear(&self) -> &BKPSRAMEDEAR {
        &self.bkpsramedear
    }
    ///0x494 - RAMCFG BKPSRAM interrupt clear register
    #[inline(always)]
    pub const fn bkpsramicr(&self) -> &BKPSRAMICR {
        &self.bkpsramicr
    }
    ///0x4a4 - RAMCFG BKPSRAM ECC key register
    #[inline(always)]
    pub const fn bkpsramecckeyr(&self) -> &BKPSRAMECCKEYR {
        &self.bkpsramecckeyr
    }
    ///0x4a8 - RAMCFG BKPSRAM erase key register
    #[inline(always)]
    pub const fn bkpsramerkeyr(&self) -> &BKPSRAMERKEYR {
        &self.bkpsramerkeyr
    }
    ///0x500 - RAMCFG FLEXRAM control register
    #[inline(always)]
    pub const fn flexramcr(&self) -> &FLEXRAMCR {
        &self.flexramcr
    }
    ///0x508 - RAMCFG FLEXRAM interrupt status register
    #[inline(always)]
    pub const fn flexramisr(&self) -> &FLEXRAMISR {
        &self.flexramisr
    }
    ///0x528 - RAMCFG FLEXRAM erase key register
    #[inline(always)]
    pub const fn flexramerkeyr(&self) -> &FLEXRAMERKEYR {
        &self.flexramerkeyr
    }
}
/**AXISRAM1CR (rw) register accessor: RAMCFG AXISRAM1 control register

You can [`read`](crate::Reg::read) this register and get [`axisram1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM1CR)

For information about available fields see [`mod@axisram1cr`] module*/
pub type AXISRAM1CR = crate::Reg<axisram1cr::AXISRAM1CRrs>;
///RAMCFG AXISRAM1 control register
pub mod axisram1cr;
/**AXISRAM1ISR (r) register accessor: RAMCFG AXISRAM1 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM1ISR)

For information about available fields see [`mod@axisram1isr`] module*/
pub type AXISRAM1ISR = crate::Reg<axisram1isr::AXISRAM1ISRrs>;
///RAMCFG AXISRAM1 interrupt status register
pub mod axisram1isr;
/**AXISRAM1ERKEYR (w) register accessor: RAMCFG AXISRAM1 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM1ERKEYR)

For information about available fields see [`mod@axisram1erkeyr`] module*/
pub type AXISRAM1ERKEYR = crate::Reg<axisram1erkeyr::AXISRAM1ERKEYRrs>;
///RAMCFG AXISRAM1 erase key register
pub mod axisram1erkeyr;
/**AXISRAM2CR (rw) register accessor: RAMCFG AXISRAM2 control register

You can [`read`](crate::Reg::read) this register and get [`axisram2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM2CR)

For information about available fields see [`mod@axisram2cr`] module*/
pub type AXISRAM2CR = crate::Reg<axisram2cr::AXISRAM2CRrs>;
///RAMCFG AXISRAM2 control register
pub mod axisram2cr;
/**AXISRAM2ISR (r) register accessor: RAMCFG AXISRAM2 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM2ISR)

For information about available fields see [`mod@axisram2isr`] module*/
pub type AXISRAM2ISR = crate::Reg<axisram2isr::AXISRAM2ISRrs>;
///RAMCFG AXISRAM2 interrupt status register
pub mod axisram2isr;
/**AXISRAM2ERKEYR (w) register accessor: RAMCFG AXISRAM2 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM2ERKEYR)

For information about available fields see [`mod@axisram2erkeyr`] module*/
pub type AXISRAM2ERKEYR = crate::Reg<axisram2erkeyr::AXISRAM2ERKEYRrs>;
///RAMCFG AXISRAM2 erase key register
pub mod axisram2erkeyr;
/**AXISRAM3CR (rw) register accessor: RAMCFG AXISRAM3 control register

You can [`read`](crate::Reg::read) this register and get [`axisram3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM3CR)

For information about available fields see [`mod@axisram3cr`] module*/
pub type AXISRAM3CR = crate::Reg<axisram3cr::AXISRAM3CRrs>;
///RAMCFG AXISRAM3 control register
pub mod axisram3cr;
/**AXISRAM3ISR (r) register accessor: RAMCFG AXISRAM3 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM3ISR)

For information about available fields see [`mod@axisram3isr`] module*/
pub type AXISRAM3ISR = crate::Reg<axisram3isr::AXISRAM3ISRrs>;
///RAMCFG AXISRAM3 interrupt status register
pub mod axisram3isr;
/**AXISRAM3ERKEYR (w) register accessor: RAMCFG AXISRAM3 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM3ERKEYR)

For information about available fields see [`mod@axisram3erkeyr`] module*/
pub type AXISRAM3ERKEYR = crate::Reg<axisram3erkeyr::AXISRAM3ERKEYRrs>;
///RAMCFG AXISRAM3 erase key register
pub mod axisram3erkeyr;
/**AXISRAM4CR (rw) register accessor: RAMCFG AXISRAM4 control register

You can [`read`](crate::Reg::read) this register and get [`axisram4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM4CR)

For information about available fields see [`mod@axisram4cr`] module*/
pub type AXISRAM4CR = crate::Reg<axisram4cr::AXISRAM4CRrs>;
///RAMCFG AXISRAM4 control register
pub mod axisram4cr;
/**AXISRAM4ISR (r) register accessor: RAMCFG AXISRAM4 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM4ISR)

For information about available fields see [`mod@axisram4isr`] module*/
pub type AXISRAM4ISR = crate::Reg<axisram4isr::AXISRAM4ISRrs>;
///RAMCFG AXISRAM4 interrupt status register
pub mod axisram4isr;
/**AXISRAM4ERKEYR (w) register accessor: RAMCFG AXISRAM4 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM4ERKEYR)

For information about available fields see [`mod@axisram4erkeyr`] module*/
pub type AXISRAM4ERKEYR = crate::Reg<axisram4erkeyr::AXISRAM4ERKEYRrs>;
///RAMCFG AXISRAM4 erase key register
pub mod axisram4erkeyr;
/**AXISRAM5CR (rw) register accessor: RAMCFG AXISRAM5 control register

You can [`read`](crate::Reg::read) this register and get [`axisram5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM5CR)

For information about available fields see [`mod@axisram5cr`] module*/
pub type AXISRAM5CR = crate::Reg<axisram5cr::AXISRAM5CRrs>;
///RAMCFG AXISRAM5 control register
pub mod axisram5cr;
/**AXISRAM5ISR (r) register accessor: RAMCFG AXISRAM5 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM5ISR)

For information about available fields see [`mod@axisram5isr`] module*/
pub type AXISRAM5ISR = crate::Reg<axisram5isr::AXISRAM5ISRrs>;
///RAMCFG AXISRAM5 interrupt status register
pub mod axisram5isr;
/**AXISRAM5ERKEYR (w) register accessor: RAMCFG AXISRAM5 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram5erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM5ERKEYR)

For information about available fields see [`mod@axisram5erkeyr`] module*/
pub type AXISRAM5ERKEYR = crate::Reg<axisram5erkeyr::AXISRAM5ERKEYRrs>;
///RAMCFG AXISRAM5 erase key register
pub mod axisram5erkeyr;
/**AXISRAM6CR (rw) register accessor: RAMCFG AXISRAM6 control register

You can [`read`](crate::Reg::read) this register and get [`axisram6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM6CR)

For information about available fields see [`mod@axisram6cr`] module*/
pub type AXISRAM6CR = crate::Reg<axisram6cr::AXISRAM6CRrs>;
///RAMCFG AXISRAM6 control register
pub mod axisram6cr;
/**AXISRAM6ISR (r) register accessor: RAMCFG AXISRAM6 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram6isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM6ISR)

For information about available fields see [`mod@axisram6isr`] module*/
pub type AXISRAM6ISR = crate::Reg<axisram6isr::AXISRAM6ISRrs>;
///RAMCFG AXISRAM6 interrupt status register
pub mod axisram6isr;
/**AXISRAM6ERKEYR (w) register accessor: RAMCFG AXISRAM6 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram6erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM6ERKEYR)

For information about available fields see [`mod@axisram6erkeyr`] module*/
pub type AXISRAM6ERKEYR = crate::Reg<axisram6erkeyr::AXISRAM6ERKEYRrs>;
///RAMCFG AXISRAM6 erase key register
pub mod axisram6erkeyr;
/**AHBSRAM1CR (rw) register accessor: RAMCFG AHBSRAM1 control register

You can [`read`](crate::Reg::read) this register and get [`ahbsram1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsram1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AHBSRAM1CR)

For information about available fields see [`mod@ahbsram1cr`] module*/
pub type AHBSRAM1CR = crate::Reg<ahbsram1cr::AHBSRAM1CRrs>;
///RAMCFG AHBSRAM1 control register
pub mod ahbsram1cr;
/**AHBSRAM1ISR (r) register accessor: RAMCFG AHBSRAM1 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ahbsram1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AHBSRAM1ISR)

For information about available fields see [`mod@ahbsram1isr`] module*/
pub type AHBSRAM1ISR = crate::Reg<ahbsram1isr::AHBSRAM1ISRrs>;
///RAMCFG AHBSRAM1 interrupt status register
pub mod ahbsram1isr;
/**AHBSRAM1ERKEYR (w) register accessor: RAMCFG AHBSRAM1 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsram1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AHBSRAM1ERKEYR)

For information about available fields see [`mod@ahbsram1erkeyr`] module*/
pub type AHBSRAM1ERKEYR = crate::Reg<ahbsram1erkeyr::AHBSRAM1ERKEYRrs>;
///RAMCFG AHBSRAM1 erase key register
pub mod ahbsram1erkeyr;
/**AHBSRAM2CR (rw) register accessor: RAMCFG AHBSRAM2 control register

You can [`read`](crate::Reg::read) this register and get [`ahbsram2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsram2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AHBSRAM2CR)

For information about available fields see [`mod@ahbsram2cr`] module*/
pub type AHBSRAM2CR = crate::Reg<ahbsram2cr::AHBSRAM2CRrs>;
///RAMCFG AHBSRAM2 control register
pub mod ahbsram2cr;
/**AHBSRAM2ISR (r) register accessor: RAMCFG AHBSRAM2 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ahbsram2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AHBSRAM2ISR)

For information about available fields see [`mod@ahbsram2isr`] module*/
pub type AHBSRAM2ISR = crate::Reg<ahbsram2isr::AHBSRAM2ISRrs>;
///RAMCFG AHBSRAM2 interrupt status register
pub mod ahbsram2isr;
/**AHBSRAM2ERKEYR (w) register accessor: RAMCFG AHBSRAM2 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsram2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AHBSRAM2ERKEYR)

For information about available fields see [`mod@ahbsram2erkeyr`] module*/
pub type AHBSRAM2ERKEYR = crate::Reg<ahbsram2erkeyr::AHBSRAM2ERKEYRrs>;
///RAMCFG AHBSRAM2 erase key register
pub mod ahbsram2erkeyr;
/**VENCRAMCR (rw) register accessor: RAMCFG VENCRAM control register

You can [`read`](crate::Reg::read) this register and get [`vencramcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vencramcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:VENCRAMCR)

For information about available fields see [`mod@vencramcr`] module*/
pub type VENCRAMCR = crate::Reg<vencramcr::VENCRAMCRrs>;
///RAMCFG VENCRAM control register
pub mod vencramcr;
/**VENCRAMISR (r) register accessor: RAMCFG VENCRAM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`vencramisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:VENCRAMISR)

For information about available fields see [`mod@vencramisr`] module*/
pub type VENCRAMISR = crate::Reg<vencramisr::VENCRAMISRrs>;
///RAMCFG VENCRAM interrupt status register
pub mod vencramisr;
/**VENCRAMERKEYR (w) register accessor: RAMCFG VENCRAM erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vencramerkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:VENCRAMERKEYR)

For information about available fields see [`mod@vencramerkeyr`] module*/
pub type VENCRAMERKEYR = crate::Reg<vencramerkeyr::VENCRAMERKEYRrs>;
///RAMCFG VENCRAM erase key register
pub mod vencramerkeyr;
/**BKPSRAMCR (rw) register accessor: RAMCFG BKPSRAM control register

You can [`read`](crate::Reg::read) this register and get [`bkpsramcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMCR)

For information about available fields see [`mod@bkpsramcr`] module*/
pub type BKPSRAMCR = crate::Reg<bkpsramcr::BKPSRAMCRrs>;
///RAMCFG BKPSRAM control register
pub mod bkpsramcr;
/**BKPSRAMIER (rw) register accessor: RAMCFG BKPSRAM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`bkpsramier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMIER)

For information about available fields see [`mod@bkpsramier`] module*/
pub type BKPSRAMIER = crate::Reg<bkpsramier::BKPSRAMIERrs>;
///RAMCFG BKPSRAM interrupt enable register
pub mod bkpsramier;
/**BKPSRAMISR (r) register accessor: RAMCFG BKPSRAM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`bkpsramisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMISR)

For information about available fields see [`mod@bkpsramisr`] module*/
pub type BKPSRAMISR = crate::Reg<bkpsramisr::BKPSRAMISRrs>;
///RAMCFG BKPSRAM interrupt status register
pub mod bkpsramisr;
/**BKPSRAMESEAR (w) register accessor: RAMCFG BKPSRAM single error address register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramesear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMESEAR)

For information about available fields see [`mod@bkpsramesear`] module*/
pub type BKPSRAMESEAR = crate::Reg<bkpsramesear::BKPSRAMESEARrs>;
///RAMCFG BKPSRAM single error address register
pub mod bkpsramesear;
/**BKPSRAMEDEAR (w) register accessor: RAMCFG BKPSRAM double error address register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramedear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMEDEAR)

For information about available fields see [`mod@bkpsramedear`] module*/
pub type BKPSRAMEDEAR = crate::Reg<bkpsramedear::BKPSRAMEDEARrs>;
///RAMCFG BKPSRAM double error address register
pub mod bkpsramedear;
/**BKPSRAMICR (w) register accessor: RAMCFG BKPSRAM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMICR)

For information about available fields see [`mod@bkpsramicr`] module*/
pub type BKPSRAMICR = crate::Reg<bkpsramicr::BKPSRAMICRrs>;
///RAMCFG BKPSRAM interrupt clear register
pub mod bkpsramicr;
/**BKPSRAMECCKEYR (w) register accessor: RAMCFG BKPSRAM ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMECCKEYR)

For information about available fields see [`mod@bkpsramecckeyr`] module*/
pub type BKPSRAMECCKEYR = crate::Reg<bkpsramecckeyr::BKPSRAMECCKEYRrs>;
///RAMCFG BKPSRAM ECC key register
pub mod bkpsramecckeyr;
/**BKPSRAMERKEYR (w) register accessor: RAMCFG BKPSRAM erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramerkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMERKEYR)

For information about available fields see [`mod@bkpsramerkeyr`] module*/
pub type BKPSRAMERKEYR = crate::Reg<bkpsramerkeyr::BKPSRAMERKEYRrs>;
///RAMCFG BKPSRAM erase key register
pub mod bkpsramerkeyr;
/**FLEXRAMCR (rw) register accessor: RAMCFG FLEXRAM control register

You can [`read`](crate::Reg::read) this register and get [`flexramcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexramcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:FLEXRAMCR)

For information about available fields see [`mod@flexramcr`] module*/
pub type FLEXRAMCR = crate::Reg<flexramcr::FLEXRAMCRrs>;
///RAMCFG FLEXRAM control register
pub mod flexramcr;
/**FLEXRAMISR (r) register accessor: RAMCFG FLEXRAM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`flexramisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:FLEXRAMISR)

For information about available fields see [`mod@flexramisr`] module*/
pub type FLEXRAMISR = crate::Reg<flexramisr::FLEXRAMISRrs>;
///RAMCFG FLEXRAM interrupt status register
pub mod flexramisr;
/**FLEXRAMERKEYR (w) register accessor: RAMCFG FLEXRAM erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexramerkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:FLEXRAMERKEYR)

For information about available fields see [`mod@flexramerkeyr`] module*/
pub type FLEXRAMERKEYR = crate::Reg<flexramerkeyr::FLEXRAMERKEYRrs>;
///RAMCFG FLEXRAM erase key register
pub mod flexramerkeyr;
