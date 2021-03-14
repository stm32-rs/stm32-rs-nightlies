#[doc = "Reader of register SECCFGR"]
pub type R = crate::R<u32, super::SECCFGR>;
#[doc = "Writer for register SECCFGR"]
pub type W = crate::W<u32, super::SECCFGR>;
#[doc = "Register SECCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSISEC`"]
pub type HSISEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSISEC`"]
pub struct HSISEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSISEC_W<'a> {
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
#[doc = "Reader of field `HSESEC`"]
pub type HSESEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSESEC`"]
pub struct HSESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSESEC_W<'a> {
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
#[doc = "Reader of field `MSISEC`"]
pub type MSISEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSISEC`"]
pub struct MSISEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSISEC_W<'a> {
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
#[doc = "Reader of field `LSISEC`"]
pub type LSISEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSISEC`"]
pub struct LSISEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSISEC_W<'a> {
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
#[doc = "Reader of field `LSESEC`"]
pub type LSESEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSESEC`"]
pub struct LSESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSESEC_W<'a> {
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
#[doc = "Reader of field `SYSCLKSEC`"]
pub type SYSCLKSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCLKSEC`"]
pub struct SYSCLKSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLKSEC_W<'a> {
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
#[doc = "Reader of field `PRESCSEC`"]
pub type PRESCSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRESCSEC`"]
pub struct PRESCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCSEC_W<'a> {
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
#[doc = "Reader of field `PLLSEC`"]
pub type PLLSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSEC`"]
pub struct PLLSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSEC_W<'a> {
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
#[doc = "Reader of field `PLLSAI1SEC`"]
pub type PLLSAI1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI1SEC`"]
pub struct PLLSAI1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1SEC_W<'a> {
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
#[doc = "Reader of field `PLLSAI2SEC`"]
pub type PLLSAI2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI2SEC`"]
pub struct PLLSAI2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2SEC_W<'a> {
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
#[doc = "Reader of field `CLK48MSEC`"]
pub type CLK48MSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK48MSEC`"]
pub struct CLK48MSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK48MSEC_W<'a> {
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
#[doc = "Reader of field `HSI48SEC`"]
pub type HSI48SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSI48SEC`"]
pub struct HSI48SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48SEC_W<'a> {
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
#[doc = "Reader of field `RMVFSEC`"]
pub type RMVFSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMVFSEC`"]
pub struct RMVFSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVFSEC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    pub fn pllsec(&self) -> PLLSEC_R {
        PLLSEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    pub fn pllsai1sec(&self) -> PLLSAI1SEC_R {
        PLLSAI1SEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    pub fn pllsai2sec(&self) -> PLLSAI2SEC_R {
        PLLSAI2SEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    pub fn clk48msec(&self) -> CLK48MSEC_R {
        CLK48MSEC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    pub fn hsisec(&mut self) -> HSISEC_W {
        HSISEC_W { w: self }
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    pub fn hsesec(&mut self) -> HSESEC_W {
        HSESEC_W { w: self }
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    pub fn msisec(&mut self) -> MSISEC_W {
        MSISEC_W { w: self }
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    pub fn lsisec(&mut self) -> LSISEC_W {
        LSISEC_W { w: self }
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    pub fn lsesec(&mut self) -> LSESEC_W {
        LSESEC_W { w: self }
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W {
        SYSCLKSEC_W { w: self }
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    pub fn prescsec(&mut self) -> PRESCSEC_W {
        PRESCSEC_W { w: self }
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    pub fn pllsec(&mut self) -> PLLSEC_W {
        PLLSEC_W { w: self }
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    pub fn pllsai1sec(&mut self) -> PLLSAI1SEC_W {
        PLLSAI1SEC_W { w: self }
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    pub fn pllsai2sec(&mut self) -> PLLSAI2SEC_W {
        PLLSAI2SEC_W { w: self }
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    pub fn clk48msec(&mut self) -> CLK48MSEC_W {
        CLK48MSEC_W { w: self }
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W {
        HSI48SEC_W { w: self }
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W {
        RMVFSEC_W { w: self }
    }
}
