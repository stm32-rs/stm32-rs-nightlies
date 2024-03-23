#[doc = "Register `HSLVR` reader"]
pub type R = crate::R<HSLVRrs>;
#[doc = "Register `HSLVR` writer"]
pub type W = crate::W<HSLVRrs>;
#[doc = "Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSLV0 {
    #[doc = "0: I/O speed optimization disabled"]
    Disabled = 0,
    #[doc = "1: I/O speed optimization enabled"]
    Enabled = 1,
}
impl From<HSLV0> for bool {
    #[inline(always)]
    fn from(variant: HSLV0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSLV0` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV0_R = crate::BitReader<HSLV0>;
impl HSLV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSLV0 {
        match self.bits {
            false => HSLV0::Disabled,
            true => HSLV0::Enabled,
        }
    }
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSLV0::Disabled
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSLV0::Enabled
    }
}
#[doc = "Field `HSLV0` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV0_W<'a, REG> = crate::BitWriter<'a, REG, HSLV0>;
impl<'a, REG> HSLV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O speed optimization disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV0::Disabled)
    }
    #[doc = "I/O speed optimization enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSLV0::Enabled)
    }
}
#[doc = "Field `HSLV1` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV1_R;
#[doc = "Field `HSLV2` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV2_R;
#[doc = "Field `HSLV3` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV3_R;
#[doc = "Field `HSLV4` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV4_R;
#[doc = "Field `HSLV5` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV5_R;
#[doc = "Field `HSLV6` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV6_R;
#[doc = "Field `HSLV7` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV7_R;
#[doc = "Field `HSLV8` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV8_R;
#[doc = "Field `HSLV9` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV9_R;
#[doc = "Field `HSLV10` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV10_R;
#[doc = "Field `HSLV11` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV11_R;
#[doc = "Field `HSLV12` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV12_R;
#[doc = "Field `HSLV13` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV13_R;
#[doc = "Field `HSLV14` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV14_R;
#[doc = "Field `HSLV15` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_R as HSLV15_R;
#[doc = "Field `HSLV1` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV1_W;
#[doc = "Field `HSLV2` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV2_W;
#[doc = "Field `HSLV3` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV3_W;
#[doc = "Field `HSLV4` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV4_W;
#[doc = "Field `HSLV5` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV5_W;
#[doc = "Field `HSLV6` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV6_W;
#[doc = "Field `HSLV7` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV7_W;
#[doc = "Field `HSLV8` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV8_W;
#[doc = "Field `HSLV9` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV9_W;
#[doc = "Field `HSLV10` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV10_W;
#[doc = "Field `HSLV11` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV11_W;
#[doc = "Field `HSLV12` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV12_W;
#[doc = "Field `HSLV13` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV13_W;
#[doc = "Field `HSLV14` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV14_W;
#[doc = "Field `HSLV15` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use HSLV0_W as HSLV15_W;
impl R {
    #[doc = "Bit 0 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv0(&self) -> HSLV0_R {
        HSLV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv1(&self) -> HSLV1_R {
        HSLV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv2(&self) -> HSLV2_R {
        HSLV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv3(&self) -> HSLV3_R {
        HSLV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv4(&self) -> HSLV4_R {
        HSLV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv5(&self) -> HSLV5_R {
        HSLV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv6(&self) -> HSLV6_R {
        HSLV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv7(&self) -> HSLV7_R {
        HSLV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv8(&self) -> HSLV8_R {
        HSLV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv9(&self) -> HSLV9_R {
        HSLV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv10(&self) -> HSLV10_R {
        HSLV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv11(&self) -> HSLV11_R {
        HSLV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv12(&self) -> HSLV12_R {
        HSLV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv13(&self) -> HSLV13_R {
        HSLV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv14(&self) -> HSLV14_R {
        HSLV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv15(&self) -> HSLV15_R {
        HSLV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv0(&mut self) -> HSLV0_W<HSLVRrs> {
        HSLV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv1(&mut self) -> HSLV1_W<HSLVRrs> {
        HSLV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv2(&mut self) -> HSLV2_W<HSLVRrs> {
        HSLV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv3(&mut self) -> HSLV3_W<HSLVRrs> {
        HSLV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv4(&mut self) -> HSLV4_W<HSLVRrs> {
        HSLV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv5(&mut self) -> HSLV5_W<HSLVRrs> {
        HSLV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv6(&mut self) -> HSLV6_W<HSLVRrs> {
        HSLV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv7(&mut self) -> HSLV7_W<HSLVRrs> {
        HSLV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv8(&mut self) -> HSLV8_W<HSLVRrs> {
        HSLV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv9(&mut self) -> HSLV9_W<HSLVRrs> {
        HSLV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv10(&mut self) -> HSLV10_W<HSLVRrs> {
        HSLV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv11(&mut self) -> HSLV11_W<HSLVRrs> {
        HSLV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv12(&mut self) -> HSLV12_W<HSLVRrs> {
        HSLV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv13(&mut self) -> HSLV13_W<HSLVRrs> {
        HSLV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv14(&mut self) -> HSLV14_W<HSLVRrs> {
        HSLV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv15(&mut self) -> HSLV15_W<HSLVRrs> {
        HSLV15_W::new(self, 15)
    }
}
#[doc = "GPIO high-speed low-voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hslvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hslvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSLVRrs;
impl crate::RegisterSpec for HSLVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hslvr::R`](R) reader structure"]
impl crate::Readable for HSLVRrs {}
#[doc = "`write(|w| ..)` method takes [`hslvr::W`](W) writer structure"]
impl crate::Writable for HSLVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSLVR to value 0"]
impl crate::Resettable for HSLVRrs {
    const RESET_VALUE: u32 = 0;
}
