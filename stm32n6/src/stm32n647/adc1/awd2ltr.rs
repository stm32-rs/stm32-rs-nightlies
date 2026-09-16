///Register `AWD2LTR` reader
pub type R = crate::R<AWD2LTRrs>;
///Register `AWD2LTR` writer
pub type W = crate::W<AWD2LTRrs>;
///Field `LTR` reader - Analog watchdog 2 lower threshold
pub type LTR_R = crate::FieldReader<u32>;
///Field `LTR` writer - Analog watchdog 2 lower threshold
pub type LTR_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bits 0:22 - Analog watchdog 2 lower threshold
    #[inline(always)]
    pub fn ltr(&self) -> LTR_R {
        LTR_R::new(self.bits & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2LTR").field("ltr", &self.ltr()).finish()
    }
}
impl W {
    ///Bits 0:22 - Analog watchdog 2 lower threshold
    #[inline(always)]
    pub fn ltr(&mut self) -> LTR_W<'_, AWD2LTRrs> {
        LTR_W::new(self, 0)
    }
}
/**ADC analog watchdog 2 lower threshold register

You can [`read`](crate::Reg::read) this register and get [`awd2ltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2ltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ADC1:AWD2LTR)*/
pub struct AWD2LTRrs;
impl crate::RegisterSpec for AWD2LTRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2ltr::R`](R) reader structure
impl crate::Readable for AWD2LTRrs {}
///`write(|w| ..)` method takes [`awd2ltr::W`](W) writer structure
impl crate::Writable for AWD2LTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD2LTR to value 0
impl crate::Resettable for AWD2LTRrs {}
