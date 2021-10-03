#[doc = "Register `FDCAN_ENDN` reader"]
pub struct R(crate::R<FDCAN_ENDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ENDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ENDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ENDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ETV` reader - ETV"]
pub struct ETV_R(crate::FieldReader<u32, u32>);
impl ETV_R {
    pub(crate) fn new(bits: u32) -> Self {
        ETV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ETV"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "FDCAN Endian register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_endn](index.html) module"]
pub struct FDCAN_ENDN_SPEC;
impl crate::RegisterSpec for FDCAN_ENDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_endn::R](R) reader structure"]
impl crate::Readable for FDCAN_ENDN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_ENDN to value 0x8765_4321"]
impl crate::Resettable for FDCAN_ENDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8765_4321
    }
}
