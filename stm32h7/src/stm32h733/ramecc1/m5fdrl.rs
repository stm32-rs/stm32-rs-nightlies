///Register `M5FDRL` reader
pub type R = crate::R<M5FDRLrs>;
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
        f.debug_struct("M5FDRL").field("fec", &self.fec()).finish()
    }
}
/**RAMECC monitor x failing data low register

You can [`read`](crate::Reg::read) this register and get [`m5fdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#RAMECC1:M5FDRL)*/
pub struct M5FDRLrs;
impl crate::RegisterSpec for M5FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`m5fdrl::R`](R) reader structure
impl crate::Readable for M5FDRLrs {}
///`reset()` method sets M5FDRL to value 0
impl crate::Resettable for M5FDRLrs {
    const RESET_VALUE: u32 = 0;
}
