#[doc = "Register `OTG_DAINT` reader"]
pub struct R(crate::R<OTG_DAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEPINT` reader - IEPINT"]
pub struct IEPINT_R(crate::FieldReader<u16, u16>);
impl IEPINT_R {
    pub(crate) fn new(bits: u16) -> Self {
        IEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEPINT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEPINT` reader - OEPINT"]
pub struct OEPINT_R(crate::FieldReader<u16, u16>);
impl OEPINT_R {
    pub(crate) fn new(bits: u16) -> Self {
        OEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEPINT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_daint](index.html) module"]
pub struct OTG_DAINT_SPEC;
impl crate::RegisterSpec for OTG_DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_daint::R](R) reader structure"]
impl crate::Readable for OTG_DAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_DAINT to value 0"]
impl crate::Resettable for OTG_DAINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
