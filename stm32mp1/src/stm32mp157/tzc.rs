#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Provides information about TZC configuration."]
    pub tzc_build_config: TZC_BUILD_CONFIG,
    #[doc = "0x04 - Controls interrupt and bus error response behavior when regions permission failures occur."]
    pub tzc_action: TZC_ACTION,
    #[doc = "0x08 - Provides control and status for the gate keeper in each filter unit implemented."]
    pub tzc_gate_keeper: TZC_GATE_KEEPER,
    #[doc = "0x0c - Controls read and write access speculation."]
    pub tzc_speculation_ctrl: TZC_SPECULATION_CTRL,
    #[doc = "0x10 - Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors."]
    pub tzc_int_status: TZC_INT_STATUS,
    #[doc = "0x14 - Interrupt clear for each filter."]
    pub tzc_int_clear: TZC_INT_CLEAR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Address low bits of the first failed access in the associated filter (0 to 1)."]
    pub tzc_fail_address_low0: TZC_FAIL_ADDRESS_LOW0,
    #[doc = "0x24 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
    pub tzc_fail_address_high0: TZC_FAIL_ADDRESS_HIGH0,
    #[doc = "0x28 - Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
    pub tzc_fail_control0: TZC_FAIL_CONTROL0,
    #[doc = "0x2c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
    pub tzc_fail_id0: TZC_FAIL_ID0,
    #[doc = "0x30 - Address low bits of the first failed access in the associated filter (0 to 1)."]
    pub tzc_fail_address_low1: TZC_FAIL_ADDRESS_LOW1,
    #[doc = "0x34 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
    pub tzc_fail_address_high1: TZC_FAIL_ADDRESS_HIGH1,
    #[doc = "0x38 - Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
    pub tzc_fail_control1: TZC_FAIL_CONTROL1,
    #[doc = "0x3c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
    pub tzc_fail_id1: TZC_FAIL_ID1,
    _reserved14: [u8; 196usize],
    #[doc = "0x104 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high0: TZC_REGION_BASE_HIGH0,
    #[doc = "0x108 - Top address bits \\[31:12\\]
for region 0."]
    pub tzc_region_top_low0: TZC_REGION_TOP_LOW0,
    #[doc = "0x10c - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high0: TZC_REGION_TOP_HIGH0,
    #[doc = "0x110 - Region 0 attributes."]
    pub tzc_region_attribute0: TZC_REGION_ATTRIBUTE0,
    #[doc = "0x114 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access0: TZC_REGION_ID_ACCESS0,
    _reserved19: [u8; 8usize],
    #[doc = "0x120 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low1: TZC_REGION_BASE_LOW1,
    #[doc = "0x124 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high1: TZC_REGION_BASE_HIGH1,
    #[doc = "0x128 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low1: TZC_REGION_TOP_LOW1,
    #[doc = "0x12c - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high1: TZC_REGION_TOP_HIGH1,
    #[doc = "0x130 - Region x attributes."]
    pub tzc_region_attribute1: TZC_REGION_ATTRIBUTE1,
    #[doc = "0x134 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access1: TZC_REGION_ID_ACCESS1,
    _reserved25: [u8; 8usize],
    #[doc = "0x140 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low2: TZC_REGION_BASE_LOW2,
    #[doc = "0x144 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high2: TZC_REGION_BASE_HIGH2,
    #[doc = "0x148 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low2: TZC_REGION_TOP_LOW2,
    #[doc = "0x14c - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high2: TZC_REGION_TOP_HIGH2,
    #[doc = "0x150 - Region x attributes."]
    pub tzc_region_attribute2: TZC_REGION_ATTRIBUTE2,
    #[doc = "0x154 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access2: TZC_REGION_ID_ACCESS2,
    _reserved31: [u8; 8usize],
    #[doc = "0x160 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low3: TZC_REGION_BASE_LOW3,
    #[doc = "0x164 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high3: TZC_REGION_BASE_HIGH3,
    #[doc = "0x168 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low3: TZC_REGION_TOP_LOW3,
    #[doc = "0x16c - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high3: TZC_REGION_TOP_HIGH3,
    #[doc = "0x170 - Region x attributes."]
    pub tzc_region_attribute3: TZC_REGION_ATTRIBUTE3,
    #[doc = "0x174 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access3: TZC_REGION_ID_ACCESS3,
    _reserved37: [u8; 8usize],
    #[doc = "0x180 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low4: TZC_REGION_BASE_LOW4,
    #[doc = "0x184 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high4: TZC_REGION_BASE_HIGH4,
    #[doc = "0x188 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low4: TZC_REGION_TOP_LOW4,
    #[doc = "0x18c - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high4: TZC_REGION_TOP_HIGH4,
    #[doc = "0x190 - Region x attributes."]
    pub tzc_region_attribute4: TZC_REGION_ATTRIBUTE4,
    #[doc = "0x194 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access4: TZC_REGION_ID_ACCESS4,
    _reserved43: [u8; 8usize],
    #[doc = "0x1a0 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low5: TZC_REGION_BASE_LOW5,
    #[doc = "0x1a4 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high5: TZC_REGION_BASE_HIGH5,
    #[doc = "0x1a8 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low5: TZC_REGION_TOP_LOW5,
    #[doc = "0x1ac - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high5: TZC_REGION_TOP_HIGH5,
    #[doc = "0x1b0 - Region x attributes."]
    pub tzc_region_attribute5: TZC_REGION_ATTRIBUTE5,
    #[doc = "0x1b4 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access5: TZC_REGION_ID_ACCESS5,
    _reserved49: [u8; 8usize],
    #[doc = "0x1c0 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low6: TZC_REGION_BASE_LOW6,
    #[doc = "0x1c4 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high6: TZC_REGION_BASE_HIGH6,
    #[doc = "0x1c8 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low6: TZC_REGION_TOP_LOW6,
    #[doc = "0x1cc - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high6: TZC_REGION_TOP_HIGH6,
    #[doc = "0x1d0 - Region x attributes."]
    pub tzc_region_attribute6: TZC_REGION_ATTRIBUTE6,
    #[doc = "0x1d4 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access6: TZC_REGION_ID_ACCESS6,
    _reserved55: [u8; 16usize],
    #[doc = "0x1e8 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low7: TZC_REGION_TOP_LOW7,
    _reserved56: [u8; 4usize],
    #[doc = "0x1f0 - Region x attributes."]
    pub tzc_region_attribute7: TZC_REGION_ATTRIBUTE7,
    _reserved57: [u8; 12usize],
    #[doc = "0x200 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low8: TZC_REGION_BASE_LOW8,
    #[doc = "0x204 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high8: TZC_REGION_BASE_HIGH8,
    _reserved59: [u8; 8usize],
    #[doc = "0x210 - Region x attributes."]
    pub tzc_region_attribute8: TZC_REGION_ATTRIBUTE8,
    _reserved60: [u8; 204usize],
    #[doc = "0x2e0 - Base address low for regions 1 to 8."]
    pub tzc_region_base_low7: TZC_REGION_BASE_LOW7,
    #[doc = "0x2e4 - Base address high are not used with 32-bit address."]
    pub tzc_region_base_high7: TZC_REGION_BASE_HIGH7,
    _reserved62: [u8; 4usize],
    #[doc = "0x2ec - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high7: TZC_REGION_TOP_HIGH7,
    _reserved63: [u8; 4usize],
    #[doc = "0x2f4 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access7: TZC_REGION_ID_ACCESS7,
    _reserved64: [u8; 16usize],
    #[doc = "0x308 - Top address bits \\[31:12\\]
for region x."]
    pub tzc_region_top_low8: TZC_REGION_TOP_LOW8,
    #[doc = "0x30c - Top address high of region are not used with 32-bit address."]
    pub tzc_region_top_high8: TZC_REGION_TOP_HIGH8,
    _reserved66: [u8; 4usize],
    #[doc = "0x314 - Region non-secure access based on NSAID."]
    pub tzc_region_id_access8: TZC_REGION_ID_ACCESS8,
    _reserved67: [u8; 3256usize],
    #[doc = "0xfd0 - Peripheral ID 4."]
    pub tzc_pid4: TZC_PID4,
    #[doc = "0xfd4 - Peripheral ID 5."]
    pub tzc_pid5: TZC_PID5,
    #[doc = "0xfd8 - Peripheral ID 6."]
    pub tzc_pid6: TZC_PID6,
    #[doc = "0xfdc - Peripheral ID 7."]
    pub tzc_pid7: TZC_PID7,
    #[doc = "0xfe0 - Peripheral ID 0."]
    pub tzc_pid0: TZC_PID0,
    #[doc = "0xfe4 - Peripheral ID 1."]
    pub tzc_pid1: TZC_PID1,
    #[doc = "0xfe8 - Peripheral ID 2."]
    pub tzc_pid2: TZC_PID2,
    #[doc = "0xfec - Peripheral ID 3."]
    pub tzc_pid3: TZC_PID3,
    #[doc = "0xff0 - Component ID 0."]
    pub tzc_cid0: TZC_CID0,
    #[doc = "0xff4 - Component ID 1."]
    pub tzc_cid1: TZC_CID1,
    #[doc = "0xff8 - Component ID 2."]
    pub tzc_cid2: TZC_CID2,
    #[doc = "0xffc - Component ID 3."]
    pub tzc_cid3: TZC_CID3,
}
#[doc = "Provides information about TZC configuration.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_build_config](tzc_build_config) module"]
pub type TZC_BUILD_CONFIG = crate::Reg<u32, _TZC_BUILD_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_BUILD_CONFIG;
#[doc = "`read()` method returns [tzc_build_config::R](tzc_build_config::R) reader structure"]
impl crate::Readable for TZC_BUILD_CONFIG {}
#[doc = "Provides information about TZC configuration."]
pub mod tzc_build_config;
#[doc = "Controls interrupt and bus error response behavior when regions permission failures occur.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_action](tzc_action) module"]
pub type TZC_ACTION = crate::Reg<u32, _TZC_ACTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_ACTION;
#[doc = "`read()` method returns [tzc_action::R](tzc_action::R) reader structure"]
impl crate::Readable for TZC_ACTION {}
#[doc = "`write(|w| ..)` method takes [tzc_action::W](tzc_action::W) writer structure"]
impl crate::Writable for TZC_ACTION {}
#[doc = "Controls interrupt and bus error response behavior when regions permission failures occur."]
pub mod tzc_action;
#[doc = "Provides control and status for the gate keeper in each filter unit implemented.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_gate_keeper](tzc_gate_keeper) module"]
pub type TZC_GATE_KEEPER = crate::Reg<u32, _TZC_GATE_KEEPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_GATE_KEEPER;
#[doc = "`read()` method returns [tzc_gate_keeper::R](tzc_gate_keeper::R) reader structure"]
impl crate::Readable for TZC_GATE_KEEPER {}
#[doc = "`write(|w| ..)` method takes [tzc_gate_keeper::W](tzc_gate_keeper::W) writer structure"]
impl crate::Writable for TZC_GATE_KEEPER {}
#[doc = "Provides control and status for the gate keeper in each filter unit implemented."]
pub mod tzc_gate_keeper;
#[doc = "Controls read and write access speculation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_speculation_ctrl](tzc_speculation_ctrl) module"]
pub type TZC_SPECULATION_CTRL = crate::Reg<u32, _TZC_SPECULATION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_SPECULATION_CTRL;
#[doc = "`read()` method returns [tzc_speculation_ctrl::R](tzc_speculation_ctrl::R) reader structure"]
impl crate::Readable for TZC_SPECULATION_CTRL {}
#[doc = "`write(|w| ..)` method takes [tzc_speculation_ctrl::W](tzc_speculation_ctrl::W) writer structure"]
impl crate::Writable for TZC_SPECULATION_CTRL {}
#[doc = "Controls read and write access speculation."]
pub mod tzc_speculation_ctrl;
#[doc = "Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_int_status](tzc_int_status) module"]
pub type TZC_INT_STATUS = crate::Reg<u32, _TZC_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_INT_STATUS;
#[doc = "`read()` method returns [tzc_int_status::R](tzc_int_status::R) reader structure"]
impl crate::Readable for TZC_INT_STATUS {}
#[doc = "Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors."]
pub mod tzc_int_status;
#[doc = "Interrupt clear for each filter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_int_clear](tzc_int_clear) module"]
pub type TZC_INT_CLEAR = crate::Reg<u32, _TZC_INT_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_INT_CLEAR;
#[doc = "`read()` method returns [tzc_int_clear::R](tzc_int_clear::R) reader structure"]
impl crate::Readable for TZC_INT_CLEAR {}
#[doc = "`write(|w| ..)` method takes [tzc_int_clear::W](tzc_int_clear::W) writer structure"]
impl crate::Writable for TZC_INT_CLEAR {}
#[doc = "Interrupt clear for each filter."]
pub mod tzc_int_clear;
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_control0](tzc_fail_control0) module"]
pub type TZC_FAIL_CONTROL0 = crate::Reg<u32, _TZC_FAIL_CONTROL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_CONTROL0;
#[doc = "`read()` method returns [tzc_fail_control0::R](tzc_fail_control0::R) reader structure"]
impl crate::Readable for TZC_FAIL_CONTROL0 {}
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
pub mod tzc_fail_control0;
#[doc = "Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_id0](tzc_fail_id0) module"]
pub type TZC_FAIL_ID0 = crate::Reg<u32, _TZC_FAIL_ID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_ID0;
#[doc = "`read()` method returns [tzc_fail_id0::R](tzc_fail_id0::R) reader structure"]
impl crate::Readable for TZC_FAIL_ID0 {}
#[doc = "Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
pub mod tzc_fail_id0;
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_control1](tzc_fail_control1) module"]
pub type TZC_FAIL_CONTROL1 = crate::Reg<u32, _TZC_FAIL_CONTROL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_CONTROL1;
#[doc = "`read()` method returns [tzc_fail_control1::R](tzc_fail_control1::R) reader structure"]
impl crate::Readable for TZC_FAIL_CONTROL1 {}
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1)."]
pub mod tzc_fail_control1;
#[doc = "Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_id1](tzc_fail_id1) module"]
pub type TZC_FAIL_ID1 = crate::Reg<u32, _TZC_FAIL_ID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_ID1;
#[doc = "`read()` method returns [tzc_fail_id1::R](tzc_fail_id1::R) reader structure"]
impl crate::Readable for TZC_FAIL_ID1 {}
#[doc = "Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD)."]
pub mod tzc_fail_id1;
#[doc = "Region 0 attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute0](tzc_region_attribute0) module"]
pub type TZC_REGION_ATTRIBUTE0 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE0;
#[doc = "`read()` method returns [tzc_region_attribute0::R](tzc_region_attribute0::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE0 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute0::W](tzc_region_attribute0::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE0 {}
#[doc = "Region 0 attributes."]
pub mod tzc_region_attribute0;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute1](tzc_region_attribute1) module"]
pub type TZC_REGION_ATTRIBUTE1 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE1;
#[doc = "`read()` method returns [tzc_region_attribute1::R](tzc_region_attribute1::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE1 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute1::W](tzc_region_attribute1::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE1 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute1;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute2](tzc_region_attribute2) module"]
pub type TZC_REGION_ATTRIBUTE2 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE2;
#[doc = "`read()` method returns [tzc_region_attribute2::R](tzc_region_attribute2::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE2 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute2::W](tzc_region_attribute2::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE2 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute2;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute3](tzc_region_attribute3) module"]
pub type TZC_REGION_ATTRIBUTE3 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE3;
#[doc = "`read()` method returns [tzc_region_attribute3::R](tzc_region_attribute3::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE3 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute3::W](tzc_region_attribute3::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE3 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute3;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute4](tzc_region_attribute4) module"]
pub type TZC_REGION_ATTRIBUTE4 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE4;
#[doc = "`read()` method returns [tzc_region_attribute4::R](tzc_region_attribute4::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE4 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute4::W](tzc_region_attribute4::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE4 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute4;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute5](tzc_region_attribute5) module"]
pub type TZC_REGION_ATTRIBUTE5 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE5;
#[doc = "`read()` method returns [tzc_region_attribute5::R](tzc_region_attribute5::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE5 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute5::W](tzc_region_attribute5::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE5 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute5;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute6](tzc_region_attribute6) module"]
pub type TZC_REGION_ATTRIBUTE6 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE6;
#[doc = "`read()` method returns [tzc_region_attribute6::R](tzc_region_attribute6::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE6 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute6::W](tzc_region_attribute6::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE6 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute6;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute7](tzc_region_attribute7) module"]
pub type TZC_REGION_ATTRIBUTE7 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE7;
#[doc = "`read()` method returns [tzc_region_attribute7::R](tzc_region_attribute7::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE7 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute7::W](tzc_region_attribute7::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE7 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute7;
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute8](tzc_region_attribute8) module"]
pub type TZC_REGION_ATTRIBUTE8 = crate::Reg<u32, _TZC_REGION_ATTRIBUTE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ATTRIBUTE8;
#[doc = "`read()` method returns [tzc_region_attribute8::R](tzc_region_attribute8::R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE8 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute8::W](tzc_region_attribute8::W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE8 {}
#[doc = "Region x attributes."]
pub mod tzc_region_attribute8;
#[doc = "Peripheral ID 4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid4](tzc_pid4) module"]
pub type TZC_PID4 = crate::Reg<u32, _TZC_PID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID4;
#[doc = "`read()` method returns [tzc_pid4::R](tzc_pid4::R) reader structure"]
impl crate::Readable for TZC_PID4 {}
#[doc = "Peripheral ID 4."]
pub mod tzc_pid4;
#[doc = "Peripheral ID 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid5](tzc_pid5) module"]
pub type TZC_PID5 = crate::Reg<u32, _TZC_PID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID5;
#[doc = "`read()` method returns [tzc_pid5::R](tzc_pid5::R) reader structure"]
impl crate::Readable for TZC_PID5 {}
#[doc = "Peripheral ID 5."]
pub mod tzc_pid5;
#[doc = "Peripheral ID 6.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid6](tzc_pid6) module"]
pub type TZC_PID6 = crate::Reg<u32, _TZC_PID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID6;
#[doc = "`read()` method returns [tzc_pid6::R](tzc_pid6::R) reader structure"]
impl crate::Readable for TZC_PID6 {}
#[doc = "Peripheral ID 6."]
pub mod tzc_pid6;
#[doc = "Peripheral ID 7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid7](tzc_pid7) module"]
pub type TZC_PID7 = crate::Reg<u32, _TZC_PID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID7;
#[doc = "`read()` method returns [tzc_pid7::R](tzc_pid7::R) reader structure"]
impl crate::Readable for TZC_PID7 {}
#[doc = "Peripheral ID 7."]
pub mod tzc_pid7;
#[doc = "Peripheral ID 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid0](tzc_pid0) module"]
pub type TZC_PID0 = crate::Reg<u32, _TZC_PID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID0;
#[doc = "`read()` method returns [tzc_pid0::R](tzc_pid0::R) reader structure"]
impl crate::Readable for TZC_PID0 {}
#[doc = "Peripheral ID 0."]
pub mod tzc_pid0;
#[doc = "Peripheral ID 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid1](tzc_pid1) module"]
pub type TZC_PID1 = crate::Reg<u32, _TZC_PID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID1;
#[doc = "`read()` method returns [tzc_pid1::R](tzc_pid1::R) reader structure"]
impl crate::Readable for TZC_PID1 {}
#[doc = "Peripheral ID 1."]
pub mod tzc_pid1;
#[doc = "Peripheral ID 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid2](tzc_pid2) module"]
pub type TZC_PID2 = crate::Reg<u32, _TZC_PID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID2;
#[doc = "`read()` method returns [tzc_pid2::R](tzc_pid2::R) reader structure"]
impl crate::Readable for TZC_PID2 {}
#[doc = "Peripheral ID 2."]
pub mod tzc_pid2;
#[doc = "Peripheral ID 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid3](tzc_pid3) module"]
pub type TZC_PID3 = crate::Reg<u32, _TZC_PID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_PID3;
#[doc = "`read()` method returns [tzc_pid3::R](tzc_pid3::R) reader structure"]
impl crate::Readable for TZC_PID3 {}
#[doc = "Peripheral ID 3."]
pub mod tzc_pid3;
#[doc = "Component ID 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_cid0](tzc_cid0) module"]
pub type TZC_CID0 = crate::Reg<u32, _TZC_CID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_CID0;
#[doc = "`read()` method returns [tzc_cid0::R](tzc_cid0::R) reader structure"]
impl crate::Readable for TZC_CID0 {}
#[doc = "Component ID 0."]
pub mod tzc_cid0;
#[doc = "Component ID 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_cid1](tzc_cid1) module"]
pub type TZC_CID1 = crate::Reg<u32, _TZC_CID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_CID1;
#[doc = "`read()` method returns [tzc_cid1::R](tzc_cid1::R) reader structure"]
impl crate::Readable for TZC_CID1 {}
#[doc = "Component ID 1."]
pub mod tzc_cid1;
#[doc = "Component ID 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_cid2](tzc_cid2) module"]
pub type TZC_CID2 = crate::Reg<u32, _TZC_CID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_CID2;
#[doc = "`read()` method returns [tzc_cid2::R](tzc_cid2::R) reader structure"]
impl crate::Readable for TZC_CID2 {}
#[doc = "Component ID 2."]
pub mod tzc_cid2;
#[doc = "Component ID 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_cid3](tzc_cid3) module"]
pub type TZC_CID3 = crate::Reg<u32, _TZC_CID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_CID3;
#[doc = "`read()` method returns [tzc_cid3::R](tzc_cid3::R) reader structure"]
impl crate::Readable for TZC_CID3 {}
#[doc = "Component ID 3."]
pub mod tzc_cid3;
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_address_low0](tzc_fail_address_low0) module"]
pub type TZC_FAIL_ADDRESS_LOW0 = crate::Reg<u32, _TZC_FAIL_ADDRESS_LOW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_ADDRESS_LOW0;
#[doc = "`read()` method returns [tzc_fail_address_low0::R](tzc_fail_address_low0::R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_LOW0 {}
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1)."]
pub mod tzc_fail_address_low0;
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_address_high0](tzc_fail_address_high0) module"]
pub type TZC_FAIL_ADDRESS_HIGH0 = crate::Reg<u32, _TZC_FAIL_ADDRESS_HIGH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_ADDRESS_HIGH0;
#[doc = "`read()` method returns [tzc_fail_address_high0::R](tzc_fail_address_high0::R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_HIGH0 {}
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
pub mod tzc_fail_address_high0;
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_address_low1](tzc_fail_address_low1) module"]
pub type TZC_FAIL_ADDRESS_LOW1 = crate::Reg<u32, _TZC_FAIL_ADDRESS_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_ADDRESS_LOW1;
#[doc = "`read()` method returns [tzc_fail_address_low1::R](tzc_fail_address_low1::R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_LOW1 {}
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1)."]
pub mod tzc_fail_address_low1;
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_address_high1](tzc_fail_address_high1) module"]
pub type TZC_FAIL_ADDRESS_HIGH1 = crate::Reg<u32, _TZC_FAIL_ADDRESS_HIGH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_FAIL_ADDRESS_HIGH1;
#[doc = "`read()` method returns [tzc_fail_address_high1::R](tzc_fail_address_high1::R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_HIGH1 {}
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address."]
pub mod tzc_fail_address_high1;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high0](tzc_region_base_high0) module"]
pub type TZC_REGION_BASE_HIGH0 = crate::Reg<u32, _TZC_REGION_BASE_HIGH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH0;
#[doc = "`read()` method returns [tzc_region_base_high0::R](tzc_region_base_high0::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH0 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high0;
#[doc = "Top address bits \\[31:12\\]
for region 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low0](tzc_region_top_low0) module"]
pub type TZC_REGION_TOP_LOW0 = crate::Reg<u32, _TZC_REGION_TOP_LOW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW0;
#[doc = "`read()` method returns [tzc_region_top_low0::R](tzc_region_top_low0::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW0 {}
#[doc = "Top address bits \\[31:12\\]
for region 0."]
pub mod tzc_region_top_low0;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high0](tzc_region_top_high0) module"]
pub type TZC_REGION_TOP_HIGH0 = crate::Reg<u32, _TZC_REGION_TOP_HIGH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH0;
#[doc = "`read()` method returns [tzc_region_top_high0::R](tzc_region_top_high0::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH0 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high0;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access0](tzc_region_id_access0) module"]
pub type TZC_REGION_ID_ACCESS0 = crate::Reg<u32, _TZC_REGION_ID_ACCESS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS0;
#[doc = "`read()` method returns [tzc_region_id_access0::R](tzc_region_id_access0::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS0 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access0::W](tzc_region_id_access0::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS0 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access0;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low1](tzc_region_base_low1) module"]
pub type TZC_REGION_BASE_LOW1 = crate::Reg<u32, _TZC_REGION_BASE_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW1;
#[doc = "`read()` method returns [tzc_region_base_low1::R](tzc_region_base_low1::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW1 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low1::W](tzc_region_base_low1::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW1 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low1;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high1](tzc_region_base_high1) module"]
pub type TZC_REGION_BASE_HIGH1 = crate::Reg<u32, _TZC_REGION_BASE_HIGH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH1;
#[doc = "`read()` method returns [tzc_region_base_high1::R](tzc_region_base_high1::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH1 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high1;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low1](tzc_region_top_low1) module"]
pub type TZC_REGION_TOP_LOW1 = crate::Reg<u32, _TZC_REGION_TOP_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW1;
#[doc = "`read()` method returns [tzc_region_top_low1::R](tzc_region_top_low1::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW1 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low1::W](tzc_region_top_low1::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW1 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low1;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high1](tzc_region_top_high1) module"]
pub type TZC_REGION_TOP_HIGH1 = crate::Reg<u32, _TZC_REGION_TOP_HIGH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH1;
#[doc = "`read()` method returns [tzc_region_top_high1::R](tzc_region_top_high1::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH1 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high1;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access1](tzc_region_id_access1) module"]
pub type TZC_REGION_ID_ACCESS1 = crate::Reg<u32, _TZC_REGION_ID_ACCESS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS1;
#[doc = "`read()` method returns [tzc_region_id_access1::R](tzc_region_id_access1::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS1 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access1::W](tzc_region_id_access1::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS1 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access1;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low2](tzc_region_base_low2) module"]
pub type TZC_REGION_BASE_LOW2 = crate::Reg<u32, _TZC_REGION_BASE_LOW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW2;
#[doc = "`read()` method returns [tzc_region_base_low2::R](tzc_region_base_low2::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW2 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low2::W](tzc_region_base_low2::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW2 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low2;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high2](tzc_region_base_high2) module"]
pub type TZC_REGION_BASE_HIGH2 = crate::Reg<u32, _TZC_REGION_BASE_HIGH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH2;
#[doc = "`read()` method returns [tzc_region_base_high2::R](tzc_region_base_high2::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH2 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high2;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low2](tzc_region_top_low2) module"]
pub type TZC_REGION_TOP_LOW2 = crate::Reg<u32, _TZC_REGION_TOP_LOW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW2;
#[doc = "`read()` method returns [tzc_region_top_low2::R](tzc_region_top_low2::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW2 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low2::W](tzc_region_top_low2::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW2 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low2;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high2](tzc_region_top_high2) module"]
pub type TZC_REGION_TOP_HIGH2 = crate::Reg<u32, _TZC_REGION_TOP_HIGH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH2;
#[doc = "`read()` method returns [tzc_region_top_high2::R](tzc_region_top_high2::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH2 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high2;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access2](tzc_region_id_access2) module"]
pub type TZC_REGION_ID_ACCESS2 = crate::Reg<u32, _TZC_REGION_ID_ACCESS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS2;
#[doc = "`read()` method returns [tzc_region_id_access2::R](tzc_region_id_access2::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS2 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access2::W](tzc_region_id_access2::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS2 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access2;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low3](tzc_region_base_low3) module"]
pub type TZC_REGION_BASE_LOW3 = crate::Reg<u32, _TZC_REGION_BASE_LOW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW3;
#[doc = "`read()` method returns [tzc_region_base_low3::R](tzc_region_base_low3::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW3 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low3::W](tzc_region_base_low3::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW3 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low3;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high3](tzc_region_base_high3) module"]
pub type TZC_REGION_BASE_HIGH3 = crate::Reg<u32, _TZC_REGION_BASE_HIGH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH3;
#[doc = "`read()` method returns [tzc_region_base_high3::R](tzc_region_base_high3::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH3 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high3;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low3](tzc_region_top_low3) module"]
pub type TZC_REGION_TOP_LOW3 = crate::Reg<u32, _TZC_REGION_TOP_LOW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW3;
#[doc = "`read()` method returns [tzc_region_top_low3::R](tzc_region_top_low3::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW3 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low3::W](tzc_region_top_low3::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW3 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low3;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high3](tzc_region_top_high3) module"]
pub type TZC_REGION_TOP_HIGH3 = crate::Reg<u32, _TZC_REGION_TOP_HIGH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH3;
#[doc = "`read()` method returns [tzc_region_top_high3::R](tzc_region_top_high3::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH3 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high3;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access3](tzc_region_id_access3) module"]
pub type TZC_REGION_ID_ACCESS3 = crate::Reg<u32, _TZC_REGION_ID_ACCESS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS3;
#[doc = "`read()` method returns [tzc_region_id_access3::R](tzc_region_id_access3::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS3 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access3::W](tzc_region_id_access3::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS3 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access3;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low4](tzc_region_base_low4) module"]
pub type TZC_REGION_BASE_LOW4 = crate::Reg<u32, _TZC_REGION_BASE_LOW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW4;
#[doc = "`read()` method returns [tzc_region_base_low4::R](tzc_region_base_low4::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW4 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low4::W](tzc_region_base_low4::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW4 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low4;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high4](tzc_region_base_high4) module"]
pub type TZC_REGION_BASE_HIGH4 = crate::Reg<u32, _TZC_REGION_BASE_HIGH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH4;
#[doc = "`read()` method returns [tzc_region_base_high4::R](tzc_region_base_high4::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH4 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high4;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low4](tzc_region_top_low4) module"]
pub type TZC_REGION_TOP_LOW4 = crate::Reg<u32, _TZC_REGION_TOP_LOW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW4;
#[doc = "`read()` method returns [tzc_region_top_low4::R](tzc_region_top_low4::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW4 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low4::W](tzc_region_top_low4::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW4 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low4;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high4](tzc_region_top_high4) module"]
pub type TZC_REGION_TOP_HIGH4 = crate::Reg<u32, _TZC_REGION_TOP_HIGH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH4;
#[doc = "`read()` method returns [tzc_region_top_high4::R](tzc_region_top_high4::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH4 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high4;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access4](tzc_region_id_access4) module"]
pub type TZC_REGION_ID_ACCESS4 = crate::Reg<u32, _TZC_REGION_ID_ACCESS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS4;
#[doc = "`read()` method returns [tzc_region_id_access4::R](tzc_region_id_access4::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS4 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access4::W](tzc_region_id_access4::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS4 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access4;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low5](tzc_region_base_low5) module"]
pub type TZC_REGION_BASE_LOW5 = crate::Reg<u32, _TZC_REGION_BASE_LOW5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW5;
#[doc = "`read()` method returns [tzc_region_base_low5::R](tzc_region_base_low5::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW5 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low5::W](tzc_region_base_low5::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW5 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low5;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high5](tzc_region_base_high5) module"]
pub type TZC_REGION_BASE_HIGH5 = crate::Reg<u32, _TZC_REGION_BASE_HIGH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH5;
#[doc = "`read()` method returns [tzc_region_base_high5::R](tzc_region_base_high5::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH5 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high5;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low5](tzc_region_top_low5) module"]
pub type TZC_REGION_TOP_LOW5 = crate::Reg<u32, _TZC_REGION_TOP_LOW5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW5;
#[doc = "`read()` method returns [tzc_region_top_low5::R](tzc_region_top_low5::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW5 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low5::W](tzc_region_top_low5::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW5 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low5;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high5](tzc_region_top_high5) module"]
pub type TZC_REGION_TOP_HIGH5 = crate::Reg<u32, _TZC_REGION_TOP_HIGH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH5;
#[doc = "`read()` method returns [tzc_region_top_high5::R](tzc_region_top_high5::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH5 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high5;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access5](tzc_region_id_access5) module"]
pub type TZC_REGION_ID_ACCESS5 = crate::Reg<u32, _TZC_REGION_ID_ACCESS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS5;
#[doc = "`read()` method returns [tzc_region_id_access5::R](tzc_region_id_access5::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS5 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access5::W](tzc_region_id_access5::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS5 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access5;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low6](tzc_region_base_low6) module"]
pub type TZC_REGION_BASE_LOW6 = crate::Reg<u32, _TZC_REGION_BASE_LOW6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW6;
#[doc = "`read()` method returns [tzc_region_base_low6::R](tzc_region_base_low6::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW6 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low6::W](tzc_region_base_low6::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW6 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low6;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high6](tzc_region_base_high6) module"]
pub type TZC_REGION_BASE_HIGH6 = crate::Reg<u32, _TZC_REGION_BASE_HIGH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH6;
#[doc = "`read()` method returns [tzc_region_base_high6::R](tzc_region_base_high6::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH6 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high6;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low6](tzc_region_top_low6) module"]
pub type TZC_REGION_TOP_LOW6 = crate::Reg<u32, _TZC_REGION_TOP_LOW6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW6;
#[doc = "`read()` method returns [tzc_region_top_low6::R](tzc_region_top_low6::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW6 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low6::W](tzc_region_top_low6::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW6 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low6;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high6](tzc_region_top_high6) module"]
pub type TZC_REGION_TOP_HIGH6 = crate::Reg<u32, _TZC_REGION_TOP_HIGH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH6;
#[doc = "`read()` method returns [tzc_region_top_high6::R](tzc_region_top_high6::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH6 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high6;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access6](tzc_region_id_access6) module"]
pub type TZC_REGION_ID_ACCESS6 = crate::Reg<u32, _TZC_REGION_ID_ACCESS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS6;
#[doc = "`read()` method returns [tzc_region_id_access6::R](tzc_region_id_access6::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS6 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access6::W](tzc_region_id_access6::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS6 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access6;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low7](tzc_region_base_low7) module"]
pub type TZC_REGION_BASE_LOW7 = crate::Reg<u32, _TZC_REGION_BASE_LOW7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW7;
#[doc = "`read()` method returns [tzc_region_base_low7::R](tzc_region_base_low7::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW7 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low7::W](tzc_region_base_low7::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW7 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low7;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high7](tzc_region_base_high7) module"]
pub type TZC_REGION_BASE_HIGH7 = crate::Reg<u32, _TZC_REGION_BASE_HIGH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH7;
#[doc = "`read()` method returns [tzc_region_base_high7::R](tzc_region_base_high7::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH7 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high7;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low7](tzc_region_top_low7) module"]
pub type TZC_REGION_TOP_LOW7 = crate::Reg<u32, _TZC_REGION_TOP_LOW7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW7;
#[doc = "`read()` method returns [tzc_region_top_low7::R](tzc_region_top_low7::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW7 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low7::W](tzc_region_top_low7::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW7 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low7;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high7](tzc_region_top_high7) module"]
pub type TZC_REGION_TOP_HIGH7 = crate::Reg<u32, _TZC_REGION_TOP_HIGH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH7;
#[doc = "`read()` method returns [tzc_region_top_high7::R](tzc_region_top_high7::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH7 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high7;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access7](tzc_region_id_access7) module"]
pub type TZC_REGION_ID_ACCESS7 = crate::Reg<u32, _TZC_REGION_ID_ACCESS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS7;
#[doc = "`read()` method returns [tzc_region_id_access7::R](tzc_region_id_access7::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS7 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access7::W](tzc_region_id_access7::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS7 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access7;
#[doc = "Base address low for regions 1 to 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_low8](tzc_region_base_low8) module"]
pub type TZC_REGION_BASE_LOW8 = crate::Reg<u32, _TZC_REGION_BASE_LOW8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_LOW8;
#[doc = "`read()` method returns [tzc_region_base_low8::R](tzc_region_base_low8::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_LOW8 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_base_low8::W](tzc_region_base_low8::W) writer structure"]
impl crate::Writable for TZC_REGION_BASE_LOW8 {}
#[doc = "Base address low for regions 1 to 8."]
pub mod tzc_region_base_low8;
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high8](tzc_region_base_high8) module"]
pub type TZC_REGION_BASE_HIGH8 = crate::Reg<u32, _TZC_REGION_BASE_HIGH8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_BASE_HIGH8;
#[doc = "`read()` method returns [tzc_region_base_high8::R](tzc_region_base_high8::R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH8 {}
#[doc = "Base address high are not used with 32-bit address."]
pub mod tzc_region_base_high8;
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low8](tzc_region_top_low8) module"]
pub type TZC_REGION_TOP_LOW8 = crate::Reg<u32, _TZC_REGION_TOP_LOW8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_LOW8;
#[doc = "`read()` method returns [tzc_region_top_low8::R](tzc_region_top_low8::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW8 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low8::W](tzc_region_top_low8::W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW8 {}
#[doc = "Top address bits \\[31:12\\]
for region x."]
pub mod tzc_region_top_low8;
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high8](tzc_region_top_high8) module"]
pub type TZC_REGION_TOP_HIGH8 = crate::Reg<u32, _TZC_REGION_TOP_HIGH8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_TOP_HIGH8;
#[doc = "`read()` method returns [tzc_region_top_high8::R](tzc_region_top_high8::R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH8 {}
#[doc = "Top address high of region are not used with 32-bit address."]
pub mod tzc_region_top_high8;
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access8](tzc_region_id_access8) module"]
pub type TZC_REGION_ID_ACCESS8 = crate::Reg<u32, _TZC_REGION_ID_ACCESS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_REGION_ID_ACCESS8;
#[doc = "`read()` method returns [tzc_region_id_access8::R](tzc_region_id_access8::R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS8 {}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access8::W](tzc_region_id_access8::W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS8 {}
#[doc = "Region non-secure access based on NSAID."]
pub mod tzc_region_id_access8;
