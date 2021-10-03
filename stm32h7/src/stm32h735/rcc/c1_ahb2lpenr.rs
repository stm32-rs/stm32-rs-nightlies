#[doc = "Register `C1_AHB2LPENR` reader"]
pub struct R(crate::R<C1_AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_AHB2LPENR` writer"]
pub struct W(crate::W<C1_AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB2LPENR_SPEC>;
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
impl From<crate::W<C1_AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DCMI peripheral clock enable during csleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMILPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<DCMILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMILPEN` reader - DCMI peripheral clock enable during csleep mode"]
pub struct DCMILPEN_R(crate::FieldReader<bool, DCMILPEN_A>);
impl DCMILPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMILPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMILPEN_A {
        match self.bits {
            false => DCMILPEN_A::DISABLED,
            true => DCMILPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DCMILPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DCMILPEN_A::ENABLED
    }
}
impl core::ops::Deref for DCMILPEN_R {
    type Target = crate::FieldReader<bool, DCMILPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMILPEN` writer - DCMI peripheral clock enable during csleep mode"]
pub struct DCMILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMILPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMILPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMILPEN_A::ENABLED)
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
#[doc = "CRYPT peripheral clock enable during CSleep mode"]
pub type CRYPTLPEN_A = DCMILPEN_A;
#[doc = "Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode"]
pub type CRYPTLPEN_R = DCMILPEN_R;
#[doc = "Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode"]
pub struct CRYPTLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRYPTLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRYPTLPEN_A::ENABLED)
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
#[doc = "HASH peripheral clock enable during CSleep mode"]
pub type HASHLPEN_A = DCMILPEN_A;
#[doc = "Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode"]
pub type HASHLPEN_R = DCMILPEN_R;
#[doc = "Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode"]
pub struct HASHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HASHLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HASHLPEN_A::ENABLED)
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
#[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub type SDMMC2LPEN_A = DCMILPEN_A;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub type SDMMC2LPEN_R = DCMILPEN_R;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub struct SDMMC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDMMC2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDMMC2LPEN_A::ENABLED)
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
#[doc = "RNG peripheral clock enable during CSleep mode"]
pub type RNGLPEN_A = DCMILPEN_A;
#[doc = "Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode"]
pub type RNGLPEN_R = DCMILPEN_R;
#[doc = "Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode"]
pub struct RNGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGLPEN_A::ENABLED)
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
#[doc = "SRAM1 Clock Enable During CSleep Mode"]
pub type SRAM1LPEN_A = DCMILPEN_A;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 Clock Enable During CSleep Mode"]
pub type SRAM1LPEN_R = DCMILPEN_R;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 Clock Enable During CSleep Mode"]
pub struct SRAM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "SRAM2 Clock Enable During CSleep Mode"]
pub type SRAM2LPEN_A = DCMILPEN_A;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 Clock Enable During CSleep Mode"]
pub type SRAM2LPEN_R = DCMILPEN_R;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 Clock Enable During CSleep Mode"]
pub struct SRAM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM2LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "FMAC enable during CSleep Mode"]
pub type FMACLPEN_A = DCMILPEN_A;
#[doc = "Field `FMACLPEN` reader - FMAC enable during CSleep Mode"]
pub type FMACLPEN_R = DCMILPEN_R;
#[doc = "Field `FMACLPEN` writer - FMAC enable during CSleep Mode"]
pub struct FMACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMACLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMACLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMACLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMACLPEN_A::ENABLED)
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
#[doc = "CORDIC enable during CSleep Mode"]
pub type CORDICLPEN_A = DCMILPEN_A;
#[doc = "Field `CORDICLPEN` reader - CORDIC enable during CSleep Mode"]
pub type CORDICLPEN_R = DCMILPEN_R;
#[doc = "Field `CORDICLPEN` writer - CORDIC enable during CSleep Mode"]
pub struct CORDICLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORDICLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORDICLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CORDICLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CORDICLPEN_A::ENABLED)
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
    #[doc = "Bit 0 - DCMI peripheral clock enable during csleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FMAC enable during CSleep Mode"]
    #[inline(always)]
    pub fn fmaclpen(&self) -> FMACLPEN_R {
        FMACLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CORDIC enable during CSleep Mode"]
    #[inline(always)]
    pub fn cordiclpen(&self) -> CORDICLPEN_R {
        CORDICLPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMI peripheral clock enable during csleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W {
        DCMILPEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W {
        CRYPTLPEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W {
        HASHLPEN_W { w: self }
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W {
        SDMMC2LPEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W {
        RNGLPEN_W { w: self }
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W {
        SRAM1LPEN_W { w: self }
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W {
        SRAM2LPEN_W { w: self }
    }
    #[doc = "Bit 16 - FMAC enable during CSleep Mode"]
    #[inline(always)]
    pub fn fmaclpen(&mut self) -> FMACLPEN_W {
        FMACLPEN_W { w: self }
    }
    #[doc = "Bit 17 - CORDIC enable during CSleep Mode"]
    #[inline(always)]
    pub fn cordiclpen(&mut self) -> CORDICLPEN_W {
        CORDICLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb2lpenr](index.html) module"]
pub struct C1_AHB2LPENR_SPEC;
impl crate::RegisterSpec for C1_AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_ahb2lpenr::R](R) reader structure"]
impl crate::Readable for C1_AHB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_ahb2lpenr::W](W) writer structure"]
impl crate::Writable for C1_AHB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_AHB2LPENR to value 0"]
impl crate::Resettable for C1_AHB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
