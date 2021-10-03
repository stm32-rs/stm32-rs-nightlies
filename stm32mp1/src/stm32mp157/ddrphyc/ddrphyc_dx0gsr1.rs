#[doc = "Register `DDRPHYC_DX0GSR1` reader"]
pub struct R(crate::R<DDRPHYC_DX0GSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX0GSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX0GSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX0GSR1_SPEC>) -> Self {
        R(reader)
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
#[doc = "Field `DQSDFT` reader - DQSDFT"]
pub struct DQSDFT_R(crate::FieldReader<u8, u8>);
impl DQSDFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSDFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSDFT_R {
    type Target = crate::FieldReader<u8, u8>;
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
#[doc = "Field `RVIERR` reader - RVIERR"]
pub struct RVIERR_R(crate::FieldReader<bool, bool>);
impl RVIERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVIERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVIERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVPASS` reader - RVPASS"]
pub struct RVPASS_R(crate::FieldReader<u8, u8>);
impl RVPASS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RVPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVPASS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DFTERR"]
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - DQSDFT"]
    #[inline(always)]
    pub fn dqsdft(&self) -> DQSDFT_R {
        DQSDFT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 12 - RVERR"]
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RVIERR"]
    #[inline(always)]
    pub fn rvierr(&self) -> RVIERR_R {
        RVIERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - RVPASS"]
    #[inline(always)]
    pub fn rvpass(&self) -> RVPASS_R {
        RVPASS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
#[doc = "DDRPHYC byte lane 0 GS register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gsr1](index.html) module"]
pub struct DDRPHYC_DX0GSR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX0GSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dx0gsr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_DX0GSR1 to value 0"]
impl crate::Resettable for DDRPHYC_DX0GSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
