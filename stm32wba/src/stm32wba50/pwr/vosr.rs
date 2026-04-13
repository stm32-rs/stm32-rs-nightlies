///Register `VOSR` reader
pub type R = crate::R<VOSRrs>;
///Register `VOSR` writer
pub type W = crate::W<VOSRrs>;
///Field `VOSRDY` reader - Ready bit for VsubCORE/sub voltage scaling output selection Set and cleared by hardware. When decreasing the voltage scaling range, VOSRDY must be one before increasing the SYSCLK frequency.
pub type VOSRDY_R = crate::BitReader;
///Field `VOS` reader - Voltage scaling range selection Set a and cleared by software. Cleared by hardware when entering Stop 1 mode. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type VOS_R = crate::BitReader;
///Field `VOS` writer - Voltage scaling range selection Set a and cleared by software. Cleared by hardware when entering Stop 1 mode. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type VOS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 15 - Ready bit for VsubCORE/sub voltage scaling output selection Set and cleared by hardware. When decreasing the voltage scaling range, VOSRDY must be one before increasing the SYSCLK frequency.
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Voltage scaling range selection Set a and cleared by software. Cleared by hardware when entering Stop 1 mode. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOSR")
            .field("vosrdy", &self.vosrdy())
            .field("vos", &self.vos())
            .finish()
    }
}
impl W {
    ///Bit 16 - Voltage scaling range selection Set a and cleared by software. Cleared by hardware when entering Stop 1 mode. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, VOSRrs> {
        VOS_W::new(self, 16)
    }
}
/**PWR voltage scaling register

You can [`read`](crate::Reg::read) this register and get [`vosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:VOSR)*/
pub struct VOSRrs;
impl crate::RegisterSpec for VOSRrs {
    type Ux = u32;
}
///`read()` method returns [`vosr::R`](R) reader structure
impl crate::Readable for VOSRrs {}
///`write(|w| ..)` method takes [`vosr::W`](W) writer structure
impl crate::Writable for VOSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VOSR to value 0x8000
impl crate::Resettable for VOSRrs {
    const RESET_VALUE: u32 = 0x8000;
}
