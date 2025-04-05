///Register `REGION_BASE_HIGH3` reader
pub type R = crate::R<REGION_BASE_HIGH3rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Base address high are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_base_high3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_HIGH3)*/
pub struct REGION_BASE_HIGH3rs;
impl crate::RegisterSpec for REGION_BASE_HIGH3rs {
    type Ux = u32;
}
///`read()` method returns [`region_base_high3::R`](R) reader structure
impl crate::Readable for REGION_BASE_HIGH3rs {}
///`reset()` method sets REGION_BASE_HIGH3 to value 0
impl crate::Resettable for REGION_BASE_HIGH3rs {}
