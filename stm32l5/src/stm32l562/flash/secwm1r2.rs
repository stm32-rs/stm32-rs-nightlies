#[doc = "Reader of register SECWM1R2"]
pub type R = crate::R<u32, super::SECWM1R2>;
#[doc = "Writer for register SECWM1R2"]
pub type W = crate::W<u32, super::SECWM1R2>;
#[doc = "Register SECWM1R2 `reset()`'s with value 0x0f00_0f00"]
impl crate::ResetValue for super::SECWM1R2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00_0f00
    }
}
#[doc = "Reader of field `PCROP1_PSTRT`"]
pub type PCROP1_PSTRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCROP1_PSTRT`"]
pub struct PCROP1_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `PCROP1EN`"]
pub type PCROP1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCROP1EN`"]
pub struct PCROP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `HDP1_PEND`"]
pub type HDP1_PEND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDP1_PEND`"]
pub struct HDP1_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP1_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HDP1EN`"]
pub type HDP1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDP1EN`"]
pub struct HDP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP1EN_W<'a> {
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
    #[doc = "Bits 0:6 - PCROP1_PSTRT"]
    #[inline(always)]
    pub fn pcrop1_pstrt(&self) -> PCROP1_PSTRT_R {
        PCROP1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PCROP1EN"]
    #[inline(always)]
    pub fn pcrop1en(&self) -> PCROP1EN_R {
        PCROP1EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - HDP1_PEND"]
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - HDP1EN"]
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PCROP1_PSTRT"]
    #[inline(always)]
    pub fn pcrop1_pstrt(&mut self) -> PCROP1_PSTRT_W {
        PCROP1_PSTRT_W { w: self }
    }
    #[doc = "Bit 15 - PCROP1EN"]
    #[inline(always)]
    pub fn pcrop1en(&mut self) -> PCROP1EN_W {
        PCROP1EN_W { w: self }
    }
    #[doc = "Bits 16:22 - HDP1_PEND"]
    #[inline(always)]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W {
        HDP1_PEND_W { w: self }
    }
    #[doc = "Bit 31 - HDP1EN"]
    #[inline(always)]
    pub fn hdp1en(&mut self) -> HDP1EN_W {
        HDP1EN_W { w: self }
    }
}
