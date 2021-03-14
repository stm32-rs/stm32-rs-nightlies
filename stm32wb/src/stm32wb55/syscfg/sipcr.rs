#[doc = "Reader of register SIPCR"]
pub type R = crate::R<u32, super::SIPCR>;
#[doc = "Writer for register SIPCR"]
pub type W = crate::W<u32, super::SIPCR>;
#[doc = "Register SIPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SIPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAES1`"]
pub type SAES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAES1`"]
pub struct SAES1_W<'a> {
    w: &'a mut W,
}
impl<'a> SAES1_W<'a> {
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
#[doc = "Reader of field `SAES2`"]
pub type SAES2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAES2`"]
pub struct SAES2_W<'a> {
    w: &'a mut W,
}
impl<'a> SAES2_W<'a> {
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
#[doc = "Reader of field `SPKA`"]
pub type SPKA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPKA`"]
pub struct SPKA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPKA_W<'a> {
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
#[doc = "Reader of field `SRNG`"]
pub type SRNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRNG`"]
pub struct SRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> SRNG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    pub fn saes1(&self) -> SAES1_R {
        SAES1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    pub fn saes2(&self) -> SAES2_R {
        SAES2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    pub fn spka(&self) -> SPKA_R {
        SPKA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    pub fn srng(&self) -> SRNG_R {
        SRNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    pub fn saes1(&mut self) -> SAES1_W {
        SAES1_W { w: self }
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    pub fn saes2(&mut self) -> SAES2_W {
        SAES2_W { w: self }
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    pub fn spka(&mut self) -> SPKA_W {
        SPKA_W { w: self }
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    pub fn srng(&mut self) -> SRNG_W {
        SRNG_W { w: self }
    }
}
