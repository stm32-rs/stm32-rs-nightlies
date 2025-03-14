///Register `M4FAR` reader
pub type R = crate::R<M4FARrs>;
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
        f.debug_struct("M4FAR").field("fadd", &self.fadd()).finish()
    }
}
/**RAMECC monitor x failing address register

You can [`read`](crate::Reg::read) this register and get [`m4far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#RAMECC2:M4FAR)*/
pub struct M4FARrs;
impl crate::RegisterSpec for M4FARrs {
    type Ux = u32;
}
///`read()` method returns [`m4far::R`](R) reader structure
impl crate::Readable for M4FARrs {}
///`reset()` method sets M4FAR to value 0
impl crate::Resettable for M4FARrs {}
