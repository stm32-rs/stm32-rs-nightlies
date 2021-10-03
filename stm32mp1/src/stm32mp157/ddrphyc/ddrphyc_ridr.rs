#[doc = "Register `DDRPHYC_RIDR` reader"]
pub struct R(crate::R<DDRPHYC_RIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_RIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_RIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_RIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PUBMNR` reader - PUBMNR"]
pub struct PUBMNR_R(crate::FieldReader<u8, u8>);
impl PUBMNR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUBMNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUBMNR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUBMDR` reader - PUBMDR"]
pub struct PUBMDR_R(crate::FieldReader<u8, u8>);
impl PUBMDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUBMDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUBMDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUBMJR` reader - PUBMJR"]
pub struct PUBMJR_R(crate::FieldReader<u8, u8>);
impl PUBMJR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUBMJR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUBMJR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYMNR` reader - PHYMNR"]
pub struct PHYMNR_R(crate::FieldReader<u8, u8>);
impl PHYMNR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHYMNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYMNR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYMDR` reader - PHYMDR"]
pub struct PHYMDR_R(crate::FieldReader<u8, u8>);
impl PHYMDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHYMDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYMDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYMJR` reader - PHYMJR"]
pub struct PHYMJR_R(crate::FieldReader<u8, u8>);
impl PHYMJR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHYMJR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYMJR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDRID` reader - UDRID"]
pub struct UDRID_R(crate::FieldReader<u8, u8>);
impl UDRID_R {
    pub(crate) fn new(bits: u8) -> Self {
        UDRID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDRID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - PUBMNR"]
    #[inline(always)]
    pub fn pubmnr(&self) -> PUBMNR_R {
        PUBMNR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PUBMDR"]
    #[inline(always)]
    pub fn pubmdr(&self) -> PUBMDR_R {
        PUBMDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PUBMJR"]
    #[inline(always)]
    pub fn pubmjr(&self) -> PUBMJR_R {
        PUBMJR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PHYMNR"]
    #[inline(always)]
    pub fn phymnr(&self) -> PHYMNR_R {
        PHYMNR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PHYMDR"]
    #[inline(always)]
    pub fn phymdr(&self) -> PHYMDR_R {
        PHYMDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PHYMJR"]
    #[inline(always)]
    pub fn phymjr(&self) -> PHYMJR_R {
        PHYMJR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - UDRID"]
    #[inline(always)]
    pub fn udrid(&self) -> UDRID_R {
        UDRID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DDRPHYC revision ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ridr](index.html) module"]
pub struct DDRPHYC_RIDR_SPEC;
impl crate::RegisterSpec for DDRPHYC_RIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_ridr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_RIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_RIDR to value 0x0041_0010"]
impl crate::Resettable for DDRPHYC_RIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0041_0010
    }
}
