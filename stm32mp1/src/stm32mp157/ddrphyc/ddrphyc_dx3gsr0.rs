#[doc = "Register `DDRPHYC_DX3GSR0` reader"]
pub struct R(crate::R<DDRPHYC_DX3GSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX3GSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX3GSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX3GSR0_SPEC>) -> Self {
        R(reader)
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
#[doc = "Field `DTPASS` reader - DTPASS"]
pub struct DTPASS_R(crate::FieldReader<u8, u8>);
impl DTPASS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTPASS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DTDONE"]
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DTIERR"]
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - DTPASS"]
    #[inline(always)]
    pub fn dtpass(&self) -> DTPASS_R {
        DTPASS_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
#[doc = "DDRPHYC byte lane 3 GS register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3gsr0](index.html) module"]
pub struct DDRPHYC_DX3GSR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX3GSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ddrphyc_dx3gsr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_DX3GSR0 to value 0"]
impl crate::Resettable for DDRPHYC_DX3GSR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
