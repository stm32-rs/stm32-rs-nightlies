///Register `TS0SDIFRDATAR` reader
pub type R = crate::R<TS0SDIFRDATARrs>;
///Field `SDIF_RDATA` reader - SDIF read data
pub type SDIF_RDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - SDIF read data
    #[inline(always)]
    pub fn sdif_rdata(&self) -> SDIF_RDATA_R {
        SDIF_RDATA_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS0SDIFRDATAR")
            .field("sdif_rdata", &self.sdif_rdata())
            .finish()
    }
}
/**DTS TS0 SDIF RDATA register

You can [`read`](crate::Reg::read) this register and get [`ts0sdifrdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS0SDIFRDATAR)*/
pub struct TS0SDIFRDATARrs;
impl crate::RegisterSpec for TS0SDIFRDATARrs {
    type Ux = u32;
}
///`read()` method returns [`ts0sdifrdatar::R`](R) reader structure
impl crate::Readable for TS0SDIFRDATARrs {}
///`reset()` method sets TS0SDIFRDATAR to value 0
impl crate::Resettable for TS0SDIFRDATARrs {}
