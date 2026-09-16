///Register `TIM15_ARR` reader
pub type R = crate::R<TIM15_ARRrs>;
///Register `TIM15_ARR` writer
pub type W = crate::W<TIM15_ARRrs>;
///Field `ARR` reader - Auto-reload value
pub type ARR_R = crate::FieldReader<u16>;
///Field `ARR` writer - Auto-reload value
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Auto-reload value
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Auto-reload value
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<'_, TIM15_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM15 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim15_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_ARR)*/
pub struct TIM15_ARRrs;
impl crate::RegisterSpec for TIM15_ARRrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_arr::R`](R) reader structure
impl crate::Readable for TIM15_ARRrs {}
///`write(|w| ..)` method takes [`tim15_arr::W`](W) writer structure
impl crate::Writable for TIM15_ARRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_ARR to value 0xffff
impl crate::Resettable for TIM15_ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
