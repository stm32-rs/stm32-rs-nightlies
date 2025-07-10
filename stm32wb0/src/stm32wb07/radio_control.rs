#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    id: ID,
    clk32count_reg: CLK32COUNT_REG,
    clk32period_reg: CLK32PERIOD_REG,
    clk32frequency_reg: CLK32FREQUENCY_REG,
    irq_status: IRQ_STATUS,
    irq_enable: IRQ_ENABLE,
}
impl RegisterBlock {
    ///0x00 - RADIO_CONTROL_ID register
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    ///0x04 - CLK32COUNT_REG register
    #[inline(always)]
    pub const fn clk32count_reg(&self) -> &CLK32COUNT_REG {
        &self.clk32count_reg
    }
    ///0x08 - CLK32PERIOD_REG register
    #[inline(always)]
    pub const fn clk32period_reg(&self) -> &CLK32PERIOD_REG {
        &self.clk32period_reg
    }
    ///0x0c - CLK32FREQUENCY_REG register
    #[inline(always)]
    pub const fn clk32frequency_reg(&self) -> &CLK32FREQUENCY_REG {
        &self.clk32frequency_reg
    }
    ///0x10 - RADIO_CONTROL_IRQ_STATUS register
    #[inline(always)]
    pub const fn irq_status(&self) -> &IRQ_STATUS {
        &self.irq_status
    }
    ///0x14 - RADIO_CONTROL_IRQ_ENABLE register
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IRQ_ENABLE {
        &self.irq_enable
    }
}
/**ID (r) register accessor: RADIO_CONTROL_ID register

You can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO_CONTROL:ID)

For information about available fields see [`mod@id`] module*/
pub type ID = crate::Reg<id::IDrs>;
///RADIO_CONTROL_ID register
pub mod id;
/**CLK32COUNT_REG (rw) register accessor: CLK32COUNT_REG register

You can [`read`](crate::Reg::read) this register and get [`clk32count_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk32count_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO_CONTROL:CLK32COUNT_REG)

For information about available fields see [`mod@clk32count_reg`] module*/
pub type CLK32COUNT_REG = crate::Reg<clk32count_reg::CLK32COUNT_REGrs>;
///CLK32COUNT_REG register
pub mod clk32count_reg;
/**CLK32PERIOD_REG (r) register accessor: CLK32PERIOD_REG register

You can [`read`](crate::Reg::read) this register and get [`clk32period_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO_CONTROL:CLK32PERIOD_REG)

For information about available fields see [`mod@clk32period_reg`] module*/
pub type CLK32PERIOD_REG = crate::Reg<clk32period_reg::CLK32PERIOD_REGrs>;
///CLK32PERIOD_REG register
pub mod clk32period_reg;
/**CLK32FREQUENCY_REG (r) register accessor: CLK32FREQUENCY_REG register

You can [`read`](crate::Reg::read) this register and get [`clk32frequency_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO_CONTROL:CLK32FREQUENCY_REG)

For information about available fields see [`mod@clk32frequency_reg`] module*/
pub type CLK32FREQUENCY_REG = crate::Reg<clk32frequency_reg::CLK32FREQUENCY_REGrs>;
///CLK32FREQUENCY_REG register
pub mod clk32frequency_reg;
/**IRQ_STATUS (rw) register accessor: RADIO_CONTROL_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO_CONTROL:IRQ_STATUS)

For information about available fields see [`mod@irq_status`] module*/
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUSrs>;
///RADIO_CONTROL_IRQ_STATUS register
pub mod irq_status;
/**IRQ_ENABLE (rw) register accessor: RADIO_CONTROL_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO_CONTROL:IRQ_ENABLE)

For information about available fields see [`mod@irq_enable`] module*/
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLErs>;
///RADIO_CONTROL_IRQ_ENABLE register
pub mod irq_enable;
