///Register `M3CR` reader
pub type R = crate::R<M3CRrs>;
///Field `FADD` reader - ECC error failing address
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC error failing address
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3CR").field("fadd", &self.fadd()).finish()
    }
}
/**RAMECC monitor x configuration register

You can [`read`](crate::Reg::read) this register and get [`m3cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#RAMECC1:M3CR)*/
pub struct M3CRrs;
impl crate::RegisterSpec for M3CRrs {
    type Ux = u32;
}
///`read()` method returns [`m3cr::R`](R) reader structure
impl crate::Readable for M3CRrs {}
///`reset()` method sets M3CR to value 0
impl crate::Resettable for M3CRrs {}
