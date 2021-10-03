#[doc = "Register `MDMA_C8ISR` reader"]
pub struct R(crate::R<MDMA_C8ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C8ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C8ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C8ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEIF` reader - TEIF"]
pub struct TEIF_R(crate::FieldReader<bool, bool>);
impl TEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF` reader - CTCIF"]
pub struct CTCIF_R(crate::FieldReader<bool, bool>);
impl CTCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRTIF` reader - BRTIF"]
pub struct BRTIF_R(crate::FieldReader<bool, bool>);
impl BRTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRTIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRTIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTIF` reader - BTIF"]
pub struct BTIF_R(crate::FieldReader<bool, bool>);
impl BTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF` reader - TCIF"]
pub struct TCIF_R(crate::FieldReader<bool, bool>);
impl TCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRQA` reader - CRQA"]
pub struct CRQA_R(crate::FieldReader<bool, bool>);
impl CRQA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRQA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRQA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TEIF"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF"]
    #[inline(always)]
    pub fn brtif(&self) -> BRTIF_R {
        BRTIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF"]
    #[inline(always)]
    pub fn btif(&self) -> BTIF_R {
        BTIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF"]
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA"]
    #[inline(always)]
    pub fn crqa(&self) -> CRQA_R {
        CRQA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "MDMA channel 8 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8isr](index.html) module"]
pub struct MDMA_C8ISR_SPEC;
impl crate::RegisterSpec for MDMA_C8ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c8isr::R](R) reader structure"]
impl crate::Readable for MDMA_C8ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDMA_C8ISR to value 0"]
impl crate::Resettable for MDMA_C8ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
