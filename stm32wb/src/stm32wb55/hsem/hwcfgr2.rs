#[doc = "Register `HWCFGR2` reader"]
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MASTERID4` reader - Hardware Configuration valid bus masters ID4"]
pub struct MASTERID4_R(crate::FieldReader<u8, u8>);
impl MASTERID4_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASTERID4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTERID4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTERID3` reader - Hardware Configuration valid bus masters ID3"]
pub struct MASTERID3_R(crate::FieldReader<u8, u8>);
impl MASTERID3_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASTERID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTERID3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTERID2` reader - Hardware Configuration valid bus masters ID2"]
pub struct MASTERID2_R(crate::FieldReader<u8, u8>);
impl MASTERID2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASTERID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTERID2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTERID1` reader - Hardware Configuration valid bus masters ID1"]
pub struct MASTERID1_R(crate::FieldReader<u8, u8>);
impl MASTERID1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASTERID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTERID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 12:15 - Hardware Configuration valid bus masters ID4"]
    #[inline(always)]
    pub fn masterid4(&self) -> MASTERID4_R {
        MASTERID4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Hardware Configuration valid bus masters ID3"]
    #[inline(always)]
    pub fn masterid3(&self) -> MASTERID3_R {
        MASTERID3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Hardware Configuration valid bus masters ID2"]
    #[inline(always)]
    pub fn masterid2(&self) -> MASTERID2_R {
        MASTERID2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Hardware Configuration valid bus masters ID1"]
    #[inline(always)]
    pub fn masterid1(&self) -> MASTERID1_R {
        MASTERID1_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Semaphore hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](index.html) module"]
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr2::R](R) reader structure"]
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR2 to value 0x84"]
impl crate::Resettable for HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x84
    }
}
