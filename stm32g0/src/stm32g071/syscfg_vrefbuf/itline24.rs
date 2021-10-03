#[doc = "Register `ITLINE24` reader"]
pub struct R(crate::R<ITLINE24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C2` reader - I2C2"]
pub struct I2C2_R(crate::FieldReader<bool, bool>);
impl I2C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 24 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline24](index.html) module"]
pub struct ITLINE24_SPEC;
impl crate::RegisterSpec for ITLINE24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline24::R](R) reader structure"]
impl crate::Readable for ITLINE24_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE24 to value 0"]
impl crate::Resettable for ITLINE24_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
