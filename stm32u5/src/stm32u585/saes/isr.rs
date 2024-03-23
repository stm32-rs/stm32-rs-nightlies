#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `CCF` reader - Computation complete flag"]
pub type CCF_R = crate::BitReader;
#[doc = "Field `RWEIF` reader - Read or write error interrupt flag"]
pub type RWEIF_R = crate::BitReader;
#[doc = "Field `KEIF` reader - Key error interrupt flag"]
pub type KEIF_R = crate::BitReader;
#[doc = "Field `RNGEIF` reader - RNGEIF"]
pub type RNGEIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Computation complete flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag"]
    #[inline(always)]
    pub fn rweif(&self) -> RWEIF_R {
        RWEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt flag"]
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGEIF"]
    #[inline(always)]
    pub fn rngeif(&self) -> RNGEIF_R {
        RNGEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
