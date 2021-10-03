#[doc = "Register `MACTSSR` reader"]
pub struct R(crate::R<MACTSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSOVF` reader - Timestamp Seconds Overflow"]
pub struct TSSOVF_R(crate::FieldReader<bool, bool>);
impl TSSOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTARGT0` reader - Timestamp Target Time Reached"]
pub struct TSTARGT0_R(crate::FieldReader<bool, bool>);
impl TSTARGT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTARGT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTARGT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXTSTRIG` reader - Auxiliary Timestamp Trigger Snapshot"]
pub struct AUXTSTRIG_R(crate::FieldReader<bool, bool>);
impl AUXTSTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXTSTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXTSTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRGTERR0` reader - Timestamp Target Time Error"]
pub struct TSTRGTERR0_R(crate::FieldReader<bool, bool>);
impl TSTRGTERR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTRGTERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRGTERR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTSSIS` reader - Tx Timestamp Status Interrupt Status"]
pub struct TXTSSIS_R(crate::FieldReader<bool, bool>);
impl TXTSSIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTSSIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSSTN` reader - Auxiliary Timestamp Snapshot Trigger Identifier"]
pub struct ATSSTN_R(crate::FieldReader<u8, u8>);
impl ATSSTN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATSSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSSTN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSSTM` reader - Auxiliary Timestamp Snapshot Trigger Missed"]
pub struct ATSSTM_R(crate::FieldReader<bool, bool>);
impl ATSSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATSSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSNS` reader - Number of Auxiliary Timestamp Snapshots"]
pub struct ATSNS_R(crate::FieldReader<u8, u8>);
impl ATSNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATSNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auxiliary Timestamp Trigger Snapshot"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Timestamp Status Interrupt Status"]
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier"]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed"]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Number of Auxiliary Timestamp Snapshots"]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
#[doc = "Timestamp status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactssr](index.html) module"]
pub struct MACTSSR_SPEC;
impl crate::RegisterSpec for MACTSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactssr::R](R) reader structure"]
impl crate::Readable for MACTSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACTSSR to value 0"]
impl crate::Resettable for MACTSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
