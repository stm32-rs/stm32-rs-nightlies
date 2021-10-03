#[doc = "Register `ISR1` reader"]
pub struct R(crate::R<ISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOHSTX` reader - TOHSTX"]
pub struct TOHSTX_R(crate::FieldReader<bool, bool>);
impl TOHSTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOHSTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOHSTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOLPRX` reader - TOLPRX"]
pub struct TOLPRX_R(crate::FieldReader<bool, bool>);
impl TOLPRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOLPRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOLPRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCSE` reader - ECCSE"]
pub struct ECCSE_R(crate::FieldReader<bool, bool>);
impl ECCSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCME` reader - ECCME"]
pub struct ECCME_R(crate::FieldReader<bool, bool>);
impl ECCME_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCE` reader - CRCE"]
pub struct CRCE_R(crate::FieldReader<bool, bool>);
impl CRCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSE` reader - PSE"]
pub struct PSE_R(crate::FieldReader<bool, bool>);
impl PSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOTPE` reader - EOTPE"]
pub struct EOTPE_R(crate::FieldReader<bool, bool>);
impl EOTPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOTPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOTPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPWRE` reader - LPWRE"]
pub struct LPWRE_R(crate::FieldReader<bool, bool>);
impl LPWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCWRE` reader - GCWRE"]
pub struct GCWRE_R(crate::FieldReader<bool, bool>);
impl GCWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPWRE` reader - GPWRE"]
pub struct GPWRE_R(crate::FieldReader<bool, bool>);
impl GPWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPTXE` reader - GPTXE"]
pub struct GPTXE_R(crate::FieldReader<bool, bool>);
impl GPTXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPTXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPTXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPRDE` reader - GPRDE"]
pub struct GPRDE_R(crate::FieldReader<bool, bool>);
impl GPRDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPRDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPRDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPRXE` reader - GPRXE"]
pub struct GPRXE_R(crate::FieldReader<bool, bool>);
impl GPRXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPRXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPRXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TOHSTX"]
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TOLPRX"]
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECCSE"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECCME"]
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRCE"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EOTPE"]
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPWRE"]
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GCWRE"]
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPWRE"]
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTXE"]
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPRDE"]
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPRXE"]
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "DSI Host interrupt and status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr1](index.html) module"]
pub struct ISR1_SPEC;
impl crate::RegisterSpec for ISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr1::R](R) reader structure"]
impl crate::Readable for ISR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR1 to value 0"]
impl crate::Resettable for ISR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
