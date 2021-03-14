#[doc = "Reader of register DDRPHYC_ZQ0CR0"]
pub type R = crate::R<u32, super::DDRPHYC_ZQ0CR0>;
#[doc = "Writer for register DDRPHYC_ZQ0CR0"]
pub type W = crate::W<u32, super::DDRPHYC_ZQ0CR0>;
#[doc = "Register DDRPHYC_ZQ0CR0 `reset()`'s with value 0x014a"]
impl crate::ResetValue for super::DDRPHYC_ZQ0CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x014a
    }
}
#[doc = "Reader of field `ZDATA`"]
pub type ZDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ZDATA`"]
pub struct ZDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ZDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
#[doc = "Reader of field `ZDEN`"]
pub type ZDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZDEN`"]
pub struct ZDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ZCALBYP`"]
pub type ZCALBYP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZCALBYP`"]
pub struct ZCALBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> ZCALBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ZCAL`"]
pub type ZCAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZCAL`"]
pub struct ZCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZCAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ZQPD`"]
pub type ZQPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZQPD`"]
pub struct ZQPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    pub fn zdata(&self) -> ZDATA_R {
        ZDATA_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    pub fn zden(&self) -> ZDEN_R {
        ZDEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    pub fn zcalbyp(&self) -> ZCALBYP_R {
        ZCALBYP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    pub fn zcal(&self) -> ZCAL_R {
        ZCAL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    pub fn zdata(&mut self) -> ZDATA_W {
        ZDATA_W { w: self }
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    pub fn zden(&mut self) -> ZDEN_W {
        ZDEN_W { w: self }
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W {
        ZCALBYP_W { w: self }
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    pub fn zcal(&mut self) -> ZCAL_W {
        ZCAL_W { w: self }
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W {
        ZQPD_W { w: self }
    }
}
