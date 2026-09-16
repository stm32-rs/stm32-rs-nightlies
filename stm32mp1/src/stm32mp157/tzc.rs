#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    build_config: BUILD_CONFIG,
    action: ACTION,
    gate_keeper: GATE_KEEPER,
    speculation_ctrl: SPECULATION_CTRL,
    int_status: INT_STATUS,
    int_clear: INT_CLEAR,
    _reserved6: [u8; 0x08],
    fail_address_low0: FAIL_ADDRESS_LOW0,
    fail_address_high0: FAIL_ADDRESS_HIGH0,
    fail_control0: FAIL_CONTROL0,
    fail_id0: FAIL_ID0,
    fail_address_low1: FAIL_ADDRESS_LOW1,
    fail_address_high1: FAIL_ADDRESS_HIGH1,
    fail_control1: FAIL_CONTROL1,
    fail_id1: FAIL_ID1,
    _reserved14: [u8; 0xc4],
    region_base_high0: REGION_BASE_HIGH0,
    region_top_low0: REGION_TOP_LOW0,
    region_top_high0: REGION_TOP_HIGH0,
    region_attribute0: REGION_ATTRIBUTE0,
    region_id_access0: REGION_ID_ACCESS0,
    _reserved19: [u8; 0x08],
    region_base_low1: REGION_BASE_LOW1,
    region_base_high1: REGION_BASE_HIGH1,
    region_top_low1: REGION_TOP_LOW1,
    region_top_high1: REGION_TOP_HIGH1,
    region_attribute1: REGION_ATTRIBUTE1,
    region_id_access1: REGION_ID_ACCESS1,
    _reserved25: [u8; 0x08],
    region_base_low2: REGION_BASE_LOW2,
    region_base_high2: REGION_BASE_HIGH2,
    region_top_low2: REGION_TOP_LOW2,
    region_top_high2: REGION_TOP_HIGH2,
    region_attribute2: REGION_ATTRIBUTE2,
    region_id_access2: REGION_ID_ACCESS2,
    _reserved31: [u8; 0x08],
    region_base_low3: REGION_BASE_LOW3,
    region_base_high3: REGION_BASE_HIGH3,
    region_top_low3: REGION_TOP_LOW3,
    region_top_high3: REGION_TOP_HIGH3,
    region_attribute3: REGION_ATTRIBUTE3,
    region_id_access3: REGION_ID_ACCESS3,
    _reserved37: [u8; 0x08],
    region_base_low4: REGION_BASE_LOW4,
    region_base_high4: REGION_BASE_HIGH4,
    region_top_low4: REGION_TOP_LOW4,
    region_top_high4: REGION_TOP_HIGH4,
    region_attribute4: REGION_ATTRIBUTE4,
    region_id_access4: REGION_ID_ACCESS4,
    _reserved43: [u8; 0x08],
    region_base_low5: REGION_BASE_LOW5,
    region_base_high5: REGION_BASE_HIGH5,
    region_top_low5: REGION_TOP_LOW5,
    region_top_high5: REGION_TOP_HIGH5,
    region_attribute5: REGION_ATTRIBUTE5,
    region_id_access5: REGION_ID_ACCESS5,
    _reserved49: [u8; 0x08],
    region_base_low6: REGION_BASE_LOW6,
    region_base_high6: REGION_BASE_HIGH6,
    region_top_low6: REGION_TOP_LOW6,
    region_top_high6: REGION_TOP_HIGH6,
    region_attribute6: REGION_ATTRIBUTE6,
    region_id_access6: REGION_ID_ACCESS6,
    _reserved55: [u8; 0x10],
    region_top_low7: REGION_TOP_LOW7,
    _reserved56: [u8; 0x04],
    region_attribute7: REGION_ATTRIBUTE7,
    _reserved57: [u8; 0x0c],
    region_base_low8: REGION_BASE_LOW8,
    region_base_high8: REGION_BASE_HIGH8,
    _reserved59: [u8; 0x08],
    region_attribute8: REGION_ATTRIBUTE8,
    _reserved60: [u8; 0xcc],
    region_base_low7: REGION_BASE_LOW7,
    region_base_high7: REGION_BASE_HIGH7,
    _reserved62: [u8; 0x04],
    region_top_high7: REGION_TOP_HIGH7,
    _reserved63: [u8; 0x04],
    region_id_access7: REGION_ID_ACCESS7,
    _reserved64: [u8; 0x10],
    region_top_low8: REGION_TOP_LOW8,
    region_top_high8: REGION_TOP_HIGH8,
    _reserved66: [u8; 0x04],
    region_id_access8: REGION_ID_ACCESS8,
    _reserved67: [u8; 0x0cb8],
    pid4: PID4,
    pid5: PID5,
    pid6: PID6,
    pid7: PID7,
    pid0: PID0,
    pid1: PID1,
    pid2: PID2,
    pid3: PID3,
    cid0: CID0,
    cid1: CID1,
    cid2: CID2,
    cid3: CID3,
}
impl RegisterBlock {
    ///0x00 - Provides information about TZC configuration.
    #[inline(always)]
    pub const fn build_config(&self) -> &BUILD_CONFIG {
        &self.build_config
    }
    ///0x04 - Controls interrupt and bus error response behavior when regions permission failures occur.
    #[inline(always)]
    pub const fn action(&self) -> &ACTION {
        &self.action
    }
    ///0x08 - Provides control and status for the gate keeper in each filter unit implemented.
    #[inline(always)]
    pub const fn gate_keeper(&self) -> &GATE_KEEPER {
        &self.gate_keeper
    }
    ///0x0c - Controls read and write access speculation.
    #[inline(always)]
    pub const fn speculation_ctrl(&self) -> &SPECULATION_CTRL {
        &self.speculation_ctrl
    }
    ///0x10 - Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.
    #[inline(always)]
    pub const fn int_status(&self) -> &INT_STATUS {
        &self.int_status
    }
    ///0x14 - Interrupt clear for each filter.
    #[inline(always)]
    pub const fn int_clear(&self) -> &INT_CLEAR {
        &self.int_clear
    }
    ///0x20 - Address low bits of the first failed access in the associated filter (0 to 1).
    #[inline(always)]
    pub const fn fail_address_low0(&self) -> &FAIL_ADDRESS_LOW0 {
        &self.fail_address_low0
    }
    ///0x24 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
    #[inline(always)]
    pub const fn fail_address_high0(&self) -> &FAIL_ADDRESS_HIGH0 {
        &self.fail_address_high0
    }
    ///0x28 - Status information about the first access that failed a region permission check in the associated filter (0 to 1).
    #[inline(always)]
    pub const fn fail_control0(&self) -> &FAIL_CONTROL0 {
        &self.fail_control0
    }
    ///0x2c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
    #[inline(always)]
    pub const fn fail_id0(&self) -> &FAIL_ID0 {
        &self.fail_id0
    }
    ///0x30 - Address low bits of the first failed access in the associated filter (0 to 1).
    #[inline(always)]
    pub const fn fail_address_low1(&self) -> &FAIL_ADDRESS_LOW1 {
        &self.fail_address_low1
    }
    ///0x34 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
    #[inline(always)]
    pub const fn fail_address_high1(&self) -> &FAIL_ADDRESS_HIGH1 {
        &self.fail_address_high1
    }
    ///0x38 - Status information about the first access that failed a region permission check in the associated filter (0 to 1).
    #[inline(always)]
    pub const fn fail_control1(&self) -> &FAIL_CONTROL1 {
        &self.fail_control1
    }
    ///0x3c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
    #[inline(always)]
    pub const fn fail_id1(&self) -> &FAIL_ID1 {
        &self.fail_id1
    }
    ///0x104 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high0(&self) -> &REGION_BASE_HIGH0 {
        &self.region_base_high0
    }
    ///0x108 - Top address bits \[31:12\] for region 0.
    #[inline(always)]
    pub const fn region_top_low0(&self) -> &REGION_TOP_LOW0 {
        &self.region_top_low0
    }
    ///0x10c - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high0(&self) -> &REGION_TOP_HIGH0 {
        &self.region_top_high0
    }
    ///0x110 - Region 0 attributes.
    #[inline(always)]
    pub const fn region_attribute0(&self) -> &REGION_ATTRIBUTE0 {
        &self.region_attribute0
    }
    ///0x114 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access0(&self) -> &REGION_ID_ACCESS0 {
        &self.region_id_access0
    }
    ///0x120 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low1(&self) -> &REGION_BASE_LOW1 {
        &self.region_base_low1
    }
    ///0x124 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high1(&self) -> &REGION_BASE_HIGH1 {
        &self.region_base_high1
    }
    ///0x128 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low1(&self) -> &REGION_TOP_LOW1 {
        &self.region_top_low1
    }
    ///0x12c - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high1(&self) -> &REGION_TOP_HIGH1 {
        &self.region_top_high1
    }
    ///0x130 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute1(&self) -> &REGION_ATTRIBUTE1 {
        &self.region_attribute1
    }
    ///0x134 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access1(&self) -> &REGION_ID_ACCESS1 {
        &self.region_id_access1
    }
    ///0x140 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low2(&self) -> &REGION_BASE_LOW2 {
        &self.region_base_low2
    }
    ///0x144 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high2(&self) -> &REGION_BASE_HIGH2 {
        &self.region_base_high2
    }
    ///0x148 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low2(&self) -> &REGION_TOP_LOW2 {
        &self.region_top_low2
    }
    ///0x14c - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high2(&self) -> &REGION_TOP_HIGH2 {
        &self.region_top_high2
    }
    ///0x150 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute2(&self) -> &REGION_ATTRIBUTE2 {
        &self.region_attribute2
    }
    ///0x154 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access2(&self) -> &REGION_ID_ACCESS2 {
        &self.region_id_access2
    }
    ///0x160 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low3(&self) -> &REGION_BASE_LOW3 {
        &self.region_base_low3
    }
    ///0x164 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high3(&self) -> &REGION_BASE_HIGH3 {
        &self.region_base_high3
    }
    ///0x168 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low3(&self) -> &REGION_TOP_LOW3 {
        &self.region_top_low3
    }
    ///0x16c - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high3(&self) -> &REGION_TOP_HIGH3 {
        &self.region_top_high3
    }
    ///0x170 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute3(&self) -> &REGION_ATTRIBUTE3 {
        &self.region_attribute3
    }
    ///0x174 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access3(&self) -> &REGION_ID_ACCESS3 {
        &self.region_id_access3
    }
    ///0x180 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low4(&self) -> &REGION_BASE_LOW4 {
        &self.region_base_low4
    }
    ///0x184 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high4(&self) -> &REGION_BASE_HIGH4 {
        &self.region_base_high4
    }
    ///0x188 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low4(&self) -> &REGION_TOP_LOW4 {
        &self.region_top_low4
    }
    ///0x18c - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high4(&self) -> &REGION_TOP_HIGH4 {
        &self.region_top_high4
    }
    ///0x190 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute4(&self) -> &REGION_ATTRIBUTE4 {
        &self.region_attribute4
    }
    ///0x194 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access4(&self) -> &REGION_ID_ACCESS4 {
        &self.region_id_access4
    }
    ///0x1a0 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low5(&self) -> &REGION_BASE_LOW5 {
        &self.region_base_low5
    }
    ///0x1a4 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high5(&self) -> &REGION_BASE_HIGH5 {
        &self.region_base_high5
    }
    ///0x1a8 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low5(&self) -> &REGION_TOP_LOW5 {
        &self.region_top_low5
    }
    ///0x1ac - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high5(&self) -> &REGION_TOP_HIGH5 {
        &self.region_top_high5
    }
    ///0x1b0 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute5(&self) -> &REGION_ATTRIBUTE5 {
        &self.region_attribute5
    }
    ///0x1b4 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access5(&self) -> &REGION_ID_ACCESS5 {
        &self.region_id_access5
    }
    ///0x1c0 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low6(&self) -> &REGION_BASE_LOW6 {
        &self.region_base_low6
    }
    ///0x1c4 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high6(&self) -> &REGION_BASE_HIGH6 {
        &self.region_base_high6
    }
    ///0x1c8 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low6(&self) -> &REGION_TOP_LOW6 {
        &self.region_top_low6
    }
    ///0x1cc - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high6(&self) -> &REGION_TOP_HIGH6 {
        &self.region_top_high6
    }
    ///0x1d0 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute6(&self) -> &REGION_ATTRIBUTE6 {
        &self.region_attribute6
    }
    ///0x1d4 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access6(&self) -> &REGION_ID_ACCESS6 {
        &self.region_id_access6
    }
    ///0x1e8 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low7(&self) -> &REGION_TOP_LOW7 {
        &self.region_top_low7
    }
    ///0x1f0 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute7(&self) -> &REGION_ATTRIBUTE7 {
        &self.region_attribute7
    }
    ///0x200 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low8(&self) -> &REGION_BASE_LOW8 {
        &self.region_base_low8
    }
    ///0x204 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high8(&self) -> &REGION_BASE_HIGH8 {
        &self.region_base_high8
    }
    ///0x210 - Region x attributes.
    #[inline(always)]
    pub const fn region_attribute8(&self) -> &REGION_ATTRIBUTE8 {
        &self.region_attribute8
    }
    ///0x2e0 - Base address low for regions 1 to 8.
    #[inline(always)]
    pub const fn region_base_low7(&self) -> &REGION_BASE_LOW7 {
        &self.region_base_low7
    }
    ///0x2e4 - Base address high are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_base_high7(&self) -> &REGION_BASE_HIGH7 {
        &self.region_base_high7
    }
    ///0x2ec - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high7(&self) -> &REGION_TOP_HIGH7 {
        &self.region_top_high7
    }
    ///0x2f4 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access7(&self) -> &REGION_ID_ACCESS7 {
        &self.region_id_access7
    }
    ///0x308 - Top address bits \[31:12\] for region x.
    #[inline(always)]
    pub const fn region_top_low8(&self) -> &REGION_TOP_LOW8 {
        &self.region_top_low8
    }
    ///0x30c - Top address high of region are not used with 32-bit address.
    #[inline(always)]
    pub const fn region_top_high8(&self) -> &REGION_TOP_HIGH8 {
        &self.region_top_high8
    }
    ///0x314 - Region non-secure access based on NSAID.
    #[inline(always)]
    pub const fn region_id_access8(&self) -> &REGION_ID_ACCESS8 {
        &self.region_id_access8
    }
    ///0xfd0 - Peripheral ID 4.
    #[inline(always)]
    pub const fn pid4(&self) -> &PID4 {
        &self.pid4
    }
    ///0xfd4 - Peripheral ID 5.
    #[inline(always)]
    pub const fn pid5(&self) -> &PID5 {
        &self.pid5
    }
    ///0xfd8 - Peripheral ID 6.
    #[inline(always)]
    pub const fn pid6(&self) -> &PID6 {
        &self.pid6
    }
    ///0xfdc - Peripheral ID 7.
    #[inline(always)]
    pub const fn pid7(&self) -> &PID7 {
        &self.pid7
    }
    ///0xfe0 - Peripheral ID 0.
    #[inline(always)]
    pub const fn pid0(&self) -> &PID0 {
        &self.pid0
    }
    ///0xfe4 - Peripheral ID 1.
    #[inline(always)]
    pub const fn pid1(&self) -> &PID1 {
        &self.pid1
    }
    ///0xfe8 - Peripheral ID 2.
    #[inline(always)]
    pub const fn pid2(&self) -> &PID2 {
        &self.pid2
    }
    ///0xfec - Peripheral ID 3.
    #[inline(always)]
    pub const fn pid3(&self) -> &PID3 {
        &self.pid3
    }
    ///0xff0 - Component ID 0.
    #[inline(always)]
    pub const fn cid0(&self) -> &CID0 {
        &self.cid0
    }
    ///0xff4 - Component ID 1.
    #[inline(always)]
    pub const fn cid1(&self) -> &CID1 {
        &self.cid1
    }
    ///0xff8 - Component ID 2.
    #[inline(always)]
    pub const fn cid2(&self) -> &CID2 {
        &self.cid2
    }
    ///0xffc - Component ID 3.
    #[inline(always)]
    pub const fn cid3(&self) -> &CID3 {
        &self.cid3
    }
}
/**BUILD_CONFIG (r) register accessor: Provides information about TZC configuration.

You can [`read`](crate::Reg::read) this register and get [`build_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:BUILD_CONFIG)

For information about available fields see [`mod@build_config`] module*/
pub type BUILD_CONFIG = crate::Reg<build_config::BUILD_CONFIGrs>;
///Provides information about TZC configuration.
pub mod build_config;
/**ACTION (rw) register accessor: Controls interrupt and bus error response behavior when regions permission failures occur.

You can [`read`](crate::Reg::read) this register and get [`action::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`action::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:ACTION)

For information about available fields see [`mod@action`] module*/
pub type ACTION = crate::Reg<action::ACTIONrs>;
///Controls interrupt and bus error response behavior when regions permission failures occur.
pub mod action;
/**GATE_KEEPER (rw) register accessor: Provides control and status for the gate keeper in each filter unit implemented.

You can [`read`](crate::Reg::read) this register and get [`gate_keeper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gate_keeper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:GATE_KEEPER)

For information about available fields see [`mod@gate_keeper`] module*/
pub type GATE_KEEPER = crate::Reg<gate_keeper::GATE_KEEPERrs>;
///Provides control and status for the gate keeper in each filter unit implemented.
pub mod gate_keeper;
/**SPECULATION_CTRL (rw) register accessor: Controls read and write access speculation.

You can [`read`](crate::Reg::read) this register and get [`speculation_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`speculation_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:SPECULATION_CTRL)

For information about available fields see [`mod@speculation_ctrl`] module*/
pub type SPECULATION_CTRL = crate::Reg<speculation_ctrl::SPECULATION_CTRLrs>;
///Controls read and write access speculation.
pub mod speculation_ctrl;
/**INT_STATUS (r) register accessor: Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.

You can [`read`](crate::Reg::read) this register and get [`int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:INT_STATUS)

For information about available fields see [`mod@int_status`] module*/
pub type INT_STATUS = crate::Reg<int_status::INT_STATUSrs>;
///Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.
pub mod int_status;
/**INT_CLEAR (rw) register accessor: Interrupt clear for each filter.

You can [`read`](crate::Reg::read) this register and get [`int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:INT_CLEAR)

For information about available fields see [`mod@int_clear`] module*/
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEARrs>;
///Interrupt clear for each filter.
pub mod int_clear;
/**FAIL_CONTROL0 (r) register accessor: Status information about the first access that failed a region permission check in the associated filter (0 to 1).

You can [`read`](crate::Reg::read) this register and get [`fail_control0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_CONTROL0)

For information about available fields see [`mod@fail_control0`] module*/
pub type FAIL_CONTROL0 = crate::Reg<fail_control0::FAIL_CONTROL0rs>;
///Status information about the first access that failed a region permission check in the associated filter (0 to 1).
pub mod fail_control0;
/**FAIL_ID0 (r) register accessor: Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).

You can [`read`](crate::Reg::read) this register and get [`fail_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_ID0)

For information about available fields see [`mod@fail_id0`] module*/
pub type FAIL_ID0 = crate::Reg<fail_id0::FAIL_ID0rs>;
///Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
pub mod fail_id0;
/**FAIL_CONTROL1 (r) register accessor: Status information about the first access that failed a region permission check in the associated filter (0 to 1).

You can [`read`](crate::Reg::read) this register and get [`fail_control1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_CONTROL1)

For information about available fields see [`mod@fail_control1`] module*/
pub type FAIL_CONTROL1 = crate::Reg<fail_control1::FAIL_CONTROL1rs>;
///Status information about the first access that failed a region permission check in the associated filter (0 to 1).
pub mod fail_control1;
/**FAIL_ID1 (r) register accessor: Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).

You can [`read`](crate::Reg::read) this register and get [`fail_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_ID1)

For information about available fields see [`mod@fail_id1`] module*/
pub type FAIL_ID1 = crate::Reg<fail_id1::FAIL_ID1rs>;
///Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
pub mod fail_id1;
/**REGION_ATTRIBUTE0 (rw) register accessor: Region 0 attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE0)

For information about available fields see [`mod@region_attribute0`] module*/
pub type REGION_ATTRIBUTE0 = crate::Reg<region_attribute0::REGION_ATTRIBUTE0rs>;
///Region 0 attributes.
pub mod region_attribute0;
/**REGION_ATTRIBUTE1 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE1)

For information about available fields see [`mod@region_attribute1`] module*/
pub type REGION_ATTRIBUTE1 = crate::Reg<region_attribute1::REGION_ATTRIBUTE1rs>;
///Region x attributes.
pub mod region_attribute1;
/**REGION_ATTRIBUTE2 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE2)

For information about available fields see [`mod@region_attribute2`] module*/
pub type REGION_ATTRIBUTE2 = crate::Reg<region_attribute2::REGION_ATTRIBUTE2rs>;
///Region x attributes.
pub mod region_attribute2;
/**REGION_ATTRIBUTE3 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE3)

For information about available fields see [`mod@region_attribute3`] module*/
pub type REGION_ATTRIBUTE3 = crate::Reg<region_attribute3::REGION_ATTRIBUTE3rs>;
///Region x attributes.
pub mod region_attribute3;
/**REGION_ATTRIBUTE4 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE4)

For information about available fields see [`mod@region_attribute4`] module*/
pub type REGION_ATTRIBUTE4 = crate::Reg<region_attribute4::REGION_ATTRIBUTE4rs>;
///Region x attributes.
pub mod region_attribute4;
/**REGION_ATTRIBUTE5 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE5)

For information about available fields see [`mod@region_attribute5`] module*/
pub type REGION_ATTRIBUTE5 = crate::Reg<region_attribute5::REGION_ATTRIBUTE5rs>;
///Region x attributes.
pub mod region_attribute5;
/**REGION_ATTRIBUTE6 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE6)

For information about available fields see [`mod@region_attribute6`] module*/
pub type REGION_ATTRIBUTE6 = crate::Reg<region_attribute6::REGION_ATTRIBUTE6rs>;
///Region x attributes.
pub mod region_attribute6;
/**REGION_ATTRIBUTE7 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE7)

For information about available fields see [`mod@region_attribute7`] module*/
pub type REGION_ATTRIBUTE7 = crate::Reg<region_attribute7::REGION_ATTRIBUTE7rs>;
///Region x attributes.
pub mod region_attribute7;
/**REGION_ATTRIBUTE8 (rw) register accessor: Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ATTRIBUTE8)

For information about available fields see [`mod@region_attribute8`] module*/
pub type REGION_ATTRIBUTE8 = crate::Reg<region_attribute8::REGION_ATTRIBUTE8rs>;
///Region x attributes.
pub mod region_attribute8;
/**PID4 (r) register accessor: Peripheral ID 4.

You can [`read`](crate::Reg::read) this register and get [`pid4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID4)

For information about available fields see [`mod@pid4`] module*/
pub type PID4 = crate::Reg<pid4::PID4rs>;
///Peripheral ID 4.
pub mod pid4;
/**PID5 (r) register accessor: Peripheral ID 5.

You can [`read`](crate::Reg::read) this register and get [`pid5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID5)

For information about available fields see [`mod@pid5`] module*/
pub type PID5 = crate::Reg<pid5::PID5rs>;
///Peripheral ID 5.
pub mod pid5;
/**PID6 (r) register accessor: Peripheral ID 6.

You can [`read`](crate::Reg::read) this register and get [`pid6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID6)

For information about available fields see [`mod@pid6`] module*/
pub type PID6 = crate::Reg<pid6::PID6rs>;
///Peripheral ID 6.
pub mod pid6;
/**PID7 (r) register accessor: Peripheral ID 7.

You can [`read`](crate::Reg::read) this register and get [`pid7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID7)

For information about available fields see [`mod@pid7`] module*/
pub type PID7 = crate::Reg<pid7::PID7rs>;
///Peripheral ID 7.
pub mod pid7;
/**PID0 (r) register accessor: Peripheral ID 0.

You can [`read`](crate::Reg::read) this register and get [`pid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID0)

For information about available fields see [`mod@pid0`] module*/
pub type PID0 = crate::Reg<pid0::PID0rs>;
///Peripheral ID 0.
pub mod pid0;
/**PID1 (r) register accessor: Peripheral ID 1.

You can [`read`](crate::Reg::read) this register and get [`pid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID1)

For information about available fields see [`mod@pid1`] module*/
pub type PID1 = crate::Reg<pid1::PID1rs>;
///Peripheral ID 1.
pub mod pid1;
/**PID2 (r) register accessor: Peripheral ID 2.

You can [`read`](crate::Reg::read) this register and get [`pid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID2)

For information about available fields see [`mod@pid2`] module*/
pub type PID2 = crate::Reg<pid2::PID2rs>;
///Peripheral ID 2.
pub mod pid2;
/**PID3 (r) register accessor: Peripheral ID 3.

You can [`read`](crate::Reg::read) this register and get [`pid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:PID3)

For information about available fields see [`mod@pid3`] module*/
pub type PID3 = crate::Reg<pid3::PID3rs>;
///Peripheral ID 3.
pub mod pid3;
/**CID0 (r) register accessor: Component ID 0.

You can [`read`](crate::Reg::read) this register and get [`cid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:CID0)

For information about available fields see [`mod@cid0`] module*/
pub type CID0 = crate::Reg<cid0::CID0rs>;
///Component ID 0.
pub mod cid0;
/**CID1 (r) register accessor: Component ID 1.

You can [`read`](crate::Reg::read) this register and get [`cid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:CID1)

For information about available fields see [`mod@cid1`] module*/
pub type CID1 = crate::Reg<cid1::CID1rs>;
///Component ID 1.
pub mod cid1;
/**CID2 (r) register accessor: Component ID 2.

You can [`read`](crate::Reg::read) this register and get [`cid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:CID2)

For information about available fields see [`mod@cid2`] module*/
pub type CID2 = crate::Reg<cid2::CID2rs>;
///Component ID 2.
pub mod cid2;
/**CID3 (r) register accessor: Component ID 3.

You can [`read`](crate::Reg::read) this register and get [`cid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:CID3)

For information about available fields see [`mod@cid3`] module*/
pub type CID3 = crate::Reg<cid3::CID3rs>;
///Component ID 3.
pub mod cid3;
/**FAIL_ADDRESS_LOW0 (r) register accessor: Address low bits of the first failed access in the associated filter (0 to 1).

You can [`read`](crate::Reg::read) this register and get [`fail_address_low0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_ADDRESS_LOW0)

For information about available fields see [`mod@fail_address_low0`] module*/
pub type FAIL_ADDRESS_LOW0 = crate::Reg<fail_address_low0::FAIL_ADDRESS_LOW0rs>;
///Address low bits of the first failed access in the associated filter (0 to 1).
pub mod fail_address_low0;
/**FAIL_ADDRESS_HIGH0 (r) register accessor: Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.

You can [`read`](crate::Reg::read) this register and get [`fail_address_high0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_ADDRESS_HIGH0)

For information about available fields see [`mod@fail_address_high0`] module*/
pub type FAIL_ADDRESS_HIGH0 = crate::Reg<fail_address_high0::FAIL_ADDRESS_HIGH0rs>;
///Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
pub mod fail_address_high0;
/**FAIL_ADDRESS_LOW1 (r) register accessor: Address low bits of the first failed access in the associated filter (0 to 1).

You can [`read`](crate::Reg::read) this register and get [`fail_address_low1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_ADDRESS_LOW1)

For information about available fields see [`mod@fail_address_low1`] module*/
pub type FAIL_ADDRESS_LOW1 = crate::Reg<fail_address_low1::FAIL_ADDRESS_LOW1rs>;
///Address low bits of the first failed access in the associated filter (0 to 1).
pub mod fail_address_low1;
/**FAIL_ADDRESS_HIGH1 (r) register accessor: Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.

You can [`read`](crate::Reg::read) this register and get [`fail_address_high1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_ADDRESS_HIGH1)

For information about available fields see [`mod@fail_address_high1`] module*/
pub type FAIL_ADDRESS_HIGH1 = crate::Reg<fail_address_high1::FAIL_ADDRESS_HIGH1rs>;
///Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
pub mod fail_address_high1;
/**REGION_BASE_HIGH0 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH0)

For information about available fields see [`mod@region_base_high0`] module*/
pub type REGION_BASE_HIGH0 = crate::Reg<region_base_high0::REGION_BASE_HIGH0rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high0;
/**REGION_TOP_LOW0 (r) register accessor: Top address bits \[31:12\] for region 0.

You can [`read`](crate::Reg::read) this register and get [`region_top_low0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW0)

For information about available fields see [`mod@region_top_low0`] module*/
pub type REGION_TOP_LOW0 = crate::Reg<region_top_low0::REGION_TOP_LOW0rs>;
///Top address bits \[31:12\] for region 0.
pub mod region_top_low0;
/**REGION_TOP_HIGH0 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH0)

For information about available fields see [`mod@region_top_high0`] module*/
pub type REGION_TOP_HIGH0 = crate::Reg<region_top_high0::REGION_TOP_HIGH0rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high0;
/**REGION_ID_ACCESS0 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS0)

For information about available fields see [`mod@region_id_access0`] module*/
pub type REGION_ID_ACCESS0 = crate::Reg<region_id_access0::REGION_ID_ACCESS0rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access0;
/**REGION_BASE_LOW1 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW1)

For information about available fields see [`mod@region_base_low1`] module*/
pub type REGION_BASE_LOW1 = crate::Reg<region_base_low1::REGION_BASE_LOW1rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low1;
/**REGION_BASE_HIGH1 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH1)

For information about available fields see [`mod@region_base_high1`] module*/
pub type REGION_BASE_HIGH1 = crate::Reg<region_base_high1::REGION_BASE_HIGH1rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high1;
/**REGION_TOP_LOW1 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW1)

For information about available fields see [`mod@region_top_low1`] module*/
pub type REGION_TOP_LOW1 = crate::Reg<region_top_low1::REGION_TOP_LOW1rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low1;
/**REGION_TOP_HIGH1 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH1)

For information about available fields see [`mod@region_top_high1`] module*/
pub type REGION_TOP_HIGH1 = crate::Reg<region_top_high1::REGION_TOP_HIGH1rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high1;
/**REGION_ID_ACCESS1 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS1)

For information about available fields see [`mod@region_id_access1`] module*/
pub type REGION_ID_ACCESS1 = crate::Reg<region_id_access1::REGION_ID_ACCESS1rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access1;
/**REGION_BASE_LOW2 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW2)

For information about available fields see [`mod@region_base_low2`] module*/
pub type REGION_BASE_LOW2 = crate::Reg<region_base_low2::REGION_BASE_LOW2rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low2;
/**REGION_BASE_HIGH2 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH2)

For information about available fields see [`mod@region_base_high2`] module*/
pub type REGION_BASE_HIGH2 = crate::Reg<region_base_high2::REGION_BASE_HIGH2rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high2;
/**REGION_TOP_LOW2 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW2)

For information about available fields see [`mod@region_top_low2`] module*/
pub type REGION_TOP_LOW2 = crate::Reg<region_top_low2::REGION_TOP_LOW2rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low2;
/**REGION_TOP_HIGH2 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH2)

For information about available fields see [`mod@region_top_high2`] module*/
pub type REGION_TOP_HIGH2 = crate::Reg<region_top_high2::REGION_TOP_HIGH2rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high2;
/**REGION_ID_ACCESS2 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS2)

For information about available fields see [`mod@region_id_access2`] module*/
pub type REGION_ID_ACCESS2 = crate::Reg<region_id_access2::REGION_ID_ACCESS2rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access2;
/**REGION_BASE_LOW3 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW3)

For information about available fields see [`mod@region_base_low3`] module*/
pub type REGION_BASE_LOW3 = crate::Reg<region_base_low3::REGION_BASE_LOW3rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low3;
/**REGION_BASE_HIGH3 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH3)

For information about available fields see [`mod@region_base_high3`] module*/
pub type REGION_BASE_HIGH3 = crate::Reg<region_base_high3::REGION_BASE_HIGH3rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high3;
/**REGION_TOP_LOW3 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW3)

For information about available fields see [`mod@region_top_low3`] module*/
pub type REGION_TOP_LOW3 = crate::Reg<region_top_low3::REGION_TOP_LOW3rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low3;
/**REGION_TOP_HIGH3 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH3)

For information about available fields see [`mod@region_top_high3`] module*/
pub type REGION_TOP_HIGH3 = crate::Reg<region_top_high3::REGION_TOP_HIGH3rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high3;
/**REGION_ID_ACCESS3 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS3)

For information about available fields see [`mod@region_id_access3`] module*/
pub type REGION_ID_ACCESS3 = crate::Reg<region_id_access3::REGION_ID_ACCESS3rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access3;
/**REGION_BASE_LOW4 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW4)

For information about available fields see [`mod@region_base_low4`] module*/
pub type REGION_BASE_LOW4 = crate::Reg<region_base_low4::REGION_BASE_LOW4rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low4;
/**REGION_BASE_HIGH4 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH4)

For information about available fields see [`mod@region_base_high4`] module*/
pub type REGION_BASE_HIGH4 = crate::Reg<region_base_high4::REGION_BASE_HIGH4rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high4;
/**REGION_TOP_LOW4 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW4)

For information about available fields see [`mod@region_top_low4`] module*/
pub type REGION_TOP_LOW4 = crate::Reg<region_top_low4::REGION_TOP_LOW4rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low4;
/**REGION_TOP_HIGH4 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH4)

For information about available fields see [`mod@region_top_high4`] module*/
pub type REGION_TOP_HIGH4 = crate::Reg<region_top_high4::REGION_TOP_HIGH4rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high4;
/**REGION_ID_ACCESS4 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS4)

For information about available fields see [`mod@region_id_access4`] module*/
pub type REGION_ID_ACCESS4 = crate::Reg<region_id_access4::REGION_ID_ACCESS4rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access4;
/**REGION_BASE_LOW5 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW5)

For information about available fields see [`mod@region_base_low5`] module*/
pub type REGION_BASE_LOW5 = crate::Reg<region_base_low5::REGION_BASE_LOW5rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low5;
/**REGION_BASE_HIGH5 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH5)

For information about available fields see [`mod@region_base_high5`] module*/
pub type REGION_BASE_HIGH5 = crate::Reg<region_base_high5::REGION_BASE_HIGH5rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high5;
/**REGION_TOP_LOW5 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW5)

For information about available fields see [`mod@region_top_low5`] module*/
pub type REGION_TOP_LOW5 = crate::Reg<region_top_low5::REGION_TOP_LOW5rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low5;
/**REGION_TOP_HIGH5 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH5)

For information about available fields see [`mod@region_top_high5`] module*/
pub type REGION_TOP_HIGH5 = crate::Reg<region_top_high5::REGION_TOP_HIGH5rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high5;
/**REGION_ID_ACCESS5 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS5)

For information about available fields see [`mod@region_id_access5`] module*/
pub type REGION_ID_ACCESS5 = crate::Reg<region_id_access5::REGION_ID_ACCESS5rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access5;
/**REGION_BASE_LOW6 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW6)

For information about available fields see [`mod@region_base_low6`] module*/
pub type REGION_BASE_LOW6 = crate::Reg<region_base_low6::REGION_BASE_LOW6rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low6;
/**REGION_BASE_HIGH6 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH6)

For information about available fields see [`mod@region_base_high6`] module*/
pub type REGION_BASE_HIGH6 = crate::Reg<region_base_high6::REGION_BASE_HIGH6rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high6;
/**REGION_TOP_LOW6 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW6)

For information about available fields see [`mod@region_top_low6`] module*/
pub type REGION_TOP_LOW6 = crate::Reg<region_top_low6::REGION_TOP_LOW6rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low6;
/**REGION_TOP_HIGH6 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH6)

For information about available fields see [`mod@region_top_high6`] module*/
pub type REGION_TOP_HIGH6 = crate::Reg<region_top_high6::REGION_TOP_HIGH6rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high6;
/**REGION_ID_ACCESS6 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS6)

For information about available fields see [`mod@region_id_access6`] module*/
pub type REGION_ID_ACCESS6 = crate::Reg<region_id_access6::REGION_ID_ACCESS6rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access6;
/**REGION_BASE_LOW7 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW7)

For information about available fields see [`mod@region_base_low7`] module*/
pub type REGION_BASE_LOW7 = crate::Reg<region_base_low7::REGION_BASE_LOW7rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low7;
/**REGION_BASE_HIGH7 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH7)

For information about available fields see [`mod@region_base_high7`] module*/
pub type REGION_BASE_HIGH7 = crate::Reg<region_base_high7::REGION_BASE_HIGH7rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high7;
/**REGION_TOP_LOW7 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW7)

For information about available fields see [`mod@region_top_low7`] module*/
pub type REGION_TOP_LOW7 = crate::Reg<region_top_low7::REGION_TOP_LOW7rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low7;
/**REGION_TOP_HIGH7 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH7)

For information about available fields see [`mod@region_top_high7`] module*/
pub type REGION_TOP_HIGH7 = crate::Reg<region_top_high7::REGION_TOP_HIGH7rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high7;
/**REGION_ID_ACCESS7 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS7)

For information about available fields see [`mod@region_id_access7`] module*/
pub type REGION_ID_ACCESS7 = crate::Reg<region_id_access7::REGION_ID_ACCESS7rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access7;
/**REGION_BASE_LOW8 (rw) register accessor: Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW8)

For information about available fields see [`mod@region_base_low8`] module*/
pub type REGION_BASE_LOW8 = crate::Reg<region_base_low8::REGION_BASE_LOW8rs>;
///Base address low for regions 1 to 8.
pub mod region_base_low8;
/**REGION_BASE_HIGH8 (r) register accessor: Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH8)

For information about available fields see [`mod@region_base_high8`] module*/
pub type REGION_BASE_HIGH8 = crate::Reg<region_base_high8::REGION_BASE_HIGH8rs>;
///Base address high are not used with 32-bit address.
pub mod region_base_high8;
/**REGION_TOP_LOW8 (rw) register accessor: Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW8)

For information about available fields see [`mod@region_top_low8`] module*/
pub type REGION_TOP_LOW8 = crate::Reg<region_top_low8::REGION_TOP_LOW8rs>;
///Top address bits \[31:12\] for region x.
pub mod region_top_low8;
/**REGION_TOP_HIGH8 (r) register accessor: Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH8)

For information about available fields see [`mod@region_top_high8`] module*/
pub type REGION_TOP_HIGH8 = crate::Reg<region_top_high8::REGION_TOP_HIGH8rs>;
///Top address high of region are not used with 32-bit address.
pub mod region_top_high8;
/**REGION_ID_ACCESS8 (rw) register accessor: Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS8)

For information about available fields see [`mod@region_id_access8`] module*/
pub type REGION_ID_ACCESS8 = crate::Reg<region_id_access8::REGION_ID_ACCESS8rs>;
///Region non-secure access based on NSAID.
pub mod region_id_access8;
