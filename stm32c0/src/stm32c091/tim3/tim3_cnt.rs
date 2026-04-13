///Register `TIM3_CNT` reader
pub type R = crate::R<TIM3_CNTrs>;
///Register `TIM3_CNT` writer
pub type W = crate::W<TIM3_CNTrs>;
///Field `CNT` reader - counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_CNT")
            .field("cnt", &self.cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, TIM3_CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM3 counter

You can [`read`](crate::Reg::read) this register and get [`tim3_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CNT)*/
pub struct TIM3_CNTrs;
impl crate::RegisterSpec for TIM3_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim3_cnt::R`](R) reader structure
impl crate::Readable for TIM3_CNTrs {}
///`write(|w| ..)` method takes [`tim3_cnt::W`](W) writer structure
impl crate::Writable for TIM3_CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_CNT to value 0
impl crate::Resettable for TIM3_CNTrs {}
