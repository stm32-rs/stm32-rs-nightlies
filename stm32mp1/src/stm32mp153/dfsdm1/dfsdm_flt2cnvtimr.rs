///Register `DFSDM_FLT2CNVTIMR` reader
pub type R = crate::R<DFSDM_FLT2CNVTIMRrs>;
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
        f.debug_struct("DFSDM_FLT2CNVTIMR")
            .field("cnvcnt", &self.cnvcnt())
            .finish()
    }
}
/**DFSDM filter 2 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cnvtimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2CNVTIMR)*/
pub struct DFSDM_FLT2CNVTIMRrs;
impl crate::RegisterSpec for DFSDM_FLT2CNVTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_flt2cnvtimr::R`](R) reader structure
impl crate::Readable for DFSDM_FLT2CNVTIMRrs {}
///`reset()` method sets DFSDM_FLT2CNVTIMR to value 0
impl crate::Resettable for DFSDM_FLT2CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
