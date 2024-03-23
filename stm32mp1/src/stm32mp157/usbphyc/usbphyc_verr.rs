#[doc = "Register `USBPHYC_VERR` reader"]
pub type R = crate::R<USBPHYC_VERRrs>;
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
#[doc = "This register defines the version of this IP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHYC_VERRrs;
impl crate::RegisterSpec for USBPHYC_VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphyc_verr::R`](R) reader structure"]
impl crate::Readable for USBPHYC_VERRrs {}
#[doc = "`reset()` method sets USBPHYC_VERR to value 0x10"]
impl crate::Resettable for USBPHYC_VERRrs {
    const RESET_VALUE: u32 = 0x10;
}
