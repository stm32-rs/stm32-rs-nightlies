#[doc = "Register `C1_AHB2ENR` reader"]
pub struct R(crate::R<C1_AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_AHB2ENR` writer"]
pub struct W(crate::W<C1_AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB2ENR_SPEC>;
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
impl From<crate::W<C1_AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DCMI peripheral clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMIEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<DCMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIEN` reader - DCMI peripheral clock"]
pub struct DCMIEN_R(crate::FieldReader<bool, DCMIEN_A>);
impl DCMIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMIEN_A {
        match self.bits {
            false => DCMIEN_A::DISABLED,
            true => DCMIEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DCMIEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DCMIEN_A::ENABLED
    }
}
impl core::ops::Deref for DCMIEN_R {
    type Target = crate::FieldReader<bool, DCMIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMIEN` writer - DCMI peripheral clock"]
pub struct DCMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::ENABLED)
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
#[doc = "CRYPT peripheral clock enable"]
pub type CRYPTEN_A = DCMIEN_A;
#[doc = "Field `CRYPTEN` reader - CRYPT peripheral clock enable"]
pub type CRYPTEN_R = DCMIEN_R;
#[doc = "Field `CRYPTEN` writer - CRYPT peripheral clock enable"]
pub struct CRYPTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRYPTEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRYPTEN_A::ENABLED)
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
#[doc = "HASH peripheral clock enable"]
pub type HASHEN_A = DCMIEN_A;
#[doc = "Field `HASHEN` reader - HASH peripheral clock enable"]
pub type HASHEN_R = DCMIEN_R;
#[doc = "Field `HASHEN` writer - HASH peripheral clock enable"]
pub struct HASHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HASHEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HASHEN_A::ENABLED)
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
#[doc = "RNG peripheral clocks enable"]
pub type RNGEN_A = DCMIEN_A;
#[doc = "Field `RNGEN` reader - RNG peripheral clocks enable"]
pub type RNGEN_R = DCMIEN_R;
#[doc = "Field `RNGEN` writer - RNG peripheral clocks enable"]
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::ENABLED)
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
#[doc = "SDMMC2 and SDMMC2 delay clock enable"]
pub type SDMMC2EN_A = DCMIEN_A;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable"]
pub type SDMMC2EN_R = DCMIEN_R;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable"]
pub struct SDMMC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDMMC2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDMMC2EN_A::ENABLED)
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
#[doc = "SRAM1 block enable"]
pub type SRAM1EN_A = DCMIEN_A;
#[doc = "Field `SRAM1EN` reader - SRAM1 block enable"]
pub type SRAM1EN_R = DCMIEN_R;
#[doc = "Field `SRAM1EN` writer - SRAM1 block enable"]
pub struct SRAM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM1EN_A::ENABLED)
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
#[doc = "SRAM2 block enable"]
pub type SRAM2EN_A = DCMIEN_A;
#[doc = "Field `SRAM2EN` reader - SRAM2 block enable"]
pub type SRAM2EN_R = DCMIEN_R;
#[doc = "Field `SRAM2EN` writer - SRAM2 block enable"]
pub struct SRAM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM2EN_A::ENABLED)
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
#[doc = "SRAM3 block enable"]
pub type SRAM3EN_A = DCMIEN_A;
#[doc = "Field `SRAM3EN` reader - SRAM3 block enable"]
pub type SRAM3EN_R = DCMIEN_R;
#[doc = "Field `SRAM3EN` writer - SRAM3 block enable"]
pub struct SRAM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM3EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DCMI peripheral clock"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    pub fn crypten(&self) -> CRYPTEN_R {
        CRYPTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMI peripheral clock"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W {
        DCMIEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    pub fn crypten(&mut self) -> CRYPTEN_W {
        CRYPTEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W {
        HASHEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W {
        SDMMC2EN_W { w: self }
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    pub fn sram1en(&mut self) -> SRAM1EN_W {
        SRAM1EN_W { w: self }
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    pub fn sram2en(&mut self) -> SRAM2EN_W {
        SRAM2EN_W { w: self }
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    pub fn sram3en(&mut self) -> SRAM3EN_W {
        SRAM3EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb2enr](index.html) module"]
pub struct C1_AHB2ENR_SPEC;
impl crate::RegisterSpec for C1_AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_ahb2enr::R](R) reader structure"]
impl crate::Readable for C1_AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_ahb2enr::W](W) writer structure"]
impl crate::Writable for C1_AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_AHB2ENR to value 0"]
impl crate::Resettable for C1_AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
