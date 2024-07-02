///Register `PWR_VOSR` reader
pub type R = crate::R<PWR_VOSRrs>;
///Register `PWR_VOSR` writer
pub type W = crate::W<PWR_VOSRrs>;
///Field `USBBOOSTRDY` reader - USB EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The USB clock can be provided only after this bit is set. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
pub type USBBOOSTRDY_R = crate::BitReader;
///Field `BOOSTRDY` reader - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.
pub type BOOSTRDY_R = crate::BitReader;
///Field `VOSRDY` reader - Ready bit for VCORE voltage scaling output selection
pub type VOSRDY_R = crate::BitReader;
///Field `VOS` reader - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
pub type VOS_R = crate::FieldReader;
///Field `VOS` writer - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BOOSTEN` reader - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3).
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3).
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBPWREN` reader - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
pub type USBPWREN_R = crate::BitReader;
///Field `USBPWREN` writer - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
pub type USBPWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBBOOSTEN` reader - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
pub type USBBOOSTEN_R = crate::BitReader;
///Field `USBBOOSTEN` writer - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
pub type USBBOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - USB EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The USB clock can be provided only after this bit is set. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
    #[inline(always)]
    pub fn usbboostrdy(&self) -> USBBOOSTRDY_R {
        USBBOOSTRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.
    #[inline(always)]
    pub fn boostrdy(&self) -> BOOSTRDY_R {
        BOOSTRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Ready bit for VCORE voltage scaling output selection
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3).
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
    #[inline(always)]
    pub fn usbpwren(&self) -> USBPWREN_R {
        USBPWREN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
    #[inline(always)]
    pub fn usbboosten(&self) -> USBBOOSTEN_R {
        USBBOOSTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_VOSR")
            .field("usbboostrdy", &self.usbboostrdy())
            .field("boostrdy", &self.boostrdy())
            .field("vosrdy", &self.vosrdy())
            .field("vos", &self.vos())
            .field("boosten", &self.boosten())
            .field("usbpwren", &self.usbpwren())
            .field("usbboosten", &self.usbboosten())
            .finish()
    }
}
impl W {
    ///Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<PWR_VOSRrs> {
        VOS_W::new(self, 16)
    }
    ///Bit 18 - EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before increasing the system clock frequency above 50 MHz. This bit is reset when going into Stop modes (0, 1, 2, 3).
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<PWR_VOSRrs> {
        BOOSTEN_W::new(self, 18)
    }
    ///Bit 19 - USB power enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
    #[inline(always)]
    #[must_use]
    pub fn usbpwren(&mut self) -> USBPWREN_W<PWR_VOSRrs> {
        USBPWREN_W::new(self, 19)
    }
    ///Bit 20 - USB EPOD booster enable This bit is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1. This bit must be set in range 1 and range 2 before enabling the USB peripheral. This bit is reset when going into Stop modes (0, 1, 2, 3). Note: This bit is only available in STM32U59x/5Ax. It is reserved in STM32U575/585.
    #[inline(always)]
    #[must_use]
    pub fn usbboosten(&mut self) -> USBBOOSTEN_W<PWR_VOSRrs> {
        USBBOOSTEN_W::new(self, 20)
    }
}
/**PWR voltage scaling register

You can [`read`](crate::Reg::read) this register and get [`pwr_vosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_vosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#PWR:PWR_VOSR)*/
pub struct PWR_VOSRrs;
impl crate::RegisterSpec for PWR_VOSRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_vosr::R`](R) reader structure
impl crate::Readable for PWR_VOSRrs {}
///`write(|w| ..)` method takes [`pwr_vosr::W`](W) writer structure
impl crate::Writable for PWR_VOSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_VOSR to value 0x8000
impl crate::Resettable for PWR_VOSRrs {
    const RESET_VALUE: u32 = 0x8000;
}
