#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Field `SEIF` reader - SEIF"]
pub type SEIF_R = crate::BitReader;
#[doc = "Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag clear"]
pub type XONEIF_R = crate::BitReader;
#[doc = "Field `KEIF` reader - KEIF"]
pub type KEIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SEIF"]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Execute-only execute-Never Error Interrupt Flag clear"]
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - KEIF"]
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "OTFDEC interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
