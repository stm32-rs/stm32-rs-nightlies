///Register `CNTR` reader
pub type R = crate::R<CNTRrs>;
///Register `CNTR` writer
pub type W = crate::W<CNTRrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - Counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTR").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CNTRrs> {
        CNT_W::new(self, 0)
    }
}
/**Master Timer Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_Master:CNTR)*/
pub struct CNTRrs;
impl crate::RegisterSpec for CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`cntr::R`](R) reader structure
impl crate::Readable for CNTRrs {}
///`write(|w| ..)` method takes [`cntr::W`](W) writer structure
impl crate::Writable for CNTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTR to value 0
impl crate::Resettable for CNTRrs {}
