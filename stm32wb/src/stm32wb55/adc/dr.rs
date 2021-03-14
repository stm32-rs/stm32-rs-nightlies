#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Writer for register DR"]
pub type W = crate::W<u32, super::DR>;
#[doc = "Register DR `reset()`'s with value 0"]
impl crate::ResetValue for super::DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDATA_0_6`"]
pub type RDATA_0_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDATA_0_6`"]
pub struct RDATA_0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATA_0_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `RDATA_7_15`"]
pub type RDATA_7_15_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Regular Data converted 0_6"]
    #[inline(always)]
    pub fn rdata_0_6(&self) -> RDATA_0_6_R {
        RDATA_0_6_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 7:15 - 15"]
    #[inline(always)]
    pub fn rdata_7_15(&self) -> RDATA_7_15_R {
        RDATA_7_15_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Regular Data converted 0_6"]
    #[inline(always)]
    pub fn rdata_0_6(&mut self) -> RDATA_0_6_W {
        RDATA_0_6_W { w: self }
    }
}
