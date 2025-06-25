///Register `TIM_CNT` reader
pub type R = crate::R<TIM_CNTrs>;
///Register `TIM_CNT` writer
pub type W = crate::W<TIM_CNTrs>;
///Field `CNT` reader - Counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register only holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - Counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register only holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved.
pub type UIFCPY_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register only holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved.
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register only holds the non-dithered part in CNT\[15:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<TIM_CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM counter

You can [`read`](crate::Reg::read) this register and get [`tim_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#TIM16:TIM_CNT)*/
pub struct TIM_CNTrs;
impl crate::RegisterSpec for TIM_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim_cnt::R`](R) reader structure
impl crate::Readable for TIM_CNTrs {}
///`write(|w| ..)` method takes [`tim_cnt::W`](W) writer structure
impl crate::Writable for TIM_CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM_CNT to value 0
impl crate::Resettable for TIM_CNTrs {}
