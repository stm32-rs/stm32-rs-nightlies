#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    offset: OFFSET,
    _reserved1: [u8; 0x04],
    absolute_time: ABSOLUTE_TIME,
    minimum_period_length: MINIMUM_PERIOD_LENGTH,
    average_period_length: AVERAGE_PERIOD_LENGTH,
    maximum_period_length: MAXIMUM_PERIOD_LENGTH,
    statistics_restart: STATISTICS_RESTART,
    blue_wakeup_time: BLUE_WAKEUP_TIME,
    blue_sleep_request_mode: BLUE_SLEEP_REQUEST_MODE,
    cm0_wakeup_time: CM0_WAKEUP_TIME,
    cm0_sleep_request_mode: CM0_SLEEP_REQUEST_MODE,
    _reserved10: [u8; 0x0c],
    ble_irq_enable: BLE_IRQ_ENABLE,
    ble_irq_status: BLE_IRQ_STATUS,
    cm0_irq_enable: CM0_IRQ_ENABLE,
    cm0_irq_status: CM0_IRQ_STATUS,
}
impl RegisterBlock {
    ///0x08 - WAKEUP_OFFSET register
    #[inline(always)]
    pub const fn offset(&self) -> &OFFSET {
        &self.offset
    }
    ///0x10 - ABSOLUTE_TIME register
    #[inline(always)]
    pub const fn absolute_time(&self) -> &ABSOLUTE_TIME {
        &self.absolute_time
    }
    ///0x14 - MINIMUM_PERIOD_LENGTH register
    #[inline(always)]
    pub const fn minimum_period_length(&self) -> &MINIMUM_PERIOD_LENGTH {
        &self.minimum_period_length
    }
    ///0x18 - AVERAGE_PERIOD_LENGTH register
    #[inline(always)]
    pub const fn average_period_length(&self) -> &AVERAGE_PERIOD_LENGTH {
        &self.average_period_length
    }
    ///0x1c - MAXIMUM_PERIOD_LENGTH register
    #[inline(always)]
    pub const fn maximum_period_length(&self) -> &MAXIMUM_PERIOD_LENGTH {
        &self.maximum_period_length
    }
    ///0x20 - STATISTICS_RESTART register
    #[inline(always)]
    pub const fn statistics_restart(&self) -> &STATISTICS_RESTART {
        &self.statistics_restart
    }
    ///0x24 - BLUE_WAKEUP_TIME register
    #[inline(always)]
    pub const fn blue_wakeup_time(&self) -> &BLUE_WAKEUP_TIME {
        &self.blue_wakeup_time
    }
    ///0x28 - BLUE_SLEEP_REQUEST_MODE register
    #[inline(always)]
    pub const fn blue_sleep_request_mode(&self) -> &BLUE_SLEEP_REQUEST_MODE {
        &self.blue_sleep_request_mode
    }
    ///0x2c - CM0_WAKEUP_TIME register
    #[inline(always)]
    pub const fn cm0_wakeup_time(&self) -> &CM0_WAKEUP_TIME {
        &self.cm0_wakeup_time
    }
    ///0x30 - CM0_SLEEP_REQUEST_MODE register
    #[inline(always)]
    pub const fn cm0_sleep_request_mode(&self) -> &CM0_SLEEP_REQUEST_MODE {
        &self.cm0_sleep_request_mode
    }
    ///0x40 - WAKEUP_BLE_IRQ_ENABLE register
    #[inline(always)]
    pub const fn ble_irq_enable(&self) -> &BLE_IRQ_ENABLE {
        &self.ble_irq_enable
    }
    ///0x44 - WAKEUP_BLE_IRQ_STATUS register
    #[inline(always)]
    pub const fn ble_irq_status(&self) -> &BLE_IRQ_STATUS {
        &self.ble_irq_status
    }
    ///0x48 - WAKEUP_CM0_IRQ_ENABLE register
    #[inline(always)]
    pub const fn cm0_irq_enable(&self) -> &CM0_IRQ_ENABLE {
        &self.cm0_irq_enable
    }
    ///0x4c - WAKEUP_CM0_IRQ_STATUS register
    #[inline(always)]
    pub const fn cm0_irq_status(&self) -> &CM0_IRQ_STATUS {
        &self.cm0_irq_status
    }
}
/**OFFSET (rw) register accessor: WAKEUP_OFFSET register

You can [`read`](crate::Reg::read) this register and get [`offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:OFFSET)

For information about available fields see [`mod@offset`] module*/
pub type OFFSET = crate::Reg<offset::OFFSETrs>;
///WAKEUP_OFFSET register
pub mod offset;
/**ABSOLUTE_TIME (r) register accessor: ABSOLUTE_TIME register

You can [`read`](crate::Reg::read) this register and get [`absolute_time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:ABSOLUTE_TIME)

For information about available fields see [`mod@absolute_time`] module*/
pub type ABSOLUTE_TIME = crate::Reg<absolute_time::ABSOLUTE_TIMErs>;
///ABSOLUTE_TIME register
pub mod absolute_time;
/**MINIMUM_PERIOD_LENGTH (r) register accessor: MINIMUM_PERIOD_LENGTH register

You can [`read`](crate::Reg::read) this register and get [`minimum_period_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:MINIMUM_PERIOD_LENGTH)

For information about available fields see [`mod@minimum_period_length`] module*/
pub type MINIMUM_PERIOD_LENGTH = crate::Reg<minimum_period_length::MINIMUM_PERIOD_LENGTHrs>;
///MINIMUM_PERIOD_LENGTH register
pub mod minimum_period_length;
/**AVERAGE_PERIOD_LENGTH (r) register accessor: AVERAGE_PERIOD_LENGTH register

You can [`read`](crate::Reg::read) this register and get [`average_period_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:AVERAGE_PERIOD_LENGTH)

For information about available fields see [`mod@average_period_length`] module*/
pub type AVERAGE_PERIOD_LENGTH = crate::Reg<average_period_length::AVERAGE_PERIOD_LENGTHrs>;
///AVERAGE_PERIOD_LENGTH register
pub mod average_period_length;
/**MAXIMUM_PERIOD_LENGTH (r) register accessor: MAXIMUM_PERIOD_LENGTH register

You can [`read`](crate::Reg::read) this register and get [`maximum_period_length::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:MAXIMUM_PERIOD_LENGTH)

For information about available fields see [`mod@maximum_period_length`] module*/
pub type MAXIMUM_PERIOD_LENGTH = crate::Reg<maximum_period_length::MAXIMUM_PERIOD_LENGTHrs>;
///MAXIMUM_PERIOD_LENGTH register
pub mod maximum_period_length;
/**STATISTICS_RESTART (rw) register accessor: STATISTICS_RESTART register

You can [`read`](crate::Reg::read) this register and get [`statistics_restart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statistics_restart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:STATISTICS_RESTART)

For information about available fields see [`mod@statistics_restart`] module*/
pub type STATISTICS_RESTART = crate::Reg<statistics_restart::STATISTICS_RESTARTrs>;
///STATISTICS_RESTART register
pub mod statistics_restart;
/**BLUE_WAKEUP_TIME (rw) register accessor: BLUE_WAKEUP_TIME register

You can [`read`](crate::Reg::read) this register and get [`blue_wakeup_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blue_wakeup_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:BLUE_WAKEUP_TIME)

For information about available fields see [`mod@blue_wakeup_time`] module*/
pub type BLUE_WAKEUP_TIME = crate::Reg<blue_wakeup_time::BLUE_WAKEUP_TIMErs>;
///BLUE_WAKEUP_TIME register
pub mod blue_wakeup_time;
/**BLUE_SLEEP_REQUEST_MODE (rw) register accessor: BLUE_SLEEP_REQUEST_MODE register

You can [`read`](crate::Reg::read) this register and get [`blue_sleep_request_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blue_sleep_request_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:BLUE_SLEEP_REQUEST_MODE)

For information about available fields see [`mod@blue_sleep_request_mode`] module*/
pub type BLUE_SLEEP_REQUEST_MODE = crate::Reg<blue_sleep_request_mode::BLUE_SLEEP_REQUEST_MODErs>;
///BLUE_SLEEP_REQUEST_MODE register
pub mod blue_sleep_request_mode;
/**CM0_WAKEUP_TIME (rw) register accessor: CM0_WAKEUP_TIME register

You can [`read`](crate::Reg::read) this register and get [`cm0_wakeup_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_wakeup_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:CM0_WAKEUP_TIME)

For information about available fields see [`mod@cm0_wakeup_time`] module*/
pub type CM0_WAKEUP_TIME = crate::Reg<cm0_wakeup_time::CM0_WAKEUP_TIMErs>;
///CM0_WAKEUP_TIME register
pub mod cm0_wakeup_time;
/**CM0_SLEEP_REQUEST_MODE (rw) register accessor: CM0_SLEEP_REQUEST_MODE register

You can [`read`](crate::Reg::read) this register and get [`cm0_sleep_request_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_sleep_request_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:CM0_SLEEP_REQUEST_MODE)

For information about available fields see [`mod@cm0_sleep_request_mode`] module*/
pub type CM0_SLEEP_REQUEST_MODE = crate::Reg<cm0_sleep_request_mode::CM0_SLEEP_REQUEST_MODErs>;
///CM0_SLEEP_REQUEST_MODE register
pub mod cm0_sleep_request_mode;
/**BLE_IRQ_ENABLE (rw) register accessor: WAKEUP_BLE_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`ble_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:BLE_IRQ_ENABLE)

For information about available fields see [`mod@ble_irq_enable`] module*/
pub type BLE_IRQ_ENABLE = crate::Reg<ble_irq_enable::BLE_IRQ_ENABLErs>;
///WAKEUP_BLE_IRQ_ENABLE register
pub mod ble_irq_enable;
/**BLE_IRQ_STATUS (rw) register accessor: WAKEUP_BLE_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`ble_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:BLE_IRQ_STATUS)

For information about available fields see [`mod@ble_irq_status`] module*/
pub type BLE_IRQ_STATUS = crate::Reg<ble_irq_status::BLE_IRQ_STATUSrs>;
///WAKEUP_BLE_IRQ_STATUS register
pub mod ble_irq_status;
/**CM0_IRQ_ENABLE (rw) register accessor: WAKEUP_CM0_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`cm0_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:CM0_IRQ_ENABLE)

For information about available fields see [`mod@cm0_irq_enable`] module*/
pub type CM0_IRQ_ENABLE = crate::Reg<cm0_irq_enable::CM0_IRQ_ENABLErs>;
///WAKEUP_CM0_IRQ_ENABLE register
pub mod cm0_irq_enable;
/**CM0_IRQ_STATUS (rw) register accessor: WAKEUP_CM0_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`cm0_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:CM0_IRQ_STATUS)

For information about available fields see [`mod@cm0_irq_status`] module*/
pub type CM0_IRQ_STATUS = crate::Reg<cm0_irq_status::CM0_IRQ_STATUSrs>;
///WAKEUP_CM0_IRQ_STATUS register
pub mod cm0_irq_status;
