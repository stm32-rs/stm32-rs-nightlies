///Register `DFSDM_FLT3CNVTIMR` reader
pub type R = crate::R<DFSDM_FLT3CNVTIMRrs>;
/**Field `CNVCNT` reader - 28-bit timer counting conversion time t = CNVCNT\[27:0\]
/ fDFSDM_CKIN*/
pub type CNVCNT_R = crate::FieldReader<u32>;
impl R {
    /**Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT\[27:0\]
    / fDFSDM_CKIN*/
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_FLT3CNVTIMR")
            .field("cnvcnt", &self.cnvcnt())
            .finish()
    }
}
/**conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cnvtimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3CNVTIMR)*/
pub struct DFSDM_FLT3CNVTIMRrs;
impl crate::RegisterSpec for DFSDM_FLT3CNVTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_flt3cnvtimr::R`](R) reader structure
impl crate::Readable for DFSDM_FLT3CNVTIMRrs {}
///`reset()` method sets DFSDM_FLT3CNVTIMR to value 0
impl crate::Resettable for DFSDM_FLT3CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
