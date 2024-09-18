///Register `TZC_REGION_BASE_HIGH5` reader
pub type R = crate::R<TZC_REGION_BASE_HIGH5rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`tzc_region_base_high5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:TZC_REGION_BASE_HIGH5)*/
pub struct TZC_REGION_BASE_HIGH5rs;
impl crate::RegisterSpec for TZC_REGION_BASE_HIGH5rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_region_base_high5::R`](R) reader structure
impl crate::Readable for TZC_REGION_BASE_HIGH5rs {}
///`reset()` method sets TZC_REGION_BASE_HIGH5 to value 0
impl crate::Resettable for TZC_REGION_BASE_HIGH5rs {
    const RESET_VALUE: u32 = 0;
}
