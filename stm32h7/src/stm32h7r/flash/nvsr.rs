///Register `NVSR` reader
pub type R = crate::R<NVSRrs>;
///Field `NVSTATE` reader - Non-volatile state others: invalid configuration.
pub type NVSTATE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Non-volatile state others: invalid configuration.
    #[inline(always)]
    pub fn nvstate(&self) -> NVSTATE_R {
        NVSTATE_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVSR")
            .field("nvstate", &self.nvstate())
            .finish()
    }
}
/**FLASH non-volatile status register

You can [`read`](crate::Reg::read) this register and get [`nvsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:NVSR)*/
pub struct NVSRrs;
impl crate::RegisterSpec for NVSRrs {
    type Ux = u32;
}
///`read()` method returns [`nvsr::R`](R) reader structure
impl crate::Readable for NVSRrs {}
///`reset()` method sets NVSR to value 0
impl crate::Resettable for NVSRrs {}
