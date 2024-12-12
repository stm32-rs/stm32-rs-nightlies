///Register `DFSDM2_CNVTIMR` reader
pub type R = crate::R<DFSDM2_CNVTIMRrs>;
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
        f.debug_struct("DFSDM2_CNVTIMR")
            .field("cnvcnt", &self.cnvcnt())
            .finish()
    }
}
/**DFSDM conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm2_cnvtimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#DFSDM1:DFSDM2_CNVTIMR)*/
pub struct DFSDM2_CNVTIMRrs;
impl crate::RegisterSpec for DFSDM2_CNVTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm2_cnvtimr::R`](R) reader structure
impl crate::Readable for DFSDM2_CNVTIMRrs {}
///`reset()` method sets DFSDM2_CNVTIMR to value 0
impl crate::Resettable for DFSDM2_CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
