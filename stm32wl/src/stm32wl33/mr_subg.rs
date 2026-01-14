#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rf_fsm0_timeout: RF_FSM0_TIMEOUT,
    rf_fsm1_timeout: RF_FSM1_TIMEOUT,
    rf_fsm2_timeout: RF_FSM2_TIMEOUT,
    rf_fsm3_timeout: RF_FSM3_TIMEOUT,
    rf_fsm4_timeout: RF_FSM4_TIMEOUT,
    rf_fsm5_timeout: RF_FSM5_TIMEOUT,
    rf_fsm6_timeout: RF_FSM6_TIMEOUT,
    rf_fsm7_timeout: RF_FSM7_TIMEOUT,
    afc0_config: AFC0_CONFIG,
    afc1_config: AFC1_CONFIG,
    afc2_config: AFC2_CONFIG,
    afc3_config: AFC3_CONFIG,
    clkrec_ctrl0: CLKREC_CTRL0,
    clkrec_ctrl1: CLKREC_CTRL1,
    dcrem_ctrl0: DCREM_CTRL0,
    _reserved15: [u8; 0x04],
    iqc_ctrl0: IQC_CTRL0,
    iqc_ctrl1: IQC_CTRL1,
    iqc_ctrl2: IQC_CTRL2,
    iqc_ctrl3: IQC_CTRL3,
    agc_ana_eng: AGC_ANA_ENG,
    agc0_ctrl: AGC0_CTRL,
    agc1_ctrl: AGC1_CTRL,
    agc2_ctrl: AGC2_CTRL,
    agc3_ctrl: AGC3_CTRL,
    agc4_ctrl: AGC4_CTRL,
    _reserved25: [u8; 0x38],
    agc_pga_hwtrim_out: AGC_PGA_HWTRIM_OUT,
    _reserved26: [u8; 0x04],
    pa_reg: PA_REG,
    pa_hwtrim_out: PA_HWTRIM_OUT,
    _reserved28: [u8; 0x0c],
    rssi_flt: RSSI_FLT,
    _reserved29: [u8; 0x08],
    synth2_ana_eng: SYNTH2_ANA_ENG,
    _reserved30: [u8; 0x1c],
    rxadc_hwdelaytrim_out: RXADC_HWDELAYTRIM_OUT,
    _reserved31: [u8; 0x08],
    rx_aaf_hwtrim_out: RX_AAF_HWTRIM_OUT,
    _reserved32: [u8; 0x08],
    singen_ana_eng: SINGEN_ANA_ENG,
    _reserved33: [u8; 0x04],
    rf_info_out: RF_INFO_OUT,
    _reserved34: [u8; 0x18],
    rf_fsm8_timeout: RF_FSM8_TIMEOUT,
    rf_fsm9_timeout: RF_FSM9_TIMEOUT,
    rf_fsm10_timeout: RF_FSM10_TIMEOUT,
    _reserved37: [u8; 0x14],
    subg_dig_ctrl0: SUBG_DIG_CTRL0,
    rx_chain_eng: RX_CHAIN_ENG,
    demod_dig_eng: DEMOD_DIG_ENG,
}
impl RegisterBlock {
    ///0x00 - RF_FSM0_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm0_timeout(&self) -> &RF_FSM0_TIMEOUT {
        &self.rf_fsm0_timeout
    }
    ///0x04 - RF_FSM1_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm1_timeout(&self) -> &RF_FSM1_TIMEOUT {
        &self.rf_fsm1_timeout
    }
    ///0x08 - RF_FSM2_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm2_timeout(&self) -> &RF_FSM2_TIMEOUT {
        &self.rf_fsm2_timeout
    }
    ///0x0c - RF_FSM3_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm3_timeout(&self) -> &RF_FSM3_TIMEOUT {
        &self.rf_fsm3_timeout
    }
    ///0x10 - RF_FSM4_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm4_timeout(&self) -> &RF_FSM4_TIMEOUT {
        &self.rf_fsm4_timeout
    }
    ///0x14 - RF_FSM5_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm5_timeout(&self) -> &RF_FSM5_TIMEOUT {
        &self.rf_fsm5_timeout
    }
    ///0x18 - RF_FSM6_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm6_timeout(&self) -> &RF_FSM6_TIMEOUT {
        &self.rf_fsm6_timeout
    }
    ///0x1c - RF_FSM7_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm7_timeout(&self) -> &RF_FSM7_TIMEOUT {
        &self.rf_fsm7_timeout
    }
    ///0x20 - AFC0_CONFIG register
    #[inline(always)]
    pub const fn afc0_config(&self) -> &AFC0_CONFIG {
        &self.afc0_config
    }
    ///0x24 - AFC1_CONFIG register
    #[inline(always)]
    pub const fn afc1_config(&self) -> &AFC1_CONFIG {
        &self.afc1_config
    }
    ///0x28 - AFC2_CONFIG register
    #[inline(always)]
    pub const fn afc2_config(&self) -> &AFC2_CONFIG {
        &self.afc2_config
    }
    ///0x2c - AFC3_CONFIG register
    #[inline(always)]
    pub const fn afc3_config(&self) -> &AFC3_CONFIG {
        &self.afc3_config
    }
    ///0x30 - CLKREC_CTRL0 register
    #[inline(always)]
    pub const fn clkrec_ctrl0(&self) -> &CLKREC_CTRL0 {
        &self.clkrec_ctrl0
    }
    ///0x34 - CLKREC_CTRL1 register
    #[inline(always)]
    pub const fn clkrec_ctrl1(&self) -> &CLKREC_CTRL1 {
        &self.clkrec_ctrl1
    }
    ///0x38 - DCREM_CTRL0 register
    #[inline(always)]
    pub const fn dcrem_ctrl0(&self) -> &DCREM_CTRL0 {
        &self.dcrem_ctrl0
    }
    ///0x40 - IQC_CTRL0 register
    #[inline(always)]
    pub const fn iqc_ctrl0(&self) -> &IQC_CTRL0 {
        &self.iqc_ctrl0
    }
    ///0x44 - IQC_CTRL1 register
    #[inline(always)]
    pub const fn iqc_ctrl1(&self) -> &IQC_CTRL1 {
        &self.iqc_ctrl1
    }
    ///0x48 - IQC_CTRL2 register
    #[inline(always)]
    pub const fn iqc_ctrl2(&self) -> &IQC_CTRL2 {
        &self.iqc_ctrl2
    }
    ///0x4c - IQC_CTRL3 register
    #[inline(always)]
    pub const fn iqc_ctrl3(&self) -> &IQC_CTRL3 {
        &self.iqc_ctrl3
    }
    ///0x50 - AGC_ANA_ENG register
    #[inline(always)]
    pub const fn agc_ana_eng(&self) -> &AGC_ANA_ENG {
        &self.agc_ana_eng
    }
    ///0x54 - AGC0_CTRL register
    #[inline(always)]
    pub const fn agc0_ctrl(&self) -> &AGC0_CTRL {
        &self.agc0_ctrl
    }
    ///0x58 - AGC1_CTRL register
    #[inline(always)]
    pub const fn agc1_ctrl(&self) -> &AGC1_CTRL {
        &self.agc1_ctrl
    }
    ///0x5c - AGC2_CTRL register
    #[inline(always)]
    pub const fn agc2_ctrl(&self) -> &AGC2_CTRL {
        &self.agc2_ctrl
    }
    ///0x60 - AGC3_CTRL register
    #[inline(always)]
    pub const fn agc3_ctrl(&self) -> &AGC3_CTRL {
        &self.agc3_ctrl
    }
    ///0x64 - AGC4_CTRL register
    #[inline(always)]
    pub const fn agc4_ctrl(&self) -> &AGC4_CTRL {
        &self.agc4_ctrl
    }
    ///0xa0 - AGC_PGA_HWTRIM_OUT register
    #[inline(always)]
    pub const fn agc_pga_hwtrim_out(&self) -> &AGC_PGA_HWTRIM_OUT {
        &self.agc_pga_hwtrim_out
    }
    ///0xa8 - PA_REG register
    #[inline(always)]
    pub const fn pa_reg(&self) -> &PA_REG {
        &self.pa_reg
    }
    ///0xac - PA_HWTRIM_OUT register
    #[inline(always)]
    pub const fn pa_hwtrim_out(&self) -> &PA_HWTRIM_OUT {
        &self.pa_hwtrim_out
    }
    ///0xbc - RSSI_FLT register
    #[inline(always)]
    pub const fn rssi_flt(&self) -> &RSSI_FLT {
        &self.rssi_flt
    }
    ///0xc8 - SYNTH2_ANA_ENG register
    #[inline(always)]
    pub const fn synth2_ana_eng(&self) -> &SYNTH2_ANA_ENG {
        &self.synth2_ana_eng
    }
    ///0xe8 - RXADC_HWDELAYTRIM_OUT register
    #[inline(always)]
    pub const fn rxadc_hwdelaytrim_out(&self) -> &RXADC_HWDELAYTRIM_OUT {
        &self.rxadc_hwdelaytrim_out
    }
    ///0xf4 - RX_AAF_HWTRIM_OUT register
    #[inline(always)]
    pub const fn rx_aaf_hwtrim_out(&self) -> &RX_AAF_HWTRIM_OUT {
        &self.rx_aaf_hwtrim_out
    }
    ///0x100 - SINGEN_ANA_ENG register
    #[inline(always)]
    pub const fn singen_ana_eng(&self) -> &SINGEN_ANA_ENG {
        &self.singen_ana_eng
    }
    ///0x108 - RF_INFO_OUT register
    #[inline(always)]
    pub const fn rf_info_out(&self) -> &RF_INFO_OUT {
        &self.rf_info_out
    }
    ///0x124 - RF_FSM8_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm8_timeout(&self) -> &RF_FSM8_TIMEOUT {
        &self.rf_fsm8_timeout
    }
    ///0x128 - RF_FSM9_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm9_timeout(&self) -> &RF_FSM9_TIMEOUT {
        &self.rf_fsm9_timeout
    }
    ///0x12c - RF_FSM10_TIMEOUT register
    #[inline(always)]
    pub const fn rf_fsm10_timeout(&self) -> &RF_FSM10_TIMEOUT {
        &self.rf_fsm10_timeout
    }
    ///0x144 - SUBG_DIG_CTRL0 register
    #[inline(always)]
    pub const fn subg_dig_ctrl0(&self) -> &SUBG_DIG_CTRL0 {
        &self.subg_dig_ctrl0
    }
    ///0x148 - RX_CHAIN_ENG register
    #[inline(always)]
    pub const fn rx_chain_eng(&self) -> &RX_CHAIN_ENG {
        &self.rx_chain_eng
    }
    ///0x14c - DEMOD_DIG_ENG register
    #[inline(always)]
    pub const fn demod_dig_eng(&self) -> &DEMOD_DIG_ENG {
        &self.demod_dig_eng
    }
}
/**RF_FSM0_TIMEOUT (rw) register accessor: RF_FSM0_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm0_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm0_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM0_TIMEOUT)

For information about available fields see [`mod@rf_fsm0_timeout`] module*/
pub type RF_FSM0_TIMEOUT = crate::Reg<rf_fsm0_timeout::RF_FSM0_TIMEOUTrs>;
///RF_FSM0_TIMEOUT register
pub mod rf_fsm0_timeout;
/**RF_FSM1_TIMEOUT (rw) register accessor: RF_FSM1_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm1_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm1_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM1_TIMEOUT)

For information about available fields see [`mod@rf_fsm1_timeout`] module*/
pub type RF_FSM1_TIMEOUT = crate::Reg<rf_fsm1_timeout::RF_FSM1_TIMEOUTrs>;
///RF_FSM1_TIMEOUT register
pub mod rf_fsm1_timeout;
/**RF_FSM2_TIMEOUT (rw) register accessor: RF_FSM2_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm2_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm2_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM2_TIMEOUT)

For information about available fields see [`mod@rf_fsm2_timeout`] module*/
pub type RF_FSM2_TIMEOUT = crate::Reg<rf_fsm2_timeout::RF_FSM2_TIMEOUTrs>;
///RF_FSM2_TIMEOUT register
pub mod rf_fsm2_timeout;
/**RF_FSM3_TIMEOUT (rw) register accessor: RF_FSM3_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm3_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm3_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM3_TIMEOUT)

For information about available fields see [`mod@rf_fsm3_timeout`] module*/
pub type RF_FSM3_TIMEOUT = crate::Reg<rf_fsm3_timeout::RF_FSM3_TIMEOUTrs>;
///RF_FSM3_TIMEOUT register
pub mod rf_fsm3_timeout;
/**RF_FSM4_TIMEOUT (rw) register accessor: RF_FSM4_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm4_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm4_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM4_TIMEOUT)

For information about available fields see [`mod@rf_fsm4_timeout`] module*/
pub type RF_FSM4_TIMEOUT = crate::Reg<rf_fsm4_timeout::RF_FSM4_TIMEOUTrs>;
///RF_FSM4_TIMEOUT register
pub mod rf_fsm4_timeout;
/**RF_FSM5_TIMEOUT (rw) register accessor: RF_FSM5_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm5_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm5_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM5_TIMEOUT)

For information about available fields see [`mod@rf_fsm5_timeout`] module*/
pub type RF_FSM5_TIMEOUT = crate::Reg<rf_fsm5_timeout::RF_FSM5_TIMEOUTrs>;
///RF_FSM5_TIMEOUT register
pub mod rf_fsm5_timeout;
/**RF_FSM6_TIMEOUT (rw) register accessor: RF_FSM6_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm6_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm6_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM6_TIMEOUT)

For information about available fields see [`mod@rf_fsm6_timeout`] module*/
pub type RF_FSM6_TIMEOUT = crate::Reg<rf_fsm6_timeout::RF_FSM6_TIMEOUTrs>;
///RF_FSM6_TIMEOUT register
pub mod rf_fsm6_timeout;
/**RF_FSM7_TIMEOUT (rw) register accessor: RF_FSM7_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm7_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm7_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM7_TIMEOUT)

For information about available fields see [`mod@rf_fsm7_timeout`] module*/
pub type RF_FSM7_TIMEOUT = crate::Reg<rf_fsm7_timeout::RF_FSM7_TIMEOUTrs>;
///RF_FSM7_TIMEOUT register
pub mod rf_fsm7_timeout;
/**AFC0_CONFIG (rw) register accessor: AFC0_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc0_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc0_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC0_CONFIG)

For information about available fields see [`mod@afc0_config`] module*/
pub type AFC0_CONFIG = crate::Reg<afc0_config::AFC0_CONFIGrs>;
///AFC0_CONFIG register
pub mod afc0_config;
/**AFC1_CONFIG (rw) register accessor: AFC1_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc1_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc1_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC1_CONFIG)

For information about available fields see [`mod@afc1_config`] module*/
pub type AFC1_CONFIG = crate::Reg<afc1_config::AFC1_CONFIGrs>;
///AFC1_CONFIG register
pub mod afc1_config;
/**AFC2_CONFIG (rw) register accessor: AFC2_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc2_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc2_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC2_CONFIG)

For information about available fields see [`mod@afc2_config`] module*/
pub type AFC2_CONFIG = crate::Reg<afc2_config::AFC2_CONFIGrs>;
///AFC2_CONFIG register
pub mod afc2_config;
/**AFC3_CONFIG (rw) register accessor: AFC3_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc3_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc3_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC3_CONFIG)

For information about available fields see [`mod@afc3_config`] module*/
pub type AFC3_CONFIG = crate::Reg<afc3_config::AFC3_CONFIGrs>;
///AFC3_CONFIG register
pub mod afc3_config;
/**CLKREC_CTRL0 (rw) register accessor: CLKREC_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:CLKREC_CTRL0)

For information about available fields see [`mod@clkrec_ctrl0`] module*/
pub type CLKREC_CTRL0 = crate::Reg<clkrec_ctrl0::CLKREC_CTRL0rs>;
///CLKREC_CTRL0 register
pub mod clkrec_ctrl0;
/**CLKREC_CTRL1 (rw) register accessor: CLKREC_CTRL1 register

You can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:CLKREC_CTRL1)

For information about available fields see [`mod@clkrec_ctrl1`] module*/
pub type CLKREC_CTRL1 = crate::Reg<clkrec_ctrl1::CLKREC_CTRL1rs>;
///CLKREC_CTRL1 register
pub mod clkrec_ctrl1;
/**DCREM_CTRL0 (rw) register accessor: DCREM_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`dcrem_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrem_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:DCREM_CTRL0)

For information about available fields see [`mod@dcrem_ctrl0`] module*/
pub type DCREM_CTRL0 = crate::Reg<dcrem_ctrl0::DCREM_CTRL0rs>;
///DCREM_CTRL0 register
pub mod dcrem_ctrl0;
/**IQC_CTRL0 (rw) register accessor: IQC_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL0)

For information about available fields see [`mod@iqc_ctrl0`] module*/
pub type IQC_CTRL0 = crate::Reg<iqc_ctrl0::IQC_CTRL0rs>;
///IQC_CTRL0 register
pub mod iqc_ctrl0;
/**IQC_CTRL1 (rw) register accessor: IQC_CTRL1 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL1)

For information about available fields see [`mod@iqc_ctrl1`] module*/
pub type IQC_CTRL1 = crate::Reg<iqc_ctrl1::IQC_CTRL1rs>;
///IQC_CTRL1 register
pub mod iqc_ctrl1;
/**IQC_CTRL2 (rw) register accessor: IQC_CTRL2 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL2)

For information about available fields see [`mod@iqc_ctrl2`] module*/
pub type IQC_CTRL2 = crate::Reg<iqc_ctrl2::IQC_CTRL2rs>;
///IQC_CTRL2 register
pub mod iqc_ctrl2;
/**IQC_CTRL3 (rw) register accessor: IQC_CTRL3 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL3)

For information about available fields see [`mod@iqc_ctrl3`] module*/
pub type IQC_CTRL3 = crate::Reg<iqc_ctrl3::IQC_CTRL3rs>;
///IQC_CTRL3 register
pub mod iqc_ctrl3;
/**AGC_ANA_ENG (rw) register accessor: AGC_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC_ANA_ENG)

For information about available fields see [`mod@agc_ana_eng`] module*/
pub type AGC_ANA_ENG = crate::Reg<agc_ana_eng::AGC_ANA_ENGrs>;
///AGC_ANA_ENG register
pub mod agc_ana_eng;
/**AGC0_CTRL (rw) register accessor: AGC0_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC0_CTRL)

For information about available fields see [`mod@agc0_ctrl`] module*/
pub type AGC0_CTRL = crate::Reg<agc0_ctrl::AGC0_CTRLrs>;
///AGC0_CTRL register
pub mod agc0_ctrl;
/**AGC1_CTRL (rw) register accessor: AGC1_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC1_CTRL)

For information about available fields see [`mod@agc1_ctrl`] module*/
pub type AGC1_CTRL = crate::Reg<agc1_ctrl::AGC1_CTRLrs>;
///AGC1_CTRL register
pub mod agc1_ctrl;
/**AGC2_CTRL (rw) register accessor: AGC2_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC2_CTRL)

For information about available fields see [`mod@agc2_ctrl`] module*/
pub type AGC2_CTRL = crate::Reg<agc2_ctrl::AGC2_CTRLrs>;
///AGC2_CTRL register
pub mod agc2_ctrl;
/**AGC3_CTRL (rw) register accessor: AGC3_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC3_CTRL)

For information about available fields see [`mod@agc3_ctrl`] module*/
pub type AGC3_CTRL = crate::Reg<agc3_ctrl::AGC3_CTRLrs>;
///AGC3_CTRL register
pub mod agc3_ctrl;
/**AGC4_CTRL (rw) register accessor: AGC4_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC4_CTRL)

For information about available fields see [`mod@agc4_ctrl`] module*/
pub type AGC4_CTRL = crate::Reg<agc4_ctrl::AGC4_CTRLrs>;
///AGC4_CTRL register
pub mod agc4_ctrl;
/**AGC_PGA_HWTRIM_OUT (r) register accessor: AGC_PGA_HWTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`agc_pga_hwtrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC_PGA_HWTRIM_OUT)

For information about available fields see [`mod@agc_pga_hwtrim_out`] module*/
pub type AGC_PGA_HWTRIM_OUT = crate::Reg<agc_pga_hwtrim_out::AGC_PGA_HWTRIM_OUTrs>;
///AGC_PGA_HWTRIM_OUT register
pub mod agc_pga_hwtrim_out;
/**PA_REG (rw) register accessor: PA_REG register

You can [`read`](crate::Reg::read) this register and get [`pa_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:PA_REG)

For information about available fields see [`mod@pa_reg`] module*/
pub type PA_REG = crate::Reg<pa_reg::PA_REGrs>;
///PA_REG register
pub mod pa_reg;
/**PA_HWTRIM_OUT (r) register accessor: PA_HWTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`pa_hwtrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:PA_HWTRIM_OUT)

For information about available fields see [`mod@pa_hwtrim_out`] module*/
pub type PA_HWTRIM_OUT = crate::Reg<pa_hwtrim_out::PA_HWTRIM_OUTrs>;
///PA_HWTRIM_OUT register
pub mod pa_hwtrim_out;
/**RSSI_FLT (rw) register accessor: RSSI_FLT register

You can [`read`](crate::Reg::read) this register and get [`rssi_flt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rssi_flt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RSSI_FLT)

For information about available fields see [`mod@rssi_flt`] module*/
pub type RSSI_FLT = crate::Reg<rssi_flt::RSSI_FLTrs>;
///RSSI_FLT register
pub mod rssi_flt;
/**SYNTH2_ANA_ENG (rw) register accessor: SYNTH2_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`synth2_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synth2_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:SYNTH2_ANA_ENG)

For information about available fields see [`mod@synth2_ana_eng`] module*/
pub type SYNTH2_ANA_ENG = crate::Reg<synth2_ana_eng::SYNTH2_ANA_ENGrs>;
///SYNTH2_ANA_ENG register
pub mod synth2_ana_eng;
/**RXADC_HWDELAYTRIM_OUT (r) register accessor: RXADC_HWDELAYTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`rxadc_hwdelaytrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RXADC_HWDELAYTRIM_OUT)

For information about available fields see [`mod@rxadc_hwdelaytrim_out`] module*/
pub type RXADC_HWDELAYTRIM_OUT = crate::Reg<rxadc_hwdelaytrim_out::RXADC_HWDELAYTRIM_OUTrs>;
///RXADC_HWDELAYTRIM_OUT register
pub mod rxadc_hwdelaytrim_out;
/**RX_AAF_HWTRIM_OUT (r) register accessor: RX_AAF_HWTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`rx_aaf_hwtrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RX_AAF_HWTRIM_OUT)

For information about available fields see [`mod@rx_aaf_hwtrim_out`] module*/
pub type RX_AAF_HWTRIM_OUT = crate::Reg<rx_aaf_hwtrim_out::RX_AAF_HWTRIM_OUTrs>;
///RX_AAF_HWTRIM_OUT register
pub mod rx_aaf_hwtrim_out;
/**SINGEN_ANA_ENG (rw) register accessor: SINGEN_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`singen_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:SINGEN_ANA_ENG)

For information about available fields see [`mod@singen_ana_eng`] module*/
pub type SINGEN_ANA_ENG = crate::Reg<singen_ana_eng::SINGEN_ANA_ENGrs>;
///SINGEN_ANA_ENG register
pub mod singen_ana_eng;
/**RF_INFO_OUT (r) register accessor: RF_INFO_OUT register

You can [`read`](crate::Reg::read) this register and get [`rf_info_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_INFO_OUT)

For information about available fields see [`mod@rf_info_out`] module*/
pub type RF_INFO_OUT = crate::Reg<rf_info_out::RF_INFO_OUTrs>;
///RF_INFO_OUT register
pub mod rf_info_out;
/**RF_FSM8_TIMEOUT (rw) register accessor: RF_FSM8_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm8_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm8_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM8_TIMEOUT)

For information about available fields see [`mod@rf_fsm8_timeout`] module*/
pub type RF_FSM8_TIMEOUT = crate::Reg<rf_fsm8_timeout::RF_FSM8_TIMEOUTrs>;
///RF_FSM8_TIMEOUT register
pub mod rf_fsm8_timeout;
/**RF_FSM9_TIMEOUT (rw) register accessor: RF_FSM9_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm9_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm9_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM9_TIMEOUT)

For information about available fields see [`mod@rf_fsm9_timeout`] module*/
pub type RF_FSM9_TIMEOUT = crate::Reg<rf_fsm9_timeout::RF_FSM9_TIMEOUTrs>;
///RF_FSM9_TIMEOUT register
pub mod rf_fsm9_timeout;
/**RF_FSM10_TIMEOUT (rw) register accessor: RF_FSM10_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm10_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm10_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM10_TIMEOUT)

For information about available fields see [`mod@rf_fsm10_timeout`] module*/
pub type RF_FSM10_TIMEOUT = crate::Reg<rf_fsm10_timeout::RF_FSM10_TIMEOUTrs>;
///RF_FSM10_TIMEOUT register
pub mod rf_fsm10_timeout;
/**SUBG_DIG_CTRL0 (rw) register accessor: SUBG_DIG_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`subg_dig_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subg_dig_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:SUBG_DIG_CTRL0)

For information about available fields see [`mod@subg_dig_ctrl0`] module*/
pub type SUBG_DIG_CTRL0 = crate::Reg<subg_dig_ctrl0::SUBG_DIG_CTRL0rs>;
///SUBG_DIG_CTRL0 register
pub mod subg_dig_ctrl0;
/**RX_CHAIN_ENG (rw) register accessor: RX_CHAIN_ENG register

You can [`read`](crate::Reg::read) this register and get [`rx_chain_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_chain_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RX_CHAIN_ENG)

For information about available fields see [`mod@rx_chain_eng`] module*/
pub type RX_CHAIN_ENG = crate::Reg<rx_chain_eng::RX_CHAIN_ENGrs>;
///RX_CHAIN_ENG register
pub mod rx_chain_eng;
/**DEMOD_DIG_ENG (rw) register accessor: DEMOD_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`demod_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demod_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:DEMOD_DIG_ENG)

For information about available fields see [`mod@demod_dig_eng`] module*/
pub type DEMOD_DIG_ENG = crate::Reg<demod_dig_eng::DEMOD_DIG_ENGrs>;
///DEMOD_DIG_ENG register
pub mod demod_dig_eng;
