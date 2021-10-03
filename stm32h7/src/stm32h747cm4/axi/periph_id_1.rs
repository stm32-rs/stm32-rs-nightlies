#[doc = "Register `PERIPH_ID_1` reader"]
pub struct R(crate::R<PERIPH_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNUM` reader - Peripheral part number bits 8 to 11"]
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
#[doc = "Field `JEP106I` reader - JEP106 identity bits 0 to 3"]
pub struct JEP106I_R(crate::FieldReader<u8, u8>);
impl JEP106I_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEP106I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral part number bits 8 to 11"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity bits 0 to 3"]
    #[inline(always)]
    pub fn jep106i(&self) -> JEP106I_R {
        JEP106I_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_1](index.html) module"]
pub struct PERIPH_ID_1_SPEC;
impl crate::RegisterSpec for PERIPH_ID_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_1::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPH_ID_1 to value 0x04"]
impl crate::Resettable for PERIPH_ID_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
