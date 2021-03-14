#[doc = "Reader of register SECWM1R1"]
pub type R = crate::R<u32, super::SECWM1R1>;
#[doc = "Writer for register SECWM1R1"]
pub type W = crate::W<u32, super::SECWM1R1>;
#[doc = "Register SECWM1R1 `reset()`'s with value 0xff00_ff00"]
impl crate::ResetValue for super::SECWM1R1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00_ff00
    }
}
#[doc = "Reader of field `SECWM1_PSTRT`"]
pub type SECWM1_PSTRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECWM1_PSTRT`"]
pub struct SECWM1_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECWM1_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SECWM1_PEND`"]
pub type SECWM1_PEND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECWM1_PEND`"]
pub struct SECWM1_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SECWM1_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    pub fn secwm1_pstrt(&self) -> SECWM1_PSTRT_R {
        SECWM1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    pub fn secwm1_pend(&self) -> SECWM1_PEND_R {
        SECWM1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    pub fn secwm1_pstrt(&mut self) -> SECWM1_PSTRT_W {
        SECWM1_PSTRT_W { w: self }
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    pub fn secwm1_pend(&mut self) -> SECWM1_PEND_W {
        SECWM1_PEND_W { w: self }
    }
}
