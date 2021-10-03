#[doc = "Register `DDRPHYC_PGSR` reader"]
pub struct R(crate::R<DDRPHYC_PGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_PGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_PGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_PGSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDONE` reader - IDONE"]
pub struct IDONE_R(crate::FieldReader<bool, bool>);
impl IDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLDONE` reader - DLDONE"]
pub struct DLDONE_R(crate::FieldReader<bool, bool>);
impl DLDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZCDDONE` reader - ZCDDONE"]
pub struct ZCDDONE_R(crate::FieldReader<bool, bool>);
impl ZCDDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZCDDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZCDDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIDONE` reader - DIDONE"]
pub struct DIDONE_R(crate::FieldReader<bool, bool>);
impl DIDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTDONE` reader - DTDONE"]
pub struct DTDONE_R(crate::FieldReader<bool, bool>);
impl DTDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTERR` reader - DTERR"]
pub struct DTERR_R(crate::FieldReader<bool, bool>);
impl DTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIERR` reader - DTIERR"]
pub struct DTIERR_R(crate::FieldReader<bool, bool>);
impl DTIERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFTERR` reader - DFTERR"]
pub struct DFTERR_R(crate::FieldReader<bool, bool>);
impl DFTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVERR` reader - RVERR"]
pub struct RVERR_R(crate::FieldReader<bool, bool>);
impl RVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVEIRR` reader - RVEIRR"]
pub struct RVEIRR_R(crate::FieldReader<bool, bool>);
impl RVEIRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVEIRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVEIRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQ` reader - TQ"]
pub struct TQ_R(crate::FieldReader<bool, bool>);
impl TQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - IDONE"]
    #[inline(always)]
    pub fn idone(&self) -> IDONE_R {
        IDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DLDONE"]
    #[inline(always)]
    pub fn dldone(&self) -> DLDONE_R {
        DLDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ZCDDONE"]
    #[inline(always)]
    pub fn zcddone(&self) -> ZCDDONE_R {
        ZCDDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIDONE"]
    #[inline(always)]
    pub fn didone(&self) -> DIDONE_R {
        DIDONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DTDONE"]
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DTIERR"]
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFTERR"]
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RVERR"]
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RVEIRR"]
    #[inline(always)]
    pub fn rveirr(&self) -> RVEIRR_R {
        RVEIRR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TQ"]
    #[inline(always)]
    pub fn tq(&self) -> TQ_R {
        TQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "DDRPHYC PHY global status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pgsr](index.html) module"]
pub struct DDRPHYC_PGSR_SPEC;
impl crate::RegisterSpec for DDRPHYC_PGSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_pgsr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_PGSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_PGSR to value 0"]
impl crate::Resettable for DDRPHYC_PGSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
