///Register `CNTL` reader
pub type R = crate::R<CNTLrs>;
///Register `CNTL` writer
pub type W = crate::W<CNTLrs>;
///Field `CNTL` reader - RTC counter register Low
pub type CNTL_R = crate::FieldReader<u16>;
///Field `CNTL` writer - RTC counter register Low
pub type CNTL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - RTC counter register Low
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL").field("cntl", &self.cntl()).finish()
    }
}
impl W {
    ///Bits 0:15 - RTC counter register Low
    #[inline(always)]
    pub fn cntl(&mut self) -> CNTL_W<'_, CNTLrs> {
        CNTL_W::new(self, 0)
    }
}
/**RTC Counter Register Low

You can [`read`](crate::Reg::read) this register and get [`cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#RTC:CNTL)*/
pub struct CNTLrs;
impl crate::RegisterSpec for CNTLrs {
    type Ux = u32;
}
///`read()` method returns [`cntl::R`](R) reader structure
impl crate::Readable for CNTLrs {}
///`write(|w| ..)` method takes [`cntl::W`](W) writer structure
impl crate::Writable for CNTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTL to value 0
impl crate::Resettable for CNTLrs {}
