#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x61"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x61
    }
}
#[doc = "Reader of field `PLLRDY`"]
pub type PLLRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLON`"]
pub type PLLON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLON`"]
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
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
#[doc = "Reader of field `HSEBYPPWR`"]
pub type HSEBYPPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSEBYPPWR`"]
pub struct HSEBYPPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYPPWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `HSEPRE`"]
pub type HSEPRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSEPRE`"]
pub struct HSEPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEPRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CSSON`"]
pub type CSSON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSON`"]
pub struct CSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `HSERDY`"]
pub type HSERDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSEON`"]
pub type HSEON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSEON`"]
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `HSIKERDY`"]
pub type HSIKERDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSIASFS`"]
pub type HSIASFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIASFS`"]
pub struct HSIASFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIASFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `HSIRDY`"]
pub type HSIRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSIKERON`"]
pub type HSIKERON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIKERON`"]
pub struct HSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIKERON_W<'a> {
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
#[doc = "Reader of field `HSION`"]
pub type HSION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSION`"]
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
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
#[doc = "Reader of field `MSIRANGE`"]
pub type MSIRANGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSIRANGE`"]
pub struct MSIRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MSIRGSEL`"]
pub type MSIRGSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSIRGSEL`"]
pub struct MSIRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRGSEL_W<'a> {
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
#[doc = "Reader of field `MSIPLLEN`"]
pub type MSIPLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSIPLLEN`"]
pub struct MSIPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIPLLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MSIRDY`"]
pub type MSIRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSION`"]
pub type MSION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSION`"]
pub struct MSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MSION_W<'a> {
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
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    pub fn hsebyppwr(&self) -> HSEBYPPWR_R {
        HSEBYPPWR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSE32 clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HSI16 kernel clock ready flag for peripherals requests."]
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    pub fn hsebyppwr(&mut self) -> HSEBYPPWR_W {
        HSEBYPPWR_W { w: self }
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    pub fn hsepre(&mut self) -> HSEPRE_W {
        HSEPRE_W { w: self }
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W {
        CSSON_W { w: self }
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HSIASFS_W {
        HSIASFS_W { w: self }
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W {
        HSIKERON_W { w: self }
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W {
        MSIRANGE_W { w: self }
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W {
        MSIRGSEL_W { w: self }
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&mut self) -> MSIPLLEN_W {
        MSIPLLEN_W { w: self }
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W {
        MSION_W { w: self }
    }
}
