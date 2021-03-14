#[doc = "Reader of register DAC_STR2"]
pub type R = crate::R<u32, super::DAC_STR2>;
#[doc = "Writer for register DAC_STR2"]
pub type W = crate::W<u32, super::DAC_STR2>;
#[doc = "Register DAC_STR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_STR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STRSTDATA2`"]
pub type STRSTDATA2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STRSTDATA2`"]
pub struct STRSTDATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSTDATA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `STDIR2`"]
pub type STDIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STDIR2`"]
pub struct STDIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> STDIR2_W<'a> {
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
#[doc = "Reader of field `STINCDATA2`"]
pub type STINCDATA2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STINCDATA2`"]
pub struct STINCDATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> STINCDATA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata2(&self) -> STRSTDATA2_R {
        STRSTDATA2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir2(&self) -> STDIR2_R {
        STDIR2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata2(&self) -> STINCDATA2_R {
        STINCDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata2(&mut self) -> STRSTDATA2_W {
        STRSTDATA2_W { w: self }
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir2(&mut self) -> STDIR2_W {
        STDIR2_W { w: self }
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata2(&mut self) -> STINCDATA2_W {
        STINCDATA2_W { w: self }
    }
}
