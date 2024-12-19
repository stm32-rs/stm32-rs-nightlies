///Register `TIM6_ARR` reader
pub type R = crate::R<TIM6_ARRrs>;
///Register `TIM6_ARR` writer
pub type W = crate::W<TIM6_ARRrs>;
///Field `ARR` reader - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
pub type ARR_R = crate::FieldReader<u16>;
///Field `ARR` writer - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM6_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Prescaler value ARR is the value to be loaded into the actual auto-reload register. Refer to Section123.3.1: Time-base unit on page1596 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<TIM6_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM6 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim6_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TIM6:TIM6_ARR)*/
pub struct TIM6_ARRrs;
impl crate::RegisterSpec for TIM6_ARRrs {
    type Ux = u16;
}
///`read()` method returns [`tim6_arr::R`](R) reader structure
impl crate::Readable for TIM6_ARRrs {}
///`write(|w| ..)` method takes [`tim6_arr::W`](W) writer structure
impl crate::Writable for TIM6_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM6_ARR to value 0xffff
impl crate::Resettable for TIM6_ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
