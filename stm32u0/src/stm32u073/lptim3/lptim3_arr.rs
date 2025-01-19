///Register `LPTIM3_ARR` reader
pub type R = crate::R<LPTIM3_ARRrs>;
///Register `LPTIM3_ARR` writer
pub type W = crate::W<LPTIM3_ARRrs>;
/**Field `ARR` reader - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\[15:0\]
value.*/
pub type ARR_R = crate::FieldReader<u16>;
/**Field `ARR` writer - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\[15:0\]
value.*/
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    /**Bits 0:15 - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\[15:0\]
    value.*/
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM3_ARR")
            .field("arr", &self.arr())
            .finish()
    }
}
impl W {
    /**Bits 0:15 - Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\[15:0\]
    value.*/
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<LPTIM3_ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**LPTIM autoreload register

You can [`read`](crate::Reg::read) this register and get [`lptim3_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim3_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM3:LPTIM3_ARR)*/
pub struct LPTIM3_ARRrs;
impl crate::RegisterSpec for LPTIM3_ARRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim3_arr::R`](R) reader structure
impl crate::Readable for LPTIM3_ARRrs {}
///`write(|w| ..)` method takes [`lptim3_arr::W`](W) writer structure
impl crate::Writable for LPTIM3_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM3_ARR to value 0x01
impl crate::Resettable for LPTIM3_ARRrs {
    const RESET_VALUE: u32 = 0x01;
}
