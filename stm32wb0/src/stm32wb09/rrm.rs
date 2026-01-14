#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    udra_ctrl0: UDRA_CTRL0,
    udra_irq_enable: UDRA_IRQ_ENABLE,
    udra_irq_status: UDRA_IRQ_STATUS,
    udra_radio_cfg_ptr: UDRA_RADIO_CFG_PTR,
    sema_irq_enable: SEMA_IRQ_ENABLE,
    sema_irq_status: SEMA_IRQ_STATUS,
    ble_irq_enable: BLE_IRQ_ENABLE,
    ble_irq_status: BLE_IRQ_STATUS,
    _reserved8: [u8; 0x30],
    vp_cpu_cmd_bus: VP_CPU_CMD_BUS,
    vp_cpu_sema_bus: VP_CPU_SEMA_BUS,
    vp_cpu_irq_enable: VP_CPU_IRQ_ENABLE,
    vp_cpu_irq_status: VP_CPU_IRQ_STATUS,
}
impl RegisterBlock {
    ///0x10 - UDRA_CTRL0 register
    #[inline(always)]
    pub const fn udra_ctrl0(&self) -> &UDRA_CTRL0 {
        &self.udra_ctrl0
    }
    ///0x14 - UDRA_IRQ_ENABLE register
    #[inline(always)]
    pub const fn udra_irq_enable(&self) -> &UDRA_IRQ_ENABLE {
        &self.udra_irq_enable
    }
    ///0x18 - UDRA_IRQ_STATUS register
    #[inline(always)]
    pub const fn udra_irq_status(&self) -> &UDRA_IRQ_STATUS {
        &self.udra_irq_status
    }
    ///0x1c - UDRA_RADIO_CFG_PTR register
    #[inline(always)]
    pub const fn udra_radio_cfg_ptr(&self) -> &UDRA_RADIO_CFG_PTR {
        &self.udra_radio_cfg_ptr
    }
    ///0x20 - SEMA_IRQ_ENABLE register
    #[inline(always)]
    pub const fn sema_irq_enable(&self) -> &SEMA_IRQ_ENABLE {
        &self.sema_irq_enable
    }
    ///0x24 - SEMA_IRQ_STATUS register
    #[inline(always)]
    pub const fn sema_irq_status(&self) -> &SEMA_IRQ_STATUS {
        &self.sema_irq_status
    }
    ///0x28 - BLE_IRQ_ENABLE register
    #[inline(always)]
    pub const fn ble_irq_enable(&self) -> &BLE_IRQ_ENABLE {
        &self.ble_irq_enable
    }
    ///0x2c - BLE_IRQ_STATUS register
    #[inline(always)]
    pub const fn ble_irq_status(&self) -> &BLE_IRQ_STATUS {
        &self.ble_irq_status
    }
    ///0x60 - VP_CPU_CMD_BUS register
    #[inline(always)]
    pub const fn vp_cpu_cmd_bus(&self) -> &VP_CPU_CMD_BUS {
        &self.vp_cpu_cmd_bus
    }
    ///0x64 - VP_CPU_SEMA_BUS register
    #[inline(always)]
    pub const fn vp_cpu_sema_bus(&self) -> &VP_CPU_SEMA_BUS {
        &self.vp_cpu_sema_bus
    }
    ///0x68 - VP_CPU_IRQ_ENABLE register
    #[inline(always)]
    pub const fn vp_cpu_irq_enable(&self) -> &VP_CPU_IRQ_ENABLE {
        &self.vp_cpu_irq_enable
    }
    ///0x6c - VP_CPU_IRQ_STATUS register
    #[inline(always)]
    pub const fn vp_cpu_irq_status(&self) -> &VP_CPU_IRQ_STATUS {
        &self.vp_cpu_irq_status
    }
}
/**UDRA_CTRL0 (rw) register accessor: UDRA_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`udra_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udra_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:UDRA_CTRL0)

For information about available fields see [`mod@udra_ctrl0`] module*/
pub type UDRA_CTRL0 = crate::Reg<udra_ctrl0::UDRA_CTRL0rs>;
///UDRA_CTRL0 register
pub mod udra_ctrl0;
/**UDRA_IRQ_ENABLE (rw) register accessor: UDRA_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`udra_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udra_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:UDRA_IRQ_ENABLE)

For information about available fields see [`mod@udra_irq_enable`] module*/
pub type UDRA_IRQ_ENABLE = crate::Reg<udra_irq_enable::UDRA_IRQ_ENABLErs>;
///UDRA_IRQ_ENABLE register
pub mod udra_irq_enable;
/**UDRA_IRQ_STATUS (rw) register accessor: UDRA_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`udra_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udra_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:UDRA_IRQ_STATUS)

For information about available fields see [`mod@udra_irq_status`] module*/
pub type UDRA_IRQ_STATUS = crate::Reg<udra_irq_status::UDRA_IRQ_STATUSrs>;
///UDRA_IRQ_STATUS register
pub mod udra_irq_status;
/**UDRA_RADIO_CFG_PTR (r) register accessor: UDRA_RADIO_CFG_PTR register

You can [`read`](crate::Reg::read) this register and get [`udra_radio_cfg_ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:UDRA_RADIO_CFG_PTR)

For information about available fields see [`mod@udra_radio_cfg_ptr`] module*/
pub type UDRA_RADIO_CFG_PTR = crate::Reg<udra_radio_cfg_ptr::UDRA_RADIO_CFG_PTRrs>;
///UDRA_RADIO_CFG_PTR register
pub mod udra_radio_cfg_ptr;
/**SEMA_IRQ_ENABLE (rw) register accessor: SEMA_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`sema_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sema_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:SEMA_IRQ_ENABLE)

For information about available fields see [`mod@sema_irq_enable`] module*/
pub type SEMA_IRQ_ENABLE = crate::Reg<sema_irq_enable::SEMA_IRQ_ENABLErs>;
///SEMA_IRQ_ENABLE register
pub mod sema_irq_enable;
/**SEMA_IRQ_STATUS (rw) register accessor: SEMA_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`sema_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sema_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:SEMA_IRQ_STATUS)

For information about available fields see [`mod@sema_irq_status`] module*/
pub type SEMA_IRQ_STATUS = crate::Reg<sema_irq_status::SEMA_IRQ_STATUSrs>;
///SEMA_IRQ_STATUS register
pub mod sema_irq_status;
/**BLE_IRQ_ENABLE (rw) register accessor: BLE_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`ble_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:BLE_IRQ_ENABLE)

For information about available fields see [`mod@ble_irq_enable`] module*/
pub type BLE_IRQ_ENABLE = crate::Reg<ble_irq_enable::BLE_IRQ_ENABLErs>;
///BLE_IRQ_ENABLE register
pub mod ble_irq_enable;
/**BLE_IRQ_STATUS (rw) register accessor: BLE_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`ble_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:BLE_IRQ_STATUS)

For information about available fields see [`mod@ble_irq_status`] module*/
pub type BLE_IRQ_STATUS = crate::Reg<ble_irq_status::BLE_IRQ_STATUSrs>;
///BLE_IRQ_STATUS register
pub mod ble_irq_status;
/**VP_CPU_CMD_BUS (rw) register accessor: VP_CPU_CMD_BUS register

You can [`read`](crate::Reg::read) this register and get [`vp_cpu_cmd_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vp_cpu_cmd_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:VP_CPU_CMD_BUS)

For information about available fields see [`mod@vp_cpu_cmd_bus`] module*/
pub type VP_CPU_CMD_BUS = crate::Reg<vp_cpu_cmd_bus::VP_CPU_CMD_BUSrs>;
///VP_CPU_CMD_BUS register
pub mod vp_cpu_cmd_bus;
/**VP_CPU_SEMA_BUS (rw) register accessor: VP_CPU_SEMA_BUS register

You can [`read`](crate::Reg::read) this register and get [`vp_cpu_sema_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vp_cpu_sema_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:VP_CPU_SEMA_BUS)

For information about available fields see [`mod@vp_cpu_sema_bus`] module*/
pub type VP_CPU_SEMA_BUS = crate::Reg<vp_cpu_sema_bus::VP_CPU_SEMA_BUSrs>;
///VP_CPU_SEMA_BUS register
pub mod vp_cpu_sema_bus;
/**VP_CPU_IRQ_ENABLE (rw) register accessor: VP_CPU_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`vp_cpu_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vp_cpu_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:VP_CPU_IRQ_ENABLE)

For information about available fields see [`mod@vp_cpu_irq_enable`] module*/
pub type VP_CPU_IRQ_ENABLE = crate::Reg<vp_cpu_irq_enable::VP_CPU_IRQ_ENABLErs>;
///VP_CPU_IRQ_ENABLE register
pub mod vp_cpu_irq_enable;
/**VP_CPU_IRQ_STATUS (rw) register accessor: VP_CPU_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`vp_cpu_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vp_cpu_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:VP_CPU_IRQ_STATUS)

For information about available fields see [`mod@vp_cpu_irq_status`] module*/
pub type VP_CPU_IRQ_STATUS = crate::Reg<vp_cpu_irq_status::VP_CPU_IRQ_STATUSrs>;
///VP_CPU_IRQ_STATUS register
pub mod vp_cpu_irq_status;
