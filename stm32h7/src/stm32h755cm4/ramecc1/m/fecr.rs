///Register `FECR` reader
pub type R = crate::R<FECRrs>;
///Field `FEC` reader - Failing error code Failing error code
pub type FEC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing error code Failing error code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FECR").field("fec", &self.fec()).finish()
    }
}
/**RAMECC monitor x failing ECC error code register

You can [`read`](crate::Reg::read) this register and get [`fecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FECRrs;
impl crate::RegisterSpec for FECRrs {
    type Ux = u32;
}
///`read()` method returns [`fecr::R`](R) reader structure
impl crate::Readable for FECRrs {}
///`reset()` method sets FECR to value 0
impl crate::Resettable for FECRrs {}
