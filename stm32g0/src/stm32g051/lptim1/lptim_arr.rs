///Register `LPTIM_ARR` reader
pub type R = crate::R<LPTIM_ARRrs>;
///Register `LPTIM_ARR` writer
pub type W = crate::W<LPTIM_ARRrs>;
///Field `ARR` reader - Auto reload value
pub type ARR_R = crate::FieldReader<u16>;
///Field `ARR` writer - Auto reload value
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Auto reload value
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Auto reload value
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<LPTIM_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**Autoreload Register

You can [`read`](crate::Reg::read) this register and get [`lptim_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPTIM1:LPTIM_ARR)*/
pub struct LPTIM_ARRrs;
impl crate::RegisterSpec for LPTIM_ARRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim_arr::R`](R) reader structure
impl crate::Readable for LPTIM_ARRrs {}
///`write(|w| ..)` method takes [`lptim_arr::W`](W) writer structure
impl crate::Writable for LPTIM_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM_ARR to value 0x01
impl crate::Resettable for LPTIM_ARRrs {
    const RESET_VALUE: u32 = 0x01;
}
