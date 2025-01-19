///Register `TIM7_CNT` reader
pub type R = crate::R<TIM7_CNTrs>;
///Register `TIM7_CNT` writer
pub type W = crate::W<TIM7_CNTrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - Counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0.
pub type UIFCPY_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0.
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM7_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<TIM7_CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM7 counter

You can [`read`](crate::Reg::read) this register and get [`tim7_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM7:TIM7_CNT)*/
pub struct TIM7_CNTrs;
impl crate::RegisterSpec for TIM7_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim7_cnt::R`](R) reader structure
impl crate::Readable for TIM7_CNTrs {}
///`write(|w| ..)` method takes [`tim7_cnt::W`](W) writer structure
impl crate::Writable for TIM7_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM7_CNT to value 0
impl crate::Resettable for TIM7_CNTrs {
    const RESET_VALUE: u32 = 0;
}
