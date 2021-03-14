#[doc = "Reader of register RSSCMDR"]
pub type R = crate::R<u32, super::RSSCMDR>;
#[doc = "Writer for register RSSCMDR"]
pub type W = crate::W<u32, super::RSSCMDR>;
#[doc = "Register RSSCMDR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSSCMDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSSCMD`"]
pub type RSSCMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSSCMD`"]
pub struct RSSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RSS commands"]
    #[inline(always)]
    pub fn rsscmd(&self) -> RSSCMD_R {
        RSSCMD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RSS commands"]
    #[inline(always)]
    pub fn rsscmd(&mut self) -> RSSCMD_W {
        RSSCMD_W { w: self }
    }
}
