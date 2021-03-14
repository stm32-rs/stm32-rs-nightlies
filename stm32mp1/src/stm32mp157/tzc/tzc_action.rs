#[doc = "Reader of register TZC_ACTION"]
pub type R = crate::R<u32, super::TZC_ACTION>;
#[doc = "Writer for register TZC_ACTION"]
pub type W = crate::W<u32, super::TZC_ACTION>;
#[doc = "Register TZC_ACTION `reset()`'s with value 0"]
impl crate::ResetValue for super::TZC_ACTION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REACTION_VALUE`"]
pub type REACTION_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REACTION_VALUE`"]
pub struct REACTION_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> REACTION_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    pub fn reaction_value(&self) -> REACTION_VALUE_R {
        REACTION_VALUE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    pub fn reaction_value(&mut self) -> REACTION_VALUE_W {
        REACTION_VALUE_W { w: self }
    }
}
