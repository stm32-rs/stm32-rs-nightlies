///Register `TIM1_CNT` reader
pub type R = crate::R<TIM1_CNTrs>;
///Register `TIM1_CNT` writer
pub type W = crate::W<TIM1_CNTrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - Counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIF copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in the TIMxCR1 is reset, bit 31 is reserved and read at 0.
pub type UIFCPY_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIF copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in the TIMxCR1 is reset, bit 31 is reserved and read at 0.
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<TIM1_CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM1 counter

You can [`read`](crate::Reg::read) this register and get [`tim1_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TIM1:TIM1_CNT)*/
pub struct TIM1_CNTrs;
impl crate::RegisterSpec for TIM1_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim1_cnt::R`](R) reader structure
impl crate::Readable for TIM1_CNTrs {}
///`write(|w| ..)` method takes [`tim1_cnt::W`](W) writer structure
impl crate::Writable for TIM1_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM1_CNT to value 0
impl crate::Resettable for TIM1_CNTrs {
    const RESET_VALUE: u32 = 0;
}
