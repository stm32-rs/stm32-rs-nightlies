///Register `REGION_TOP_HIGH2` reader
pub type R = crate::R<REGION_TOP_HIGH2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Top address high of region are not used with 32-bit address.

You can [`read`](crate::Reg::read) this register and get [`region_top_high2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_HIGH2)*/
pub struct REGION_TOP_HIGH2rs;
impl crate::RegisterSpec for REGION_TOP_HIGH2rs {
    type Ux = u32;
}
///`read()` method returns [`region_top_high2::R`](R) reader structure
impl crate::Readable for REGION_TOP_HIGH2rs {}
///`reset()` method sets REGION_TOP_HIGH2 to value 0
impl crate::Resettable for REGION_TOP_HIGH2rs {
    const RESET_VALUE: u32 = 0;
}