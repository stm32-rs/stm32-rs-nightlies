#[doc = "Register `COMP_ID_3` reader"]
pub struct R(crate::R<COMP_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREAMBLE` reader - Preamble bits 20 to 27"]
pub struct PREAMBLE_R(crate::FieldReader<u8, u8>);
impl PREAMBLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PREAMBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble bits 20 to 27"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_3](index.html) module"]
pub struct COMP_ID_3_SPEC;
impl crate::RegisterSpec for COMP_ID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_id_3::R](R) reader structure"]
impl crate::Readable for COMP_ID_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP_ID_3 to value 0x04"]
impl crate::Resettable for COMP_ID_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
