///Register `TIM17_ARR` reader
pub type R = crate::R<TIM17_ARRrs>;
///Register `TIM17_ARR` writer
pub type W = crate::W<TIM17_ARRrs>;
///Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 20.3.1: Time-base unit on page 526 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
pub type ARR_R = crate::FieldReader<u16>;
///Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 20.3.1: Time-base unit on page 526 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 20.3.1: Time-base unit on page 526 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM17_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 20.3.1: Time-base unit on page 526 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null.
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<'_, TIM17_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM17 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim17_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM17:TIM17_ARR)*/
pub struct TIM17_ARRrs;
impl crate::RegisterSpec for TIM17_ARRrs {
    type Ux = u16;
}
///`read()` method returns [`tim17_arr::R`](R) reader structure
impl crate::Readable for TIM17_ARRrs {}
///`write(|w| ..)` method takes [`tim17_arr::W`](W) writer structure
impl crate::Writable for TIM17_ARRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM17_ARR to value 0xffff
impl crate::Resettable for TIM17_ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
