#[doc = "Reader of register OAR"]
pub type R = crate::R<u32, super::OAR>;
#[doc = "Writer for register OAR"]
pub type W = crate::W<u32, super::OAR>;
#[doc = "Register OAR `reset()`'s with value 0"]
impl crate::ResetValue for super::OAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA`"]
pub type OA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA`"]
pub struct OA_W<'a> {
    w: &'a mut W,
}
impl<'a> OA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Own address"]
    #[inline(always)]
    pub fn oa(&self) -> OA_R {
        OA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Own address"]
    #[inline(always)]
    pub fn oa(&mut self) -> OA_W {
        OA_W { w: self }
    }
}
