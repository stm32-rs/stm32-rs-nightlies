#[doc = "Register `ESR` reader"]
pub struct R(crate::R<ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BTE` reader - Bit timing error"]
pub struct BTE_R(crate::FieldReader<bool, bool>);
impl BTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPE` reader - Bit period error"]
pub struct BPE_R(crate::FieldReader<bool, bool>);
impl BPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBTFE` reader - Rx block transfer finished error"]
pub struct RBTFE_R(crate::FieldReader<bool, bool>);
impl RBTFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBTFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBTFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBE` reader - Start bit error"]
pub struct SBE_R(crate::FieldReader<bool, bool>);
impl SBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKE` reader - Block acknowledge error"]
pub struct ACKE_R(crate::FieldReader<bool, bool>);
impl ACKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE` reader - Line error"]
pub struct LINE_R(crate::FieldReader<bool, bool>);
impl LINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBTFE` reader - Tx block transfer finished error"]
pub struct TBTFE_R(crate::FieldReader<bool, bool>);
impl TBTFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBTFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBTFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bit timing error"]
    #[inline(always)]
    pub fn bte(&self) -> BTE_R {
        BTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bit period error"]
    #[inline(always)]
    pub fn bpe(&self) -> BPE_R {
        BPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx block transfer finished error"]
    #[inline(always)]
    pub fn rbtfe(&self) -> RBTFE_R {
        RBTFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start bit error"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Block acknowledge error"]
    #[inline(always)]
    pub fn acke(&self) -> ACKE_R {
        ACKE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line error"]
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Tx block transfer finished error"]
    #[inline(always)]
    pub fn tbtfe(&self) -> TBTFE_R {
        TBTFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "CEC error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](index.html) module"]
pub struct ESR_SPEC;
impl crate::RegisterSpec for ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr::R](R) reader structure"]
impl crate::Readable for ESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
