#[doc = "Register `EXTI_PRIVCFGR1` reader"]
pub type R = crate::R<EXTI_PRIVCFGR1rs>;
#[doc = "Register `EXTI_PRIVCFGR1` writer"]
pub type W = crate::W<EXTI_PRIVCFGR1rs>;
#[doc = "Field `PRIV0` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV0_R = crate::BitReader;
#[doc = "Field `PRIV0` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV1` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV1_R = crate::BitReader;
#[doc = "Field `PRIV1` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV2` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV2_R = crate::BitReader;
#[doc = "Field `PRIV2` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV3` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV3_R = crate::BitReader;
#[doc = "Field `PRIV3` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV4` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV4_R = crate::BitReader;
#[doc = "Field `PRIV4` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV5` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV5_R = crate::BitReader;
#[doc = "Field `PRIV5` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV6` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV6_R = crate::BitReader;
#[doc = "Field `PRIV6` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV7` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV7_R = crate::BitReader;
#[doc = "Field `PRIV7` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV8` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV8_R = crate::BitReader;
#[doc = "Field `PRIV8` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV9` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV9_R = crate::BitReader;
#[doc = "Field `PRIV9` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV10` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV10_R = crate::BitReader;
#[doc = "Field `PRIV10` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV11` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV11_R = crate::BitReader;
#[doc = "Field `PRIV11` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV12` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV12_R = crate::BitReader;
#[doc = "Field `PRIV12` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV13` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV13_R = crate::BitReader;
#[doc = "Field `PRIV13` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV14` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV14_R = crate::BitReader;
#[doc = "Field `PRIV14` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV15` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV15_R = crate::BitReader;
#[doc = "Field `PRIV15` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV16` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV16_R = crate::BitReader;
#[doc = "Field `PRIV16` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV17` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV17_R = crate::BitReader;
#[doc = "Field `PRIV17` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV18` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV18_R = crate::BitReader;
#[doc = "Field `PRIV18` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV19` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV19_R = crate::BitReader;
#[doc = "Field `PRIV19` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV20` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV20_R = crate::BitReader;
#[doc = "Field `PRIV20` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV21` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV21_R = crate::BitReader;
#[doc = "Field `PRIV21` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV22` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV22_R = crate::BitReader;
#[doc = "Field `PRIV22` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV23` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV23_R = crate::BitReader;
#[doc = "Field `PRIV23` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV24` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV24_R = crate::BitReader;
#[doc = "Field `PRIV24` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV25` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV25_R = crate::BitReader;
#[doc = "Field `PRIV25` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type PRIV25_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv18(&self) -> PRIV18_R {
        PRIV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv20(&self) -> PRIV20_R {
        PRIV20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv23(&self) -> PRIV23_R {
        PRIV23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn priv25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<EXTI_PRIVCFGR1rs> {
        PRIV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<EXTI_PRIVCFGR1rs> {
        PRIV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<EXTI_PRIVCFGR1rs> {
        PRIV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<EXTI_PRIVCFGR1rs> {
        PRIV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv4(&mut self) -> PRIV4_W<EXTI_PRIVCFGR1rs> {
        PRIV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv5(&mut self) -> PRIV5_W<EXTI_PRIVCFGR1rs> {
        PRIV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv6(&mut self) -> PRIV6_W<EXTI_PRIVCFGR1rs> {
        PRIV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv7(&mut self) -> PRIV7_W<EXTI_PRIVCFGR1rs> {
        PRIV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv8(&mut self) -> PRIV8_W<EXTI_PRIVCFGR1rs> {
        PRIV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv9(&mut self) -> PRIV9_W<EXTI_PRIVCFGR1rs> {
        PRIV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv10(&mut self) -> PRIV10_W<EXTI_PRIVCFGR1rs> {
        PRIV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv11(&mut self) -> PRIV11_W<EXTI_PRIVCFGR1rs> {
        PRIV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv12(&mut self) -> PRIV12_W<EXTI_PRIVCFGR1rs> {
        PRIV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv13(&mut self) -> PRIV13_W<EXTI_PRIVCFGR1rs> {
        PRIV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv14(&mut self) -> PRIV14_W<EXTI_PRIVCFGR1rs> {
        PRIV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv15(&mut self) -> PRIV15_W<EXTI_PRIVCFGR1rs> {
        PRIV15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv16(&mut self) -> PRIV16_W<EXTI_PRIVCFGR1rs> {
        PRIV16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv17(&mut self) -> PRIV17_W<EXTI_PRIVCFGR1rs> {
        PRIV17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv18(&mut self) -> PRIV18_W<EXTI_PRIVCFGR1rs> {
        PRIV18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv19(&mut self) -> PRIV19_W<EXTI_PRIVCFGR1rs> {
        PRIV19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv20(&mut self) -> PRIV20_W<EXTI_PRIVCFGR1rs> {
        PRIV20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv21(&mut self) -> PRIV21_W<EXTI_PRIVCFGR1rs> {
        PRIV21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv22(&mut self) -> PRIV22_W<EXTI_PRIVCFGR1rs> {
        PRIV22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv23(&mut self) -> PRIV23_W<EXTI_PRIVCFGR1rs> {
        PRIV23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv24(&mut self) -> PRIV24_W<EXTI_PRIVCFGR1rs> {
        PRIV24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn priv25(&mut self) -> PRIV25_W<EXTI_PRIVCFGR1rs> {
        PRIV25_W::new(self, 25)
    }
}
#[doc = "EXTI privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_privcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_privcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_PRIVCFGR1rs;
impl crate::RegisterSpec for EXTI_PRIVCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_privcfgr1::R`](R) reader structure"]
impl crate::Readable for EXTI_PRIVCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`exti_privcfgr1::W`](W) writer structure"]
impl crate::Writable for EXTI_PRIVCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_PRIVCFGR1 to value 0"]
impl crate::Resettable for EXTI_PRIVCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
