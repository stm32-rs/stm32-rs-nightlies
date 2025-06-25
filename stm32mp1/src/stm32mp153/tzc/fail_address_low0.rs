///Register `FAIL_ADDRESS_LOW0` reader
pub type R = crate::R<FAIL_ADDRESS_LOW0rs>;
///Field `ADDR_STATUS_LOW` reader - ADDR_STATUS_LOW
pub type ADDR_STATUS_LOW_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ADDR_STATUS_LOW
    #[inline(always)]
    pub fn addr_status_low(&self) -> ADDR_STATUS_LOW_R {
        ADDR_STATUS_LOW_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAIL_ADDRESS_LOW0")
            .field("addr_status_low", &self.addr_status_low())
            .finish()
    }
}
/**Address low bits of the first failed access in the associated filter (0 to 1).

You can [`read`](crate::Reg::read) this register and get [`fail_address_low0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:FAIL_ADDRESS_LOW0)*/
pub struct FAIL_ADDRESS_LOW0rs;
impl crate::RegisterSpec for FAIL_ADDRESS_LOW0rs {
    type Ux = u32;
}
///`read()` method returns [`fail_address_low0::R`](R) reader structure
impl crate::Readable for FAIL_ADDRESS_LOW0rs {}
///`reset()` method sets FAIL_ADDRESS_LOW0 to value 0
impl crate::Resettable for FAIL_ADDRESS_LOW0rs {}
