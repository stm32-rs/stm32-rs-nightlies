#[doc = "Register `PCROP1AER` reader"]
pub struct R(crate::R<PCROP1AER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1AER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1AER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1AER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCROP1A_END` reader - PCROP1A area end offset"]
pub struct PCROP1A_END_R(crate::FieldReader<u8, u8>);
impl PCROP1A_END_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROP1A_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1A_END_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub struct PCROP_RDP_R(crate::FieldReader<bool, bool>);
impl PCROP_RDP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCROP_RDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP_RDP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PCROP1A area end offset"]
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Flash PCROP zone A End address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1aer](index.html) module"]
pub struct PCROP1AER_SPEC;
impl crate::RegisterSpec for PCROP1AER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1aer::R](R) reader structure"]
impl crate::Readable for PCROP1AER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCROP1AER to value 0xf000_0000"]
impl crate::Resettable for PCROP1AER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
