#[doc = "Register `C2CR` reader"]
pub struct R(crate::R<C2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR` writer"]
pub struct W(crate::W<C2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR_SPEC>;
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
impl From<crate::W<C2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RXOIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOIE_A {
    #[doc = "1: Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt"]
    ENABLED = 1,
    #[doc = "0: Processor RX occupied interrupt disabled"]
    DISABLED = 0,
}
impl From<RXOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOIE` reader - RXOIE"]
pub struct RXOIE_R(crate::FieldReader<bool, RXOIE_A>);
impl RXOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOIE_A {
        match self.bits {
            true => RXOIE_A::ENABLED,
            false => RXOIE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXOIE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXOIE_A::DISABLED
    }
}
impl core::ops::Deref for RXOIE_R {
    type Target = crate::FieldReader<bool, RXOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIE` writer - RXOIE"]
pub struct RXOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXOIE_A::ENABLED)
    }
    #[doc = "Processor RX occupied interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXOIE_A::DISABLED)
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
#[doc = "TXFIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIE_A {
    #[doc = "1: Enable an unmasked processor transmit channel free to generate a TX free interrupt"]
    ENABLED = 1,
    #[doc = "0: Processor TX free interrupt disabled"]
    DISABLED = 0,
}
impl From<TXFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIE` reader - TXFIE"]
pub struct TXFIE_R(crate::FieldReader<bool, TXFIE_A>);
impl TXFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIE_A {
        match self.bits {
            true => TXFIE_A::ENABLED,
            false => TXFIE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXFIE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXFIE_A::DISABLED
    }
}
impl core::ops::Deref for TXFIE_R {
    type Target = crate::FieldReader<bool, TXFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIE` writer - TXFIE"]
pub struct TXFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable an unmasked processor transmit channel free to generate a TX free interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFIE_A::ENABLED)
    }
    #[doc = "Processor TX free interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFIE_A::DISABLED)
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
impl R {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W {
        RXOIE_W { w: self }
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&mut self) -> TXFIE_W {
        TXFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPCC Processor 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr](index.html) module"]
pub struct C2CR_SPEC;
impl crate::RegisterSpec for C2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr::R](R) reader structure"]
impl crate::Readable for C2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr::W](W) writer structure"]
impl crate::Writable for C2CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2CR to value 0"]
impl crate::Resettable for C2CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
