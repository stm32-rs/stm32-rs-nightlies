#[doc = "Reader of register ACR2"]
pub type R = crate::R<u32, super::ACR2>;
#[doc = "Writer for register ACR2"]
pub type W = crate::W<u32, super::ACR2>;
#[doc = "Register ACR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ACR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIVMODE`"]
pub type PRIVMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIVMODE`"]
pub struct PRIVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVMODE_W<'a> {
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
#[doc = "Reader of field `HDPADIS`"]
pub type HDPADIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDPADIS`"]
pub struct HDPADIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPADIS_W<'a> {
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
#[doc = "Reader of field `C2SWDBGEN`"]
pub type C2SWDBGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C2SWDBGEN`"]
pub struct C2SWDBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C2SWDBGEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    pub fn privmode(&self) -> PRIVMODE_R {
        PRIVMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    pub fn hdpadis(&self) -> HDPADIS_R {
        HDPADIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    pub fn c2swdbgen(&self) -> C2SWDBGEN_R {
        C2SWDBGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    pub fn privmode(&mut self) -> PRIVMODE_W {
        PRIVMODE_W { w: self }
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    pub fn hdpadis(&mut self) -> HDPADIS_W {
        HDPADIS_W { w: self }
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    pub fn c2swdbgen(&mut self) -> C2SWDBGEN_W {
        C2SWDBGEN_W { w: self }
    }
}
