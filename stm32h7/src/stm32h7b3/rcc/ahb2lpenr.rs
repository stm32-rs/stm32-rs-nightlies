#[doc = "Register `AHB2LPENR` reader"]
pub struct R(crate::R<AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2LPENR` writer"]
pub struct W(crate::W<AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2LPENR_SPEC>;
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
impl From<crate::W<AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMI_PSSILPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<DCMI_PSSILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSILPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMI_PSSILPEN` reader - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub struct DCMI_PSSILPEN_R(crate::FieldReader<bool, DCMI_PSSILPEN_A>);
impl DCMI_PSSILPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMI_PSSILPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMI_PSSILPEN_A {
        match self.bits {
            false => DCMI_PSSILPEN_A::DISABLED,
            true => DCMI_PSSILPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DCMI_PSSILPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DCMI_PSSILPEN_A::ENABLED
    }
}
impl core::ops::Deref for DCMI_PSSILPEN_R {
    type Target = crate::FieldReader<bool, DCMI_PSSILPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMI_PSSILPEN` writer - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub struct DCMI_PSSILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMI_PSSILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMI_PSSILPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMI_PSSILPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMI_PSSILPEN_A::ENABLED)
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
#[doc = "CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
pub type CRYPTLPEN_A = DCMI_PSSILPEN_A;
#[doc = "Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
pub type CRYPTLPEN_R = DCMI_PSSILPEN_R;
#[doc = "Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
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
#[doc = "HASH peripheral clock enable during CSleep mode Set and reset by software."]
pub type HASHLPEN_A = DCMI_PSSILPEN_A;
#[doc = "Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode Set and reset by software."]
pub type HASHLPEN_R = DCMI_PSSILPEN_R;
#[doc = "Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode Set and reset by software."]
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
#[doc = "RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
pub type RNGLPEN_A = DCMI_PSSILPEN_A;
#[doc = "Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
pub type RNGLPEN_R = DCMI_PSSILPEN_R;
#[doc = "Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
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
#[doc = "SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
pub type SDMMC2LPEN_A = DCMI_PSSILPEN_A;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
pub type SDMMC2LPEN_R = DCMI_PSSILPEN_R;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
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
#[doc = "DFSDMDMA clock enable during CSleep mode Set and reset by software."]
pub type DFSDMDMALPEN_A = DCMI_PSSILPEN_A;
#[doc = "Field `DFSDMDMALPEN` reader - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
pub type DFSDMDMALPEN_R = DCMI_PSSILPEN_R;
#[doc = "Field `DFSDMDMALPEN` writer - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
pub struct DFSDMDMALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMDMALPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDMDMALPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFSDMDMALPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DFSDMDMALPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
pub type AHBSRAM1LPEN_A = DCMI_PSSILPEN_A;
#[doc = "Field `AHBSRAM1LPEN` reader - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
pub type AHBSRAM1LPEN_R = DCMI_PSSILPEN_R;
#[doc = "Field `AHBSRAM1LPEN` writer - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
pub struct AHBSRAM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBSRAM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHBSRAM1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AHBSRAM1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AHBSRAM1LPEN_A::ENABLED)
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
#[doc = "AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
pub type AHBSRAM2LPEN_A = DCMI_PSSILPEN_A;
#[doc = "Field `AHBSRAM2LPEN` reader - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
pub type AHBSRAM2LPEN_R = DCMI_PSSILPEN_R;
#[doc = "Field `AHBSRAM2LPEN` writer - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
pub struct AHBSRAM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBSRAM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHBSRAM2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AHBSRAM2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AHBSRAM2LPEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssilpen(&self) -> DCMI_PSSILPEN_R {
        DCMI_PSSILPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dfsdmdmalpen(&self) -> DFSDMDMALPEN_R {
        DFSDMDMALPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ahbsram1lpen(&self) -> AHBSRAM1LPEN_R {
        AHBSRAM1LPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ahbsram2lpen(&self) -> AHBSRAM2LPEN_R {
        AHBSRAM2LPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssilpen(&mut self) -> DCMI_PSSILPEN_W {
        DCMI_PSSILPEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W {
        CRYPTLPEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W {
        HASHLPEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W {
        RNGLPEN_W { w: self }
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W {
        SDMMC2LPEN_W { w: self }
    }
    #[doc = "Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dfsdmdmalpen(&mut self) -> DFSDMDMALPEN_W {
        DFSDMDMALPEN_W { w: self }
    }
    #[doc = "Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ahbsram1lpen(&mut self) -> AHBSRAM1LPEN_W {
        AHBSRAM1LPEN_W { w: self }
    }
    #[doc = "Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ahbsram2lpen(&mut self) -> AHBSRAM2LPEN_W {
        AHBSRAM2LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2lpenr](index.html) module"]
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2lpenr::R](R) reader structure"]
impl crate::Readable for AHB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2lpenr::W](W) writer structure"]
impl crate::Writable for AHB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0x6000_0a71"]
impl crate::Resettable for AHB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_0a71
    }
}
