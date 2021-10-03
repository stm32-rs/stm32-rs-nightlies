#[doc = "Register `GICD_TYPER` reader"]
pub struct R(crate::R<GICD_TYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_TYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_TYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_TYPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ITLINESNUMBER` reader - ITLINESNUMBER"]
pub struct ITLINESNUMBER_R(crate::FieldReader<u8, u8>);
impl ITLINESNUMBER_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITLINESNUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITLINESNUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUNUMBER` reader - CPUNUMBER"]
pub struct CPUNUMBER_R(crate::FieldReader<u8, u8>);
impl CPUNUMBER_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPUNUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUNUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURITYEXTN` reader - SECURITYEXTN"]
pub struct SECURITYEXTN_R(crate::FieldReader<bool, bool>);
impl SECURITYEXTN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURITYEXTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURITYEXTN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSPI` reader - LSPI"]
pub struct LSPI_R(crate::FieldReader<u8, u8>);
impl LSPI_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - ITLINESNUMBER"]
    #[inline(always)]
    pub fn itlinesnumber(&self) -> ITLINESNUMBER_R {
        ITLINESNUMBER_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - CPUNUMBER"]
    #[inline(always)]
    pub fn cpunumber(&self) -> CPUNUMBER_R {
        CPUNUMBER_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 10 - SECURITYEXTN"]
    #[inline(always)]
    pub fn securityextn(&self) -> SECURITYEXTN_R {
        SECURITYEXTN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - LSPI"]
    #[inline(always)]
    pub fn lspi(&self) -> LSPI_R {
        LSPI_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
#[doc = "GICD interrupt controller type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_typer](index.html) module"]
pub struct GICD_TYPER_SPEC;
impl crate::RegisterSpec for GICD_TYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_typer::R](R) reader structure"]
impl crate::Readable for GICD_TYPER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_TYPER to value 0xfc28"]
impl crate::Resettable for GICD_TYPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfc28
    }
}
