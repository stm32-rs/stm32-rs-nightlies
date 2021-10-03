#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLVL` reader - FIFO level threshold"]
pub struct FLVL_R(crate::FieldReader<u8, u8>);
impl FLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFSDET` reader - Late frame synchronization detection"]
pub struct LFSDET_R(crate::FieldReader<bool, bool>);
impl LFSDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFSDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFSDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSDET` reader - Anticipated frame synchronization detection"]
pub struct AFSDET_R(crate::FieldReader<bool, bool>);
impl AFSDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        AFSDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNRDY` reader - Codec not ready"]
pub struct CNRDY_R(crate::FieldReader<bool, bool>);
impl CNRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ` reader - FIFO request"]
pub struct FREQ_R(crate::FieldReader<bool, bool>);
impl FREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCKCFG` reader - Wrong clock configuration flag. This bit is read only"]
pub struct WCKCFG_R(crate::FieldReader<bool, bool>);
impl WCKCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCKCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCKCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTEDET` reader - Mute detection"]
pub struct MUTEDET_R(crate::FieldReader<bool, bool>);
impl MUTEDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTEDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTEDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRUDR` reader - Overrun / underrun"]
pub struct OVRUDR_R(crate::FieldReader<bool, bool>);
impl OVRUDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRUDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRUDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:18 - FIFO level threshold"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Late frame synchronization detection"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Codec not ready"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO request"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag. This bit is read only"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mute detection"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "AStatus register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x08"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
