#[doc = "Register `SPDIFRX_CSR` reader"]
pub struct R(crate::R<SPDIFRX_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USR` reader - USR"]
pub struct USR_R(crate::FieldReader<u16, u16>);
impl USR_R {
    pub(crate) fn new(bits: u16) -> Self {
        USR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS` reader - CS"]
pub struct CS_R(crate::FieldReader<u8, u8>);
impl CS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOB` reader - SOB"]
pub struct SOB_R(crate::FieldReader<bool, bool>);
impl SOB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - USR"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - CS"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - SOB"]
    #[inline(always)]
    pub fn sob(&self) -> SOB_R {
        SOB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "Channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_csr](index.html) module"]
pub struct SPDIFRX_CSR_SPEC;
impl crate::RegisterSpec for SPDIFRX_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_csr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPDIFRX_CSR to value 0"]
impl crate::Resettable for SPDIFRX_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
