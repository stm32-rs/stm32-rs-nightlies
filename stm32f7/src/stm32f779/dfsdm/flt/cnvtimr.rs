///Register `CNVTIMR` reader
pub type R = crate::R<CNVTIMRrs>;
///Field `CNVCNT` reader - 28-bit timer counting conversion time
pub type CNVCNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 4:31 - 28-bit timer counting conversion time
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNVTIMR")
            .field("cnvcnt", &self.cnvcnt())
            .finish()
    }
}
/**DFSDM conversion timer register

You can [`read`](crate::Reg::read) this register and get [`cnvtimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CNVTIMRrs;
impl crate::RegisterSpec for CNVTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`cnvtimr::R`](R) reader structure
impl crate::Readable for CNVTIMRrs {}
///`reset()` method sets CNVTIMR to value 0
impl crate::Resettable for CNVTIMRrs {}
