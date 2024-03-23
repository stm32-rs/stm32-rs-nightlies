#[doc = "Register `LTDC_LCR` reader"]
pub type R = crate::R<LTDC_LCRrs>;
#[doc = "Field `LNBR` reader - LNBR"]
pub type LNBR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - LNBR"]
    #[inline(always)]
    pub fn lnbr(&self) -> LNBR_R {
        LNBR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "LDTC layer count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_lcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_LCRrs;
impl crate::RegisterSpec for LTDC_LCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_lcr::R`](R) reader structure"]
impl crate::Readable for LTDC_LCRrs {}
#[doc = "`reset()` method sets LTDC_LCR to value 0x02"]
impl crate::Resettable for LTDC_LCRrs {
    const RESET_VALUE: u32 = 0x02;
}
