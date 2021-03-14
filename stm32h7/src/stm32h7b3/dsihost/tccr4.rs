#[doc = "Reader of register TCCR4"]
pub type R = crate::R<u32, super::TCCR4>;
#[doc = "Writer for register TCCR4"]
pub type W = crate::W<u32, super::TCCR4>;
#[doc = "Register TCCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPWR_TOCNT`"]
pub type LPWR_TOCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LPWR_TOCNT`"]
pub struct LPWR_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWR_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low-power write timeout counter"]
    #[inline(always)]
    pub fn lpwr_tocnt(&self) -> LPWR_TOCNT_R {
        LPWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-power write timeout counter"]
    #[inline(always)]
    pub fn lpwr_tocnt(&mut self) -> LPWR_TOCNT_W {
        LPWR_TOCNT_W { w: self }
    }
}
