#[doc = "Register `EXTI_SECCFGR1` reader"]
pub type R = crate::R<EXTI_SECCFGR1rs>;
#[doc = "Register `EXTI_SECCFGR1` writer"]
pub type W = crate::W<EXTI_SECCFGR1rs>;
#[doc = "Field `SEC0` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC0_R = crate::BitReader;
#[doc = "Field `SEC0` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC1` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC1_R = crate::BitReader;
#[doc = "Field `SEC1` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC2` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC2_R = crate::BitReader;
#[doc = "Field `SEC2` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC3` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC3_R = crate::BitReader;
#[doc = "Field `SEC3` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC4` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC4_R = crate::BitReader;
#[doc = "Field `SEC4` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC5` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC5_R = crate::BitReader;
#[doc = "Field `SEC5` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC6` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC6_R = crate::BitReader;
#[doc = "Field `SEC6` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC7` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC7_R = crate::BitReader;
#[doc = "Field `SEC7` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC8` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC8_R = crate::BitReader;
#[doc = "Field `SEC8` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC9` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC9_R = crate::BitReader;
#[doc = "Field `SEC9` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC10` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC10_R = crate::BitReader;
#[doc = "Field `SEC10` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC11` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC11_R = crate::BitReader;
#[doc = "Field `SEC11` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC12` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC12_R = crate::BitReader;
#[doc = "Field `SEC12` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC13` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC13_R = crate::BitReader;
#[doc = "Field `SEC13` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC14` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC14_R = crate::BitReader;
#[doc = "Field `SEC14` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC15` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC15_R = crate::BitReader;
#[doc = "Field `SEC15` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC16` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC16_R = crate::BitReader;
#[doc = "Field `SEC16` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC17` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC17_R = crate::BitReader;
#[doc = "Field `SEC17` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC18` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC18_R = crate::BitReader;
#[doc = "Field `SEC18` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC19` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC19_R = crate::BitReader;
#[doc = "Field `SEC19` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC20` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC20_R = crate::BitReader;
#[doc = "Field `SEC20` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC21` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC21_R = crate::BitReader;
#[doc = "Field `SEC21` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC22` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC22_R = crate::BitReader;
#[doc = "Field `SEC22` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC23` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC23_R = crate::BitReader;
#[doc = "Field `SEC23` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC24` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC24_R = crate::BitReader;
#[doc = "Field `SEC24` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC25` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC25_R = crate::BitReader;
#[doc = "Field `SEC25` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
pub type SEC25_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec16(&self) -> SEC16_R {
        SEC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec17(&self) -> SEC17_R {
        SEC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec18(&self) -> SEC18_R {
        SEC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec19(&self) -> SEC19_R {
        SEC19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec20(&self) -> SEC20_R {
        SEC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec21(&self) -> SEC21_R {
        SEC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec22(&self) -> SEC22_R {
        SEC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec23(&self) -> SEC23_R {
        SEC23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec24(&self) -> SEC24_R {
        SEC24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    pub fn sec25(&self) -> SEC25_R {
        SEC25_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<EXTI_SECCFGR1rs> {
        SEC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<EXTI_SECCFGR1rs> {
        SEC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<EXTI_SECCFGR1rs> {
        SEC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<EXTI_SECCFGR1rs> {
        SEC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<EXTI_SECCFGR1rs> {
        SEC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<EXTI_SECCFGR1rs> {
        SEC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<EXTI_SECCFGR1rs> {
        SEC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<EXTI_SECCFGR1rs> {
        SEC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec8(&mut self) -> SEC8_W<EXTI_SECCFGR1rs> {
        SEC8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec9(&mut self) -> SEC9_W<EXTI_SECCFGR1rs> {
        SEC9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<EXTI_SECCFGR1rs> {
        SEC10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec11(&mut self) -> SEC11_W<EXTI_SECCFGR1rs> {
        SEC11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec12(&mut self) -> SEC12_W<EXTI_SECCFGR1rs> {
        SEC12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec13(&mut self) -> SEC13_W<EXTI_SECCFGR1rs> {
        SEC13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec14(&mut self) -> SEC14_W<EXTI_SECCFGR1rs> {
        SEC14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec15(&mut self) -> SEC15_W<EXTI_SECCFGR1rs> {
        SEC15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec16(&mut self) -> SEC16_W<EXTI_SECCFGR1rs> {
        SEC16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec17(&mut self) -> SEC17_W<EXTI_SECCFGR1rs> {
        SEC17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec18(&mut self) -> SEC18_W<EXTI_SECCFGR1rs> {
        SEC18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec19(&mut self) -> SEC19_W<EXTI_SECCFGR1rs> {
        SEC19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec20(&mut self) -> SEC20_W<EXTI_SECCFGR1rs> {
        SEC20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec21(&mut self) -> SEC21_W<EXTI_SECCFGR1rs> {
        SEC21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec22(&mut self) -> SEC22_W<EXTI_SECCFGR1rs> {
        SEC22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec23(&mut self) -> SEC23_W<EXTI_SECCFGR1rs> {
        SEC23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec24(&mut self) -> SEC24_W<EXTI_SECCFGR1rs> {
        SEC24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded. Note: SEC25, SEC24, and SEC23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sec25(&mut self) -> SEC25_W<EXTI_SECCFGR1rs> {
        SEC25_W::new(self, 25)
    }
}
#[doc = "EXTI security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_seccfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_seccfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_SECCFGR1rs;
impl crate::RegisterSpec for EXTI_SECCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_seccfgr1::R`](R) reader structure"]
impl crate::Readable for EXTI_SECCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`exti_seccfgr1::W`](W) writer structure"]
impl crate::Writable for EXTI_SECCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_SECCFGR1 to value 0"]
impl crate::Resettable for EXTI_SECCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
