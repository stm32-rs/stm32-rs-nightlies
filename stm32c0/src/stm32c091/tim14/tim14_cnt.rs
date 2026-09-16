///Register `TIM14_CNT` reader
pub type R = crate::R<TIM14_CNTrs>;
///Register `TIM14_CNT` writer
pub type W = crate::W<TIM14_CNTrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - Counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit in the TIMx_ISR register.
pub type UIFCPY_R = crate::BitReader;
///Field `UIFCPY` writer - UIF Copy This bit is a read-only copy of the UIF bit in the TIMx_ISR register.
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit in the TIMx_ISR register.
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM14_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, TIM14_CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit in the TIMx_ISR register.
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<'_, TIM14_CNTrs> {
        UIFCPY_W::new(self, 31)
    }
}
/**TIM14 counter

You can [`read`](crate::Reg::read) this register and get [`tim14_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM14:TIM14_CNT)*/
pub struct TIM14_CNTrs;
impl crate::RegisterSpec for TIM14_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim14_cnt::R`](R) reader structure
impl crate::Readable for TIM14_CNTrs {}
///`write(|w| ..)` method takes [`tim14_cnt::W`](W) writer structure
impl crate::Writable for TIM14_CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM14_CNT to value 0
impl crate::Resettable for TIM14_CNTrs {}
