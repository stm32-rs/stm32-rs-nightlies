///Register `TIM1_ARR` reader
pub type R = crate::R<TIM1_ARRrs>;
///Register `TIM1_ARR` writer
pub type W = crate::W<TIM1_ARRrs>;
///Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 17.3.1: Time-base unit on page 331 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
pub type ARR_R = crate::FieldReader<u16>;
///Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 17.3.1: Time-base unit on page 331 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 17.3.1: Time-base unit on page 331 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 17.3.1: Time-base unit on page 331 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<'_, TIM1_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM1 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim1_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_ARR)*/
pub struct TIM1_ARRrs;
impl crate::RegisterSpec for TIM1_ARRrs {
    type Ux = u16;
}
///`read()` method returns [`tim1_arr::R`](R) reader structure
impl crate::Readable for TIM1_ARRrs {}
///`write(|w| ..)` method takes [`tim1_arr::W`](W) writer structure
impl crate::Writable for TIM1_ARRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_ARR to value 0xffff
impl crate::Resettable for TIM1_ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
