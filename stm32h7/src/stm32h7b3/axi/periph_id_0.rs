#[doc = "Register `PERIPH_ID_0` reader"]
pub struct R(crate::R<PERIPH_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNUM` reader - Peripheral part number bits 0 to 7"]
pub struct PARTNUM_R(crate::FieldReader<u8, u8>);
impl PARTNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PARTNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral part number bits 0 to 7"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_0](index.html) module"]
pub struct PERIPH_ID_0_SPEC;
impl crate::RegisterSpec for PERIPH_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_0::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPH_ID_0 to value 0x04"]
impl crate::Resettable for PERIPH_ID_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
