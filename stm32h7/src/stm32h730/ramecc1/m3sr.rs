///Register `M3SR` reader
pub type R = crate::R<M3SRrs>;
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
        f.debug_struct("M3SR").field("fadd", &self.fadd()).finish()
    }
}
/**RAMECC monitor x status register

You can [`read`](crate::Reg::read) this register and get [`m3sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#RAMECC1:M3SR)*/
pub struct M3SRrs;
impl crate::RegisterSpec for M3SRrs {
    type Ux = u32;
}
///`read()` method returns [`m3sr::R`](R) reader structure
impl crate::Readable for M3SRrs {}
///`reset()` method sets M3SR to value 0
impl crate::Resettable for M3SRrs {}
