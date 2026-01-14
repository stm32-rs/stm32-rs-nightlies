///Register `TIM3_CNT_ALTERNATE1` reader
pub type R = crate::R<TIM3_CNT_ALTERNATE1rs>;
///Register `TIM3_CNT_ALTERNATE1` writer
pub type W = crate::W<TIM3_CNT_ALTERNATE1rs>;
///Field `CNT` reader - counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_R = crate::BitReader;
///Field `UIFCPY` writer - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_CNT_ALTERNATE1")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, TIM3_CNT_ALTERNATE1rs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<'_, TIM3_CNT_ALTERNATE1rs> {
        UIFCPY_W::new(self, 31)
    }
}
/**TIM3 counter

You can [`read`](crate::Reg::read) this register and get [`tim3_cnt_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_cnt_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM3:TIM3_CNT_ALTERNATE1)*/
pub struct TIM3_CNT_ALTERNATE1rs;
impl crate::RegisterSpec for TIM3_CNT_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`tim3_cnt_alternate1::R`](R) reader structure
impl crate::Readable for TIM3_CNT_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`tim3_cnt_alternate1::W`](W) writer structure
impl crate::Writable for TIM3_CNT_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_CNT_ALTERNATE1 to value 0
impl crate::Resettable for TIM3_CNT_ALTERNATE1rs {}
