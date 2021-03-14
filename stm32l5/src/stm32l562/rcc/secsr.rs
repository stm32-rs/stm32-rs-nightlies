#[doc = "Reader of register SECSR"]
pub type R = crate::R<u32, super::SECSR>;
#[doc = "Writer for register SECSR"]
pub type W = crate::W<u32, super::SECSR>;
#[doc = "Register SECSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMVFSECF`"]
pub type RMVFSECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMVFSECF`"]
pub struct RMVFSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVFSECF_W<'a> {
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
#[doc = "Reader of field `HSI48SECF`"]
pub type HSI48SECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSI48SECF`"]
pub struct HSI48SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48SECF_W<'a> {
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
#[doc = "Reader of field `CLK48MSECF`"]
pub type CLK48MSECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK48MSECF`"]
pub struct CLK48MSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK48MSECF_W<'a> {
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
#[doc = "Reader of field `PLLSAI2SECF`"]
pub type PLLSAI2SECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI2SECF`"]
pub struct PLLSAI2SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2SECF_W<'a> {
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
#[doc = "Reader of field `PLLSAI1SECF`"]
pub type PLLSAI1SECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI1SECF`"]
pub struct PLLSAI1SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1SECF_W<'a> {
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
#[doc = "Reader of field `PLLSECF`"]
pub type PLLSECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSECF`"]
pub struct PLLSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSECF_W<'a> {
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
#[doc = "Reader of field `PRESCSECF`"]
pub type PRESCSECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRESCSECF`"]
pub struct PRESCSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCSECF_W<'a> {
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
#[doc = "Reader of field `SYSCLKSECF`"]
pub type SYSCLKSECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCLKSECF`"]
pub struct SYSCLKSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLKSECF_W<'a> {
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
#[doc = "Reader of field `LSESECF`"]
pub type LSESECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSESECF`"]
pub struct LSESECF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSESECF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `LSISECF`"]
pub type LSISECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSISECF`"]
pub struct LSISECF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSISECF_W<'a> {
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
#[doc = "Reader of field `MSISECF`"]
pub type MSISECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSISECF`"]
pub struct MSISECF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSISECF_W<'a> {
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
#[doc = "Reader of field `HSESECF`"]
pub type HSESECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSESECF`"]
pub struct HSESECF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSESECF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HSISECF`"]
pub type HSISECF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSISECF`"]
pub struct HSISECF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSISECF_W<'a> {
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
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    pub fn rmvfsecf(&self) -> RMVFSECF_R {
        RMVFSECF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    pub fn hsi48secf(&self) -> HSI48SECF_R {
        HSI48SECF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    pub fn clk48msecf(&self) -> CLK48MSECF_R {
        CLK48MSECF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    pub fn pllsai2secf(&self) -> PLLSAI2SECF_R {
        PLLSAI2SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    pub fn pllsai1secf(&self) -> PLLSAI1SECF_R {
        PLLSAI1SECF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    pub fn pllsecf(&self) -> PLLSECF_R {
        PLLSECF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    pub fn prescsecf(&self) -> PRESCSECF_R {
        PRESCSECF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    pub fn sysclksecf(&self) -> SYSCLKSECF_R {
        SYSCLKSECF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    pub fn lsesecf(&self) -> LSESECF_R {
        LSESECF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    pub fn lsisecf(&self) -> LSISECF_R {
        LSISECF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    pub fn msisecf(&self) -> MSISECF_R {
        MSISECF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    pub fn hsesecf(&self) -> HSESECF_R {
        HSESECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    pub fn hsisecf(&self) -> HSISECF_R {
        HSISECF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    pub fn rmvfsecf(&mut self) -> RMVFSECF_W {
        RMVFSECF_W { w: self }
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    pub fn hsi48secf(&mut self) -> HSI48SECF_W {
        HSI48SECF_W { w: self }
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    pub fn clk48msecf(&mut self) -> CLK48MSECF_W {
        CLK48MSECF_W { w: self }
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    pub fn pllsai2secf(&mut self) -> PLLSAI2SECF_W {
        PLLSAI2SECF_W { w: self }
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    pub fn pllsai1secf(&mut self) -> PLLSAI1SECF_W {
        PLLSAI1SECF_W { w: self }
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    pub fn pllsecf(&mut self) -> PLLSECF_W {
        PLLSECF_W { w: self }
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    pub fn prescsecf(&mut self) -> PRESCSECF_W {
        PRESCSECF_W { w: self }
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    pub fn sysclksecf(&mut self) -> SYSCLKSECF_W {
        SYSCLKSECF_W { w: self }
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    pub fn lsesecf(&mut self) -> LSESECF_W {
        LSESECF_W { w: self }
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    pub fn lsisecf(&mut self) -> LSISECF_W {
        LSISECF_W { w: self }
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    pub fn msisecf(&mut self) -> MSISECF_W {
        MSISECF_W { w: self }
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    pub fn hsesecf(&mut self) -> HSESECF_W {
        HSESECF_W { w: self }
    }
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    pub fn hsisecf(&mut self) -> HSISECF_W {
        HSISECF_W { w: self }
    }
}
