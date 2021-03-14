#[doc = "Reader of register TTTMC"]
pub type R = crate::R<u32, super::TTTMC>;
#[doc = "Writer for register TTTMC"]
pub type W = crate::W<u32, super::TTTMC>;
#[doc = "Register TTTMC `reset()`'s with value 0"]
impl crate::ResetValue for super::TTTMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMSA`"]
pub type TMSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TMSA`"]
pub struct TMSA_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `TME`"]
pub type TME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TME`"]
pub struct TME_W<'a> {
    w: &'a mut W,
}
impl<'a> TME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&mut self) -> TMSA_W {
        TMSA_W { w: self }
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&mut self) -> TME_W {
        TME_W { w: self }
    }
}
