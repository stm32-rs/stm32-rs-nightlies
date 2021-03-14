#[doc = "Reader of register TXEFS"]
pub type R = crate::R<u32, super::TXEFS>;
#[doc = "Writer for register TXEFS"]
pub type W = crate::W<u32, super::TXEFS>;
#[doc = "Register TXEFS `reset()`'s with value 0"]
impl crate::ResetValue for super::TXEFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFFL`"]
pub type EFFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFFL`"]
pub struct EFFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `EFGI`"]
pub type EFGI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFGI`"]
pub struct EFGI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFGI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EFF`"]
pub type EFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFF`"]
pub struct EFF_W<'a> {
    w: &'a mut W,
}
impl<'a> EFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TEFL`"]
pub type TEFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFL`"]
pub struct TEFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `EFPI`"]
pub type EFPI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFPI`"]
pub struct EFPI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Event FIFO put index"]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&mut self) -> EFFL_W {
        EFFL_W { w: self }
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    pub fn efgi(&mut self) -> EFGI_W {
        EFGI_W { w: self }
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    pub fn eff(&mut self) -> EFF_W {
        EFF_W { w: self }
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W {
        TEFL_W { w: self }
    }
    #[doc = "Bits 16:20 - Event FIFO put index"]
    #[inline(always)]
    pub fn efpi(&mut self) -> EFPI_W {
        EFPI_W { w: self }
    }
}
