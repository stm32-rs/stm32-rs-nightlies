#[doc = "Register `HWCFGR1` reader"]
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NBINT` reader - Hardware Configuration number of interrupts supported number of master IDs"]
pub struct NBINT_R(crate::FieldReader<u8, u8>);
impl NBINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBSEM` reader - Hardware Configuration number of semaphores"]
pub struct NBSEM_R(crate::FieldReader<u8, u8>);
impl NBSEM_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBSEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBSEM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:11 - Hardware Configuration number of interrupts supported number of master IDs"]
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - Hardware Configuration number of semaphores"]
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Semaphore hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](index.html) module"]
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr1::R](R) reader structure"]
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR1 to value 0x0220"]
impl crate::Resettable for HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0220
    }
}
