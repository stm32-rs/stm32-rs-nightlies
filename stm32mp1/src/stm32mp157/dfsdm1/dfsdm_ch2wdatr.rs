#[doc = "Register `DFSDM_CH2WDATR` reader"]
pub type R = crate::R<DFSDM_CH2WDATRrs>;
#[doc = "Field `WDATA` reader - WDATA"]
pub type WDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_ch2wdatr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_CH2WDATRrs;
impl crate::RegisterSpec for DFSDM_CH2WDATRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_ch2wdatr::R`](R) reader structure"]
impl crate::Readable for DFSDM_CH2WDATRrs {}
#[doc = "`reset()` method sets DFSDM_CH2WDATR to value 0"]
impl crate::Resettable for DFSDM_CH2WDATRrs {
    const RESET_VALUE: u32 = 0;
}
