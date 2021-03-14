#[doc = "Reader of register DAC_STR1"]
pub type R = crate::R<u32, super::DAC_STR1>;
#[doc = "Writer for register DAC_STR1"]
pub type W = crate::W<u32, super::DAC_STR1>;
#[doc = "Register DAC_STR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_STR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STRSTDATA1`"]
pub type STRSTDATA1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STRSTDATA1`"]
pub struct STRSTDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSTDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `STDIR1`"]
pub type STDIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STDIR1`"]
pub struct STDIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> STDIR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `STINCDATA1`"]
pub type STINCDATA1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STINCDATA1`"]
pub struct STINCDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> STINCDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata1(&self) -> STRSTDATA1_R {
        STRSTDATA1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir1(&self) -> STDIR1_R {
        STDIR1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata1(&self) -> STINCDATA1_R {
        STINCDATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata1(&mut self) -> STRSTDATA1_W {
        STRSTDATA1_W { w: self }
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir1(&mut self) -> STDIR1_W {
        STDIR1_W { w: self }
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata1(&mut self) -> STINCDATA1_W {
        STINCDATA1_W { w: self }
    }
}
