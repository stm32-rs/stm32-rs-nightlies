#[doc = "Register `HSEM_RLR27` reader"]
pub struct R(crate::R<HSEM_RLR27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_RLR27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_RLR27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_RLR27_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCID` reader - PROCID"]
pub struct PROCID_R(crate::FieldReader<u8, u8>);
impl PROCID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROCID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COREID` reader - COREID"]
pub struct COREID_R(crate::FieldReader<u8, u8>);
impl COREID_R {
    pub(crate) fn new(bits: u8) -> Self {
        COREID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PROCID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr27](index.html) module"]
pub struct HSEM_RLR27_SPEC;
impl crate::RegisterSpec for HSEM_RLR27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_rlr27::R](R) reader structure"]
impl crate::Readable for HSEM_RLR27_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSEM_RLR27 to value 0"]
impl crate::Resettable for HSEM_RLR27_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
