#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    version_id: VERSION_ID,
    conf: CONF,
    ctrl: CTRL,
    ocm_ctrl: OCM_CTRL,
    pga_conf: PGA_CONF,
    switch: SWITCH,
    df_conf: DF_CONF,
    ds_conf: DS_CONF,
    seq_1: SEQ_1,
    seq_2: SEQ_2,
    comp_1: COMP_1,
    comp_2: COMP_2,
    comp_3: COMP_3,
    comp_4: COMP_4,
    comp_sel: COMP_SEL,
    wd_th: WD_TH,
    wd_conf: WD_CONF,
    ds_dataout: DS_DATAOUT,
    df_dataout: DF_DATAOUT,
    irq_status: IRQ_STATUS,
    irq_enable: IRQ_ENABLE,
    timer_conf: TIMER_CONF,
}
impl RegisterBlock {
    ///0x00 - VERSION_ID register
    #[inline(always)]
    pub const fn version_id(&self) -> &VERSION_ID {
        &self.version_id
    }
    ///0x04 - ADC configuration register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x08 - ADC control register
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x0c - Occasionnal mode control register
    #[inline(always)]
    pub const fn ocm_ctrl(&self) -> &OCM_CTRL {
        &self.ocm_ctrl
    }
    ///0x10 - PGA configuration register
    #[inline(always)]
    pub const fn pga_conf(&self) -> &PGA_CONF {
        &self.pga_conf
    }
    ///0x14 - ADC switch control for Input Selection
    #[inline(always)]
    pub const fn switch(&self) -> &SWITCH {
        &self.switch
    }
    ///0x18 - Decimation filter configuration register
    #[inline(always)]
    pub const fn df_conf(&self) -> &DF_CONF {
        &self.df_conf
    }
    ///0x1c - Downsampler configuration register
    #[inline(always)]
    pub const fn ds_conf(&self) -> &DS_CONF {
        &self.ds_conf
    }
    ///0x20 - ADC regular sequence configuration register 1
    #[inline(always)]
    pub const fn seq_1(&self) -> &SEQ_1 {
        &self.seq_1
    }
    ///0x24 - ADC regular sequence configuration register 2
    #[inline(always)]
    pub const fn seq_2(&self) -> &SEQ_2 {
        &self.seq_2
    }
    ///0x28 - ADC Gain and offset correction values register 1
    #[inline(always)]
    pub const fn comp_1(&self) -> &COMP_1 {
        &self.comp_1
    }
    ///0x2c - ADC Gain and offset correction values register 2
    #[inline(always)]
    pub const fn comp_2(&self) -> &COMP_2 {
        &self.comp_2
    }
    ///0x30 - ADC Gain and offset correction values register 3
    #[inline(always)]
    pub const fn comp_3(&self) -> &COMP_3 {
        &self.comp_3
    }
    ///0x34 - ADC Gain and offset correction values register 4
    #[inline(always)]
    pub const fn comp_4(&self) -> &COMP_4 {
        &self.comp_4
    }
    ///0x38 - ADC Gain and Offset selection values register
    #[inline(always)]
    pub const fn comp_sel(&self) -> &COMP_SEL {
        &self.comp_sel
    }
    ///0x3c - High/low limits for event monitoring a channel register
    #[inline(always)]
    pub const fn wd_th(&self) -> &WD_TH {
        &self.wd_th
    }
    ///0x40 - Channel selection for event monitoring register
    #[inline(always)]
    pub const fn wd_conf(&self) -> &WD_CONF {
        &self.wd_conf
    }
    ///0x44 - Downsampler Data output register
    #[inline(always)]
    pub const fn ds_dataout(&self) -> &DS_DATAOUT {
        &self.ds_dataout
    }
    ///0x48 - Decimation filter Data output register
    #[inline(always)]
    pub const fn df_dataout(&self) -> &DF_DATAOUT {
        &self.df_dataout
    }
    ///0x4c - Interrupt Status register
    #[inline(always)]
    pub const fn irq_status(&self) -> &IRQ_STATUS {
        &self.irq_status
    }
    ///0x50 - Enable/disable Interrupts
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IRQ_ENABLE {
        &self.irq_enable
    }
    ///0x54 - Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it
    #[inline(always)]
    pub const fn timer_conf(&self) -> &TIMER_CONF {
        &self.timer_conf
    }
}
/**VERSION_ID (r) register accessor: VERSION_ID register

You can [`read`](crate::Reg::read) this register and get [`version_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:VERSION_ID)

For information about available fields see [`mod@version_id`] module*/
pub type VERSION_ID = crate::Reg<version_id::VERSION_IDrs>;
///VERSION_ID register
pub mod version_id;
/**CONF (rw) register accessor: ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:CONF)

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONFrs>;
///ADC configuration register
pub mod conf;
/**CTRL (rw) register accessor: ADC control register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:CTRL)

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
///ADC control register
pub mod ctrl;
/**OCM_CTRL (rw) register accessor: Occasionnal mode control register

You can [`read`](crate::Reg::read) this register and get [`ocm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:OCM_CTRL)

For information about available fields see [`mod@ocm_ctrl`] module*/
pub type OCM_CTRL = crate::Reg<ocm_ctrl::OCM_CTRLrs>;
///Occasionnal mode control register
pub mod ocm_ctrl;
/**PGA_CONF (rw) register accessor: PGA configuration register

You can [`read`](crate::Reg::read) this register and get [`pga_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pga_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:PGA_CONF)

For information about available fields see [`mod@pga_conf`] module*/
pub type PGA_CONF = crate::Reg<pga_conf::PGA_CONFrs>;
///PGA configuration register
pub mod pga_conf;
/**SWITCH (rw) register accessor: ADC switch control for Input Selection

You can [`read`](crate::Reg::read) this register and get [`switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:SWITCH)

For information about available fields see [`mod@switch`] module*/
pub type SWITCH = crate::Reg<switch::SWITCHrs>;
///ADC switch control for Input Selection
pub mod switch;
/**DF_CONF (rw) register accessor: Decimation filter configuration register

You can [`read`](crate::Reg::read) this register and get [`df_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`df_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:DF_CONF)

For information about available fields see [`mod@df_conf`] module*/
pub type DF_CONF = crate::Reg<df_conf::DF_CONFrs>;
///Decimation filter configuration register
pub mod df_conf;
/**DS_CONF (rw) register accessor: Downsampler configuration register

You can [`read`](crate::Reg::read) this register and get [`ds_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:DS_CONF)

For information about available fields see [`mod@ds_conf`] module*/
pub type DS_CONF = crate::Reg<ds_conf::DS_CONFrs>;
///Downsampler configuration register
pub mod ds_conf;
/**SEQ_1 (rw) register accessor: ADC regular sequence configuration register 1

You can [`read`](crate::Reg::read) this register and get [`seq_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:SEQ_1)

For information about available fields see [`mod@seq_1`] module*/
pub type SEQ_1 = crate::Reg<seq_1::SEQ_1rs>;
///ADC regular sequence configuration register 1
pub mod seq_1;
/**SEQ_2 (rw) register accessor: ADC regular sequence configuration register 2

You can [`read`](crate::Reg::read) this register and get [`seq_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:SEQ_2)

For information about available fields see [`mod@seq_2`] module*/
pub type SEQ_2 = crate::Reg<seq_2::SEQ_2rs>;
///ADC regular sequence configuration register 2
pub mod seq_2;
/**COMP_1 (rw) register accessor: ADC Gain and offset correction values register 1

You can [`read`](crate::Reg::read) this register and get [`comp_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:COMP_1)

For information about available fields see [`mod@comp_1`] module*/
pub type COMP_1 = crate::Reg<comp_1::COMP_1rs>;
///ADC Gain and offset correction values register 1
pub mod comp_1;
/**COMP_2 (rw) register accessor: ADC Gain and offset correction values register 2

You can [`read`](crate::Reg::read) this register and get [`comp_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:COMP_2)

For information about available fields see [`mod@comp_2`] module*/
pub type COMP_2 = crate::Reg<comp_2::COMP_2rs>;
///ADC Gain and offset correction values register 2
pub mod comp_2;
/**COMP_3 (rw) register accessor: ADC Gain and offset correction values register 3

You can [`read`](crate::Reg::read) this register and get [`comp_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:COMP_3)

For information about available fields see [`mod@comp_3`] module*/
pub type COMP_3 = crate::Reg<comp_3::COMP_3rs>;
///ADC Gain and offset correction values register 3
pub mod comp_3;
/**COMP_4 (rw) register accessor: ADC Gain and offset correction values register 4

You can [`read`](crate::Reg::read) this register and get [`comp_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:COMP_4)

For information about available fields see [`mod@comp_4`] module*/
pub type COMP_4 = crate::Reg<comp_4::COMP_4rs>;
///ADC Gain and offset correction values register 4
pub mod comp_4;
/**COMP_SEL (rw) register accessor: ADC Gain and Offset selection values register

You can [`read`](crate::Reg::read) this register and get [`comp_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:COMP_SEL)

For information about available fields see [`mod@comp_sel`] module*/
pub type COMP_SEL = crate::Reg<comp_sel::COMP_SELrs>;
///ADC Gain and Offset selection values register
pub mod comp_sel;
/**WD_TH (rw) register accessor: High/low limits for event monitoring a channel register

You can [`read`](crate::Reg::read) this register and get [`wd_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:WD_TH)

For information about available fields see [`mod@wd_th`] module*/
pub type WD_TH = crate::Reg<wd_th::WD_THrs>;
///High/low limits for event monitoring a channel register
pub mod wd_th;
/**WD_CONF (rw) register accessor: Channel selection for event monitoring register

You can [`read`](crate::Reg::read) this register and get [`wd_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:WD_CONF)

For information about available fields see [`mod@wd_conf`] module*/
pub type WD_CONF = crate::Reg<wd_conf::WD_CONFrs>;
///Channel selection for event monitoring register
pub mod wd_conf;
/**DS_DATAOUT (r) register accessor: Downsampler Data output register

You can [`read`](crate::Reg::read) this register and get [`ds_dataout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:DS_DATAOUT)

For information about available fields see [`mod@ds_dataout`] module*/
pub type DS_DATAOUT = crate::Reg<ds_dataout::DS_DATAOUTrs>;
///Downsampler Data output register
pub mod ds_dataout;
/**DF_DATAOUT (r) register accessor: Decimation filter Data output register

You can [`read`](crate::Reg::read) this register and get [`df_dataout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:DF_DATAOUT)

For information about available fields see [`mod@df_dataout`] module*/
pub type DF_DATAOUT = crate::Reg<df_dataout::DF_DATAOUTrs>;
///Decimation filter Data output register
pub mod df_dataout;
/**IRQ_STATUS (rw) register accessor: Interrupt Status register

You can [`read`](crate::Reg::read) this register and get [`irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:IRQ_STATUS)

For information about available fields see [`mod@irq_status`] module*/
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUSrs>;
///Interrupt Status register
pub mod irq_status;
/**IRQ_ENABLE (rw) register accessor: Enable/disable Interrupts

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:IRQ_ENABLE)

For information about available fields see [`mod@irq_enable`] module*/
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLErs>;
///Enable/disable Interrupts
pub mod irq_enable;
/**TIMER_CONF (rw) register accessor: Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it

You can [`read`](crate::Reg::read) this register and get [`timer_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:TIMER_CONF)

For information about available fields see [`mod@timer_conf`] module*/
pub type TIMER_CONF = crate::Reg<timer_conf::TIMER_CONFrs>;
///Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it
pub mod timer_conf;
