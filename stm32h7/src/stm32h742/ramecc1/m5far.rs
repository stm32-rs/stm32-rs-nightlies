///Register `M5FAR` reader
pub type R = crate::R<M5FARrs>;
///Register `M5FAR` writer
pub type W = crate::W<M5FARrs>;
///Field `FADD` reader - ECC failing address
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC failing address
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5FAR").field("fadd", &self.fadd()).finish()
    }
}
impl W {}
/**RAMECC monitor x failing address register

You can [`read`](crate::Reg::read) this register and get [`m5far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#RAMECC1:M5FAR)*/
pub struct M5FARrs;
impl crate::RegisterSpec for M5FARrs {
    type Ux = u32;
}
///`read()` method returns [`m5far::R`](R) reader structure
impl crate::Readable for M5FARrs {}
///`write(|w| ..)` method takes [`m5far::W`](W) writer structure
impl crate::Writable for M5FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M5FAR to value 0
impl crate::Resettable for M5FARrs {
    const RESET_VALUE: u32 = 0;
}
