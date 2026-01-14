///Register `TIM2_CNT` reader
pub type R = crate::R<TIM2_CNTrs>;
///Register `TIM2_CNT` writer
pub type W = crate::W<TIM2_CNTrs>;
///Field `CNT` reader - Least significant part of counter value
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Least significant part of counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Least significant part of counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_CNT")
            .field("cnt", &self.cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Least significant part of counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, TIM2_CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM2 counter

You can [`read`](crate::Reg::read) this register and get [`tim2_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM2:TIM2_CNT)*/
pub struct TIM2_CNTrs;
impl crate::RegisterSpec for TIM2_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim2_cnt::R`](R) reader structure
impl crate::Readable for TIM2_CNTrs {}
///`write(|w| ..)` method takes [`tim2_cnt::W`](W) writer structure
impl crate::Writable for TIM2_CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_CNT to value 0
impl crate::Resettable for TIM2_CNTrs {}
