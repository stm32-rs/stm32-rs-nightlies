#[doc = "Reader of register MAPR2"]
pub type R = crate::R<u32, super::MAPR2>;
#[doc = "Writer for register MAPR2"]
pub type W = crate::W<u32, super::MAPR2>;
#[doc = "Register MAPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM9_REMAP`"]
pub type TIM9_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM9_REMAP`"]
pub struct TIM9_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9_REMAP_W<'a> {
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
#[doc = "Reader of field `TIM10_REMAP`"]
pub type TIM10_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM10_REMAP`"]
pub struct TIM10_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM10_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TIM11_REMAP`"]
pub type TIM11_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM11_REMAP`"]
pub struct TIM11_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM11_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TIM13_REMAP`"]
pub type TIM13_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM13_REMAP`"]
pub struct TIM13_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIM14_REMAP`"]
pub type TIM14_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM14_REMAP`"]
pub struct TIM14_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `FSMC_NADV`"]
pub type FSMC_NADV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSMC_NADV`"]
pub struct FSMC_NADV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_NADV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&self) -> TIM9_REMAP_R {
        TIM9_REMAP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&self) -> TIM10_REMAP_R {
        TIM10_REMAP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    pub fn tim11_remap(&self) -> TIM11_REMAP_R {
        TIM11_REMAP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&mut self) -> TIM9_REMAP_W {
        TIM9_REMAP_W { w: self }
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&mut self) -> TIM10_REMAP_W {
        TIM10_REMAP_W { w: self }
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    pub fn tim11_remap(&mut self) -> TIM11_REMAP_W {
        TIM11_REMAP_W { w: self }
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W {
        TIM13_REMAP_W { w: self }
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W {
        TIM14_REMAP_W { w: self }
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W {
        FSMC_NADV_W { w: self }
    }
}
