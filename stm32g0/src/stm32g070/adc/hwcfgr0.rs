#[doc = "Register `HWCFGR0` reader"]
pub struct R(crate::R<HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_CHAN_24` reader - NUM_CHAN_24"]
pub struct NUM_CHAN_24_R(crate::FieldReader<u8, u8>);
impl NUM_CHAN_24_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_CHAN_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_CHAN_24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTRA_AWDS` reader - Extra analog watchdog"]
pub struct EXTRA_AWDS_R(crate::FieldReader<u8, u8>);
impl EXTRA_AWDS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTRA_AWDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRA_AWDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVS` reader - Oversampling"]
pub struct OVS_R(crate::FieldReader<u8, u8>);
impl OVS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - NUM_CHAN_24"]
    #[inline(always)]
    pub fn num_chan_24(&self) -> NUM_CHAN_24_R {
        NUM_CHAN_24_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Extra analog watchdog"]
    #[inline(always)]
    pub fn extra_awds(&self) -> EXTRA_AWDS_R {
        EXTRA_AWDS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr0](index.html) module"]
pub struct HWCFGR0_SPEC;
impl crate::RegisterSpec for HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr0::R](R) reader structure"]
impl crate::Readable for HWCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR0 to value 0x0110"]
impl crate::Resettable for HWCFGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110
    }
}
