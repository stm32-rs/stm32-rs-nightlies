#[doc = "Reader of register RCC_MP_GRSTCSETR"]
pub type R = crate::R<u32, super::RCC_MP_GRSTCSETR>;
#[doc = "Writer for register RCC_MP_GRSTCSETR"]
pub type W = crate::W<u32, super::RCC_MP_GRSTCSETR>;
#[doc = "Register RCC_MP_GRSTCSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_GRSTCSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPSYSRST`"]
pub type MPSYSRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPSYSRST`"]
pub struct MPSYSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSYSRST_W<'a> {
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
#[doc = "Reader of field `MCURST`"]
pub type MCURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCURST`"]
pub struct MCURST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCURST_W<'a> {
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
#[doc = "Reader of field `MPUP0RST`"]
pub type MPUP0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUP0RST`"]
pub struct MPUP0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUP0RST_W<'a> {
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
#[doc = "Reader of field `MPUP1RST`"]
pub type MPUP1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUP1RST`"]
pub struct MPUP1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUP1RST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    pub fn mpsysrst(&self) -> MPSYSRST_R {
        MPSYSRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    pub fn mcurst(&self) -> MCURST_R {
        MCURST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPUP0RST"]
    #[inline(always)]
    pub fn mpup0rst(&self) -> MPUP0RST_R {
        MPUP0RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPUP1RST"]
    #[inline(always)]
    pub fn mpup1rst(&self) -> MPUP1RST_R {
        MPUP1RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    pub fn mpsysrst(&mut self) -> MPSYSRST_W {
        MPSYSRST_W { w: self }
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    pub fn mcurst(&mut self) -> MCURST_W {
        MCURST_W { w: self }
    }
    #[doc = "Bit 4 - MPUP0RST"]
    #[inline(always)]
    pub fn mpup0rst(&mut self) -> MPUP0RST_W {
        MPUP0RST_W { w: self }
    }
    #[doc = "Bit 5 - MPUP1RST"]
    #[inline(always)]
    pub fn mpup1rst(&mut self) -> MPUP1RST_W {
        MPUP1RST_W { w: self }
    }
}
