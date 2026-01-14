///Register `FDRL` reader
pub type R = crate::R<FDRLrs>;
///Field `FDATAL` reader - Failing data low
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDRL")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
/**RAMECC monitor x failing data low register

You can [`read`](crate::Reg::read) this register and get [`fdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FDRLrs;
impl crate::RegisterSpec for FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`fdrl::R`](R) reader structure
impl crate::Readable for FDRLrs {}
///`reset()` method sets FDRL to value 0
impl crate::Resettable for FDRLrs {}
