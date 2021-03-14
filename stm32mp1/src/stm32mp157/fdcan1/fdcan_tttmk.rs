#[doc = "Reader of register FDCAN_TTTMK"]
pub type R = crate::R<u32, super::FDCAN_TTTMK>;
#[doc = "Writer for register FDCAN_TTTMK"]
pub type W = crate::W<u32, super::FDCAN_TTTMK>;
#[doc = "Register FDCAN_TTTMK `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TTTMK {
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
impl R {
    #[doc = "Bits 0:15 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - TICC"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - LCKM"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - TM"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bits 16:22 - TICC"]
    #[inline(always)]
    pub fn ticc(&mut self) -> TICC_W {
        TICC_W { w: self }
    }
}
