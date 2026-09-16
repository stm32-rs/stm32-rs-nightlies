#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    version_id: VERSION_ID,
    conf: CONF,
    ctrl: CTRL,
    _reserved3: [u8; 0x08],
    switch: SWITCH,
    _reserved4: [u8; 0x04],
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
    _reserved15: [u8; 0x04],
    irq_status: IRQ_STATUS,
    irq_enable: IRQ_ENABLE,
    _reserved17: [u8; 0x0c],
    test_conf: TEST_CONF,
    dtb_conf: DTB_CONF,
}
impl RegisterBlock {
    ///0x00 - VERSION_ID register
    #[inline(always)]
    pub const fn version_id(&self) -> &VERSION_ID {
        &self.version_id
    }
    ///0x04 - CONF register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x08 - CTRL register
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x14 - SWITCH register
    #[inline(always)]
    pub const fn switch(&self) -> &SWITCH {
        &self.switch
    }
    ///0x1c - DS_CONF register
    #[inline(always)]
    pub const fn ds_conf(&self) -> &DS_CONF {
        &self.ds_conf
    }
    ///0x20 - SEQ_1 register
    #[inline(always)]
    pub const fn seq_1(&self) -> &SEQ_1 {
        &self.seq_1
    }
    ///0x24 - SEQ_2 register
    #[inline(always)]
    pub const fn seq_2(&self) -> &SEQ_2 {
        &self.seq_2
    }
    ///0x28 - COMP_1 register
    #[inline(always)]
    pub const fn comp_1(&self) -> &COMP_1 {
        &self.comp_1
    }
    ///0x2c - COMP_2 register
    #[inline(always)]
    pub const fn comp_2(&self) -> &COMP_2 {
        &self.comp_2
    }
    ///0x30 - COMP_3 register
    #[inline(always)]
    pub const fn comp_3(&self) -> &COMP_3 {
        &self.comp_3
    }
    ///0x34 - COMP_4 register
    #[inline(always)]
    pub const fn comp_4(&self) -> &COMP_4 {
        &self.comp_4
    }
    ///0x38 - COMP_SEL register
    #[inline(always)]
    pub const fn comp_sel(&self) -> &COMP_SEL {
        &self.comp_sel
    }
    ///0x3c - WD_TH register
    #[inline(always)]
    pub const fn wd_th(&self) -> &WD_TH {
        &self.wd_th
    }
    ///0x40 - WD_CONF register
    #[inline(always)]
    pub const fn wd_conf(&self) -> &WD_CONF {
        &self.wd_conf
    }
    ///0x44 - DS_DATAOUT register
    #[inline(always)]
    pub const fn ds_dataout(&self) -> &DS_DATAOUT {
        &self.ds_dataout
    }
    ///0x4c - IRQ_STATUS register
    #[inline(always)]
    pub const fn irq_status(&self) -> &IRQ_STATUS {
        &self.irq_status
    }
    ///0x50 - IRQ_ENABLE register
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IRQ_ENABLE {
        &self.irq_enable
    }
    ///0x60 - TEST_CONF register
    #[inline(always)]
    pub const fn test_conf(&self) -> &TEST_CONF {
        &self.test_conf
    }
    ///0x64 - DTB_CONF register
    #[inline(always)]
    pub const fn dtb_conf(&self) -> &DTB_CONF {
        &self.dtb_conf
    }
}
/**VERSION_ID (r) register accessor: VERSION_ID register

You can [`read`](crate::Reg::read) this register and get [`version_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:VERSION_ID)

For information about available fields see [`mod@version_id`] module*/
pub type VERSION_ID = crate::Reg<version_id::VERSION_IDrs>;
///VERSION_ID register
pub mod version_id;
/**CONF (rw) register accessor: CONF register

You can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:CONF)

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONFrs>;
///CONF register
pub mod conf;
/**CTRL (rw) register accessor: CTRL register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:CTRL)

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
///CTRL register
pub mod ctrl;
/**SWITCH (rw) register accessor: SWITCH register

You can [`read`](crate::Reg::read) this register and get [`switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:SWITCH)

For information about available fields see [`mod@switch`] module*/
pub type SWITCH = crate::Reg<switch::SWITCHrs>;
///SWITCH register
pub mod switch;
/**DS_CONF (rw) register accessor: DS_CONF register

You can [`read`](crate::Reg::read) this register and get [`ds_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:DS_CONF)

For information about available fields see [`mod@ds_conf`] module*/
pub type DS_CONF = crate::Reg<ds_conf::DS_CONFrs>;
///DS_CONF register
pub mod ds_conf;
/**SEQ_1 (rw) register accessor: SEQ_1 register

You can [`read`](crate::Reg::read) this register and get [`seq_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:SEQ_1)

For information about available fields see [`mod@seq_1`] module*/
pub type SEQ_1 = crate::Reg<seq_1::SEQ_1rs>;
///SEQ_1 register
pub mod seq_1;
/**SEQ_2 (rw) register accessor: SEQ_2 register

You can [`read`](crate::Reg::read) this register and get [`seq_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:SEQ_2)

For information about available fields see [`mod@seq_2`] module*/
pub type SEQ_2 = crate::Reg<seq_2::SEQ_2rs>;
///SEQ_2 register
pub mod seq_2;
/**COMP_1 (rw) register accessor: COMP_1 register

You can [`read`](crate::Reg::read) this register and get [`comp_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:COMP_1)

For information about available fields see [`mod@comp_1`] module*/
pub type COMP_1 = crate::Reg<comp_1::COMP_1rs>;
///COMP_1 register
pub mod comp_1;
/**COMP_2 (rw) register accessor: COMP_2 register

You can [`read`](crate::Reg::read) this register and get [`comp_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:COMP_2)

For information about available fields see [`mod@comp_2`] module*/
pub type COMP_2 = crate::Reg<comp_2::COMP_2rs>;
///COMP_2 register
pub mod comp_2;
/**COMP_3 (rw) register accessor: COMP_3 register

You can [`read`](crate::Reg::read) this register and get [`comp_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:COMP_3)

For information about available fields see [`mod@comp_3`] module*/
pub type COMP_3 = crate::Reg<comp_3::COMP_3rs>;
///COMP_3 register
pub mod comp_3;
/**COMP_4 (rw) register accessor: COMP_4 register

You can [`read`](crate::Reg::read) this register and get [`comp_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:COMP_4)

For information about available fields see [`mod@comp_4`] module*/
pub type COMP_4 = crate::Reg<comp_4::COMP_4rs>;
///COMP_4 register
pub mod comp_4;
/**COMP_SEL (rw) register accessor: COMP_SEL register

You can [`read`](crate::Reg::read) this register and get [`comp_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:COMP_SEL)

For information about available fields see [`mod@comp_sel`] module*/
pub type COMP_SEL = crate::Reg<comp_sel::COMP_SELrs>;
///COMP_SEL register
pub mod comp_sel;
/**WD_TH (rw) register accessor: WD_TH register

You can [`read`](crate::Reg::read) this register and get [`wd_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:WD_TH)

For information about available fields see [`mod@wd_th`] module*/
pub type WD_TH = crate::Reg<wd_th::WD_THrs>;
///WD_TH register
pub mod wd_th;
/**WD_CONF (rw) register accessor: WD_CONF register

You can [`read`](crate::Reg::read) this register and get [`wd_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:WD_CONF)

For information about available fields see [`mod@wd_conf`] module*/
pub type WD_CONF = crate::Reg<wd_conf::WD_CONFrs>;
///WD_CONF register
pub mod wd_conf;
/**DS_DATAOUT (r) register accessor: DS_DATAOUT register

You can [`read`](crate::Reg::read) this register and get [`ds_dataout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:DS_DATAOUT)

For information about available fields see [`mod@ds_dataout`] module*/
pub type DS_DATAOUT = crate::Reg<ds_dataout::DS_DATAOUTrs>;
///DS_DATAOUT register
pub mod ds_dataout;
/**IRQ_STATUS (rw) register accessor: IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:IRQ_STATUS)

For information about available fields see [`mod@irq_status`] module*/
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUSrs>;
///IRQ_STATUS register
pub mod irq_status;
/**IRQ_ENABLE (rw) register accessor: IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:IRQ_ENABLE)

For information about available fields see [`mod@irq_enable`] module*/
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLErs>;
///IRQ_ENABLE register
pub mod irq_enable;
/**TEST_CONF (rw) register accessor: TEST_CONF register

You can [`read`](crate::Reg::read) this register and get [`test_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:TEST_CONF)

For information about available fields see [`mod@test_conf`] module*/
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONFrs>;
///TEST_CONF register
pub mod test_conf;
/**DTB_CONF (rw) register accessor: DTB_CONF register

You can [`read`](crate::Reg::read) this register and get [`dtb_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:DTB_CONF)

For information about available fields see [`mod@dtb_conf`] module*/
pub type DTB_CONF = crate::Reg<dtb_conf::DTB_CONFrs>;
///DTB_CONF register
pub mod dtb_conf;
