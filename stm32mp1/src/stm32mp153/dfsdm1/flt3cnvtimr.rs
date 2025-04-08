///Register `FLT3CNVTIMR` reader
pub type R = crate::R<FLT3CNVTIMRrs>;
///Field `CNVCNT` reader - CNVCNT
pub type CNVCNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 4:31 - CNVCNT
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT3CNVTIMR")
            .field("cnvcnt", &self.cnvcnt())
            .finish()
    }
}
/**DFSDM filter 3 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt3cnvtimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT3CNVTIMR)*/
pub struct FLT3CNVTIMRrs;
impl crate::RegisterSpec for FLT3CNVTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`flt3cnvtimr::R`](R) reader structure
impl crate::Readable for FLT3CNVTIMRrs {}
///`reset()` method sets FLT3CNVTIMR to value 0
impl crate::Resettable for FLT3CNVTIMRrs {}
