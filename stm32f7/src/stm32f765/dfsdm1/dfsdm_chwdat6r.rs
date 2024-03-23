#[doc = "Register `DFSDM_CHWDAT6R` reader"]
pub type R = crate::R<DFSDM_CHWDAT6Rrs>;
#[doc = "Field `WDATA` reader - Input channel y watchdog data"]
pub type WDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input channel y watchdog data"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat6r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_CHWDAT6Rrs;
impl crate::RegisterSpec for DFSDM_CHWDAT6Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_chwdat6r::R`](R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT6Rrs {}
#[doc = "`reset()` method sets DFSDM_CHWDAT6R to value 0"]
impl crate::Resettable for DFSDM_CHWDAT6Rrs {
    const RESET_VALUE: u32 = 0;
}
