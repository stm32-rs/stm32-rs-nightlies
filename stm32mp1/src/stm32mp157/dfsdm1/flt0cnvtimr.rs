///Register `FLT0CNVTIMR` reader
pub type R = crate::R<FLT0CNVTIMRrs>;
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
        f.debug_struct("FLT0CNVTIMR")
            .field("cnvcnt", &self.cnvcnt())
            .finish()
    }
}
/**DFSDM filter 0 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt0cnvtimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0CNVTIMR)*/
pub struct FLT0CNVTIMRrs;
impl crate::RegisterSpec for FLT0CNVTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`flt0cnvtimr::R`](R) reader structure
impl crate::Readable for FLT0CNVTIMRrs {}
///`reset()` method sets FLT0CNVTIMR to value 0
impl crate::Resettable for FLT0CNVTIMRrs {}
