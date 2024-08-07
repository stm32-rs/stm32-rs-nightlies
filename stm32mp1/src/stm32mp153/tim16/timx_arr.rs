///Register `TIMx_ARR` reader
pub type R = crate::R<TIMX_ARRrs>;
///Register `TIMx_ARR` writer
pub type W = crate::W<TIMX_ARRrs>;
///Field `ARR` reader - ARR
pub type ARR_R = crate::FieldReader<u16>;
///Field `ARR` writer - ARR
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - ARR
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMx_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ARR
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<TIMX_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM16/TIM17 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`timx_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timx_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM16:TIMx_ARR)*/
pub struct TIMX_ARRrs;
impl crate::RegisterSpec for TIMX_ARRrs {
    type Ux = u16;
}
///`read()` method returns [`timx_arr::R`](R) reader structure
impl crate::Readable for TIMX_ARRrs {}
///`write(|w| ..)` method takes [`timx_arr::W`](W) writer structure
impl crate::Writable for TIMX_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIMx_ARR to value 0xffff
impl crate::Resettable for TIMX_ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
