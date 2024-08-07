///Register `HSLVR` reader
pub type R = crate::R<HSLVRrs>;
///Register `HSLVR` writer
pub type W = crate::W<HSLVRrs>;
///Field `HSLV0` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV0_R = crate::BitReader;
///Field `HSLV0` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV1` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV1_R = crate::BitReader;
///Field `HSLV1` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV2` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV2_R = crate::BitReader;
///Field `HSLV2` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV3` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV3_R = crate::BitReader;
///Field `HSLV3` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV4` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV4_R = crate::BitReader;
///Field `HSLV4` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV5` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV5_R = crate::BitReader;
///Field `HSLV5` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV6` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV6_R = crate::BitReader;
///Field `HSLV6` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV7` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV7_R = crate::BitReader;
///Field `HSLV7` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV8` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV8_R = crate::BitReader;
///Field `HSLV8` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV9` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV9_R = crate::BitReader;
///Field `HSLV9` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV10` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV10_R = crate::BitReader;
///Field `HSLV10` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV11` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV11_R = crate::BitReader;
///Field `HSLV11` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV12` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV12_R = crate::BitReader;
///Field `HSLV12` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV13` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV13_R = crate::BitReader;
///Field `HSLV13` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV14` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV14_R = crate::BitReader;
///Field `HSLV14` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV15` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV15_R = crate::BitReader;
///Field `HSLV15` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv0(&self) -> HSLV0_R {
        HSLV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv1(&self) -> HSLV1_R {
        HSLV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv2(&self) -> HSLV2_R {
        HSLV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv3(&self) -> HSLV3_R {
        HSLV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv4(&self) -> HSLV4_R {
        HSLV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv5(&self) -> HSLV5_R {
        HSLV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv6(&self) -> HSLV6_R {
        HSLV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv7(&self) -> HSLV7_R {
        HSLV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv8(&self) -> HSLV8_R {
        HSLV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv9(&self) -> HSLV9_R {
        HSLV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv10(&self) -> HSLV10_R {
        HSLV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv11(&self) -> HSLV11_R {
        HSLV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv12(&self) -> HSLV12_R {
        HSLV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv13(&self) -> HSLV13_R {
        HSLV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv14(&self) -> HSLV14_R {
        HSLV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv15(&self) -> HSLV15_R {
        HSLV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSLVR")
            .field("hslv0", &self.hslv0())
            .field("hslv1", &self.hslv1())
            .field("hslv2", &self.hslv2())
            .field("hslv3", &self.hslv3())
            .field("hslv4", &self.hslv4())
            .field("hslv5", &self.hslv5())
            .field("hslv6", &self.hslv6())
            .field("hslv7", &self.hslv7())
            .field("hslv8", &self.hslv8())
            .field("hslv9", &self.hslv9())
            .field("hslv10", &self.hslv10())
            .field("hslv11", &self.hslv11())
            .field("hslv12", &self.hslv12())
            .field("hslv13", &self.hslv13())
            .field("hslv14", &self.hslv14())
            .field("hslv15", &self.hslv15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv0(&mut self) -> HSLV0_W<HSLVRrs> {
        HSLV0_W::new(self, 0)
    }
    ///Bit 1 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv1(&mut self) -> HSLV1_W<HSLVRrs> {
        HSLV1_W::new(self, 1)
    }
    ///Bit 2 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv2(&mut self) -> HSLV2_W<HSLVRrs> {
        HSLV2_W::new(self, 2)
    }
    ///Bit 3 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv3(&mut self) -> HSLV3_W<HSLVRrs> {
        HSLV3_W::new(self, 3)
    }
    ///Bit 4 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv4(&mut self) -> HSLV4_W<HSLVRrs> {
        HSLV4_W::new(self, 4)
    }
    ///Bit 5 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv5(&mut self) -> HSLV5_W<HSLVRrs> {
        HSLV5_W::new(self, 5)
    }
    ///Bit 6 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv6(&mut self) -> HSLV6_W<HSLVRrs> {
        HSLV6_W::new(self, 6)
    }
    ///Bit 7 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv7(&mut self) -> HSLV7_W<HSLVRrs> {
        HSLV7_W::new(self, 7)
    }
    ///Bit 8 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv8(&mut self) -> HSLV8_W<HSLVRrs> {
        HSLV8_W::new(self, 8)
    }
    ///Bit 9 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv9(&mut self) -> HSLV9_W<HSLVRrs> {
        HSLV9_W::new(self, 9)
    }
    ///Bit 10 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv10(&mut self) -> HSLV10_W<HSLVRrs> {
        HSLV10_W::new(self, 10)
    }
    ///Bit 11 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv11(&mut self) -> HSLV11_W<HSLVRrs> {
        HSLV11_W::new(self, 11)
    }
    ///Bit 12 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv12(&mut self) -> HSLV12_W<HSLVRrs> {
        HSLV12_W::new(self, 12)
    }
    ///Bit 13 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv13(&mut self) -> HSLV13_W<HSLVRrs> {
        HSLV13_W::new(self, 13)
    }
    ///Bit 14 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv14(&mut self) -> HSLV14_W<HSLVRrs> {
        HSLV14_W::new(self, 14)
    }
    ///Bit 15 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn hslv15(&mut self) -> HSLV15_W<HSLVRrs> {
        HSLV15_W::new(self, 15)
    }
}
/**GPIO high-speed low-voltage register

You can [`read`](crate::Reg::read) this register and get [`hslvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hslvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPIOJ:HSLVR)*/
pub struct HSLVRrs;
impl crate::RegisterSpec for HSLVRrs {
    type Ux = u32;
}
///`read()` method returns [`hslvr::R`](R) reader structure
impl crate::Readable for HSLVRrs {}
///`write(|w| ..)` method takes [`hslvr::W`](W) writer structure
impl crate::Writable for HSLVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSLVR to value 0
impl crate::Resettable for HSLVRrs {
    const RESET_VALUE: u32 = 0;
}
