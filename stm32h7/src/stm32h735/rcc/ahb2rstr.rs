#[doc = "Register `AHB2RSTR` reader"]
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2RSTR` writer"]
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `CAMITFRST` reader - CAMITF block reset"]
pub struct CAMITFRST_R(crate::FieldReader<bool, CAMITFRST_A>);
impl CAMITFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAMITFRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAMITFRST_A> {
        match self.bits {
            true => Some(CAMITFRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == CAMITFRST_A::RESET
    }
}
impl core::ops::Deref for CAMITFRST_R {
    type Target = crate::FieldReader<bool, CAMITFRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAMITFRST` writer - CAMITF block reset"]
pub struct CAMITFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMITFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAMITFRST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Cryptography block reset"]
pub type CRYPTRST_A = CAMITFRST_A;
#[doc = "Field `CRYPTRST` reader - Cryptography block reset"]
pub type CRYPTRST_R = CAMITFRST_R;
#[doc = "Field `CRYPTRST` writer - Cryptography block reset"]
pub struct CRYPTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRYPTRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Hash block reset"]
pub type HASHRST_A = CAMITFRST_A;
#[doc = "Field `HASHRST` reader - Hash block reset"]
pub type HASHRST_R = CAMITFRST_R;
#[doc = "Field `HASHRST` writer - Hash block reset"]
pub struct HASHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HASHRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Random Number Generator block reset"]
pub type RNGRST_A = CAMITFRST_A;
#[doc = "Field `RNGRST` reader - Random Number Generator block reset"]
pub type RNGRST_R = CAMITFRST_R;
#[doc = "Field `RNGRST` writer - Random Number Generator block reset"]
pub struct RNGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RNGRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "SDMMC2 and SDMMC2 Delay block reset"]
pub type SDMMC2RST_A = CAMITFRST_A;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 Delay block reset"]
pub type SDMMC2RST_R = CAMITFRST_R;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 Delay block reset"]
pub struct SDMMC2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDMMC2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "FMAC reset"]
pub type FMACRST_A = CAMITFRST_A;
#[doc = "Field `FMACRST` reader - FMAC reset"]
pub type FMACRST_R = CAMITFRST_R;
#[doc = "Field `FMACRST` writer - FMAC reset"]
pub struct FMACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMACRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMACRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMACRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "CORDIC reset"]
pub type CORDICRST_A = CAMITFRST_A;
#[doc = "Field `CORDICRST` reader - CORDIC reset"]
pub type CORDICRST_R = CAMITFRST_R;
#[doc = "Field `CORDICRST` writer - CORDIC reset"]
pub struct CORDICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CORDICRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORDICRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CORDICRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
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
    #[doc = "Bit 16 - FMAC reset"]
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CORDIC reset"]
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 17) & 0x01) != 0)
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
    #[doc = "Bit 16 - FMAC reset"]
    #[inline(always)]
    pub fn fmacrst(&mut self) -> FMACRST_W {
        FMACRST_W { w: self }
    }
    #[doc = "Bit 17 - CORDIC reset"]
    #[inline(always)]
    pub fn cordicrst(&mut self) -> CORDICRST_W {
        CORDICRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](index.html) module"]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2rstr::R](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
