///Register `M5FDRL` reader
pub type R = crate::R<M5FDRLrs>;
///Field `FDATAL` reader - Failing data low
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5FDRL")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
/**RAMECC monitor x failing data low register

You can [`read`](crate::Reg::read) this register and get [`m5fdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#RAMECC2:M5FDRL)*/
pub struct M5FDRLrs;
impl crate::RegisterSpec for M5FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`m5fdrl::R`](R) reader structure
impl crate::Readable for M5FDRLrs {}
///`reset()` method sets M5FDRL to value 0
impl crate::Resettable for M5FDRLrs {}
