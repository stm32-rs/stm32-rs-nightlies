#[doc = "Reader of register AHB2RSTR"]
pub type R = crate::R<u32, super::AHB2RSTR>;
#[doc = "Writer for register AHB2RSTR"]
pub type W = crate::W<u32, super::AHB2RSTR>;
#[doc = "Register AHB2RSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB2RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CAMITF block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAMITFRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<CAMITFRST_A> for bool {
    #[inline(always)]
    fn from(variant: CAMITFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAMITFRST`"]
pub type CAMITFRST_R = crate::R<bool, CAMITFRST_A>;
impl CAMITFRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CAMITFRST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CAMITFRST_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CAMITFRST_A::RESET
    }
}
#[doc = "Write proxy for field `CAMITFRST`"]
pub struct CAMITFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMITFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAMITFRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CAMITFRST_A::RESET)
    }
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
#[doc = "Cryptography block reset"]
pub type CRYPTRST_A = CAMITFRST_A;
#[doc = "Reader of field `CRYPTRST`"]
pub type CRYPTRST_R = crate::R<bool, CAMITFRST_A>;
#[doc = "Write proxy for field `CRYPTRST`"]
pub struct CRYPTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CAMITFRST_A::RESET)
    }
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
#[doc = "Hash block reset"]
pub type HASHRST_A = CAMITFRST_A;
#[doc = "Reader of field `HASHRST`"]
pub type HASHRST_R = crate::R<bool, CAMITFRST_A>;
#[doc = "Write proxy for field `HASHRST`"]
pub struct HASHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CAMITFRST_A::RESET)
    }
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
#[doc = "Random Number Generator block reset"]
pub type RNGRST_A = CAMITFRST_A;
#[doc = "Reader of field `RNGRST`"]
pub type RNGRST_R = crate::R<bool, CAMITFRST_A>;
#[doc = "Write proxy for field `RNGRST`"]
pub struct RNGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CAMITFRST_A::RESET)
    }
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
#[doc = "SDMMC2 and SDMMC2 Delay block reset"]
pub type SDMMC2RST_A = CAMITFRST_A;
#[doc = "Reader of field `SDMMC2RST`"]
pub type SDMMC2RST_R = crate::R<bool, CAMITFRST_A>;
#[doc = "Write proxy for field `SDMMC2RST`"]
pub struct SDMMC2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CAMITFRST_A::RESET)
    }
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
impl R {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    pub fn camitfrst(&self) -> CAMITFRST_R {
        CAMITFRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    pub fn camitfrst(&mut self) -> CAMITFRST_W {
        CAMITFRST_W { w: self }
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    pub fn cryptrst(&mut self) -> CRYPTRST_W {
        CRYPTRST_W { w: self }
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W {
        HASHRST_W { w: self }
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W {
        RNGRST_W { w: self }
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W {
        SDMMC2RST_W { w: self }
    }
}
