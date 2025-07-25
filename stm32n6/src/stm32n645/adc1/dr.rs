///Register `DR` reader
pub type R = crate::R<DRrs>;
///Field `RDATA` reader - Regular data converted
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Regular data converted
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR").field("rdata", &self.rdata()).finish()
    }
}
/**ADC regular data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ADC1:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
