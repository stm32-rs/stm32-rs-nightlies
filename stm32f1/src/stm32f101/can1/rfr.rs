#[doc = "Reader of register RF%sR"]
pub type R = crate::R<u32, super::RFR>;
#[doc = "Writer for register RF%sR"]
pub type W = crate::W<u32, super::RFR>;
#[doc = "Register RF%sR `reset()`'s with value 0"]
impl crate::ResetValue for super::RFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFOM`"]
pub type RFOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFOM`"]
pub struct RFOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FOVR`"]
pub type FOVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOVR`"]
pub struct FOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULL`"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FMP`"]
pub type FMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom(&self) -> RFOM_R {
        RFOM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr(&self) -> FOVR_R {
        FOVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom(&mut self) -> RFOM_W {
        RFOM_W { w: self }
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr(&mut self) -> FOVR_W {
        FOVR_W { w: self }
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
}
