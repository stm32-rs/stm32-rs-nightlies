#[doc = "Register `WRP1AR` reader"]
pub struct R(crate::R<WRP1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP1AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRP1A_STRT` reader - WRP area A start offset"]
pub struct WRP1A_STRT_R(crate::FieldReader<u8, u8>);
impl WRP1A_STRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRP1A_STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP1A_STRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRP1A_END` reader - WRP area A end offset"]
pub struct WRP1A_END_R(crate::FieldReader<u8, u8>);
impl WRP1A_END_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRP1A_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP1A_END_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - WRP area A start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> WRP1A_STRT_R {
        WRP1A_STRT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - WRP area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> WRP1A_END_R {
        WRP1A_END_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Flash WRP area A address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1ar](index.html) module"]
pub struct WRP1AR_SPEC;
impl crate::RegisterSpec for WRP1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp1ar::R](R) reader structure"]
impl crate::Readable for WRP1AR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRP1AR to value 0xf000_0000"]
impl crate::Resettable for WRP1AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
