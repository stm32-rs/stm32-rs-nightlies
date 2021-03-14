#[doc = "Reader of register PMC"]
pub type R = crate::R<u32, super::PMC>;
#[doc = "Writer for register PMC"]
pub type W = crate::W<u32, super::PMC>;
#[doc = "Register PMC `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USB_PU`"]
pub type USB_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_PU`"]
pub struct USB_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PU_W<'a> {
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
#[doc = "Reader of field `LCD_CAPA`"]
pub type LCD_CAPA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LCD_CAPA`"]
pub struct LCD_CAPA_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CAPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    pub fn usb_pu(&self) -> USB_PU_R {
        USB_PU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    pub fn lcd_capa(&self) -> LCD_CAPA_R {
        LCD_CAPA_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    pub fn usb_pu(&mut self) -> USB_PU_W {
        USB_PU_W { w: self }
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    pub fn lcd_capa(&mut self) -> LCD_CAPA_W {
        LCD_CAPA_W { w: self }
    }
}
