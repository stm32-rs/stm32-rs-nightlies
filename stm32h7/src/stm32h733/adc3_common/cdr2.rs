///Register `CDR2` reader
pub type R = crate::R<CDR2rs>;
///Field `RDATA_ALT` reader - Regular data of the master/slave alternated ADCs
pub type RDATA_ALT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Regular data of the master/slave alternated ADCs
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR2")
            .field("rdata_alt", &self.rdata_alt())
            .finish()
    }
}
/**ADC x common regular data register for 32-bit dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#ADC3_Common:CDR2)*/
pub struct CDR2rs;
impl crate::RegisterSpec for CDR2rs {
    type Ux = u32;
}
///`read()` method returns [`cdr2::R`](R) reader structure
impl crate::Readable for CDR2rs {}
///`reset()` method sets CDR2 to value 0
impl crate::Resettable for CDR2rs {}
