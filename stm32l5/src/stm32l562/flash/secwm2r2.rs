#[doc = "Reader of register SECWM2R2"]
pub type R = crate::R<u32, super::SECWM2R2>;
#[doc = "Writer for register SECWM2R2"]
pub type W = crate::W<u32, super::SECWM2R2>;
#[doc = "Register SECWM2R2 `reset()`'s with value 0x0f00_0f00"]
impl crate::ResetValue for super::SECWM2R2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00_0f00
    }
}
#[doc = "Reader of field `PCROP2_PSTRT`"]
pub type PCROP2_PSTRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCROP2_PSTRT`"]
pub struct PCROP2_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `PCROP2EN`"]
pub type PCROP2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCROP2EN`"]
pub struct PCROP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2EN_W<'a> {
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
#[doc = "Reader of field `HDP2_PEND`"]
pub type HDP2_PEND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDP2_PEND`"]
pub struct HDP2_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP2_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HDP2EN`"]
pub type HDP2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDP2EN`"]
pub struct HDP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP2EN_W<'a> {
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
    #[doc = "Bits 0:6 - PCROP2_PSTRT"]
    #[inline(always)]
    pub fn pcrop2_pstrt(&self) -> PCROP2_PSTRT_R {
        PCROP2_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PCROP2EN"]
    #[inline(always)]
    pub fn pcrop2en(&self) -> PCROP2EN_R {
        PCROP2EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - HDP2_PEND"]
    #[inline(always)]
    pub fn hdp2_pend(&self) -> HDP2_PEND_R {
        HDP2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - HDP2EN"]
    #[inline(always)]
    pub fn hdp2en(&self) -> HDP2EN_R {
        HDP2EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PCROP2_PSTRT"]
    #[inline(always)]
    pub fn pcrop2_pstrt(&mut self) -> PCROP2_PSTRT_W {
        PCROP2_PSTRT_W { w: self }
    }
    #[doc = "Bit 15 - PCROP2EN"]
    #[inline(always)]
    pub fn pcrop2en(&mut self) -> PCROP2EN_W {
        PCROP2EN_W { w: self }
    }
    #[doc = "Bits 16:22 - HDP2_PEND"]
    #[inline(always)]
    pub fn hdp2_pend(&mut self) -> HDP2_PEND_W {
        HDP2_PEND_W { w: self }
    }
    #[doc = "Bit 31 - HDP2EN"]
    #[inline(always)]
    pub fn hdp2en(&mut self) -> HDP2EN_W {
        HDP2EN_W { w: self }
    }
}
