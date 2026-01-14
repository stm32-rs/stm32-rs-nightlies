///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Register `CNT` writer
pub type W = crate::W<CNTrs>;
///Field `CNT` reader - CNT
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - CNT
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - CNT
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:15 - CNT
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM6 counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#TIM6:CNT)*/
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u16;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`write(|w| ..)` method takes [`cnt::W`](W) writer structure
impl crate::Writable for CNTrs {
    type Safety = crate::Safe;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {}
