#[doc = "Register `LPTIM_VERR` reader"]
pub type R = crate::R<LPTIM_VERRrs>;
#[doc = "Field `MINREV` reader - MINREV"]
pub type MINREV_R = crate::FieldReader;
#[doc = "Field `MAJREV` reader - MAJREV"]
pub type MAJREV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - MINREV"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MAJREV"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "LPTIM peripheral version identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_VERRrs;
impl crate::RegisterSpec for LPTIM_VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_verr::R`](R) reader structure"]
impl crate::Readable for LPTIM_VERRrs {}
#[doc = "`reset()` method sets LPTIM_VERR to value 0x14"]
impl crate::Resettable for LPTIM_VERRrs {
    const RESET_VALUE: u32 = 0x14;
}
