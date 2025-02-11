///Register `M2FDRH` reader
pub type R = crate::R<M2FDRHrs>;
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
        f.debug_struct("M2FDRH")
            .field("fadd", &self.fadd())
            .finish()
    }
}
/**RAMECC monitor x failing data high register

You can [`read`](crate::Reg::read) this register and get [`m2fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RAMECC:M2FDRH)*/
pub struct M2FDRHrs;
impl crate::RegisterSpec for M2FDRHrs {
    type Ux = u32;
}
///`read()` method returns [`m2fdrh::R`](R) reader structure
impl crate::Readable for M2FDRHrs {}
///`reset()` method sets M2FDRH to value 0
impl crate::Resettable for M2FDRHrs {
    const RESET_VALUE: u32 = 0;
}
