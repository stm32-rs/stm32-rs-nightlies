///Register `FAIL_ADDRESS_HIGH1` reader
pub type R = crate::R<FAIL_ADDRESS_HIGH1rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.

You can [`read`](crate::Reg::read) this register and get [`fail_address_high1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:FAIL_ADDRESS_HIGH1)*/
pub struct FAIL_ADDRESS_HIGH1rs;
impl crate::RegisterSpec for FAIL_ADDRESS_HIGH1rs {
    type Ux = u32;
}
///`read()` method returns [`fail_address_high1::R`](R) reader structure
impl crate::Readable for FAIL_ADDRESS_HIGH1rs {}
///`reset()` method sets FAIL_ADDRESS_HIGH1 to value 0
impl crate::Resettable for FAIL_ADDRESS_HIGH1rs {
    const RESET_VALUE: u32 = 0;
}