#[doc = "Reader of register TZSC_PRIVCFGR1"]
pub type R = crate::R<u32, super::TZSC_PRIVCFGR1>;
#[doc = "Writer for register TZSC_PRIVCFGR1"]
pub type W = crate::W<u32, super::TZSC_PRIVCFGR1>;
#[doc = "Register TZSC_PRIVCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_PRIVCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESPRIV`"]
pub type AESPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESPRIV`"]
pub struct AESPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AESPRIV_W<'a> {
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
#[doc = "Reader of field `RNGPRIV`"]
pub type RNGPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGPRIV`"]
pub struct RNGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGPRIV_W<'a> {
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
#[doc = "Reader of field `SUBGHZSPIPRIV`"]
pub type SUBGHZSPIPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBGHZSPIPRIV`"]
pub struct SUBGHZSPIPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPIPRIV_W<'a> {
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
#[doc = "Reader of field `PKAPRIV`"]
pub type PKAPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKAPRIV`"]
pub struct PKAPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAPRIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIPRIV"]
    #[inline(always)]
    pub fn subghzspipriv(&self) -> SUBGHZSPIPRIV_R {
        SUBGHZSPIPRIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W {
        AESPRIV_W { w: self }
    }
    #[doc = "Bit 3 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W {
        RNGPRIV_W { w: self }
    }
    #[doc = "Bit 4 - SUBGHZSPIPRIV"]
    #[inline(always)]
    pub fn subghzspipriv(&mut self) -> SUBGHZSPIPRIV_W {
        SUBGHZSPIPRIV_W { w: self }
    }
    #[doc = "Bit 13 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W {
        PKAPRIV_W { w: self }
    }
}
