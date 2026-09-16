///Register `FAIL_ADDRESS_HIGH0` reader
pub type R = crate::R<FAIL_ADDRESS_HIGH0rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.

You can [`read`](crate::Reg::read) this register and get [`fail_address_high0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:FAIL_ADDRESS_HIGH0)*/
pub struct FAIL_ADDRESS_HIGH0rs;
impl crate::RegisterSpec for FAIL_ADDRESS_HIGH0rs {
    type Ux = u32;
}
///`read()` method returns [`fail_address_high0::R`](R) reader structure
impl crate::Readable for FAIL_ADDRESS_HIGH0rs {}
///`reset()` method sets FAIL_ADDRESS_HIGH0 to value 0
impl crate::Resettable for FAIL_ADDRESS_HIGH0rs {}
