///Register `SMPR2` reader
pub type R = crate::R<SMPR2rs>;
///Register `SMPR2` writer
pub type W = crate::W<SMPR2rs>;
///Field `SMPx_x` reader - Sample time bits
pub type SMPX_X_R = crate::FieldReader<u32>;
///Field `SMPx_x` writer - Sample time bits
pub type SMPX_X_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Sample time bits
    #[inline(always)]
    pub fn smpx_x(&self) -> SMPX_X_R {
        SMPX_X_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR2")
            .field("smpx_x", &self.smpx_x())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Sample time bits
    #[inline(always)]
    pub fn smpx_x(&mut self) -> SMPX_X_W<'_, SMPR2rs> {
        SMPX_X_W::new(self, 0)
    }
}
/**sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#ADC1:SMPR2)*/
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
///`read()` method returns [`smpr2::R`](R) reader structure
impl crate::Readable for SMPR2rs {}
///`write(|w| ..)` method takes [`smpr2::W`](W) writer structure
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR2 to value 0
impl crate::Resettable for SMPR2rs {}
