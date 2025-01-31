///Register `M1FDRL` reader
pub type R = crate::R<M1FDRLrs>;
///Field `FDATAL` reader - ECC failing data low
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M1FDRL")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
/**RAMECC monitor x failing data low register

You can [`read`](crate::Reg::read) this register and get [`m1fdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#RAMECC2:M1FDRL)*/
pub struct M1FDRLrs;
impl crate::RegisterSpec for M1FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`m1fdrl::R`](R) reader structure
impl crate::Readable for M1FDRLrs {}
///`reset()` method sets M1FDRL to value 0
impl crate::Resettable for M1FDRLrs {
    const RESET_VALUE: u32 = 0;
}
