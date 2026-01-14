#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rfseq_irq_status: RFSEQ_IRQ_STATUS,
    rfseq_status_detail: RFSEQ_STATUS_DETAIL,
    radio_fsm_info: RADIO_FSM_INFO,
    rx_indicator: RX_INDICATOR,
    rx_info_reg: RX_INFO_REG,
    rx_crc_reg: RX_CRC_REG,
    qi_info: QI_INFO,
    databuffer_info: DATABUFFER_INFO,
    time_capture: TIME_CAPTURE,
    iqc_correction_out: IQC_CORRECTION_OUT,
    pa_safeask_out: PA_SAFEASK_OUT,
    vco_calib_out: VCO_CALIB_OUT,
    seq_info: SEQ_INFO,
    seq_event_status: SEQ_EVENT_STATUS,
}
impl RegisterBlock {
    ///0x00 - RFSEQ_IRQ_STATUS register
    #[inline(always)]
    pub const fn rfseq_irq_status(&self) -> &RFSEQ_IRQ_STATUS {
        &self.rfseq_irq_status
    }
    ///0x04 - RFSEQ_STATUS_DETAIL register
    #[inline(always)]
    pub const fn rfseq_status_detail(&self) -> &RFSEQ_STATUS_DETAIL {
        &self.rfseq_status_detail
    }
    ///0x08 - RADIO_FSM_INFO register
    #[inline(always)]
    pub const fn radio_fsm_info(&self) -> &RADIO_FSM_INFO {
        &self.radio_fsm_info
    }
    ///0x0c - RX_INDICATOR register
    #[inline(always)]
    pub const fn rx_indicator(&self) -> &RX_INDICATOR {
        &self.rx_indicator
    }
    ///0x10 - RX_INFO_REG register
    #[inline(always)]
    pub const fn rx_info_reg(&self) -> &RX_INFO_REG {
        &self.rx_info_reg
    }
    ///0x14 - RX_CRC_REG register
    #[inline(always)]
    pub const fn rx_crc_reg(&self) -> &RX_CRC_REG {
        &self.rx_crc_reg
    }
    ///0x18 - QI_INFO register
    #[inline(always)]
    pub const fn qi_info(&self) -> &QI_INFO {
        &self.qi_info
    }
    ///0x1c - DATABUFFER_INFO register
    #[inline(always)]
    pub const fn databuffer_info(&self) -> &DATABUFFER_INFO {
        &self.databuffer_info
    }
    ///0x20 - TIME_CAPTURE register
    #[inline(always)]
    pub const fn time_capture(&self) -> &TIME_CAPTURE {
        &self.time_capture
    }
    ///0x24 - IQC_CORRECTION_OUT register
    #[inline(always)]
    pub const fn iqc_correction_out(&self) -> &IQC_CORRECTION_OUT {
        &self.iqc_correction_out
    }
    ///0x28 - PA_SAFEASK_OUT register
    #[inline(always)]
    pub const fn pa_safeask_out(&self) -> &PA_SAFEASK_OUT {
        &self.pa_safeask_out
    }
    ///0x2c - VCO_CALIB_OUT register
    #[inline(always)]
    pub const fn vco_calib_out(&self) -> &VCO_CALIB_OUT {
        &self.vco_calib_out
    }
    ///0x30 - SEQ_INFO register
    #[inline(always)]
    pub const fn seq_info(&self) -> &SEQ_INFO {
        &self.seq_info
    }
    ///0x34 - SEQ_EVENT_STATUS register
    #[inline(always)]
    pub const fn seq_event_status(&self) -> &SEQ_EVENT_STATUS {
        &self.seq_event_status
    }
}
/**RFSEQ_IRQ_STATUS (rw) register accessor: RFSEQ_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`rfseq_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RFSEQ_IRQ_STATUS)

For information about available fields see [`mod@rfseq_irq_status`] module*/
pub type RFSEQ_IRQ_STATUS = crate::Reg<rfseq_irq_status::RFSEQ_IRQ_STATUSrs>;
///RFSEQ_IRQ_STATUS register
pub mod rfseq_irq_status;
/**RFSEQ_STATUS_DETAIL (rw) register accessor: RFSEQ_STATUS_DETAIL register

You can [`read`](crate::Reg::read) this register and get [`rfseq_status_detail::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_status_detail::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RFSEQ_STATUS_DETAIL)

For information about available fields see [`mod@rfseq_status_detail`] module*/
pub type RFSEQ_STATUS_DETAIL = crate::Reg<rfseq_status_detail::RFSEQ_STATUS_DETAILrs>;
///RFSEQ_STATUS_DETAIL register
pub mod rfseq_status_detail;
/**RADIO_FSM_INFO (r) register accessor: RADIO_FSM_INFO register

You can [`read`](crate::Reg::read) this register and get [`radio_fsm_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RADIO_FSM_INFO)

For information about available fields see [`mod@radio_fsm_info`] module*/
pub type RADIO_FSM_INFO = crate::Reg<radio_fsm_info::RADIO_FSM_INFOrs>;
///RADIO_FSM_INFO register
pub mod radio_fsm_info;
/**RX_INDICATOR (r) register accessor: RX_INDICATOR register

You can [`read`](crate::Reg::read) this register and get [`rx_indicator::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RX_INDICATOR)

For information about available fields see [`mod@rx_indicator`] module*/
pub type RX_INDICATOR = crate::Reg<rx_indicator::RX_INDICATORrs>;
///RX_INDICATOR register
pub mod rx_indicator;
/**RX_INFO_REG (r) register accessor: RX_INFO_REG register

You can [`read`](crate::Reg::read) this register and get [`rx_info_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RX_INFO_REG)

For information about available fields see [`mod@rx_info_reg`] module*/
pub type RX_INFO_REG = crate::Reg<rx_info_reg::RX_INFO_REGrs>;
///RX_INFO_REG register
pub mod rx_info_reg;
/**RX_CRC_REG (r) register accessor: RX_CRC_REG register

You can [`read`](crate::Reg::read) this register and get [`rx_crc_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RX_CRC_REG)

For information about available fields see [`mod@rx_crc_reg`] module*/
pub type RX_CRC_REG = crate::Reg<rx_crc_reg::RX_CRC_REGrs>;
///RX_CRC_REG register
pub mod rx_crc_reg;
/**QI_INFO (r) register accessor: QI_INFO register

You can [`read`](crate::Reg::read) this register and get [`qi_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:QI_INFO)

For information about available fields see [`mod@qi_info`] module*/
pub type QI_INFO = crate::Reg<qi_info::QI_INFOrs>;
///QI_INFO register
pub mod qi_info;
/**DATABUFFER_INFO (r) register accessor: DATABUFFER_INFO register

You can [`read`](crate::Reg::read) this register and get [`databuffer_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:DATABUFFER_INFO)

For information about available fields see [`mod@databuffer_info`] module*/
pub type DATABUFFER_INFO = crate::Reg<databuffer_info::DATABUFFER_INFOrs>;
///DATABUFFER_INFO register
pub mod databuffer_info;
/**TIME_CAPTURE (r) register accessor: TIME_CAPTURE register

You can [`read`](crate::Reg::read) this register and get [`time_capture::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:TIME_CAPTURE)

For information about available fields see [`mod@time_capture`] module*/
pub type TIME_CAPTURE = crate::Reg<time_capture::TIME_CAPTURErs>;
///TIME_CAPTURE register
pub mod time_capture;
/**IQC_CORRECTION_OUT (r) register accessor: IQC_CORRECTION_OUT register

You can [`read`](crate::Reg::read) this register and get [`iqc_correction_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:IQC_CORRECTION_OUT)

For information about available fields see [`mod@iqc_correction_out`] module*/
pub type IQC_CORRECTION_OUT = crate::Reg<iqc_correction_out::IQC_CORRECTION_OUTrs>;
///IQC_CORRECTION_OUT register
pub mod iqc_correction_out;
/**PA_SAFEASK_OUT (r) register accessor: PA_SAFEASK_OUT register

You can [`read`](crate::Reg::read) this register and get [`pa_safeask_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:PA_SAFEASK_OUT)

For information about available fields see [`mod@pa_safeask_out`] module*/
pub type PA_SAFEASK_OUT = crate::Reg<pa_safeask_out::PA_SAFEASK_OUTrs>;
///PA_SAFEASK_OUT register
pub mod pa_safeask_out;
/**VCO_CALIB_OUT (r) register accessor: VCO_CALIB_OUT register

You can [`read`](crate::Reg::read) this register and get [`vco_calib_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:VCO_CALIB_OUT)

For information about available fields see [`mod@vco_calib_out`] module*/
pub type VCO_CALIB_OUT = crate::Reg<vco_calib_out::VCO_CALIB_OUTrs>;
///VCO_CALIB_OUT register
pub mod vco_calib_out;
/**SEQ_INFO (r) register accessor: SEQ_INFO register

You can [`read`](crate::Reg::read) this register and get [`seq_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:SEQ_INFO)

For information about available fields see [`mod@seq_info`] module*/
pub type SEQ_INFO = crate::Reg<seq_info::SEQ_INFOrs>;
///SEQ_INFO register
pub mod seq_info;
/**SEQ_EVENT_STATUS (r) register accessor: SEQ_EVENT_STATUS register

You can [`read`](crate::Reg::read) this register and get [`seq_event_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:SEQ_EVENT_STATUS)

For information about available fields see [`mod@seq_event_status`] module*/
pub type SEQ_EVENT_STATUS = crate::Reg<seq_event_status::SEQ_EVENT_STATUSrs>;
///SEQ_EVENT_STATUS register
pub mod seq_event_status;
