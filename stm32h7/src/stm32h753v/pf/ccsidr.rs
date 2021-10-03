#[doc = "Register `CCSIDR` reader"]
pub struct R(crate::R<CCSIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCSIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCSIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCSIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LineSize` reader - LineSize"]
pub struct LINESIZE_R(crate::FieldReader<u8, u8>);
impl LINESIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINESIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINESIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Associativity` reader - Associativity"]
pub struct ASSOCIATIVITY_R(crate::FieldReader<u16, u16>);
impl ASSOCIATIVITY_R {
    pub(crate) fn new(bits: u16) -> Self {
        ASSOCIATIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASSOCIATIVITY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NumSets` reader - NumSets"]
pub struct NUMSETS_R(crate::FieldReader<u16, u16>);
impl NUMSETS_R {
    pub(crate) fn new(bits: u16) -> Self {
        NUMSETS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMSETS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WA` reader - WA"]
pub struct WA_R(crate::FieldReader<bool, bool>);
impl WA_R {
    pub(crate) fn new(bits: bool) -> Self {
        WA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` reader - RA"]
pub struct RA_R(crate::FieldReader<bool, bool>);
impl RA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WB` reader - WB"]
pub struct WB_R(crate::FieldReader<bool, bool>);
impl WB_R {
    pub(crate) fn new(bits: bool) -> Self {
        WB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WT` reader - WT"]
pub struct WT_R(crate::FieldReader<bool, bool>);
impl WT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - LineSize"]
    #[inline(always)]
    pub fn line_size(&self) -> LINESIZE_R {
        LINESIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:12 - Associativity"]
    #[inline(always)]
    pub fn associativity(&self) -> ASSOCIATIVITY_R {
        ASSOCIATIVITY_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 13:27 - NumSets"]
    #[inline(always)]
    pub fn num_sets(&self) -> NUMSETS_R {
        NUMSETS_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 28 - WA"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WB"]
    #[inline(always)]
    pub fn wb(&self) -> WB_R {
        WB_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Cache Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccsidr](index.html) module"]
pub struct CCSIDR_SPEC;
impl crate::RegisterSpec for CCSIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccsidr::R](R) reader structure"]
impl crate::Readable for CCSIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCSIDR to value 0"]
impl crate::Resettable for CCSIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
