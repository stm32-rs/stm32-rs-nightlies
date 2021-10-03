#[doc = "Register `SDSR` reader"]
pub struct R(crate::R<SDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RE` reader - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
pub struct RE_R(crate::FieldReader<bool, bool>);
impl RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODES1` reader - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1."]
pub struct MODES1_R(crate::FieldReader<u8, u8>);
impl MODES1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODES1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODES2` reader - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2."]
pub struct MODES2_R(crate::FieldReader<u8, u8>);
impl MODES2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODES2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1."]
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2."]
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
#[doc = "SDRAM Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdsr](index.html) module"]
pub struct SDSR_SPEC;
impl crate::RegisterSpec for SDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdsr::R](R) reader structure"]
impl crate::Readable for SDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDSR to value 0"]
impl crate::Resettable for SDSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
