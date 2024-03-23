#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tzc_build_config: TZC_BUILD_CONFIG,
    tzc_action: TZC_ACTION,
    tzc_gate_keeper: TZC_GATE_KEEPER,
    tzc_speculation_ctrl: TZC_SPECULATION_CTRL,
    tzc_int_status: TZC_INT_STATUS,
    tzc_int_clear: TZC_INT_CLEAR,
    _reserved6: [u8; 0x08],
    tzc_fail_address_low0: TZC_FAIL_ADDRESS_LOW0,
    tzc_fail_address_high0: TZC_FAIL_ADDRESS_HIGH0,
    tzc_fail_control0: TZC_FAIL_CONTROL0,
    tzc_fail_id0: TZC_FAIL_ID0,
    tzc_fail_address_low1: TZC_FAIL_ADDRESS_LOW1,
    tzc_fail_address_high1: TZC_FAIL_ADDRESS_HIGH1,
    tzc_fail_control1: TZC_FAIL_CONTROL1,
    tzc_fail_id1: TZC_FAIL_ID1,
    _reserved14: [u8; 0xc4],
    tzc_region_base_high0: TZC_REGION_BASE_HIGH0,
    tzc_region_top_low0: TZC_REGION_TOP_LOW0,
    tzc_region_top_high0: TZC_REGION_TOP_HIGH0,
    tzc_region_attribute0: TZC_REGION_ATTRIBUTE0,
    tzc_region_id_access0: TZC_REGION_ID_ACCESS0,
    _reserved19: [u8; 0x08],
    tzc_region_base_low1: TZC_REGION_BASE_LOW1,
    tzc_region_base_high1: TZC_REGION_BASE_HIGH1,
    tzc_region_top_low1: TZC_REGION_TOP_LOW1,
    tzc_region_top_high1: TZC_REGION_TOP_HIGH1,
    tzc_region_attribute1: TZC_REGION_ATTRIBUTE1,
    tzc_region_id_access1: TZC_REGION_ID_ACCESS1,
    _reserved25: [u8; 0x08],
    tzc_region_base_low2: TZC_REGION_BASE_LOW2,
    tzc_region_base_high2: TZC_REGION_BASE_HIGH2,
    tzc_region_top_low2: TZC_REGION_TOP_LOW2,
    tzc_region_top_high2: TZC_REGION_TOP_HIGH2,
    tzc_region_attribute2: TZC_REGION_ATTRIBUTE2,
    tzc_region_id_access2: TZC_REGION_ID_ACCESS2,
    _reserved31: [u8; 0x08],
    tzc_region_base_low3: TZC_REGION_BASE_LOW3,
    tzc_region_base_high3: TZC_REGION_BASE_HIGH3,
    tzc_region_top_low3: TZC_REGION_TOP_LOW3,
    tzc_region_top_high3: TZC_REGION_TOP_HIGH3,
    tzc_region_attribute3: TZC_REGION_ATTRIBUTE3,
    tzc_region_id_access3: TZC_REGION_ID_ACCESS3,
    _reserved37: [u8; 0x08],
    tzc_region_base_low4: TZC_REGION_BASE_LOW4,
    tzc_region_base_high4: TZC_REGION_BASE_HIGH4,
    tzc_region_top_low4: TZC_REGION_TOP_LOW4,
    tzc_region_top_high4: TZC_REGION_TOP_HIGH4,
    tzc_region_attribute4: TZC_REGION_ATTRIBUTE4,
    tzc_region_id_access4: TZC_REGION_ID_ACCESS4,
    _reserved43: [u8; 0x08],
    tzc_region_base_low5: TZC_REGION_BASE_LOW5,
    tzc_region_base_high5: TZC_REGION_BASE_HIGH5,
    tzc_region_top_low5: TZC_REGION_TOP_LOW5,
    tzc_region_top_high5: TZC_REGION_TOP_HIGH5,
    tzc_region_attribute5: TZC_REGION_ATTRIBUTE5,
    tzc_region_id_access5: TZC_REGION_ID_ACCESS5,
    _reserved49: [u8; 0x08],
    tzc_region_base_low6: TZC_REGION_BASE_LOW6,
    tzc_region_base_high6: TZC_REGION_BASE_HIGH6,
    tzc_region_top_low6: TZC_REGION_TOP_LOW6,
    tzc_region_top_high6: TZC_REGION_TOP_HIGH6,
    tzc_region_attribute6: TZC_REGION_ATTRIBUTE6,
    tzc_region_id_access6: TZC_REGION_ID_ACCESS6,
    _reserved55: [u8; 0x10],
    tzc_region_top_low7: TZC_REGION_TOP_LOW7,
    _reserved56: [u8; 0x04],
    tzc_region_attribute7: TZC_REGION_ATTRIBUTE7,
    _reserved57: [u8; 0x0c],
    tzc_region_base_low8: TZC_REGION_BASE_LOW8,
    tzc_region_base_high8: TZC_REGION_BASE_HIGH8,
    _reserved59: [u8; 0x08],
    tzc_region_attribute8: TZC_REGION_ATTRIBUTE8,
    _reserved60: [u8; 0xcc],
    tzc_region_base_low7: TZC_REGION_BASE_LOW7,
    tzc_region_base_high7: TZC_REGION_BASE_HIGH7,
    _reserved62: [u8; 0x04],
    tzc_region_top_high7: TZC_REGION_TOP_HIGH7,
    _reserved63: [u8; 0x04],
    tzc_region_id_access7: TZC_REGION_ID_ACCESS7,
    _reserved64: [u8; 0x10],
    tzc_region_top_low8: TZC_REGION_TOP_LOW8,
    tzc_region_top_high8: TZC_REGION_TOP_HIGH8,
    _reserved66: [u8; 0x04],
    tzc_region_id_access8: TZC_REGION_ID_ACCESS8,
    _reserved67: [u8; 0x0cb8],
    tzc_pid4: TZC_PID4,
    tzc_pid5: TZC_PID5,
    tzc_pid6: TZC_PID6,
    tzc_pid7: TZC_PID7,
    tzc_pid0: TZC_PID0,
    tzc_pid1: TZC_PID1,
    tzc_pid2: TZC_PID2,
    tzc_pid3: TZC_PID3,
    tzc_cid0: TZC_CID0,
    tzc_cid1: TZC_CID1,
    tzc_cid2: TZC_CID2,
    tzc_cid3: TZC_CID3,
}
impl RegisterBlock {
    #[doc = "0x00 - Provides information about TZC configuration."]
    #[inline(always)]
    pub const fn tzc_build_config(&self) -> &TZC_BUILD_CONFIG {
        &self.tzc_build_config
    }
    #[doc = "0x04 - Controls interrupt and bus error response behavior when regions permission failures occur."]
    #[inline(always)]
    pub const fn tzc_action(&self) -> &TZC_ACTION {
        &self.tzc_action
    }
    #[doc = "0x08 - Provides control and status for the gate keeper in each filter unit implemented."]
    #[inline(always)]
    pub const fn tzc_gate_keeper(&self) -> &TZC_GATE_KEEPER {
        &self.tzc_gate_keeper
    }
    #[doc = "0x0c - Controls read and write access speculation."]
    #[inline(always)]
    pub const fn tzc_speculation_ctrl(&self) -> &TZC_SPECULATION_CTRL {
        &self.tzc_speculation_ctrl
    }
    #[doc = "0x10 - Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors."]
    #[inline(always)]
    pub const fn tzc_int_status(&self) -> &TZC_INT_STATUS {
        &self.tzc_int_status
    }
    #[doc = "0x14 - Interrupt clear for each filter."]
    #[inline(always)]
    pub const fn tzc_int_clear(&self) -> &TZC_INT_CLEAR {
        &self.tzc_int_clear
    }
    #[doc = "0x20 - Address low bits of the first failed access in the associated filter (0 to 1)."]
    #[inline(always)]
    pub const fn tzc_fail_address_low0(&self) -> &TZC_FAIL_ADDRESS_LOW0 {
        &self.tzc_fail_address_low0
    }
    #[doc = "0x24 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
    #[inline(always)]
    pub const fn tzc_fail_address_high0(&self) -> &TZC_FAIL_ADDRESS_HIGH0 {
        &self.tzc_fail_address_high0
    }
    #[doc = "0x28 - Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
    #[inline(always)]
    pub const fn tzc_fail_control0(&self) -> &TZC_FAIL_CONTROL0 {
        &self.tzc_fail_control0
    }
    #[doc = "0x2c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
    #[inline(always)]
    pub const fn tzc_fail_id0(&self) -> &TZC_FAIL_ID0 {
        &self.tzc_fail_id0
    }
    #[doc = "0x30 - Address low bits of the first failed access in the associated filter (0 to 1)."]
    #[inline(always)]
    pub const fn tzc_fail_address_low1(&self) -> &TZC_FAIL_ADDRESS_LOW1 {
        &self.tzc_fail_address_low1
    }
    #[doc = "0x34 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
    #[inline(always)]
    pub const fn tzc_fail_address_high1(&self) -> &TZC_FAIL_ADDRESS_HIGH1 {
        &self.tzc_fail_address_high1
    }
    #[doc = "0x38 - Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
    #[inline(always)]
    pub const fn tzc_fail_control1(&self) -> &TZC_FAIL_CONTROL1 {
        &self.tzc_fail_control1
    }
    #[doc = "0x3c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
    #[inline(always)]
    pub const fn tzc_fail_id1(&self) -> &TZC_FAIL_ID1 {
        &self.tzc_fail_id1
    }
    #[doc = "0x104 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high0(&self) -> &TZC_REGION_BASE_HIGH0 {
        &self.tzc_region_base_high0
    }
    #[doc = "0x108 - Top address bits \\[31:12\\]
for region 0."]
    #[inline(always)]
    pub const fn tzc_region_top_low0(&self) -> &TZC_REGION_TOP_LOW0 {
        &self.tzc_region_top_low0
    }
    #[doc = "0x10c - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high0(&self) -> &TZC_REGION_TOP_HIGH0 {
        &self.tzc_region_top_high0
    }
    #[doc = "0x110 - Region 0 attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute0(&self) -> &TZC_REGION_ATTRIBUTE0 {
        &self.tzc_region_attribute0
    }
    #[doc = "0x114 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access0(&self) -> &TZC_REGION_ID_ACCESS0 {
        &self.tzc_region_id_access0
    }
    #[doc = "0x120 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low1(&self) -> &TZC_REGION_BASE_LOW1 {
        &self.tzc_region_base_low1
    }
    #[doc = "0x124 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high1(&self) -> &TZC_REGION_BASE_HIGH1 {
        &self.tzc_region_base_high1
    }
    #[doc = "0x128 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low1(&self) -> &TZC_REGION_TOP_LOW1 {
        &self.tzc_region_top_low1
    }
    #[doc = "0x12c - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high1(&self) -> &TZC_REGION_TOP_HIGH1 {
        &self.tzc_region_top_high1
    }
    #[doc = "0x130 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute1(&self) -> &TZC_REGION_ATTRIBUTE1 {
        &self.tzc_region_attribute1
    }
    #[doc = "0x134 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access1(&self) -> &TZC_REGION_ID_ACCESS1 {
        &self.tzc_region_id_access1
    }
    #[doc = "0x140 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low2(&self) -> &TZC_REGION_BASE_LOW2 {
        &self.tzc_region_base_low2
    }
    #[doc = "0x144 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high2(&self) -> &TZC_REGION_BASE_HIGH2 {
        &self.tzc_region_base_high2
    }
    #[doc = "0x148 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low2(&self) -> &TZC_REGION_TOP_LOW2 {
        &self.tzc_region_top_low2
    }
    #[doc = "0x14c - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high2(&self) -> &TZC_REGION_TOP_HIGH2 {
        &self.tzc_region_top_high2
    }
    #[doc = "0x150 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute2(&self) -> &TZC_REGION_ATTRIBUTE2 {
        &self.tzc_region_attribute2
    }
    #[doc = "0x154 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access2(&self) -> &TZC_REGION_ID_ACCESS2 {
        &self.tzc_region_id_access2
    }
    #[doc = "0x160 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low3(&self) -> &TZC_REGION_BASE_LOW3 {
        &self.tzc_region_base_low3
    }
    #[doc = "0x164 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high3(&self) -> &TZC_REGION_BASE_HIGH3 {
        &self.tzc_region_base_high3
    }
    #[doc = "0x168 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low3(&self) -> &TZC_REGION_TOP_LOW3 {
        &self.tzc_region_top_low3
    }
    #[doc = "0x16c - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high3(&self) -> &TZC_REGION_TOP_HIGH3 {
        &self.tzc_region_top_high3
    }
    #[doc = "0x170 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute3(&self) -> &TZC_REGION_ATTRIBUTE3 {
        &self.tzc_region_attribute3
    }
    #[doc = "0x174 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access3(&self) -> &TZC_REGION_ID_ACCESS3 {
        &self.tzc_region_id_access3
    }
    #[doc = "0x180 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low4(&self) -> &TZC_REGION_BASE_LOW4 {
        &self.tzc_region_base_low4
    }
    #[doc = "0x184 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high4(&self) -> &TZC_REGION_BASE_HIGH4 {
        &self.tzc_region_base_high4
    }
    #[doc = "0x188 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low4(&self) -> &TZC_REGION_TOP_LOW4 {
        &self.tzc_region_top_low4
    }
    #[doc = "0x18c - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high4(&self) -> &TZC_REGION_TOP_HIGH4 {
        &self.tzc_region_top_high4
    }
    #[doc = "0x190 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute4(&self) -> &TZC_REGION_ATTRIBUTE4 {
        &self.tzc_region_attribute4
    }
    #[doc = "0x194 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access4(&self) -> &TZC_REGION_ID_ACCESS4 {
        &self.tzc_region_id_access4
    }
    #[doc = "0x1a0 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low5(&self) -> &TZC_REGION_BASE_LOW5 {
        &self.tzc_region_base_low5
    }
    #[doc = "0x1a4 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high5(&self) -> &TZC_REGION_BASE_HIGH5 {
        &self.tzc_region_base_high5
    }
    #[doc = "0x1a8 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low5(&self) -> &TZC_REGION_TOP_LOW5 {
        &self.tzc_region_top_low5
    }
    #[doc = "0x1ac - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high5(&self) -> &TZC_REGION_TOP_HIGH5 {
        &self.tzc_region_top_high5
    }
    #[doc = "0x1b0 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute5(&self) -> &TZC_REGION_ATTRIBUTE5 {
        &self.tzc_region_attribute5
    }
    #[doc = "0x1b4 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access5(&self) -> &TZC_REGION_ID_ACCESS5 {
        &self.tzc_region_id_access5
    }
    #[doc = "0x1c0 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low6(&self) -> &TZC_REGION_BASE_LOW6 {
        &self.tzc_region_base_low6
    }
    #[doc = "0x1c4 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high6(&self) -> &TZC_REGION_BASE_HIGH6 {
        &self.tzc_region_base_high6
    }
    #[doc = "0x1c8 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low6(&self) -> &TZC_REGION_TOP_LOW6 {
        &self.tzc_region_top_low6
    }
    #[doc = "0x1cc - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high6(&self) -> &TZC_REGION_TOP_HIGH6 {
        &self.tzc_region_top_high6
    }
    #[doc = "0x1d0 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute6(&self) -> &TZC_REGION_ATTRIBUTE6 {
        &self.tzc_region_attribute6
    }
    #[doc = "0x1d4 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access6(&self) -> &TZC_REGION_ID_ACCESS6 {
        &self.tzc_region_id_access6
    }
    #[doc = "0x1e8 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low7(&self) -> &TZC_REGION_TOP_LOW7 {
        &self.tzc_region_top_low7
    }
    #[doc = "0x1f0 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute7(&self) -> &TZC_REGION_ATTRIBUTE7 {
        &self.tzc_region_attribute7
    }
    #[doc = "0x200 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low8(&self) -> &TZC_REGION_BASE_LOW8 {
        &self.tzc_region_base_low8
    }
    #[doc = "0x204 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high8(&self) -> &TZC_REGION_BASE_HIGH8 {
        &self.tzc_region_base_high8
    }
    #[doc = "0x210 - Region x attributes."]
    #[inline(always)]
    pub const fn tzc_region_attribute8(&self) -> &TZC_REGION_ATTRIBUTE8 {
        &self.tzc_region_attribute8
    }
    #[doc = "0x2e0 - Base address low for regions 1 to 8."]
    #[inline(always)]
    pub const fn tzc_region_base_low7(&self) -> &TZC_REGION_BASE_LOW7 {
        &self.tzc_region_base_low7
    }
    #[doc = "0x2e4 - Base address high are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_base_high7(&self) -> &TZC_REGION_BASE_HIGH7 {
        &self.tzc_region_base_high7
    }
    #[doc = "0x2ec - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high7(&self) -> &TZC_REGION_TOP_HIGH7 {
        &self.tzc_region_top_high7
    }
    #[doc = "0x2f4 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access7(&self) -> &TZC_REGION_ID_ACCESS7 {
        &self.tzc_region_id_access7
    }
    #[doc = "0x308 - Top address bits \\[31:12\\]
for region x."]
    #[inline(always)]
    pub const fn tzc_region_top_low8(&self) -> &TZC_REGION_TOP_LOW8 {
        &self.tzc_region_top_low8
    }
    #[doc = "0x30c - Top address high of region are not used with 32-bit address."]
    #[inline(always)]
    pub const fn tzc_region_top_high8(&self) -> &TZC_REGION_TOP_HIGH8 {
        &self.tzc_region_top_high8
    }
    #[doc = "0x314 - Region non-secure access based on NSAID."]
    #[inline(always)]
    pub const fn tzc_region_id_access8(&self) -> &TZC_REGION_ID_ACCESS8 {
        &self.tzc_region_id_access8
    }
    #[doc = "0xfd0 - Peripheral ID 4."]
    #[inline(always)]
    pub const fn tzc_pid4(&self) -> &TZC_PID4 {
        &self.tzc_pid4
    }
    #[doc = "0xfd4 - Peripheral ID 5."]
    #[inline(always)]
    pub const fn tzc_pid5(&self) -> &TZC_PID5 {
        &self.tzc_pid5
    }
    #[doc = "0xfd8 - Peripheral ID 6."]
    #[inline(always)]
    pub const fn tzc_pid6(&self) -> &TZC_PID6 {
        &self.tzc_pid6
    }
    #[doc = "0xfdc - Peripheral ID 7."]
    #[inline(always)]
    pub const fn tzc_pid7(&self) -> &TZC_PID7 {
        &self.tzc_pid7
    }
    #[doc = "0xfe0 - Peripheral ID 0."]
    #[inline(always)]
    pub const fn tzc_pid0(&self) -> &TZC_PID0 {
        &self.tzc_pid0
    }
    #[doc = "0xfe4 - Peripheral ID 1."]
    #[inline(always)]
    pub const fn tzc_pid1(&self) -> &TZC_PID1 {
        &self.tzc_pid1
    }
    #[doc = "0xfe8 - Peripheral ID 2."]
    #[inline(always)]
    pub const fn tzc_pid2(&self) -> &TZC_PID2 {
        &self.tzc_pid2
    }
    #[doc = "0xfec - Peripheral ID 3."]
    #[inline(always)]
    pub const fn tzc_pid3(&self) -> &TZC_PID3 {
        &self.tzc_pid3
    }
    #[doc = "0xff0 - Component ID 0."]
    #[inline(always)]
    pub const fn tzc_cid0(&self) -> &TZC_CID0 {
        &self.tzc_cid0
    }
    #[doc = "0xff4 - Component ID 1."]
    #[inline(always)]
    pub const fn tzc_cid1(&self) -> &TZC_CID1 {
        &self.tzc_cid1
    }
    #[doc = "0xff8 - Component ID 2."]
    #[inline(always)]
    pub const fn tzc_cid2(&self) -> &TZC_CID2 {
        &self.tzc_cid2
    }
    #[doc = "0xffc - Component ID 3."]
    #[inline(always)]
    pub const fn tzc_cid3(&self) -> &TZC_CID3 {
        &self.tzc_cid3
    }
}
#[doc = "TZC_BUILD_CONFIG (r) register accessor: Provides information about TZC configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_build_config::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_build_config`]
module"]
pub type TZC_BUILD_CONFIG = crate::Reg<tzc_build_config::TZC_BUILD_CONFIGrs>;
#[doc = "Provides information about TZC configuration."]
pub mod tzc_build_config;
#[doc = "TZC_ACTION (rw) register accessor: Controls interrupt and bus error response behavior when regions permission failures occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_action::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_action::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_action`]
module"]
pub type TZC_ACTION = crate::Reg<tzc_action::TZC_ACTIONrs>;
#[doc = "Controls interrupt and bus error response behavior when regions permission failures occur."]
pub mod tzc_action;
#[doc = "TZC_GATE_KEEPER (rw) register accessor: Provides control and status for the gate keeper in each filter unit implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_gate_keeper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_gate_keeper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_gate_keeper`]
module"]
pub type TZC_GATE_KEEPER = crate::Reg<tzc_gate_keeper::TZC_GATE_KEEPERrs>;
#[doc = "Provides control and status for the gate keeper in each filter unit implemented."]
pub mod tzc_gate_keeper;
#[doc = "TZC_SPECULATION_CTRL (rw) register accessor: Controls read and write access speculation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_speculation_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_speculation_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_speculation_ctrl`]
module"]
pub type TZC_SPECULATION_CTRL = crate::Reg<tzc_speculation_ctrl::TZC_SPECULATION_CTRLrs>;
#[doc = "Controls read and write access speculation."]
pub mod tzc_speculation_ctrl;
#[doc = "TZC_INT_STATUS (r) register accessor: Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_int_status`]
module"]
pub type TZC_INT_STATUS = crate::Reg<tzc_int_status::TZC_INT_STATUSrs>;
#[doc = "Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors."]
pub mod tzc_int_status;
#[doc = "TZC_INT_CLEAR (rw) register accessor: Interrupt clear for each filter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_int_clear`]
module"]
pub type TZC_INT_CLEAR = crate::Reg<tzc_int_clear::TZC_INT_CLEARrs>;
#[doc = "Interrupt clear for each filter."]
pub mod tzc_int_clear;
#[doc = "TZC_FAIL_CONTROL0 (r) register accessor: Status information about the first access that failed a region permission check in the associated filter (0 to 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_control0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_control0`]
module"]
pub type TZC_FAIL_CONTROL0 = crate::Reg<tzc_fail_control0::TZC_FAIL_CONTROL0rs>;
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
pub mod tzc_fail_control0;
#[doc = "TZC_FAIL_ID0 (r) register accessor: Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_id0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_id0`]
module"]
pub type TZC_FAIL_ID0 = crate::Reg<tzc_fail_id0::TZC_FAIL_ID0rs>;
#[doc = "Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
pub mod tzc_fail_id0;
#[doc = "TZC_FAIL_CONTROL1 (r) register accessor: Status information about the first access that failed a region permission check in the associated filter (0 to 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_control1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_control1`]
module"]
pub type TZC_FAIL_CONTROL1 = crate::Reg<tzc_fail_control1::TZC_FAIL_CONTROL1rs>;
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
pub mod tzc_fail_control1;
#[doc = "TZC_FAIL_ID1 (r) register accessor: Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_id1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_id1`]
module"]
pub type TZC_FAIL_ID1 = crate::Reg<tzc_fail_id1::TZC_FAIL_ID1rs>;
#[doc = "Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
pub mod tzc_fail_id1;
#[doc = "TZC_REGION_ATTRIBUTE0 (rw) register accessor: Region 0 attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute0`]
module"]
pub type TZC_REGION_ATTRIBUTE0 = crate::Reg<tzc_region_attribute0::TZC_REGION_ATTRIBUTE0rs>;
#[doc = "Region 0 attributes."]
pub mod tzc_region_attribute0;
#[doc = "TZC_REGION_ATTRIBUTE1 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute1`]
module"]
pub type TZC_REGION_ATTRIBUTE1 = crate::Reg<tzc_region_attribute1::TZC_REGION_ATTRIBUTE1rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute1;
#[doc = "TZC_REGION_ATTRIBUTE2 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute2`]
module"]
pub type TZC_REGION_ATTRIBUTE2 = crate::Reg<tzc_region_attribute2::TZC_REGION_ATTRIBUTE2rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute2;
#[doc = "TZC_REGION_ATTRIBUTE3 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute3`]
module"]
pub type TZC_REGION_ATTRIBUTE3 = crate::Reg<tzc_region_attribute3::TZC_REGION_ATTRIBUTE3rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute3;
#[doc = "TZC_REGION_ATTRIBUTE4 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute4`]
module"]
pub type TZC_REGION_ATTRIBUTE4 = crate::Reg<tzc_region_attribute4::TZC_REGION_ATTRIBUTE4rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute4;
#[doc = "TZC_REGION_ATTRIBUTE5 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute5`]
module"]
pub type TZC_REGION_ATTRIBUTE5 = crate::Reg<tzc_region_attribute5::TZC_REGION_ATTRIBUTE5rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute5;
#[doc = "TZC_REGION_ATTRIBUTE6 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute6`]
module"]
pub type TZC_REGION_ATTRIBUTE6 = crate::Reg<tzc_region_attribute6::TZC_REGION_ATTRIBUTE6rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute6;
#[doc = "TZC_REGION_ATTRIBUTE7 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute7`]
module"]
pub type TZC_REGION_ATTRIBUTE7 = crate::Reg<tzc_region_attribute7::TZC_REGION_ATTRIBUTE7rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute7;
#[doc = "TZC_REGION_ATTRIBUTE8 (rw) register accessor: Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_attribute8`]
module"]
pub type TZC_REGION_ATTRIBUTE8 = crate::Reg<tzc_region_attribute8::TZC_REGION_ATTRIBUTE8rs>;
#[doc = "Region x attributes."]
pub mod tzc_region_attribute8;
#[doc = "TZC_PID4 (r) register accessor: Peripheral ID 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid4`]
module"]
pub type TZC_PID4 = crate::Reg<tzc_pid4::TZC_PID4rs>;
#[doc = "Peripheral ID 4."]
pub mod tzc_pid4;
#[doc = "TZC_PID5 (r) register accessor: Peripheral ID 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid5`]
module"]
pub type TZC_PID5 = crate::Reg<tzc_pid5::TZC_PID5rs>;
#[doc = "Peripheral ID 5."]
pub mod tzc_pid5;
#[doc = "TZC_PID6 (r) register accessor: Peripheral ID 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid6`]
module"]
pub type TZC_PID6 = crate::Reg<tzc_pid6::TZC_PID6rs>;
#[doc = "Peripheral ID 6."]
pub mod tzc_pid6;
#[doc = "TZC_PID7 (r) register accessor: Peripheral ID 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid7`]
module"]
pub type TZC_PID7 = crate::Reg<tzc_pid7::TZC_PID7rs>;
#[doc = "Peripheral ID 7."]
pub mod tzc_pid7;
#[doc = "TZC_PID0 (r) register accessor: Peripheral ID 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid0`]
module"]
pub type TZC_PID0 = crate::Reg<tzc_pid0::TZC_PID0rs>;
#[doc = "Peripheral ID 0."]
pub mod tzc_pid0;
#[doc = "TZC_PID1 (r) register accessor: Peripheral ID 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid1`]
module"]
pub type TZC_PID1 = crate::Reg<tzc_pid1::TZC_PID1rs>;
#[doc = "Peripheral ID 1."]
pub mod tzc_pid1;
#[doc = "TZC_PID2 (r) register accessor: Peripheral ID 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid2`]
module"]
pub type TZC_PID2 = crate::Reg<tzc_pid2::TZC_PID2rs>;
#[doc = "Peripheral ID 2."]
pub mod tzc_pid2;
#[doc = "TZC_PID3 (r) register accessor: Peripheral ID 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pid3`]
module"]
pub type TZC_PID3 = crate::Reg<tzc_pid3::TZC_PID3rs>;
#[doc = "Peripheral ID 3."]
pub mod tzc_pid3;
#[doc = "TZC_CID0 (r) register accessor: Component ID 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_cid0`]
module"]
pub type TZC_CID0 = crate::Reg<tzc_cid0::TZC_CID0rs>;
#[doc = "Component ID 0."]
pub mod tzc_cid0;
#[doc = "TZC_CID1 (r) register accessor: Component ID 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_cid1`]
module"]
pub type TZC_CID1 = crate::Reg<tzc_cid1::TZC_CID1rs>;
#[doc = "Component ID 1."]
pub mod tzc_cid1;
#[doc = "TZC_CID2 (r) register accessor: Component ID 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_cid2`]
module"]
pub type TZC_CID2 = crate::Reg<tzc_cid2::TZC_CID2rs>;
#[doc = "Component ID 2."]
pub mod tzc_cid2;
#[doc = "TZC_CID3 (r) register accessor: Component ID 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_cid3`]
module"]
pub type TZC_CID3 = crate::Reg<tzc_cid3::TZC_CID3rs>;
#[doc = "Component ID 3."]
pub mod tzc_cid3;
#[doc = "TZC_FAIL_ADDRESS_LOW0 (r) register accessor: Address low bits of the first failed access in the associated filter (0 to 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_address_low0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_address_low0`]
module"]
pub type TZC_FAIL_ADDRESS_LOW0 = crate::Reg<tzc_fail_address_low0::TZC_FAIL_ADDRESS_LOW0rs>;
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1)."]
pub mod tzc_fail_address_low0;
#[doc = "TZC_FAIL_ADDRESS_HIGH0 (r) register accessor: Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_address_high0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_address_high0`]
module"]
pub type TZC_FAIL_ADDRESS_HIGH0 = crate::Reg<tzc_fail_address_high0::TZC_FAIL_ADDRESS_HIGH0rs>;
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
pub mod tzc_fail_address_high0;
#[doc = "TZC_FAIL_ADDRESS_LOW1 (r) register accessor: Address low bits of the first failed access in the associated filter (0 to 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_address_low1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_address_low1`]
module"]
pub type TZC_FAIL_ADDRESS_LOW1 = crate::Reg<tzc_fail_address_low1::TZC_FAIL_ADDRESS_LOW1rs>;
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1)."]
pub mod tzc_fail_address_low1;
#[doc = "TZC_FAIL_ADDRESS_HIGH1 (r) register accessor: Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_address_high1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_fail_address_high1`]
module"]
pub type TZC_FAIL_ADDRESS_HIGH1 = crate::Reg<tzc_fail_address_high1::TZC_FAIL_ADDRESS_HIGH1rs>;
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
pub mod tzc_fail_address_high1;
#[doc = "TZC_REGION_BASE_HIGH0 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high0`]
module"]
pub type TZC_REGION_BASE_HIGH0 = crate::Reg<tzc_region_base_high0::TZC_REGION_BASE_HIGH0rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high0;
#[doc = "TZC_REGION_TOP_LOW0 (r) register accessor: Top address bits \\[31:12\\]
for region 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low0`]
module"]
pub type TZC_REGION_TOP_LOW0 = crate::Reg<tzc_region_top_low0::TZC_REGION_TOP_LOW0rs>;
#[doc = "Top address bits \\[31:12\\]
for region 0."]
pub mod tzc_region_top_low0;
#[doc = "TZC_REGION_TOP_HIGH0 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high0`]
module"]
pub type TZC_REGION_TOP_HIGH0 = crate::Reg<tzc_region_top_high0::TZC_REGION_TOP_HIGH0rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high0;
#[doc = "TZC_REGION_ID_ACCESS0 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access0`]
module"]
pub type TZC_REGION_ID_ACCESS0 = crate::Reg<tzc_region_id_access0::TZC_REGION_ID_ACCESS0rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access0;
#[doc = "TZC_REGION_BASE_LOW1 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low1`]
module"]
pub type TZC_REGION_BASE_LOW1 = crate::Reg<tzc_region_base_low1::TZC_REGION_BASE_LOW1rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low1;
#[doc = "TZC_REGION_BASE_HIGH1 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high1`]
module"]
pub type TZC_REGION_BASE_HIGH1 = crate::Reg<tzc_region_base_high1::TZC_REGION_BASE_HIGH1rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high1;
#[doc = "TZC_REGION_TOP_LOW1 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low1`]
module"]
pub type TZC_REGION_TOP_LOW1 = crate::Reg<tzc_region_top_low1::TZC_REGION_TOP_LOW1rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low1;
#[doc = "TZC_REGION_TOP_HIGH1 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high1`]
module"]
pub type TZC_REGION_TOP_HIGH1 = crate::Reg<tzc_region_top_high1::TZC_REGION_TOP_HIGH1rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high1;
#[doc = "TZC_REGION_ID_ACCESS1 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access1`]
module"]
pub type TZC_REGION_ID_ACCESS1 = crate::Reg<tzc_region_id_access1::TZC_REGION_ID_ACCESS1rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access1;
#[doc = "TZC_REGION_BASE_LOW2 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low2`]
module"]
pub type TZC_REGION_BASE_LOW2 = crate::Reg<tzc_region_base_low2::TZC_REGION_BASE_LOW2rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low2;
#[doc = "TZC_REGION_BASE_HIGH2 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high2`]
module"]
pub type TZC_REGION_BASE_HIGH2 = crate::Reg<tzc_region_base_high2::TZC_REGION_BASE_HIGH2rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high2;
#[doc = "TZC_REGION_TOP_LOW2 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low2`]
module"]
pub type TZC_REGION_TOP_LOW2 = crate::Reg<tzc_region_top_low2::TZC_REGION_TOP_LOW2rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low2;
#[doc = "TZC_REGION_TOP_HIGH2 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high2`]
module"]
pub type TZC_REGION_TOP_HIGH2 = crate::Reg<tzc_region_top_high2::TZC_REGION_TOP_HIGH2rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high2;
#[doc = "TZC_REGION_ID_ACCESS2 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access2`]
module"]
pub type TZC_REGION_ID_ACCESS2 = crate::Reg<tzc_region_id_access2::TZC_REGION_ID_ACCESS2rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access2;
#[doc = "TZC_REGION_BASE_LOW3 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low3`]
module"]
pub type TZC_REGION_BASE_LOW3 = crate::Reg<tzc_region_base_low3::TZC_REGION_BASE_LOW3rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low3;
#[doc = "TZC_REGION_BASE_HIGH3 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high3`]
module"]
pub type TZC_REGION_BASE_HIGH3 = crate::Reg<tzc_region_base_high3::TZC_REGION_BASE_HIGH3rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high3;
#[doc = "TZC_REGION_TOP_LOW3 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low3`]
module"]
pub type TZC_REGION_TOP_LOW3 = crate::Reg<tzc_region_top_low3::TZC_REGION_TOP_LOW3rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low3;
#[doc = "TZC_REGION_TOP_HIGH3 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high3`]
module"]
pub type TZC_REGION_TOP_HIGH3 = crate::Reg<tzc_region_top_high3::TZC_REGION_TOP_HIGH3rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high3;
#[doc = "TZC_REGION_ID_ACCESS3 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access3`]
module"]
pub type TZC_REGION_ID_ACCESS3 = crate::Reg<tzc_region_id_access3::TZC_REGION_ID_ACCESS3rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access3;
#[doc = "TZC_REGION_BASE_LOW4 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low4`]
module"]
pub type TZC_REGION_BASE_LOW4 = crate::Reg<tzc_region_base_low4::TZC_REGION_BASE_LOW4rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low4;
#[doc = "TZC_REGION_BASE_HIGH4 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high4`]
module"]
pub type TZC_REGION_BASE_HIGH4 = crate::Reg<tzc_region_base_high4::TZC_REGION_BASE_HIGH4rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high4;
#[doc = "TZC_REGION_TOP_LOW4 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low4`]
module"]
pub type TZC_REGION_TOP_LOW4 = crate::Reg<tzc_region_top_low4::TZC_REGION_TOP_LOW4rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low4;
#[doc = "TZC_REGION_TOP_HIGH4 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high4`]
module"]
pub type TZC_REGION_TOP_HIGH4 = crate::Reg<tzc_region_top_high4::TZC_REGION_TOP_HIGH4rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high4;
#[doc = "TZC_REGION_ID_ACCESS4 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access4`]
module"]
pub type TZC_REGION_ID_ACCESS4 = crate::Reg<tzc_region_id_access4::TZC_REGION_ID_ACCESS4rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access4;
#[doc = "TZC_REGION_BASE_LOW5 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low5`]
module"]
pub type TZC_REGION_BASE_LOW5 = crate::Reg<tzc_region_base_low5::TZC_REGION_BASE_LOW5rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low5;
#[doc = "TZC_REGION_BASE_HIGH5 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high5`]
module"]
pub type TZC_REGION_BASE_HIGH5 = crate::Reg<tzc_region_base_high5::TZC_REGION_BASE_HIGH5rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high5;
#[doc = "TZC_REGION_TOP_LOW5 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low5`]
module"]
pub type TZC_REGION_TOP_LOW5 = crate::Reg<tzc_region_top_low5::TZC_REGION_TOP_LOW5rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low5;
#[doc = "TZC_REGION_TOP_HIGH5 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high5`]
module"]
pub type TZC_REGION_TOP_HIGH5 = crate::Reg<tzc_region_top_high5::TZC_REGION_TOP_HIGH5rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high5;
#[doc = "TZC_REGION_ID_ACCESS5 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access5`]
module"]
pub type TZC_REGION_ID_ACCESS5 = crate::Reg<tzc_region_id_access5::TZC_REGION_ID_ACCESS5rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access5;
#[doc = "TZC_REGION_BASE_LOW6 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low6`]
module"]
pub type TZC_REGION_BASE_LOW6 = crate::Reg<tzc_region_base_low6::TZC_REGION_BASE_LOW6rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low6;
#[doc = "TZC_REGION_BASE_HIGH6 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high6`]
module"]
pub type TZC_REGION_BASE_HIGH6 = crate::Reg<tzc_region_base_high6::TZC_REGION_BASE_HIGH6rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high6;
#[doc = "TZC_REGION_TOP_LOW6 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low6`]
module"]
pub type TZC_REGION_TOP_LOW6 = crate::Reg<tzc_region_top_low6::TZC_REGION_TOP_LOW6rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low6;
#[doc = "TZC_REGION_TOP_HIGH6 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high6`]
module"]
pub type TZC_REGION_TOP_HIGH6 = crate::Reg<tzc_region_top_high6::TZC_REGION_TOP_HIGH6rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high6;
#[doc = "TZC_REGION_ID_ACCESS6 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access6`]
module"]
pub type TZC_REGION_ID_ACCESS6 = crate::Reg<tzc_region_id_access6::TZC_REGION_ID_ACCESS6rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access6;
#[doc = "TZC_REGION_BASE_LOW7 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low7`]
module"]
pub type TZC_REGION_BASE_LOW7 = crate::Reg<tzc_region_base_low7::TZC_REGION_BASE_LOW7rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low7;
#[doc = "TZC_REGION_BASE_HIGH7 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high7`]
module"]
pub type TZC_REGION_BASE_HIGH7 = crate::Reg<tzc_region_base_high7::TZC_REGION_BASE_HIGH7rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high7;
#[doc = "TZC_REGION_TOP_LOW7 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low7`]
module"]
pub type TZC_REGION_TOP_LOW7 = crate::Reg<tzc_region_top_low7::TZC_REGION_TOP_LOW7rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low7;
#[doc = "TZC_REGION_TOP_HIGH7 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high7`]
module"]
pub type TZC_REGION_TOP_HIGH7 = crate::Reg<tzc_region_top_high7::TZC_REGION_TOP_HIGH7rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high7;
#[doc = "TZC_REGION_ID_ACCESS7 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access7`]
module"]
pub type TZC_REGION_ID_ACCESS7 = crate::Reg<tzc_region_id_access7::TZC_REGION_ID_ACCESS7rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access7;
#[doc = "TZC_REGION_BASE_LOW8 (rw) register accessor: Base address low for regions 1 to 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_low8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_base_low8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_low8`]
module"]
pub type TZC_REGION_BASE_LOW8 = crate::Reg<tzc_region_base_low8::TZC_REGION_BASE_LOW8rs>;
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low8;
#[doc = "TZC_REGION_BASE_HIGH8 (r) register accessor: Base address high are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_base_high8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_base_high8`]
module"]
pub type TZC_REGION_BASE_HIGH8 = crate::Reg<tzc_region_base_high8::TZC_REGION_BASE_HIGH8rs>;
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high8;
#[doc = "TZC_REGION_TOP_LOW8 (rw) register accessor: Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_low8`]
module"]
pub type TZC_REGION_TOP_LOW8 = crate::Reg<tzc_region_top_low8::TZC_REGION_TOP_LOW8rs>;
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low8;
#[doc = "TZC_REGION_TOP_HIGH8 (r) register accessor: Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_top_high8`]
module"]
pub type TZC_REGION_TOP_HIGH8 = crate::Reg<tzc_region_top_high8::TZC_REGION_TOP_HIGH8rs>;
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high8;
#[doc = "TZC_REGION_ID_ACCESS8 (rw) register accessor: Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_region_id_access8`]
module"]
pub type TZC_REGION_ID_ACCESS8 = crate::Reg<tzc_region_id_access8::TZC_REGION_ID_ACCESS8rs>;
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access8;
