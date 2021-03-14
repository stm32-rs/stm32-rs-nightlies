#[doc = "Reader of register MACTSECNR"]
pub type R = crate::R<u32, super::MACTSECNR>;
#[doc = "Writer for register MACTSECNR"]
pub type W = crate::W<u32, super::MACTSECNR>;
#[doc = "Register MACTSECNR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACTSECNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSEC`"]
pub type TSEC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSEC`"]
pub struct TSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timestamp Egress Correction"]
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Egress Correction"]
    #[inline(always)]
    pub fn tsec(&mut self) -> TSEC_W {
        TSEC_W { w: self }
    }
}
