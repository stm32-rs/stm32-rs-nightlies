#[doc = "Reader of register SR3"]
pub type R = crate::R<u32, super::SR3>;
#[doc = "Writer for register SR3"]
pub type W = crate::W<u32, super::SR3>;
#[doc = "Register SR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TZSCF`"]
pub type TZSCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZSCF`"]
pub struct TZSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCF_W<'a> {
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
#[doc = "Reader of field `TZICF`"]
pub type TZICF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZICF`"]
pub struct TZICF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICF_W<'a> {
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
#[doc = "Reader of field `MPCWM1F`"]
pub type MPCWM1F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCWM1F`"]
pub struct MPCWM1F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM1F_W<'a> {
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
#[doc = "Reader of field `MPCWM2F`"]
pub type MPCWM2F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCWM2F`"]
pub struct MPCWM2F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM2F_W<'a> {
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
#[doc = "Reader of field `MPCBB1F`"]
pub type MPCBB1F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB1F`"]
pub struct MPCBB1F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1F_W<'a> {
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
#[doc = "Reader of field `MPCBB1_REGF`"]
pub type MPCBB1_REGF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB1_REGF`"]
pub struct MPCBB1_REGF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1_REGF_W<'a> {
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
#[doc = "Reader of field `MPCBB2F`"]
pub type MPCBB2F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB2F`"]
pub struct MPCBB2F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2F_W<'a> {
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
#[doc = "Reader of field `MPCBB2_REGF`"]
pub type MPCBB2_REGF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB2_REGF`"]
pub struct MPCBB2_REGF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2_REGF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TZSCF"]
    #[inline(always)]
    pub fn tzscf(&self) -> TZSCF_R {
        TZSCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZICF"]
    #[inline(always)]
    pub fn tzicf(&self) -> TZICF_R {
        TZICF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPCWM1F"]
    #[inline(always)]
    pub fn mpcwm1f(&self) -> MPCWM1F_R {
        MPCWM1F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPCWM2F"]
    #[inline(always)]
    pub fn mpcwm2f(&self) -> MPCWM2F_R {
        MPCWM2F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPCBB1F"]
    #[inline(always)]
    pub fn mpcbb1f(&self) -> MPCBB1F_R {
        MPCBB1F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGF"]
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPCBB2F"]
    #[inline(always)]
    pub fn mpcbb2f(&self) -> MPCBB2F_R {
        MPCBB2F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGF"]
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCF"]
    #[inline(always)]
    pub fn tzscf(&mut self) -> TZSCF_W {
        TZSCF_W { w: self }
    }
    #[doc = "Bit 1 - TZICF"]
    #[inline(always)]
    pub fn tzicf(&mut self) -> TZICF_W {
        TZICF_W { w: self }
    }
    #[doc = "Bit 2 - MPCWM1F"]
    #[inline(always)]
    pub fn mpcwm1f(&mut self) -> MPCWM1F_W {
        MPCWM1F_W { w: self }
    }
    #[doc = "Bit 3 - MPCWM2F"]
    #[inline(always)]
    pub fn mpcwm2f(&mut self) -> MPCWM2F_W {
        MPCWM2F_W { w: self }
    }
    #[doc = "Bit 4 - MPCBB1F"]
    #[inline(always)]
    pub fn mpcbb1f(&mut self) -> MPCBB1F_W {
        MPCBB1F_W { w: self }
    }
    #[doc = "Bit 5 - MPCBB1_REGF"]
    #[inline(always)]
    pub fn mpcbb1_regf(&mut self) -> MPCBB1_REGF_W {
        MPCBB1_REGF_W { w: self }
    }
    #[doc = "Bit 6 - MPCBB2F"]
    #[inline(always)]
    pub fn mpcbb2f(&mut self) -> MPCBB2F_W {
        MPCBB2F_W { w: self }
    }
    #[doc = "Bit 7 - MPCBB2_REGF"]
    #[inline(always)]
    pub fn mpcbb2_regf(&mut self) -> MPCBB2_REGF_W {
        MPCBB2_REGF_W { w: self }
    }
}
