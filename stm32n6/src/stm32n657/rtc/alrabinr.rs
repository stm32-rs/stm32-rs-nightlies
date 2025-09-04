///Register `ALRABINR` reader
pub type R = crate::R<ALRABINRrs>;
///Register `ALRABINR` writer
pub type W = crate::W<ALRABINRrs>;
///Field `SS` reader - Synchronous counter alarm value in Binary mode
pub type SS_R = crate::FieldReader<u32>;
///Field `SS` writer - Synchronous counter alarm value in Binary mode
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRABINR").field("ss", &self.ss()).finish()
    }
}
impl W {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<ALRABINRrs> {
        SS_W::new(self, 0)
    }
}
/**RTC alarm A binary mode register

You can [`read`](crate::Reg::read) this register and get [`alrabinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrabinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RTC:ALRABINR)*/
pub struct ALRABINRrs;
impl crate::RegisterSpec for ALRABINRrs {
    type Ux = u32;
}
///`read()` method returns [`alrabinr::R`](R) reader structure
impl crate::Readable for ALRABINRrs {}
///`write(|w| ..)` method takes [`alrabinr::W`](W) writer structure
impl crate::Writable for ALRABINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRABINR to value 0
impl crate::Resettable for ALRABINRrs {}
