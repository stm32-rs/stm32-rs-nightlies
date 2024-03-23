#[doc = "Register `PWR_VOSR` reader"]
pub type R = crate::R<PWR_VOSRrs>;
#[doc = "Register `PWR_VOSR` writer"]
pub type W = crate::W<PWR_VOSRrs>;
#[doc = "Field `USBBOOSTRDY` reader - USB EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The USB clock can be provided only after this bit is set. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
pub type USBBOOSTRDY_R = crate::BitReader;
#[doc = "Field `BOOSTRDY` reader - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set."]
pub type BOOSTRDY_R = crate::BitReader;
#[doc = "Field `VOSRDY` reader - Ready bit for VCORE voltage scaling output selection"]
pub type VOSRDY_R = crate::BitReader;
#[doc = "Field `VOS` reader - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1."]
pub type VOS_R = crate::FieldReader;
#[doc = "Field `VOS` writer - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1."]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BOOSTEN` reader - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3)."]
pub type BOOSTEN_R = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3)."]
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPWREN` reader - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
pub type USBPWREN_R = crate::BitReader;
#[doc = "Field `USBPWREN` writer - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
pub type USBPWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBBOOSTEN` reader - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
pub type USBBOOSTEN_R = crate::BitReader;
#[doc = "Field `USBBOOSTEN` writer - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
pub type USBBOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - USB EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The USB clock can be provided only after this bit is set. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
    #[inline(always)]
    pub fn usbboostrdy(&self) -> USBBOOSTRDY_R {
        USBBOOSTRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set."]
    #[inline(always)]
    pub fn boostrdy(&self) -> BOOSTRDY_R {
        BOOSTRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ready bit for VCORE voltage scaling output selection"]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1."]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3)."]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
    #[inline(always)]
    pub fn usbpwren(&self) -> USBPWREN_R {
        USBPWREN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
    #[inline(always)]
    pub fn usbboosten(&self) -> USBBOOSTEN_R {
        USBBOOSTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1."]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<PWR_VOSRrs> {
        VOS_W::new(self, 16)
    }
    #[doc = "Bit 18 - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3)."]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<PWR_VOSRrs> {
        BOOSTEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
    #[inline(always)]
    #[must_use]
    pub fn usbpwren(&mut self) -> USBPWREN_W<PWR_VOSRrs> {
        USBPWREN_W::new(self, 19)
    }
    #[doc = "Bit 20 - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585."]
    #[inline(always)]
    #[must_use]
    pub fn usbboosten(&mut self) -> USBBOOSTEN_W<PWR_VOSRrs> {
        USBBOOSTEN_W::new(self, 20)
    }
}
#[doc = "PWR voltage scaling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_vosr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_vosr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_VOSRrs;
impl crate::RegisterSpec for PWR_VOSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_vosr::R`](R) reader structure"]
impl crate::Readable for PWR_VOSRrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_vosr::W`](W) writer structure"]
impl crate::Writable for PWR_VOSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_VOSR to value 0x8000"]
impl crate::Resettable for PWR_VOSRrs {
    const RESET_VALUE: u32 = 0x8000;
}
