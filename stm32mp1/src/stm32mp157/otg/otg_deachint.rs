#[doc = "Register `OTG_DEACHINT` reader"]
pub struct R(crate::R<OTG_DEACHINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DEACHINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DEACHINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DEACHINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEP1INT` reader - IEP1INT"]
pub struct IEP1INT_R(crate::FieldReader<bool, bool>);
impl IEP1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEP1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEP1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEP1INT` reader - OEP1INT"]
pub struct OEP1INT_R(crate::FieldReader<bool, bool>);
impl OEP1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEP1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEP1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - IEP1INT"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OEP1INT"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "OTG device each endpoint interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_deachint](index.html) module"]
pub struct OTG_DEACHINT_SPEC;
impl crate::RegisterSpec for OTG_DEACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_deachint::R](R) reader structure"]
impl crate::Readable for OTG_DEACHINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_DEACHINT to value 0"]
impl crate::Resettable for OTG_DEACHINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
