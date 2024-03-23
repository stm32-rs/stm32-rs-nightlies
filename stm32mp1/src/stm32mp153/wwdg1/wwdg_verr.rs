#[doc = "Register `WWDG_VERR` reader"]
pub type R = crate::R<WWDG_VERRrs>;
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
#[doc = "WWDG version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_VERRrs;
impl crate::RegisterSpec for WWDG_VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_verr::R`](R) reader structure"]
impl crate::Readable for WWDG_VERRrs {}
#[doc = "`reset()` method sets WWDG_VERR to value 0x21"]
impl crate::Resettable for WWDG_VERRrs {
    const RESET_VALUE: u32 = 0x21;
}
