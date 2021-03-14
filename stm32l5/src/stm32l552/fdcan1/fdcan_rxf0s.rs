#[doc = "Reader of register FDCAN_RXF0S"]
pub type R = crate::R<u32, super::FDCAN_RXF0S>;
#[doc = "Writer for register FDCAN_RXF0S"]
pub type W = crate::W<u32, super::FDCAN_RXF0S>;
#[doc = "Register FDCAN_RXF0S `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_RXF0S {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F0FL`"]
pub type F0FL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0FL`"]
pub struct F0FL_W<'a> {
    w: &'a mut W,
}
impl<'a> F0FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `F0GI`"]
pub type F0GI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0GI`"]
pub struct F0GI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0GI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `F0PI`"]
pub type F0PI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0PI`"]
pub struct F0PI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0PI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `F0F`"]
pub type F0F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `F0F`"]
pub struct F0F_W<'a> {
    w: &'a mut W,
}
impl<'a> F0F_W<'a> {
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
#[doc = "Reader of field `RF0L`"]
pub type RF0L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0L`"]
pub struct RF0L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&mut self) -> F0FL_W {
        F0FL_W { w: self }
    }
    #[doc = "Bits 8:9 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&mut self) -> F0GI_W {
        F0GI_W { w: self }
    }
    #[doc = "Bits 16:17 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&mut self) -> F0PI_W {
        F0PI_W { w: self }
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&mut self) -> F0F_W {
        F0F_W { w: self }
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W {
        RF0L_W { w: self }
    }
}
