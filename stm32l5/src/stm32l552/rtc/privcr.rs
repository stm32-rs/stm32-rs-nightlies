#[doc = "Reader of register PRIVCR"]
pub type R = crate::R<u32, super::PRIVCR>;
#[doc = "Writer for register PRIVCR"]
pub type W = crate::W<u32, super::PRIVCR>;
#[doc = "Register PRIVCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PRIVCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIV`"]
pub type PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV`"]
pub struct PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `INITPRIV`"]
pub type INITPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITPRIV`"]
pub struct INITPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INITPRIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CALPRIV`"]
pub type CALPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALPRIV`"]
pub struct CALPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CALPRIV_W<'a> {
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
#[doc = "Reader of field `TSPRIV`"]
pub type TSPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSPRIV`"]
pub struct TSPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPRIV_W<'a> {
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
#[doc = "Reader of field `WUTPRIV`"]
pub type WUTPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUTPRIV`"]
pub struct WUTPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTPRIV_W<'a> {
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
#[doc = "Reader of field `ALRBPRIV`"]
pub type ALRBPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRBPRIV`"]
pub struct ALRBPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBPRIV_W<'a> {
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
#[doc = "Reader of field `ALRAPRIV`"]
pub type ALRAPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRAPRIV`"]
pub struct ALRAPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAPRIV_W<'a> {
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
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W {
        PRIV_W { w: self }
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    pub fn initpriv(&mut self) -> INITPRIV_W {
        INITPRIV_W { w: self }
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    pub fn calpriv(&mut self) -> CALPRIV_W {
        CALPRIV_W { w: self }
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    pub fn tspriv(&mut self) -> TSPRIV_W {
        TSPRIV_W { w: self }
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    pub fn wutpriv(&mut self) -> WUTPRIV_W {
        WUTPRIV_W { w: self }
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W {
        ALRBPRIV_W { w: self }
    }
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W {
        ALRAPRIV_W { w: self }
    }
}
