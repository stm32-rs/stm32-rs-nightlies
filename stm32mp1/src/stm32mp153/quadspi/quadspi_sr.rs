#[doc = "Register `QUADSPI_SR` reader"]
pub struct R(crate::R<QUADSPI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEF` reader - TEF"]
pub struct TEF_R(crate::FieldReader<bool, bool>);
impl TEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCF` reader - TCF"]
pub struct TCF_R(crate::FieldReader<bool, bool>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTF` reader - FTF"]
pub struct FTF_R(crate::FieldReader<bool, bool>);
impl FTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMF` reader - SMF"]
pub struct SMF_R(crate::FieldReader<bool, bool>);
impl SMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOF` reader - TOF"]
pub struct TOF_R(crate::FieldReader<bool, bool>);
impl TOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - BUSY"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEVEL` reader - FLEVEL"]
pub struct FLEVEL_R(crate::FieldReader<u8, u8>);
impl FLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TEF"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTF"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMF"]
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TOF"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - FLEVEL"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "QUADSPI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_sr](index.html) module"]
pub struct QUADSPI_SR_SPEC;
impl crate::RegisterSpec for QUADSPI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_sr::R](R) reader structure"]
impl crate::Readable for QUADSPI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUADSPI_SR to value 0"]
impl crate::Resettable for QUADSPI_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
