///Register `M3FDRH` reader
pub type R = crate::R<M3FDRHrs>;
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
        f.debug_struct("M3FDRH")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
/**RAMECC monitor x failing data high register

You can [`read`](crate::Reg::read) this register and get [`m3fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#RAMECC1:M3FDRH)*/
pub struct M3FDRHrs;
impl crate::RegisterSpec for M3FDRHrs {
    type Ux = u32;
}
///`read()` method returns [`m3fdrh::R`](R) reader structure
impl crate::Readable for M3FDRHrs {}
///`reset()` method sets M3FDRH to value 0
impl crate::Resettable for M3FDRHrs {}
