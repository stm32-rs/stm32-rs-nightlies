///Register `AWD1LTR` reader
pub type R = crate::R<AWD1LTRrs>;
///Register `AWD1LTR` writer
pub type W = crate::W<AWD1LTRrs>;
///Field `LTR` reader - Analog watchdog 1 lower threshold
pub type LTR_R = crate::FieldReader<u32>;
///Field `LTR` writer - Analog watchdog 1 lower threshold
pub type LTR_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bits 0:22 - Analog watchdog 1 lower threshold
    #[inline(always)]
    pub fn ltr(&self) -> LTR_R {
        LTR_R::new(self.bits & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD1LTR").field("ltr", &self.ltr()).finish()
    }
}
impl W {
    ///Bits 0:22 - Analog watchdog 1 lower threshold
    #[inline(always)]
    pub fn ltr(&mut self) -> LTR_W<'_, AWD1LTRrs> {
        LTR_W::new(self, 0)
    }
}
/**ADC analog watchdog 1 lower threshold register

You can [`read`](crate::Reg::read) this register and get [`awd1ltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1ltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ADC1:AWD1LTR)*/
pub struct AWD1LTRrs;
impl crate::RegisterSpec for AWD1LTRrs {
    type Ux = u32;
}
///`read()` method returns [`awd1ltr::R`](R) reader structure
impl crate::Readable for AWD1LTRrs {}
///`write(|w| ..)` method takes [`awd1ltr::W`](W) writer structure
impl crate::Writable for AWD1LTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD1LTR to value 0
impl crate::Resettable for AWD1LTRrs {}
