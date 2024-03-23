#[doc = "Register `RCC_VERR` reader"]
pub type R = crate::R<RCC_VERRrs>;
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
#[doc = "This register gives the IP version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_VERRrs;
impl crate::RegisterSpec for RCC_VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_verr::R`](R) reader structure"]
impl crate::Readable for RCC_VERRrs {}
#[doc = "`reset()` method sets RCC_VERR to value 0x11"]
impl crate::Resettable for RCC_VERRrs {
    const RESET_VALUE: u32 = 0x11;
}
