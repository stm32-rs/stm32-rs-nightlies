///Register `TIM8_ARR` reader
pub type R = crate::R<TIM8_ARRrs>;
///Register `TIM8_ARR` writer
pub type W = crate::W<TIM8_ARRrs>;
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
        f.debug_struct("TIM8_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ARR
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<TIM8_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM8 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim8_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_ARR)*/
pub struct TIM8_ARRrs;
impl crate::RegisterSpec for TIM8_ARRrs {
    type Ux = u16;
}
///`read()` method returns [`tim8_arr::R`](R) reader structure
impl crate::Readable for TIM8_ARRrs {}
///`write(|w| ..)` method takes [`tim8_arr::W`](W) writer structure
impl crate::Writable for TIM8_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM8_ARR to value 0xffff
impl crate::Resettable for TIM8_ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
