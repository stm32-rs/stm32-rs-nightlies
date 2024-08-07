///Register `TZC_INT_CLEAR` reader
pub type R = crate::R<TZC_INT_CLEARrs>;
///Register `TZC_INT_CLEAR` writer
pub type W = crate::W<TZC_INT_CLEARrs>;
///Field `CLEAR` writer - CLEAR
pub type CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_INT_CLEAR").finish()
    }
}
impl W {
    ///Bits 0:1 - CLEAR
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<TZC_INT_CLEARrs> {
        CLEAR_W::new(self, 0)
    }
}
/**Interrupt clear for each filter.

You can [`read`](crate::Reg::read) this register and get [`tzc_int_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_int_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:TZC_INT_CLEAR)*/
pub struct TZC_INT_CLEARrs;
impl crate::RegisterSpec for TZC_INT_CLEARrs {
    type Ux = u32;
}
///`read()` method returns [`tzc_int_clear::R`](R) reader structure
impl crate::Readable for TZC_INT_CLEARrs {}
///`write(|w| ..)` method takes [`tzc_int_clear::W`](W) writer structure
impl crate::Writable for TZC_INT_CLEARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TZC_INT_CLEAR to value 0
impl crate::Resettable for TZC_INT_CLEARrs {
    const RESET_VALUE: u32 = 0;
}
