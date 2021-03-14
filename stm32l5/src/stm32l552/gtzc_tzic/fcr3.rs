#[doc = "Reader of register FCR3"]
pub type R = crate::R<u32, super::FCR3>;
#[doc = "Writer for register FCR3"]
pub type W = crate::W<u32, super::FCR3>;
#[doc = "Register FCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TZSCFC`"]
pub type TZSCFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZSCFC`"]
pub struct TZSCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCFC_W<'a> {
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
#[doc = "Reader of field `TZICFC`"]
pub type TZICFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZICFC`"]
pub struct TZICFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICFC_W<'a> {
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
#[doc = "Reader of field `MPCWM1FC`"]
pub type MPCWM1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCWM1FC`"]
pub struct MPCWM1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM1FC_W<'a> {
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
#[doc = "Reader of field `MPCWM2FC`"]
pub type MPCWM2FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCWM2FC`"]
pub struct MPCWM2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM2FC_W<'a> {
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
#[doc = "Reader of field `MPCBB1FC`"]
pub type MPCBB1FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB1FC`"]
pub struct MPCBB1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1FC_W<'a> {
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
#[doc = "Reader of field `MPCBB1_REGFC`"]
pub type MPCBB1_REGFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB1_REGFC`"]
pub struct MPCBB1_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1_REGFC_W<'a> {
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
#[doc = "Reader of field `MPCBB2FC`"]
pub type MPCBB2FC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB2FC`"]
pub struct MPCBB2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2FC_W<'a> {
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
#[doc = "Reader of field `MPCBB2_REGFC`"]
pub type MPCBB2_REGFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB2_REGFC`"]
pub struct MPCBB2_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2_REGFC_W<'a> {
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
    #[doc = "Bit 0 - TZSCFC"]
    #[inline(always)]
    pub fn tzscfc(&self) -> TZSCFC_R {
        TZSCFC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZICFC"]
    #[inline(always)]
    pub fn tzicfc(&self) -> TZICFC_R {
        TZICFC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPCWM1FC"]
    #[inline(always)]
    pub fn mpcwm1fc(&self) -> MPCWM1FC_R {
        MPCWM1FC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPCWM2FC"]
    #[inline(always)]
    pub fn mpcwm2fc(&self) -> MPCWM2FC_R {
        MPCWM2FC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPCBB1FC"]
    #[inline(always)]
    pub fn mpcbb1fc(&self) -> MPCBB1FC_R {
        MPCBB1FC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGFC"]
    #[inline(always)]
    pub fn mpcbb1_regfc(&self) -> MPCBB1_REGFC_R {
        MPCBB1_REGFC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPCBB2FC"]
    #[inline(always)]
    pub fn mpcbb2fc(&self) -> MPCBB2FC_R {
        MPCBB2FC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGFC"]
    #[inline(always)]
    pub fn mpcbb2_regfc(&self) -> MPCBB2_REGFC_R {
        MPCBB2_REGFC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCFC"]
    #[inline(always)]
    pub fn tzscfc(&mut self) -> TZSCFC_W {
        TZSCFC_W { w: self }
    }
    #[doc = "Bit 1 - TZICFC"]
    #[inline(always)]
    pub fn tzicfc(&mut self) -> TZICFC_W {
        TZICFC_W { w: self }
    }
    #[doc = "Bit 2 - MPCWM1FC"]
    #[inline(always)]
    pub fn mpcwm1fc(&mut self) -> MPCWM1FC_W {
        MPCWM1FC_W { w: self }
    }
    #[doc = "Bit 3 - MPCWM2FC"]
    #[inline(always)]
    pub fn mpcwm2fc(&mut self) -> MPCWM2FC_W {
        MPCWM2FC_W { w: self }
    }
    #[doc = "Bit 4 - MPCBB1FC"]
    #[inline(always)]
    pub fn mpcbb1fc(&mut self) -> MPCBB1FC_W {
        MPCBB1FC_W { w: self }
    }
    #[doc = "Bit 5 - MPCBB1_REGFC"]
    #[inline(always)]
    pub fn mpcbb1_regfc(&mut self) -> MPCBB1_REGFC_W {
        MPCBB1_REGFC_W { w: self }
    }
    #[doc = "Bit 6 - MPCBB2FC"]
    #[inline(always)]
    pub fn mpcbb2fc(&mut self) -> MPCBB2FC_W {
        MPCBB2FC_W { w: self }
    }
    #[doc = "Bit 7 - MPCBB2_REGFC"]
    #[inline(always)]
    pub fn mpcbb2_regfc(&mut self) -> MPCBB2_REGFC_W {
        MPCBB2_REGFC_W { w: self }
    }
}
