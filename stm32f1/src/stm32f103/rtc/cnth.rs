///Register `CNTH` reader
pub type R = crate::R<CNTHrs>;
///Register `CNTH` writer
pub type W = crate::W<CNTHrs>;
///Field `CNTH` reader - RTC counter register high
pub type CNTH_R = crate::FieldReader<u16>;
///Field `CNTH` writer - RTC counter register high
pub type CNTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - RTC counter register high
    #[inline(always)]
    pub fn cnth(&self) -> CNTH_R {
        CNTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTH").field("cnth", &self.cnth()).finish()
    }
}
impl W {
    ///Bits 0:15 - RTC counter register high
    #[inline(always)]
    pub fn cnth(&mut self) -> CNTH_W<'_, CNTHrs> {
        CNTH_W::new(self, 0)
    }
}
/**RTC Counter Register High

You can [`read`](crate::Reg::read) this register and get [`cnth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:CNTH)*/
pub struct CNTHrs;
impl crate::RegisterSpec for CNTHrs {
    type Ux = u32;
}
///`read()` method returns [`cnth::R`](R) reader structure
impl crate::Readable for CNTHrs {}
///`write(|w| ..)` method takes [`cnth::W`](W) writer structure
impl crate::Writable for CNTHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTH to value 0
impl crate::Resettable for CNTHrs {}
