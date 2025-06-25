///Register `TIM_CNT` reader
pub type R = crate::R<TIM_CNTrs>;
///Register `TIM_CNT` writer
pub type W = crate::W<TIM_CNTrs>;
///Field `CNT` reader - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `UIFCPY` reader - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_R = crate::BitReader;
///Field `UIFCPY` writer - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
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
    ///Bits 0:30 - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<TIM_CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<TIM_CNTrs> {
        UIFCPY_W::new(self, 31)
    }
}
/**TIM counter

You can [`read`](crate::Reg::read) this register and get [`tim_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CNT)*/
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
