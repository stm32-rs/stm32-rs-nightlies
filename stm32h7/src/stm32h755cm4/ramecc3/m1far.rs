///Register `M1FAR` reader
pub type R = crate::R<M1FARrs>;
///Register `M1FAR` writer
pub type W = crate::W<M1FARrs>;
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
        f.debug_struct("M1FAR").field("fadd", &self.fadd()).finish()
    }
}
impl W {}
/**RAMECC monitor 1 failing address register

You can [`read`](crate::Reg::read) this register and get [`m1far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC3:M1FAR)*/
pub struct M1FARrs;
impl crate::RegisterSpec for M1FARrs {
    type Ux = u32;
}
///`read()` method returns [`m1far::R`](R) reader structure
impl crate::Readable for M1FARrs {}
///`write(|w| ..)` method takes [`m1far::W`](W) writer structure
impl crate::Writable for M1FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M1FAR to value 0
impl crate::Resettable for M1FARrs {
    const RESET_VALUE: u32 = 0;
}
