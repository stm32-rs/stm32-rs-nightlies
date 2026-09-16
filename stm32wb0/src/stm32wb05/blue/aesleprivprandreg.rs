///Register `AESLEPRIVPRANDREG` reader
pub type R = crate::R<AESLEPRIVPRANDREGrs>;
///Register `AESLEPRIVPRANDREG` writer
pub type W = crate::W<AESLEPRIVPRANDREGrs>;
///Field `PRAND` reader - AES Le privacy Prand
pub type PRAND_R = crate::FieldReader<u32>;
///Field `PRAND` writer - AES Le privacy Prand
pub type PRAND_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - AES Le privacy Prand
    #[inline(always)]
    pub fn prand(&self) -> PRAND_R {
        PRAND_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESLEPRIVPRANDREG")
            .field("prand", &self.prand())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - AES Le privacy Prand
    #[inline(always)]
    pub fn prand(&mut self) -> PRAND_W<'_, AESLEPRIVPRANDREGrs> {
        PRAND_W::new(self, 0)
    }
}
/**AESLEPRIVPRANDREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivprandreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivprandreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:AESLEPRIVPRANDREG)*/
pub struct AESLEPRIVPRANDREGrs;
impl crate::RegisterSpec for AESLEPRIVPRANDREGrs {
    type Ux = u32;
}
///`read()` method returns [`aesleprivprandreg::R`](R) reader structure
impl crate::Readable for AESLEPRIVPRANDREGrs {}
///`write(|w| ..)` method takes [`aesleprivprandreg::W`](W) writer structure
impl crate::Writable for AESLEPRIVPRANDREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AESLEPRIVPRANDREG to value 0
impl crate::Resettable for AESLEPRIVPRANDREGrs {}
