#[doc = "Reader of register CNTL"]
pub type R = crate::R<u32, super::CNTL>;
#[doc = "Writer for register CNTL"]
pub type W = crate::W<u32, super::CNTL>;
#[doc = "Register CNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTL`"]
pub type CNTL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNTL`"]
pub struct CNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC counter register Low"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter register Low"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CNTL_W {
        CNTL_W { w: self }
    }
}
