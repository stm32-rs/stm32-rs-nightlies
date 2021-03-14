#[doc = "Reader of register DDRCTRL_DERATEEN"]
pub type R = crate::R<u32, super::DDRCTRL_DERATEEN>;
#[doc = "Writer for register DDRCTRL_DERATEEN"]
pub type W = crate::W<u32, super::DDRCTRL_DERATEEN>;
#[doc = "Register DDRCTRL_DERATEEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_DERATEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DERATE_ENABLE`"]
pub type DERATE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DERATE_ENABLE`"]
pub struct DERATE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERATE_ENABLE_W<'a> {
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
#[doc = "Reader of field `DERATE_VALUE`"]
pub type DERATE_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DERATE_VALUE`"]
pub struct DERATE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERATE_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `DERATE_BYTE`"]
pub type DERATE_BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DERATE_BYTE`"]
pub struct DERATE_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERATE_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DERATE_ENABLE"]
    #[inline(always)]
    pub fn derate_enable(&self) -> DERATE_ENABLE_R {
        DERATE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - DERATE_VALUE"]
    #[inline(always)]
    pub fn derate_value(&self) -> DERATE_VALUE_R {
        DERATE_VALUE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - DERATE_BYTE"]
    #[inline(always)]
    pub fn derate_byte(&self) -> DERATE_BYTE_R {
        DERATE_BYTE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DERATE_ENABLE"]
    #[inline(always)]
    pub fn derate_enable(&mut self) -> DERATE_ENABLE_W {
        DERATE_ENABLE_W { w: self }
    }
    #[doc = "Bits 1:2 - DERATE_VALUE"]
    #[inline(always)]
    pub fn derate_value(&mut self) -> DERATE_VALUE_W {
        DERATE_VALUE_W { w: self }
    }
    #[doc = "Bits 4:7 - DERATE_BYTE"]
    #[inline(always)]
    pub fn derate_byte(&mut self) -> DERATE_BYTE_W {
        DERATE_BYTE_W { w: self }
    }
}
