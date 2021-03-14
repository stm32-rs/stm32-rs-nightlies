#[doc = "Reader of register SCAR_PRG"]
pub type R = crate::R<u32, super::SCAR_PRG>;
#[doc = "Writer for register SCAR_PRG"]
pub type W = crate::W<u32, super::SCAR_PRG>;
#[doc = "Register SCAR_PRG `reset()`'s with value 0"]
impl crate::ResetValue for super::SCAR_PRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMES`"]
pub type DMES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMES`"]
pub struct DMES_W<'a> {
    w: &'a mut W,
}
impl<'a> DMES_W<'a> {
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
#[doc = "Reader of field `SEC_AREA_END`"]
pub type SEC_AREA_END_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SEC_AREA_END`"]
pub struct SEC_AREA_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_AREA_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SEC_AREA_START`"]
pub type SEC_AREA_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SEC_AREA_START`"]
pub struct SEC_AREA_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_AREA_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Bank 1 secure access protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmes(&self) -> DMES_R {
        DMES_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Bank 1 secure-only area end configuration bits"]
    #[inline(always)]
    pub fn sec_area_end(&self) -> SEC_AREA_END_R {
        SEC_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Bank 1 secure-only area start configuration bits"]
    #[inline(always)]
    pub fn sec_area_start(&self) -> SEC_AREA_START_R {
        SEC_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Bank 1 secure access protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmes(&mut self) -> DMES_W {
        DMES_W { w: self }
    }
    #[doc = "Bits 16:27 - Bank 1 secure-only area end configuration bits"]
    #[inline(always)]
    pub fn sec_area_end(&mut self) -> SEC_AREA_END_W {
        SEC_AREA_END_W { w: self }
    }
    #[doc = "Bits 0:11 - Bank 1 secure-only area start configuration bits"]
    #[inline(always)]
    pub fn sec_area_start(&mut self) -> SEC_AREA_START_W {
        SEC_AREA_START_W { w: self }
    }
}
