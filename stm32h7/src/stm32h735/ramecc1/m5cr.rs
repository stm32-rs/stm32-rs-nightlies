///Register `M5CR` reader
pub type R = crate::R<M5CRrs>;
///Field `FEC` reader - Failing error code
pub type FEC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing error code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5CR").field("fec", &self.fec()).finish()
    }
}
/**RAMECC monitor x configuration register

You can [`read`](crate::Reg::read) this register and get [`m5cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#RAMECC1:M5CR)*/
pub struct M5CRrs;
impl crate::RegisterSpec for M5CRrs {
    type Ux = u32;
}
///`read()` method returns [`m5cr::R`](R) reader structure
impl crate::Readable for M5CRrs {}
///`reset()` method sets M5CR to value 0
impl crate::Resettable for M5CRrs {}
