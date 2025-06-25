#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    val: VAL,
    _reserved3: [u8; 0x24],
    oscs_cr: OSCS_CR,
    postp_cr: POSTP_CR,
    postp_sr: POSTP_SR,
    _reserved6: [u8; 0x04],
    defkey0: DEFKEY0,
    defkey1: DEFKEY1,
    defkey2: DEFKEY2,
    defkey3: DEFKEY3,
    _reserved10: [u8; 0x10],
    health_cr: HEALTH_CR,
    _reserved11: [u8; 0x04],
    health_osc1_cr: HEALTH_OSC1_CR,
    health_osc2_cr: HEALTH_OSC2_CR,
    health_osc3_cr: HEALTH_OSC3_CR,
    health_osc1_sr: HEALTH_OSC1_SR,
    health_osc2_sr: HEALTH_OSC2_SR,
    health_osc3_sr: HEALTH_OSC3_SR,
    irq_cr: IRQ_CR,
    irq_sr: IRQ_SR,
}
impl RegisterBlock {
    ///0x00 - TRNG_CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - TRNG_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - TRNG_VAL register
    #[inline(always)]
    pub const fn val(&self) -> &VAL {
        &self.val
    }
    ///0x30 - TRNG_OSCS_CR register
    #[inline(always)]
    pub const fn oscs_cr(&self) -> &OSCS_CR {
        &self.oscs_cr
    }
    ///0x34 - TRNG_POSTP_CR register
    #[inline(always)]
    pub const fn postp_cr(&self) -> &POSTP_CR {
        &self.postp_cr
    }
    ///0x38 - TRNG_POSTP_SR register
    #[inline(always)]
    pub const fn postp_sr(&self) -> &POSTP_SR {
        &self.postp_sr
    }
    ///0x40 - TRNG_DEFKEY0 register
    #[inline(always)]
    pub const fn defkey0(&self) -> &DEFKEY0 {
        &self.defkey0
    }
    ///0x44 - TRNG_DEFKEY1 register
    #[inline(always)]
    pub const fn defkey1(&self) -> &DEFKEY1 {
        &self.defkey1
    }
    ///0x48 - TRNG_DEFKEY2 register
    #[inline(always)]
    pub const fn defkey2(&self) -> &DEFKEY2 {
        &self.defkey2
    }
    ///0x4c - TRNG_DEFKEY3 register
    #[inline(always)]
    pub const fn defkey3(&self) -> &DEFKEY3 {
        &self.defkey3
    }
    ///0x60 - TRNG_HEALTH_CR register
    #[inline(always)]
    pub const fn health_cr(&self) -> &HEALTH_CR {
        &self.health_cr
    }
    ///0x68 - TRNG_HEALTH_OSC1_CR register
    #[inline(always)]
    pub const fn health_osc1_cr(&self) -> &HEALTH_OSC1_CR {
        &self.health_osc1_cr
    }
    ///0x6c - TRNG_HEALTH_OSC2_CR register
    #[inline(always)]
    pub const fn health_osc2_cr(&self) -> &HEALTH_OSC2_CR {
        &self.health_osc2_cr
    }
    ///0x70 - TRNG_HEALTH_OSC3_CR register
    #[inline(always)]
    pub const fn health_osc3_cr(&self) -> &HEALTH_OSC3_CR {
        &self.health_osc3_cr
    }
    ///0x74 - TRNG_HEALTH_OSC1_SR register
    #[inline(always)]
    pub const fn health_osc1_sr(&self) -> &HEALTH_OSC1_SR {
        &self.health_osc1_sr
    }
    ///0x78 - TRNG_HEALTH_OSC2_SR register
    #[inline(always)]
    pub const fn health_osc2_sr(&self) -> &HEALTH_OSC2_SR {
        &self.health_osc2_sr
    }
    ///0x7c - TRNG_HEALTH_OSC3_SR register
    #[inline(always)]
    pub const fn health_osc3_sr(&self) -> &HEALTH_OSC3_SR {
        &self.health_osc3_sr
    }
    ///0x80 - TRNG_IRQ_CR register
    #[inline(always)]
    pub const fn irq_cr(&self) -> &IRQ_CR {
        &self.irq_cr
    }
    ///0x84 - TRNG_IRQ_SR register
    #[inline(always)]
    pub const fn irq_sr(&self) -> &IRQ_SR {
        &self.irq_sr
    }
}
/**CR (rw) register accessor: TRNG_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///TRNG_CR register
pub mod cr;
/**SR (r) register accessor: TRNG_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///TRNG_SR register
pub mod sr;
/**VAL (r) register accessor: TRNG_VAL register

You can [`read`](crate::Reg::read) this register and get [`val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:VAL)

For information about available fields see [`mod@val`] module*/
pub type VAL = crate::Reg<val::VALrs>;
///TRNG_VAL register
pub mod val;
/**OSCS_CR (rw) register accessor: TRNG_OSCS_CR register

You can [`read`](crate::Reg::read) this register and get [`oscs_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscs_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:OSCS_CR)

For information about available fields see [`mod@oscs_cr`] module*/
pub type OSCS_CR = crate::Reg<oscs_cr::OSCS_CRrs>;
///TRNG_OSCS_CR register
pub mod oscs_cr;
/**POSTP_CR (rw) register accessor: TRNG_POSTP_CR register

You can [`read`](crate::Reg::read) this register and get [`postp_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`postp_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:POSTP_CR)

For information about available fields see [`mod@postp_cr`] module*/
pub type POSTP_CR = crate::Reg<postp_cr::POSTP_CRrs>;
///TRNG_POSTP_CR register
pub mod postp_cr;
/**POSTP_SR (r) register accessor: TRNG_POSTP_SR register

You can [`read`](crate::Reg::read) this register and get [`postp_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:POSTP_SR)

For information about available fields see [`mod@postp_sr`] module*/
pub type POSTP_SR = crate::Reg<postp_sr::POSTP_SRrs>;
///TRNG_POSTP_SR register
pub mod postp_sr;
/**DEFKEY0 (rw) register accessor: TRNG_DEFKEY0 register

You can [`read`](crate::Reg::read) this register and get [`defkey0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY0)

For information about available fields see [`mod@defkey0`] module*/
pub type DEFKEY0 = crate::Reg<defkey0::DEFKEY0rs>;
///TRNG_DEFKEY0 register
pub mod defkey0;
/**DEFKEY1 (rw) register accessor: TRNG_DEFKEY1 register

You can [`read`](crate::Reg::read) this register and get [`defkey1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY1)

For information about available fields see [`mod@defkey1`] module*/
pub type DEFKEY1 = crate::Reg<defkey1::DEFKEY1rs>;
///TRNG_DEFKEY1 register
pub mod defkey1;
/**DEFKEY2 (rw) register accessor: TRNG_DEFKEY2 register

You can [`read`](crate::Reg::read) this register and get [`defkey2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY2)

For information about available fields see [`mod@defkey2`] module*/
pub type DEFKEY2 = crate::Reg<defkey2::DEFKEY2rs>;
///TRNG_DEFKEY2 register
pub mod defkey2;
/**DEFKEY3 (rw) register accessor: TRNG_DEFKEY3 register

You can [`read`](crate::Reg::read) this register and get [`defkey3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY3)

For information about available fields see [`mod@defkey3`] module*/
pub type DEFKEY3 = crate::Reg<defkey3::DEFKEY3rs>;
///TRNG_DEFKEY3 register
pub mod defkey3;
/**HEALTH_CR (rw) register accessor: TRNG_HEALTH_CR register

You can [`read`](crate::Reg::read) this register and get [`health_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`health_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_CR)

For information about available fields see [`mod@health_cr`] module*/
pub type HEALTH_CR = crate::Reg<health_cr::HEALTH_CRrs>;
///TRNG_HEALTH_CR register
pub mod health_cr;
/**HEALTH_OSC1_CR (rw) register accessor: TRNG_HEALTH_OSC1_CR register

You can [`read`](crate::Reg::read) this register and get [`health_osc1_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`health_osc1_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC1_CR)

For information about available fields see [`mod@health_osc1_cr`] module*/
pub type HEALTH_OSC1_CR = crate::Reg<health_osc1_cr::HEALTH_OSC1_CRrs>;
///TRNG_HEALTH_OSC1_CR register
pub mod health_osc1_cr;
/**HEALTH_OSC2_CR (rw) register accessor: TRNG_HEALTH_OSC2_CR register

You can [`read`](crate::Reg::read) this register and get [`health_osc2_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`health_osc2_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC2_CR)

For information about available fields see [`mod@health_osc2_cr`] module*/
pub type HEALTH_OSC2_CR = crate::Reg<health_osc2_cr::HEALTH_OSC2_CRrs>;
///TRNG_HEALTH_OSC2_CR register
pub mod health_osc2_cr;
/**HEALTH_OSC3_CR (rw) register accessor: TRNG_HEALTH_OSC3_CR register

You can [`read`](crate::Reg::read) this register and get [`health_osc3_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`health_osc3_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC3_CR)

For information about available fields see [`mod@health_osc3_cr`] module*/
pub type HEALTH_OSC3_CR = crate::Reg<health_osc3_cr::HEALTH_OSC3_CRrs>;
///TRNG_HEALTH_OSC3_CR register
pub mod health_osc3_cr;
/**HEALTH_OSC1_SR (r) register accessor: TRNG_HEALTH_OSC1_SR register

You can [`read`](crate::Reg::read) this register and get [`health_osc1_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC1_SR)

For information about available fields see [`mod@health_osc1_sr`] module*/
pub type HEALTH_OSC1_SR = crate::Reg<health_osc1_sr::HEALTH_OSC1_SRrs>;
///TRNG_HEALTH_OSC1_SR register
pub mod health_osc1_sr;
/**HEALTH_OSC2_SR (r) register accessor: TRNG_HEALTH_OSC2_SR register

You can [`read`](crate::Reg::read) this register and get [`health_osc2_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC2_SR)

For information about available fields see [`mod@health_osc2_sr`] module*/
pub type HEALTH_OSC2_SR = crate::Reg<health_osc2_sr::HEALTH_OSC2_SRrs>;
///TRNG_HEALTH_OSC2_SR register
pub mod health_osc2_sr;
/**HEALTH_OSC3_SR (r) register accessor: TRNG_HEALTH_OSC3_SR register

You can [`read`](crate::Reg::read) this register and get [`health_osc3_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC3_SR)

For information about available fields see [`mod@health_osc3_sr`] module*/
pub type HEALTH_OSC3_SR = crate::Reg<health_osc3_sr::HEALTH_OSC3_SRrs>;
///TRNG_HEALTH_OSC3_SR register
pub mod health_osc3_sr;
/**IRQ_CR (rw) register accessor: TRNG_IRQ_CR register

You can [`read`](crate::Reg::read) this register and get [`irq_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:IRQ_CR)

For information about available fields see [`mod@irq_cr`] module*/
pub type IRQ_CR = crate::Reg<irq_cr::IRQ_CRrs>;
///TRNG_IRQ_CR register
pub mod irq_cr;
/**IRQ_SR (r) register accessor: TRNG_IRQ_SR register

You can [`read`](crate::Reg::read) this register and get [`irq_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:IRQ_SR)

For information about available fields see [`mod@irq_sr`] module*/
pub type IRQ_SR = crate::Reg<irq_sr::IRQ_SRrs>;
///TRNG_IRQ_SR register
pub mod irq_sr;
