#[doc = "Reader of register GICC_PMR"]
pub type R = crate::R<u32, super::GICC_PMR>;
#[doc = "Writer for register GICC_PMR"]
pub type W = crate::W<u32, super::GICC_PMR>;
#[doc = "Register GICC_PMR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICC_PMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIORITY`"]
pub type PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY`"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
}
