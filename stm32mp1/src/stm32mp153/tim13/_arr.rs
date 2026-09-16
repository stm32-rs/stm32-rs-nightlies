///Register `_ARR` reader
pub type R = crate::R<_ARRrs>;
///Register `_ARR` writer
pub type W = crate::W<_ARRrs>;
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
        f.debug_struct("_ARR").field("arr", &self.arr()).finish()
    }
}
impl W {
    ///Bits 0:15 - ARR
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<'_, _ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM13 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM13:_ARR)*/
pub struct _ARRrs;
impl crate::RegisterSpec for _ARRrs {
    type Ux = u16;
}
///`read()` method returns [`_arr::R`](R) reader structure
impl crate::Readable for _ARRrs {}
///`write(|w| ..)` method takes [`_arr::W`](W) writer structure
impl crate::Writable for _ARRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _ARR to value 0xffff
impl crate::Resettable for _ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
