#[doc = "Reader of register MMC_CONTROL"]
pub type R = crate::R<u32, super::MMC_CONTROL>;
#[doc = "Writer for register MMC_CONTROL"]
pub type W = crate::W<u32, super::MMC_CONTROL>;
#[doc = "Register MMC_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::MMC_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTRST`"]
pub type CNTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTRST`"]
pub struct CNTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTRST_W<'a> {
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
#[doc = "Reader of field `CNTSTOPRO`"]
pub type CNTSTOPRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTSTOPRO`"]
pub struct CNTSTOPRO_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSTOPRO_W<'a> {
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
#[doc = "Reader of field `RSTONRD`"]
pub type RSTONRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTONRD`"]
pub struct RSTONRD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTONRD_W<'a> {
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
#[doc = "Reader of field `CNTFREEZ`"]
pub type CNTFREEZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTFREEZ`"]
pub struct CNTFREEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTFREEZ_W<'a> {
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
#[doc = "Reader of field `CNTPRST`"]
pub type CNTPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTPRST`"]
pub struct CNTPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRST_W<'a> {
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
#[doc = "Reader of field `CNTPRSTLVL`"]
pub type CNTPRSTLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTPRSTLVL`"]
pub struct CNTPRSTLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRSTLVL_W<'a> {
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
#[doc = "Reader of field `UCDBC`"]
pub type UCDBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCDBC`"]
pub struct UCDBC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDBC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CNTRST"]
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CNTSTOPRO"]
    #[inline(always)]
    pub fn cntstopro(&self) -> CNTSTOPRO_R {
        CNTSTOPRO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RSTONRD"]
    #[inline(always)]
    pub fn rstonrd(&self) -> RSTONRD_R {
        RSTONRD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CNTFREEZ"]
    #[inline(always)]
    pub fn cntfreez(&self) -> CNTFREEZ_R {
        CNTFREEZ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CNTPRSTLVL"]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CNTPRSTLVL_R {
        CNTPRSTLVL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UCDBC"]
    #[inline(always)]
    pub fn ucdbc(&self) -> UCDBC_R {
        UCDBC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNTRST"]
    #[inline(always)]
    pub fn cntrst(&mut self) -> CNTRST_W {
        CNTRST_W { w: self }
    }
    #[doc = "Bit 1 - CNTSTOPRO"]
    #[inline(always)]
    pub fn cntstopro(&mut self) -> CNTSTOPRO_W {
        CNTSTOPRO_W { w: self }
    }
    #[doc = "Bit 2 - RSTONRD"]
    #[inline(always)]
    pub fn rstonrd(&mut self) -> RSTONRD_W {
        RSTONRD_W { w: self }
    }
    #[doc = "Bit 3 - CNTFREEZ"]
    #[inline(always)]
    pub fn cntfreez(&mut self) -> CNTFREEZ_W {
        CNTFREEZ_W { w: self }
    }
    #[doc = "Bit 4 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W {
        CNTPRST_W { w: self }
    }
    #[doc = "Bit 5 - CNTPRSTLVL"]
    #[inline(always)]
    pub fn cntprstlvl(&mut self) -> CNTPRSTLVL_W {
        CNTPRSTLVL_W { w: self }
    }
    #[doc = "Bit 8 - UCDBC"]
    #[inline(always)]
    pub fn ucdbc(&mut self) -> UCDBC_W {
        UCDBC_W { w: self }
    }
}
