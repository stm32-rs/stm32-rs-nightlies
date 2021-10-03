#[doc = "Register `CRYP_MISR` reader"]
pub struct R(crate::R<CRYP_MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INMIS` reader - INMIS"]
pub struct INMIS_R(crate::FieldReader<bool, bool>);
impl INMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTMIS` reader - OUTMIS"]
pub struct OUTMIS_R(crate::FieldReader<bool, bool>);
impl OUTMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - INMIS"]
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OUTMIS"]
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_misr](index.html) module"]
pub struct CRYP_MISR_SPEC;
impl crate::RegisterSpec for CRYP_MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_misr::R](R) reader structure"]
impl crate::Readable for CRYP_MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRYP_MISR to value 0"]
impl crate::Resettable for CRYP_MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
