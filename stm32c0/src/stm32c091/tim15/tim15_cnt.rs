///Register `TIM15_CNT` reader
pub type R = crate::R<TIM15_CNTrs>;
///Register `TIM15_CNT` writer
pub type W = crate::W<TIM15_CNTrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - Counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIF Copy
pub type UIFCPY_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIF Copy
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, TIM15_CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM15 counter

You can [`read`](crate::Reg::read) this register and get [`tim15_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CNT)*/
pub struct TIM15_CNTrs;
impl crate::RegisterSpec for TIM15_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim15_cnt::R`](R) reader structure
impl crate::Readable for TIM15_CNTrs {}
///`write(|w| ..)` method takes [`tim15_cnt::W`](W) writer structure
impl crate::Writable for TIM15_CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_CNT to value 0
impl crate::Resettable for TIM15_CNTrs {}
