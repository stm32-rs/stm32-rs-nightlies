#[doc = "Register `UR0` reader"]
pub struct R(crate::R<UR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BKS` reader - Bank Swap"]
pub struct BKS_R(crate::FieldReader<bool, bool>);
impl BKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDP` reader - Readout protection"]
pub struct RDP_R(crate::FieldReader<u8, u8>);
impl RDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bank Swap"]
    #[inline(always)]
    pub fn bks(&self) -> BKS_R {
        BKS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Readout protection"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYSCFG user register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur0](index.html) module"]
pub struct UR0_SPEC;
impl crate::RegisterSpec for UR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur0::R](R) reader structure"]
impl crate::Readable for UR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR0 to value 0"]
impl crate::Resettable for UR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}