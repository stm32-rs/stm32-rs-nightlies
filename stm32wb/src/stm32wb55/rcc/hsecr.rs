#[doc = "Reader of register HSECR"]
pub type R = crate::R<u32, super::HSECR>;
#[doc = "Writer for register HSECR"]
pub type W = crate::W<u32, super::HSECR>;
#[doc = "Register HSECR `reset()`'s with value 0x30"]
impl crate::ResetValue for super::HSECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "Reader of field `HSETUNE`"]
pub type HSETUNE_R = crate::R<u8, u8>;
#[doc = "Reader of field `HSEGMC`"]
pub type HSEGMC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSEGMC`"]
pub struct HSEGMC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEGMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `HSES`"]
pub type HSES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSES`"]
pub struct HSES_W<'a> {
    w: &'a mut W,
}
impl<'a> HSES_W<'a> {
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
#[doc = "Reader of field `UNLOCKED`"]
pub type UNLOCKED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNLOCKED`"]
pub struct UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCKED_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - HSE capacitor tuning"]
    #[inline(always)]
    pub fn hsetune(&self) -> HSETUNE_R {
        HSETUNE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:6 - HSE current control"]
    #[inline(always)]
    pub fn hsegmc(&self) -> HSEGMC_R {
        HSEGMC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - HSE Sense amplifier threshold"]
    #[inline(always)]
    pub fn hses(&self) -> HSES_R {
        HSES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Register lock system"]
    #[inline(always)]
    pub fn unlocked(&self) -> UNLOCKED_R {
        UNLOCKED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - HSE current control"]
    #[inline(always)]
    pub fn hsegmc(&mut self) -> HSEGMC_W {
        HSEGMC_W { w: self }
    }
    #[doc = "Bit 3 - HSE Sense amplifier threshold"]
    #[inline(always)]
    pub fn hses(&mut self) -> HSES_W {
        HSES_W { w: self }
    }
    #[doc = "Bit 0 - Register lock system"]
    #[inline(always)]
    pub fn unlocked(&mut self) -> UNLOCKED_W {
        UNLOCKED_W { w: self }
    }
}
