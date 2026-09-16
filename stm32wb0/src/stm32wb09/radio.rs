#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    aa0_dig_usr: AA0_DIG_USR,
    aa1_dig_usr: AA1_DIG_USR,
    aa2_dig_usr: AA2_DIG_USR,
    aa3_dig_usr: AA3_DIG_USR,
    dem_mod_dig_usr: DEM_MOD_DIG_USR,
    fsm_usr: FSM_USR,
    phyctrl_dig_usr: PHYCTRL_DIG_USR,
    _reserved7: [u8; 0x2c],
    afc1_dig_eng: AFC1_DIG_ENG,
    _reserved8: [u8; 0x08],
    cr0_dig_eng: CR0_DIG_ENG,
    _reserved9: [u8; 0x10],
    cr0_lr: CR0_LR,
    vit_conf_dig_eng: VIT_CONF_DIG_ENG,
    _reserved11: [u8; 0x14],
    lr_pd_thr_dig_eng: LR_PD_THR_DIG_ENG,
    lr_rssi_thr_dig_eng: LR_RSSI_THR_DIG_ENG,
    lr_aac_thr_dig_eng: LR_AAC_THR_DIG_ENG,
    _reserved14: [u8; 0x18],
    synthcal0_dig_eng: SYNTHCAL0_DIG_ENG,
    _reserved15: [u8; 0x44],
    dtb5_dig_eng: DTB5_DIG_ENG,
    _reserved16: [u8; 0x54],
    rxadc_ana_usr: RXADC_ANA_USR,
    _reserved17: [u8; 0x08],
    ldo_ana_eng: LDO_ANA_ENG,
    _reserved18: [u8; 0x1c],
    cbias0_ana_eng: CBIAS0_ANA_ENG,
    cbias1_ana_eng: CBIAS1_ANA_ENG,
    _reserved20: [u8; 0x04],
    synthcal0_dig_out: SYNTHCAL0_DIG_OUT,
    synthcal1_dig_out: SYNTHCAL1_DIG_OUT,
    synthcal2_dig_out: SYNTHCAL2_DIG_OUT,
    synthcal3_dig_out: SYNTHCAL3_DIG_OUT,
    synthcal4_dig_out: SYNTHCAL4_DIG_OUT,
    synthcal5_dig_out: SYNTHCAL5_DIG_OUT,
    fsm_status_dig_out: FSM_STATUS_DIG_OUT,
    _reserved27: [u8; 0x08],
    rssi0_dig_out: RSSI0_DIG_OUT,
    rssi1_dig_out: RSSI1_DIG_OUT,
    agc_dig_out: AGC_DIG_OUT,
    demod_dig_out: DEMOD_DIG_OUT,
    _reserved31: [u8; 0x08],
    agc2_ana_tst: AGC2_ANA_TST,
    agc0_dig_eng: AGC0_DIG_ENG,
    agc1_dig_eng: AGC1_DIG_ENG,
    _reserved34: [u8; 0x20],
    agc10_dig_eng: AGC10_DIG_ENG,
    agc11_dig_eng: AGC11_DIG_ENG,
    agc12_dig_eng: AGC12_DIG_ENG,
    agc13_dig_eng: AGC13_DIG_ENG,
    agc14_dig_eng: AGC14_DIG_ENG,
    agc15_dig_eng: AGC15_DIG_ENG,
    agc16_dig_eng: AGC16_DIG_ENG,
    agc17_dig_eng: AGC17_DIG_ENG,
    agc18_dig_eng: AGC18_DIG_ENG,
    agc19_dig_eng: AGC19_DIG_ENG,
    _reserved44: [u8; 0x14],
    rxadc_hw_trim_out: RXADC_HW_TRIM_OUT,
    cbias0_hw_trim_out: CBIAS0_HW_TRIM_OUT,
    _reserved46: [u8; 0x04],
    agc_hw_trim_out: AGC_HW_TRIM_OUT,
    _reserved47: [u8; 0x08],
    demod_iq2_dig_tst: DEMOD_IQ2_DIG_TST,
    antsw0_dig_usr: ANTSW0_DIG_USR,
    antsw1_dig_usr: ANTSW1_DIG_USR,
    antsw2_dig_usr: ANTSW2_DIG_USR,
    antsw3_dig_usr: ANTSW3_DIG_USR,
}
impl RegisterBlock {
    ///0x00 - AA0_DIG_USR register
    #[inline(always)]
    pub const fn aa0_dig_usr(&self) -> &AA0_DIG_USR {
        &self.aa0_dig_usr
    }
    ///0x04 - AA1_DIG_USR register
    #[inline(always)]
    pub const fn aa1_dig_usr(&self) -> &AA1_DIG_USR {
        &self.aa1_dig_usr
    }
    ///0x08 - AA2_DIG_USR register
    #[inline(always)]
    pub const fn aa2_dig_usr(&self) -> &AA2_DIG_USR {
        &self.aa2_dig_usr
    }
    ///0x0c - AA3_DIG_USR register
    #[inline(always)]
    pub const fn aa3_dig_usr(&self) -> &AA3_DIG_USR {
        &self.aa3_dig_usr
    }
    ///0x10 - DEM_MOD_DIG_USR register
    #[inline(always)]
    pub const fn dem_mod_dig_usr(&self) -> &DEM_MOD_DIG_USR {
        &self.dem_mod_dig_usr
    }
    ///0x14 - RADIO_FSM_USR register
    #[inline(always)]
    pub const fn fsm_usr(&self) -> &FSM_USR {
        &self.fsm_usr
    }
    ///0x18 - PHYCTRL_DIG_USR register
    #[inline(always)]
    pub const fn phyctrl_dig_usr(&self) -> &PHYCTRL_DIG_USR {
        &self.phyctrl_dig_usr
    }
    ///0x48 - AFC1_DIG_ENG register
    #[inline(always)]
    pub const fn afc1_dig_eng(&self) -> &AFC1_DIG_ENG {
        &self.afc1_dig_eng
    }
    ///0x54 - CR0_DIG_ENG register
    #[inline(always)]
    pub const fn cr0_dig_eng(&self) -> &CR0_DIG_ENG {
        &self.cr0_dig_eng
    }
    ///0x68 - CR0_LR register
    #[inline(always)]
    pub const fn cr0_lr(&self) -> &CR0_LR {
        &self.cr0_lr
    }
    ///0x6c - VIT_CONF_DIG_ENG register
    #[inline(always)]
    pub const fn vit_conf_dig_eng(&self) -> &VIT_CONF_DIG_ENG {
        &self.vit_conf_dig_eng
    }
    ///0x84 - LR_PD_THR_DIG_ENG register
    #[inline(always)]
    pub const fn lr_pd_thr_dig_eng(&self) -> &LR_PD_THR_DIG_ENG {
        &self.lr_pd_thr_dig_eng
    }
    ///0x88 - LR_RSSI_THR_DIG_ENG register
    #[inline(always)]
    pub const fn lr_rssi_thr_dig_eng(&self) -> &LR_RSSI_THR_DIG_ENG {
        &self.lr_rssi_thr_dig_eng
    }
    ///0x8c - LR_AAC_THR_DIG_ENG register
    #[inline(always)]
    pub const fn lr_aac_thr_dig_eng(&self) -> &LR_AAC_THR_DIG_ENG {
        &self.lr_aac_thr_dig_eng
    }
    ///0xa8 - SYNTHCAL0_DIG_ENG register
    #[inline(always)]
    pub const fn synthcal0_dig_eng(&self) -> &SYNTHCAL0_DIG_ENG {
        &self.synthcal0_dig_eng
    }
    ///0xf0 - DTB5_DIG_ENG register
    #[inline(always)]
    pub const fn dtb5_dig_eng(&self) -> &DTB5_DIG_ENG {
        &self.dtb5_dig_eng
    }
    ///0x148 - RXADC_ANA_USR register
    #[inline(always)]
    pub const fn rxadc_ana_usr(&self) -> &RXADC_ANA_USR {
        &self.rxadc_ana_usr
    }
    ///0x154 - LDO_ANA_ENG register
    #[inline(always)]
    pub const fn ldo_ana_eng(&self) -> &LDO_ANA_ENG {
        &self.ldo_ana_eng
    }
    ///0x174 - CBIAS0_ANA_ENG register
    #[inline(always)]
    pub const fn cbias0_ana_eng(&self) -> &CBIAS0_ANA_ENG {
        &self.cbias0_ana_eng
    }
    ///0x178 - CBIAS1_ANA_ENG register
    #[inline(always)]
    pub const fn cbias1_ana_eng(&self) -> &CBIAS1_ANA_ENG {
        &self.cbias1_ana_eng
    }
    ///0x180 - SYNTHCAL0_DIG_OUT register
    #[inline(always)]
    pub const fn synthcal0_dig_out(&self) -> &SYNTHCAL0_DIG_OUT {
        &self.synthcal0_dig_out
    }
    ///0x184 - SYNTHCAL1_DIG_OUT register
    #[inline(always)]
    pub const fn synthcal1_dig_out(&self) -> &SYNTHCAL1_DIG_OUT {
        &self.synthcal1_dig_out
    }
    ///0x188 - SYNTHCAL2_DIG_OUT register
    #[inline(always)]
    pub const fn synthcal2_dig_out(&self) -> &SYNTHCAL2_DIG_OUT {
        &self.synthcal2_dig_out
    }
    ///0x18c - SYNTHCAL3_DIG_OUT register
    #[inline(always)]
    pub const fn synthcal3_dig_out(&self) -> &SYNTHCAL3_DIG_OUT {
        &self.synthcal3_dig_out
    }
    ///0x190 - SYNTHCAL4_DIG_OUT register
    #[inline(always)]
    pub const fn synthcal4_dig_out(&self) -> &SYNTHCAL4_DIG_OUT {
        &self.synthcal4_dig_out
    }
    ///0x194 - SYNTHCAL5_DIG_OUT register
    #[inline(always)]
    pub const fn synthcal5_dig_out(&self) -> &SYNTHCAL5_DIG_OUT {
        &self.synthcal5_dig_out
    }
    ///0x198 - FSM_STATUS_DIG_OUT register
    #[inline(always)]
    pub const fn fsm_status_dig_out(&self) -> &FSM_STATUS_DIG_OUT {
        &self.fsm_status_dig_out
    }
    ///0x1a4 - RSSI0_DIG_OUT register
    #[inline(always)]
    pub const fn rssi0_dig_out(&self) -> &RSSI0_DIG_OUT {
        &self.rssi0_dig_out
    }
    ///0x1a8 - RSSI1_DIG_OUT register
    #[inline(always)]
    pub const fn rssi1_dig_out(&self) -> &RSSI1_DIG_OUT {
        &self.rssi1_dig_out
    }
    ///0x1ac - AGC_DIG_OUT register
    #[inline(always)]
    pub const fn agc_dig_out(&self) -> &AGC_DIG_OUT {
        &self.agc_dig_out
    }
    ///0x1b0 - DEMOD_DIG_OUT register
    #[inline(always)]
    pub const fn demod_dig_out(&self) -> &DEMOD_DIG_OUT {
        &self.demod_dig_out
    }
    ///0x1bc - AGC2_ANA_TST register
    #[inline(always)]
    pub const fn agc2_ana_tst(&self) -> &AGC2_ANA_TST {
        &self.agc2_ana_tst
    }
    ///0x1c0 - AGC0_DIG_ENG register
    #[inline(always)]
    pub const fn agc0_dig_eng(&self) -> &AGC0_DIG_ENG {
        &self.agc0_dig_eng
    }
    ///0x1c4 - AGC1_DIG_ENG register
    #[inline(always)]
    pub const fn agc1_dig_eng(&self) -> &AGC1_DIG_ENG {
        &self.agc1_dig_eng
    }
    ///0x1e8 - AGC10_DIG_ENG register
    #[inline(always)]
    pub const fn agc10_dig_eng(&self) -> &AGC10_DIG_ENG {
        &self.agc10_dig_eng
    }
    ///0x1ec - AGC11_DIG_ENG register
    #[inline(always)]
    pub const fn agc11_dig_eng(&self) -> &AGC11_DIG_ENG {
        &self.agc11_dig_eng
    }
    ///0x1f0 - AGC12_DIG_ENG register
    #[inline(always)]
    pub const fn agc12_dig_eng(&self) -> &AGC12_DIG_ENG {
        &self.agc12_dig_eng
    }
    ///0x1f4 - AGC13_DIG_ENG register
    #[inline(always)]
    pub const fn agc13_dig_eng(&self) -> &AGC13_DIG_ENG {
        &self.agc13_dig_eng
    }
    ///0x1f8 - AGC14_DIG_ENG register
    #[inline(always)]
    pub const fn agc14_dig_eng(&self) -> &AGC14_DIG_ENG {
        &self.agc14_dig_eng
    }
    ///0x1fc - AGC15_DIG_ENG register
    #[inline(always)]
    pub const fn agc15_dig_eng(&self) -> &AGC15_DIG_ENG {
        &self.agc15_dig_eng
    }
    ///0x200 - AGC16_DIG_ENG register
    #[inline(always)]
    pub const fn agc16_dig_eng(&self) -> &AGC16_DIG_ENG {
        &self.agc16_dig_eng
    }
    ///0x204 - AGC17_DIG_ENG register
    #[inline(always)]
    pub const fn agc17_dig_eng(&self) -> &AGC17_DIG_ENG {
        &self.agc17_dig_eng
    }
    ///0x208 - AGC18_DIG_ENG register
    #[inline(always)]
    pub const fn agc18_dig_eng(&self) -> &AGC18_DIG_ENG {
        &self.agc18_dig_eng
    }
    ///0x20c - AGC19_DIG_ENG register
    #[inline(always)]
    pub const fn agc19_dig_eng(&self) -> &AGC19_DIG_ENG {
        &self.agc19_dig_eng
    }
    ///0x224 - RXADC_HW_TRIM_OUT register
    #[inline(always)]
    pub const fn rxadc_hw_trim_out(&self) -> &RXADC_HW_TRIM_OUT {
        &self.rxadc_hw_trim_out
    }
    ///0x228 - CBIAS0_HW_TRIM_OUT register
    #[inline(always)]
    pub const fn cbias0_hw_trim_out(&self) -> &CBIAS0_HW_TRIM_OUT {
        &self.cbias0_hw_trim_out
    }
    ///0x230 - AGC_HW_TRIM_OUT register
    #[inline(always)]
    pub const fn agc_hw_trim_out(&self) -> &AGC_HW_TRIM_OUT {
        &self.agc_hw_trim_out
    }
    ///0x23c - DEMOD_IQ2_DIG_TST register
    #[inline(always)]
    pub const fn demod_iq2_dig_tst(&self) -> &DEMOD_IQ2_DIG_TST {
        &self.demod_iq2_dig_tst
    }
    ///0x240 - ANTSW0_DIG_USR register
    #[inline(always)]
    pub const fn antsw0_dig_usr(&self) -> &ANTSW0_DIG_USR {
        &self.antsw0_dig_usr
    }
    ///0x244 - ANTSW1_DIG_USR register
    #[inline(always)]
    pub const fn antsw1_dig_usr(&self) -> &ANTSW1_DIG_USR {
        &self.antsw1_dig_usr
    }
    ///0x248 - ANTSW2_DIG_USR register
    #[inline(always)]
    pub const fn antsw2_dig_usr(&self) -> &ANTSW2_DIG_USR {
        &self.antsw2_dig_usr
    }
    ///0x24c - ANTSW3_DIG_USR register
    #[inline(always)]
    pub const fn antsw3_dig_usr(&self) -> &ANTSW3_DIG_USR {
        &self.antsw3_dig_usr
    }
}
/**AA0_DIG_USR (rw) register accessor: AA0_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa0_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa0_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AA0_DIG_USR)

For information about available fields see [`mod@aa0_dig_usr`] module*/
pub type AA0_DIG_USR = crate::Reg<aa0_dig_usr::AA0_DIG_USRrs>;
///AA0_DIG_USR register
pub mod aa0_dig_usr;
/**AA1_DIG_USR (rw) register accessor: AA1_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa1_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa1_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AA1_DIG_USR)

For information about available fields see [`mod@aa1_dig_usr`] module*/
pub type AA1_DIG_USR = crate::Reg<aa1_dig_usr::AA1_DIG_USRrs>;
///AA1_DIG_USR register
pub mod aa1_dig_usr;
/**AA2_DIG_USR (rw) register accessor: AA2_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa2_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa2_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AA2_DIG_USR)

For information about available fields see [`mod@aa2_dig_usr`] module*/
pub type AA2_DIG_USR = crate::Reg<aa2_dig_usr::AA2_DIG_USRrs>;
///AA2_DIG_USR register
pub mod aa2_dig_usr;
/**AA3_DIG_USR (rw) register accessor: AA3_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa3_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa3_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AA3_DIG_USR)

For information about available fields see [`mod@aa3_dig_usr`] module*/
pub type AA3_DIG_USR = crate::Reg<aa3_dig_usr::AA3_DIG_USRrs>;
///AA3_DIG_USR register
pub mod aa3_dig_usr;
/**DEM_MOD_DIG_USR (rw) register accessor: DEM_MOD_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`dem_mod_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dem_mod_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:DEM_MOD_DIG_USR)

For information about available fields see [`mod@dem_mod_dig_usr`] module*/
pub type DEM_MOD_DIG_USR = crate::Reg<dem_mod_dig_usr::DEM_MOD_DIG_USRrs>;
///DEM_MOD_DIG_USR register
pub mod dem_mod_dig_usr;
/**FSM_USR (rw) register accessor: RADIO_FSM_USR register

You can [`read`](crate::Reg::read) this register and get [`fsm_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:FSM_USR)

For information about available fields see [`mod@fsm_usr`] module*/
pub type FSM_USR = crate::Reg<fsm_usr::FSM_USRrs>;
///RADIO_FSM_USR register
pub mod fsm_usr;
/**PHYCTRL_DIG_USR (rw) register accessor: PHYCTRL_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`phyctrl_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phyctrl_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:PHYCTRL_DIG_USR)

For information about available fields see [`mod@phyctrl_dig_usr`] module*/
pub type PHYCTRL_DIG_USR = crate::Reg<phyctrl_dig_usr::PHYCTRL_DIG_USRrs>;
///PHYCTRL_DIG_USR register
pub mod phyctrl_dig_usr;
/**AFC1_DIG_ENG (rw) register accessor: AFC1_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`afc1_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc1_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AFC1_DIG_ENG)

For information about available fields see [`mod@afc1_dig_eng`] module*/
pub type AFC1_DIG_ENG = crate::Reg<afc1_dig_eng::AFC1_DIG_ENGrs>;
///AFC1_DIG_ENG register
pub mod afc1_dig_eng;
/**CR0_DIG_ENG (rw) register accessor: CR0_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`cr0_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:CR0_DIG_ENG)

For information about available fields see [`mod@cr0_dig_eng`] module*/
pub type CR0_DIG_ENG = crate::Reg<cr0_dig_eng::CR0_DIG_ENGrs>;
///CR0_DIG_ENG register
pub mod cr0_dig_eng;
/**CR0_LR (rw) register accessor: CR0_LR register

You can [`read`](crate::Reg::read) this register and get [`cr0_lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0_lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:CR0_LR)

For information about available fields see [`mod@cr0_lr`] module*/
pub type CR0_LR = crate::Reg<cr0_lr::CR0_LRrs>;
///CR0_LR register
pub mod cr0_lr;
/**VIT_CONF_DIG_ENG (rw) register accessor: VIT_CONF_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`vit_conf_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vit_conf_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:VIT_CONF_DIG_ENG)

For information about available fields see [`mod@vit_conf_dig_eng`] module*/
pub type VIT_CONF_DIG_ENG = crate::Reg<vit_conf_dig_eng::VIT_CONF_DIG_ENGrs>;
///VIT_CONF_DIG_ENG register
pub mod vit_conf_dig_eng;
/**LR_PD_THR_DIG_ENG (rw) register accessor: LR_PD_THR_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`lr_pd_thr_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr_pd_thr_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:LR_PD_THR_DIG_ENG)

For information about available fields see [`mod@lr_pd_thr_dig_eng`] module*/
pub type LR_PD_THR_DIG_ENG = crate::Reg<lr_pd_thr_dig_eng::LR_PD_THR_DIG_ENGrs>;
///LR_PD_THR_DIG_ENG register
pub mod lr_pd_thr_dig_eng;
/**LR_RSSI_THR_DIG_ENG (rw) register accessor: LR_RSSI_THR_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`lr_rssi_thr_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr_rssi_thr_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:LR_RSSI_THR_DIG_ENG)

For information about available fields see [`mod@lr_rssi_thr_dig_eng`] module*/
pub type LR_RSSI_THR_DIG_ENG = crate::Reg<lr_rssi_thr_dig_eng::LR_RSSI_THR_DIG_ENGrs>;
///LR_RSSI_THR_DIG_ENG register
pub mod lr_rssi_thr_dig_eng;
/**LR_AAC_THR_DIG_ENG (rw) register accessor: LR_AAC_THR_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`lr_aac_thr_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr_aac_thr_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:LR_AAC_THR_DIG_ENG)

For information about available fields see [`mod@lr_aac_thr_dig_eng`] module*/
pub type LR_AAC_THR_DIG_ENG = crate::Reg<lr_aac_thr_dig_eng::LR_AAC_THR_DIG_ENGrs>;
///LR_AAC_THR_DIG_ENG register
pub mod lr_aac_thr_dig_eng;
/**SYNTHCAL0_DIG_ENG (rw) register accessor: SYNTHCAL0_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`synthcal0_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synthcal0_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL0_DIG_ENG)

For information about available fields see [`mod@synthcal0_dig_eng`] module*/
pub type SYNTHCAL0_DIG_ENG = crate::Reg<synthcal0_dig_eng::SYNTHCAL0_DIG_ENGrs>;
///SYNTHCAL0_DIG_ENG register
pub mod synthcal0_dig_eng;
/**DTB5_DIG_ENG (rw) register accessor: DTB5_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`dtb5_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb5_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:DTB5_DIG_ENG)

For information about available fields see [`mod@dtb5_dig_eng`] module*/
pub type DTB5_DIG_ENG = crate::Reg<dtb5_dig_eng::DTB5_DIG_ENGrs>;
///DTB5_DIG_ENG register
pub mod dtb5_dig_eng;
/**RXADC_ANA_USR (rw) register accessor: RXADC_ANA_USR register

You can [`read`](crate::Reg::read) this register and get [`rxadc_ana_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxadc_ana_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:RXADC_ANA_USR)

For information about available fields see [`mod@rxadc_ana_usr`] module*/
pub type RXADC_ANA_USR = crate::Reg<rxadc_ana_usr::RXADC_ANA_USRrs>;
///RXADC_ANA_USR register
pub mod rxadc_ana_usr;
/**LDO_ANA_ENG (rw) register accessor: LDO_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`ldo_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:LDO_ANA_ENG)

For information about available fields see [`mod@ldo_ana_eng`] module*/
pub type LDO_ANA_ENG = crate::Reg<ldo_ana_eng::LDO_ANA_ENGrs>;
///LDO_ANA_ENG register
pub mod ldo_ana_eng;
/**CBIAS0_ANA_ENG (rw) register accessor: CBIAS0_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`cbias0_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbias0_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:CBIAS0_ANA_ENG)

For information about available fields see [`mod@cbias0_ana_eng`] module*/
pub type CBIAS0_ANA_ENG = crate::Reg<cbias0_ana_eng::CBIAS0_ANA_ENGrs>;
///CBIAS0_ANA_ENG register
pub mod cbias0_ana_eng;
/**CBIAS1_ANA_ENG (rw) register accessor: CBIAS1_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`cbias1_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbias1_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:CBIAS1_ANA_ENG)

For information about available fields see [`mod@cbias1_ana_eng`] module*/
pub type CBIAS1_ANA_ENG = crate::Reg<cbias1_ana_eng::CBIAS1_ANA_ENGrs>;
///CBIAS1_ANA_ENG register
pub mod cbias1_ana_eng;
/**SYNTHCAL0_DIG_OUT (r) register accessor: SYNTHCAL0_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal0_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL0_DIG_OUT)

For information about available fields see [`mod@synthcal0_dig_out`] module*/
pub type SYNTHCAL0_DIG_OUT = crate::Reg<synthcal0_dig_out::SYNTHCAL0_DIG_OUTrs>;
///SYNTHCAL0_DIG_OUT register
pub mod synthcal0_dig_out;
/**SYNTHCAL1_DIG_OUT (r) register accessor: SYNTHCAL1_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal1_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL1_DIG_OUT)

For information about available fields see [`mod@synthcal1_dig_out`] module*/
pub type SYNTHCAL1_DIG_OUT = crate::Reg<synthcal1_dig_out::SYNTHCAL1_DIG_OUTrs>;
///SYNTHCAL1_DIG_OUT register
pub mod synthcal1_dig_out;
/**SYNTHCAL2_DIG_OUT (r) register accessor: SYNTHCAL2_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal2_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL2_DIG_OUT)

For information about available fields see [`mod@synthcal2_dig_out`] module*/
pub type SYNTHCAL2_DIG_OUT = crate::Reg<synthcal2_dig_out::SYNTHCAL2_DIG_OUTrs>;
///SYNTHCAL2_DIG_OUT register
pub mod synthcal2_dig_out;
/**SYNTHCAL3_DIG_OUT (r) register accessor: SYNTHCAL3_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal3_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL3_DIG_OUT)

For information about available fields see [`mod@synthcal3_dig_out`] module*/
pub type SYNTHCAL3_DIG_OUT = crate::Reg<synthcal3_dig_out::SYNTHCAL3_DIG_OUTrs>;
///SYNTHCAL3_DIG_OUT register
pub mod synthcal3_dig_out;
/**SYNTHCAL4_DIG_OUT (r) register accessor: SYNTHCAL4_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal4_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL4_DIG_OUT)

For information about available fields see [`mod@synthcal4_dig_out`] module*/
pub type SYNTHCAL4_DIG_OUT = crate::Reg<synthcal4_dig_out::SYNTHCAL4_DIG_OUTrs>;
///SYNTHCAL4_DIG_OUT register
pub mod synthcal4_dig_out;
/**SYNTHCAL5_DIG_OUT (r) register accessor: SYNTHCAL5_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal5_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL5_DIG_OUT)

For information about available fields see [`mod@synthcal5_dig_out`] module*/
pub type SYNTHCAL5_DIG_OUT = crate::Reg<synthcal5_dig_out::SYNTHCAL5_DIG_OUTrs>;
///SYNTHCAL5_DIG_OUT register
pub mod synthcal5_dig_out;
/**FSM_STATUS_DIG_OUT (r) register accessor: FSM_STATUS_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`fsm_status_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:FSM_STATUS_DIG_OUT)

For information about available fields see [`mod@fsm_status_dig_out`] module*/
pub type FSM_STATUS_DIG_OUT = crate::Reg<fsm_status_dig_out::FSM_STATUS_DIG_OUTrs>;
///FSM_STATUS_DIG_OUT register
pub mod fsm_status_dig_out;
/**RSSI0_DIG_OUT (r) register accessor: RSSI0_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`rssi0_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:RSSI0_DIG_OUT)

For information about available fields see [`mod@rssi0_dig_out`] module*/
pub type RSSI0_DIG_OUT = crate::Reg<rssi0_dig_out::RSSI0_DIG_OUTrs>;
///RSSI0_DIG_OUT register
pub mod rssi0_dig_out;
/**RSSI1_DIG_OUT (r) register accessor: RSSI1_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`rssi1_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:RSSI1_DIG_OUT)

For information about available fields see [`mod@rssi1_dig_out`] module*/
pub type RSSI1_DIG_OUT = crate::Reg<rssi1_dig_out::RSSI1_DIG_OUTrs>;
///RSSI1_DIG_OUT register
pub mod rssi1_dig_out;
/**AGC_DIG_OUT (r) register accessor: AGC_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`agc_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC_DIG_OUT)

For information about available fields see [`mod@agc_dig_out`] module*/
pub type AGC_DIG_OUT = crate::Reg<agc_dig_out::AGC_DIG_OUTrs>;
///AGC_DIG_OUT register
pub mod agc_dig_out;
/**DEMOD_DIG_OUT (r) register accessor: DEMOD_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`demod_dig_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:DEMOD_DIG_OUT)

For information about available fields see [`mod@demod_dig_out`] module*/
pub type DEMOD_DIG_OUT = crate::Reg<demod_dig_out::DEMOD_DIG_OUTrs>;
///DEMOD_DIG_OUT register
pub mod demod_dig_out;
/**AGC2_ANA_TST (rw) register accessor: AGC2_ANA_TST register

You can [`read`](crate::Reg::read) this register and get [`agc2_ana_tst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc2_ana_tst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC2_ANA_TST)

For information about available fields see [`mod@agc2_ana_tst`] module*/
pub type AGC2_ANA_TST = crate::Reg<agc2_ana_tst::AGC2_ANA_TSTrs>;
///AGC2_ANA_TST register
pub mod agc2_ana_tst;
/**AGC0_DIG_ENG (rw) register accessor: AGC0_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc0_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc0_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC0_DIG_ENG)

For information about available fields see [`mod@agc0_dig_eng`] module*/
pub type AGC0_DIG_ENG = crate::Reg<agc0_dig_eng::AGC0_DIG_ENGrs>;
///AGC0_DIG_ENG register
pub mod agc0_dig_eng;
/**AGC1_DIG_ENG (rw) register accessor: AGC1_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc1_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc1_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC1_DIG_ENG)

For information about available fields see [`mod@agc1_dig_eng`] module*/
pub type AGC1_DIG_ENG = crate::Reg<agc1_dig_eng::AGC1_DIG_ENGrs>;
///AGC1_DIG_ENG register
pub mod agc1_dig_eng;
/**AGC10_DIG_ENG (rw) register accessor: AGC10_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc10_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc10_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC10_DIG_ENG)

For information about available fields see [`mod@agc10_dig_eng`] module*/
pub type AGC10_DIG_ENG = crate::Reg<agc10_dig_eng::AGC10_DIG_ENGrs>;
///AGC10_DIG_ENG register
pub mod agc10_dig_eng;
/**AGC11_DIG_ENG (rw) register accessor: AGC11_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc11_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc11_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC11_DIG_ENG)

For information about available fields see [`mod@agc11_dig_eng`] module*/
pub type AGC11_DIG_ENG = crate::Reg<agc11_dig_eng::AGC11_DIG_ENGrs>;
///AGC11_DIG_ENG register
pub mod agc11_dig_eng;
/**AGC12_DIG_ENG (rw) register accessor: AGC12_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc12_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc12_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC12_DIG_ENG)

For information about available fields see [`mod@agc12_dig_eng`] module*/
pub type AGC12_DIG_ENG = crate::Reg<agc12_dig_eng::AGC12_DIG_ENGrs>;
///AGC12_DIG_ENG register
pub mod agc12_dig_eng;
/**AGC13_DIG_ENG (rw) register accessor: AGC13_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc13_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc13_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC13_DIG_ENG)

For information about available fields see [`mod@agc13_dig_eng`] module*/
pub type AGC13_DIG_ENG = crate::Reg<agc13_dig_eng::AGC13_DIG_ENGrs>;
///AGC13_DIG_ENG register
pub mod agc13_dig_eng;
/**AGC14_DIG_ENG (rw) register accessor: AGC14_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc14_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc14_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC14_DIG_ENG)

For information about available fields see [`mod@agc14_dig_eng`] module*/
pub type AGC14_DIG_ENG = crate::Reg<agc14_dig_eng::AGC14_DIG_ENGrs>;
///AGC14_DIG_ENG register
pub mod agc14_dig_eng;
/**AGC15_DIG_ENG (rw) register accessor: AGC15_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc15_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc15_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC15_DIG_ENG)

For information about available fields see [`mod@agc15_dig_eng`] module*/
pub type AGC15_DIG_ENG = crate::Reg<agc15_dig_eng::AGC15_DIG_ENGrs>;
///AGC15_DIG_ENG register
pub mod agc15_dig_eng;
/**AGC16_DIG_ENG (rw) register accessor: AGC16_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc16_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc16_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC16_DIG_ENG)

For information about available fields see [`mod@agc16_dig_eng`] module*/
pub type AGC16_DIG_ENG = crate::Reg<agc16_dig_eng::AGC16_DIG_ENGrs>;
///AGC16_DIG_ENG register
pub mod agc16_dig_eng;
/**AGC17_DIG_ENG (rw) register accessor: AGC17_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc17_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc17_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC17_DIG_ENG)

For information about available fields see [`mod@agc17_dig_eng`] module*/
pub type AGC17_DIG_ENG = crate::Reg<agc17_dig_eng::AGC17_DIG_ENGrs>;
///AGC17_DIG_ENG register
pub mod agc17_dig_eng;
/**AGC18_DIG_ENG (rw) register accessor: AGC18_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc18_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc18_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC18_DIG_ENG)

For information about available fields see [`mod@agc18_dig_eng`] module*/
pub type AGC18_DIG_ENG = crate::Reg<agc18_dig_eng::AGC18_DIG_ENGrs>;
///AGC18_DIG_ENG register
pub mod agc18_dig_eng;
/**AGC19_DIG_ENG (rw) register accessor: AGC19_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc19_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc19_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC19_DIG_ENG)

For information about available fields see [`mod@agc19_dig_eng`] module*/
pub type AGC19_DIG_ENG = crate::Reg<agc19_dig_eng::AGC19_DIG_ENGrs>;
///AGC19_DIG_ENG register
pub mod agc19_dig_eng;
/**RXADC_HW_TRIM_OUT (r) register accessor: RXADC_HW_TRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`rxadc_hw_trim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:RXADC_HW_TRIM_OUT)

For information about available fields see [`mod@rxadc_hw_trim_out`] module*/
pub type RXADC_HW_TRIM_OUT = crate::Reg<rxadc_hw_trim_out::RXADC_HW_TRIM_OUTrs>;
///RXADC_HW_TRIM_OUT register
pub mod rxadc_hw_trim_out;
/**CBIAS0_HW_TRIM_OUT (r) register accessor: CBIAS0_HW_TRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`cbias0_hw_trim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:CBIAS0_HW_TRIM_OUT)

For information about available fields see [`mod@cbias0_hw_trim_out`] module*/
pub type CBIAS0_HW_TRIM_OUT = crate::Reg<cbias0_hw_trim_out::CBIAS0_HW_TRIM_OUTrs>;
///CBIAS0_HW_TRIM_OUT register
pub mod cbias0_hw_trim_out;
/**AGC_HW_TRIM_OUT (r) register accessor: AGC_HW_TRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`agc_hw_trim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC_HW_TRIM_OUT)

For information about available fields see [`mod@agc_hw_trim_out`] module*/
pub type AGC_HW_TRIM_OUT = crate::Reg<agc_hw_trim_out::AGC_HW_TRIM_OUTrs>;
///AGC_HW_TRIM_OUT register
pub mod agc_hw_trim_out;
/**DEMOD_IQ2_DIG_TST (rw) register accessor: DEMOD_IQ2_DIG_TST register

You can [`read`](crate::Reg::read) this register and get [`demod_iq2_dig_tst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demod_iq2_dig_tst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:DEMOD_IQ2_DIG_TST)

For information about available fields see [`mod@demod_iq2_dig_tst`] module*/
pub type DEMOD_IQ2_DIG_TST = crate::Reg<demod_iq2_dig_tst::DEMOD_IQ2_DIG_TSTrs>;
///DEMOD_IQ2_DIG_TST register
pub mod demod_iq2_dig_tst;
/**ANTSW0_DIG_USR (rw) register accessor: ANTSW0_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`antsw0_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`antsw0_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:ANTSW0_DIG_USR)

For information about available fields see [`mod@antsw0_dig_usr`] module*/
pub type ANTSW0_DIG_USR = crate::Reg<antsw0_dig_usr::ANTSW0_DIG_USRrs>;
///ANTSW0_DIG_USR register
pub mod antsw0_dig_usr;
/**ANTSW1_DIG_USR (rw) register accessor: ANTSW1_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`antsw1_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`antsw1_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:ANTSW1_DIG_USR)

For information about available fields see [`mod@antsw1_dig_usr`] module*/
pub type ANTSW1_DIG_USR = crate::Reg<antsw1_dig_usr::ANTSW1_DIG_USRrs>;
///ANTSW1_DIG_USR register
pub mod antsw1_dig_usr;
/**ANTSW2_DIG_USR (rw) register accessor: ANTSW2_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`antsw2_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`antsw2_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:ANTSW2_DIG_USR)

For information about available fields see [`mod@antsw2_dig_usr`] module*/
pub type ANTSW2_DIG_USR = crate::Reg<antsw2_dig_usr::ANTSW2_DIG_USRrs>;
///ANTSW2_DIG_USR register
pub mod antsw2_dig_usr;
/**ANTSW3_DIG_USR (rw) register accessor: ANTSW3_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`antsw3_dig_usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`antsw3_dig_usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:ANTSW3_DIG_USR)

For information about available fields see [`mod@antsw3_dig_usr`] module*/
pub type ANTSW3_DIG_USR = crate::Reg<antsw3_dig_usr::ANTSW3_DIG_USRrs>;
///ANTSW3_DIG_USR register
pub mod antsw3_dig_usr;
