///Register `OTPLSRP` reader
pub type R = crate::R<OTPLSRPrs>;
///Register `OTPLSRP` writer
pub type W = crate::W<OTPLSRPrs>;
///Field `OTPL` reader - OTP lock n programming Write to change corresponding option byte bit in FLASH_OTPLSR. OTPL bits can be only be set, not cleared.
pub type OTPL_R = crate::FieldReader<u16>;
///Field `OTPL` writer - OTP lock n programming Write to change corresponding option byte bit in FLASH_OTPLSR. OTPL bits can be only be set, not cleared.
pub type OTPL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - OTP lock n programming Write to change corresponding option byte bit in FLASH_OTPLSR. OTPL bits can be only be set, not cleared.
    #[inline(always)]
    pub fn otpl(&self) -> OTPL_R {
        OTPL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPLSRP")
            .field("otpl", &self.otpl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - OTP lock n programming Write to change corresponding option byte bit in FLASH_OTPLSR. OTPL bits can be only be set, not cleared.
    #[inline(always)]
    pub fn otpl(&mut self) -> OTPL_W<'_, OTPLSRPrs> {
        OTPL_W::new(self, 0)
    }
}
/**FLASH OTP lock status register programming

You can [`read`](crate::Reg::read) this register and get [`otplsrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otplsrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:OTPLSRP)*/
pub struct OTPLSRPrs;
impl crate::RegisterSpec for OTPLSRPrs {
    type Ux = u32;
}
///`read()` method returns [`otplsrp::R`](R) reader structure
impl crate::Readable for OTPLSRPrs {}
///`write(|w| ..)` method takes [`otplsrp::W`](W) writer structure
impl crate::Writable for OTPLSRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTPLSRP to value 0
impl crate::Resettable for OTPLSRPrs {}
