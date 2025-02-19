///Register `M2FAR` reader
pub type R = crate::R<M2FARrs>;
///Register `M2FAR` writer
pub type W = crate::W<M2FARrs>;
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
        f.debug_struct("M2FAR").field("fadd", &self.fadd()).finish()
    }
}
impl W {}
/**RAMECC monitor 2 failing address register

You can [`read`](crate::Reg::read) this register and get [`m2far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#RAMECC1:M2FAR)*/
pub struct M2FARrs;
impl crate::RegisterSpec for M2FARrs {
    type Ux = u32;
}
///`read()` method returns [`m2far::R`](R) reader structure
impl crate::Readable for M2FARrs {}
///`write(|w| ..)` method takes [`m2far::W`](W) writer structure
impl crate::Writable for M2FARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M2FAR to value 0
impl crate::Resettable for M2FARrs {
    const RESET_VALUE: u32 = 0;
}
