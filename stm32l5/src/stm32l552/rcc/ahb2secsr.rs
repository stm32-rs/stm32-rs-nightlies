#[doc = "Register `AHB2SECSR` reader"]
pub struct R(crate::R<AHB2SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDMMC1SECF` reader - SDMMC1SECF"]
pub struct SDMMC1SECF_R(crate::FieldReader<bool, bool>);
impl SDMMC1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTFDEC1SECF` reader - OTFDEC1SECF"]
pub struct OTFDEC1SECF_R(crate::FieldReader<bool, bool>);
impl OTFDEC1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTFDEC1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTFDEC1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2SECF` reader - SRAM2SECF"]
pub struct SRAM2SECF_R(crate::FieldReader<bool, bool>);
impl SRAM2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOHSECF` reader - GPIOHSECF"]
pub struct GPIOHSECF_R(crate::FieldReader<bool, bool>);
impl GPIOHSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOHSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOHSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOGSECF` reader - GPIOGSECF"]
pub struct GPIOGSECF_R(crate::FieldReader<bool, bool>);
impl GPIOGSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOGSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOGSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOFSECF` reader - GPIOFSECF"]
pub struct GPIOFSECF_R(crate::FieldReader<bool, bool>);
impl GPIOFSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOFSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOFSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOESECF` reader - GPIOESECF"]
pub struct GPIOESECF_R(crate::FieldReader<bool, bool>);
impl GPIOESECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOESECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOESECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIODSECF` reader - GPIODSECF"]
pub struct GPIODSECF_R(crate::FieldReader<bool, bool>);
impl GPIODSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIODSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIODSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOCSECF` reader - GPIOCSECF"]
pub struct GPIOCSECF_R(crate::FieldReader<bool, bool>);
impl GPIOCSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOCSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOCSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOBSECF` reader - GPIOBSECF"]
pub struct GPIOBSECF_R(crate::FieldReader<bool, bool>);
impl GPIOBSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOBSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOBSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOASECF` reader - GPIOASECF"]
pub struct GPIOASECF_R(crate::FieldReader<bool, bool>);
impl GPIOASECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOASECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOASECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 22 - SDMMC1SECF"]
    #[inline(always)]
    pub fn sdmmc1secf(&self) -> SDMMC1SECF_R {
        SDMMC1SECF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OTFDEC1SECF"]
    #[inline(always)]
    pub fn otfdec1secf(&self) -> OTFDEC1SECF_R {
        OTFDEC1SECF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM2SECF"]
    #[inline(always)]
    pub fn sram2secf(&self) -> SRAM2SECF_R {
        SRAM2SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOHSECF"]
    #[inline(always)]
    pub fn gpiohsecf(&self) -> GPIOHSECF_R {
        GPIOHSECF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOGSECF"]
    #[inline(always)]
    pub fn gpiogsecf(&self) -> GPIOGSECF_R {
        GPIOGSECF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOFSECF"]
    #[inline(always)]
    pub fn gpiofsecf(&self) -> GPIOFSECF_R {
        GPIOFSECF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOESECF"]
    #[inline(always)]
    pub fn gpioesecf(&self) -> GPIOESECF_R {
        GPIOESECF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIODSECF"]
    #[inline(always)]
    pub fn gpiodsecf(&self) -> GPIODSECF_R {
        GPIODSECF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOCSECF"]
    #[inline(always)]
    pub fn gpiocsecf(&self) -> GPIOCSECF_R {
        GPIOCSECF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOBSECF"]
    #[inline(always)]
    pub fn gpiobsecf(&self) -> GPIOBSECF_R {
        GPIOBSECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIOASECF"]
    #[inline(always)]
    pub fn gpioasecf(&self) -> GPIOASECF_R {
        GPIOASECF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RCC AHB2 security status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2secsr](index.html) module"]
pub struct AHB2SECSR_SPEC;
impl crate::RegisterSpec for AHB2SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2secsr::R](R) reader structure"]
impl crate::Readable for AHB2SECSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHB2SECSR to value 0x0020_02ff"]
impl crate::Resettable for AHB2SECSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_02ff
    }
}
