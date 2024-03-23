#[doc = "Register `LCCCR` reader"]
pub type R = crate::R<LCCCRrs>;
#[doc = "Field `COLC` reader - Color Coding"]
pub type COLC_R = crate::FieldReader;
#[doc = "Field `LPE` reader - Loosely Packed Enable"]
pub type LPE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Color Coding"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Loosely Packed Enable"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host LTDC Current Color Coding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCCCRrs;
impl crate::RegisterSpec for LCCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcccr::R`](R) reader structure"]
impl crate::Readable for LCCCRrs {}
#[doc = "`reset()` method sets LCCCR to value 0"]
impl crate::Resettable for LCCCRrs {
    const RESET_VALUE: u32 = 0;
}
