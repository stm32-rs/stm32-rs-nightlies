#[doc = "Register `LTDC_IDR` reader"]
pub type R = crate::R<LTDC_IDRrs>;
#[doc = "Field `REV` reader - REV"]
pub type REV_R = crate::FieldReader;
#[doc = "Field `MINVER` reader - MINVER"]
pub type MINVER_R = crate::FieldReader;
#[doc = "Field `MAJVER` reader - MAJVER"]
pub type MAJVER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - REV"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MINVER"]
    #[inline(always)]
    pub fn minver(&self) -> MINVER_R {
        MINVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MAJVER"]
    #[inline(always)]
    pub fn majver(&self) -> MAJVER_R {
        MAJVER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "LTDC identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_IDRrs;
impl crate::RegisterSpec for LTDC_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_idr::R`](R) reader structure"]
impl crate::Readable for LTDC_IDRrs {}
#[doc = "`reset()` method sets LTDC_IDR to value 0x0001_0300"]
impl crate::Resettable for LTDC_IDRrs {
    const RESET_VALUE: u32 = 0x0001_0300;
}
