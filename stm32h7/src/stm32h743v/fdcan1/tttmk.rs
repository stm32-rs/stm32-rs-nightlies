#[doc = "Reader of register TTTMK"]
pub type R = crate::R<u32, super::TTTMK>;
#[doc = "Writer for register TTTMK"]
pub type W = crate::W<u32, super::TTTMK>;
#[doc = "Register TTTMK `reset()`'s with value 0"]
impl crate::ResetValue for super::TTTMK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TM`"]
pub type TM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TM`"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TICC`"]
pub type TICC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TICC`"]
pub struct TICC_W<'a> {
    w: &'a mut W,
}
impl<'a> TICC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LCKM`"]
pub type LCKM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCKM`"]
pub struct LCKM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&mut self) -> TICC_W {
        TICC_W { w: self }
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&mut self) -> LCKM_W {
        LCKM_W { w: self }
    }
}
