#[doc = "Reader of register CMP1CR"]
pub type R = crate::R<u32, super::CMP1CR>;
#[doc = "Writer for register CMP1CR"]
pub type W = crate::W<u32, super::CMP1CR>;
#[doc = "Register CMP1CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP1CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP1x`"]
pub type CMP1X_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMP1x`"]
pub struct CMP1X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&mut self) -> CMP1X_W {
        CMP1X_W { w: self }
    }
}
