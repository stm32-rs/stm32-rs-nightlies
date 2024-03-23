#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `SEIF` reader - Security Error Interrupt Flag status"]
pub type SEIF_R = crate::BitReader;
#[doc = "Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag status"]
pub type XONEIF_R = crate::BitReader;
#[doc = "Field `KEIF` reader - Key Error Interrupt Flag status"]
pub type KEIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Security Error Interrupt Flag status"]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Execute-only execute-Never Error Interrupt Flag status"]
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key Error Interrupt Flag status"]
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "OTFDEC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
