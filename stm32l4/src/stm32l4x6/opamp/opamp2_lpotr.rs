#[doc = "Reader of register OPAMP2_LPOTR"]
pub type R = crate::R<u32, super::OPAMP2_LPOTR>;
#[doc = "Writer for register OPAMP2_LPOTR"]
pub type W = crate::W<u32, super::OPAMP2_LPOTR>;
#[doc = "Register OPAMP2_LPOTR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPAMP2_LPOTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIMLPOFFSETN`"]
pub type TRIMLPOFFSETN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMLPOFFSETN`"]
pub struct TRIMLPOFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMLPOFFSETN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `TRIMLPOFFSETP`"]
pub type TRIMLPOFFSETP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMLPOFFSETP`"]
pub struct TRIMLPOFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMLPOFFSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TRIMLPOFFSETN_R {
        TRIMLPOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TRIMLPOFFSETP_R {
        TRIMLPOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&mut self) -> TRIMLPOFFSETN_W {
        TRIMLPOFFSETN_W { w: self }
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&mut self) -> TRIMLPOFFSETP_W {
        TRIMLPOFFSETP_W { w: self }
    }
}
