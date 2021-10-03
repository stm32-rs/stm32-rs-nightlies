#[doc = "Register `GPIOD_HWCFGR3` reader"]
pub struct R(crate::R<GPIOD_HWCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_HWCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_HWCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_HWCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ODR_RES` reader - ODR_RES"]
pub struct ODR_RES_R(crate::FieldReader<u16, u16>);
impl ODR_RES_R {
    pub(crate) fn new(bits: u16) -> Self {
        ODR_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODR_RES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTYPER_RES` reader - OTYPER_RES"]
pub struct OTYPER_RES_R(crate::FieldReader<u16, u16>);
impl OTYPER_RES_R {
    pub(crate) fn new(bits: u16) -> Self {
        OTYPER_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTYPER_RES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - ODR_RES"]
    #[inline(always)]
    pub fn odr_res(&self) -> ODR_RES_R {
        ODR_RES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OTYPER_RES"]
    #[inline(always)]
    pub fn otyper_res(&self) -> OTYPER_RES_R {
        OTYPER_RES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr3](index.html) module"]
pub struct GPIOD_HWCFGR3_SPEC;
impl crate::RegisterSpec for GPIOD_HWCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiod_hwcfgr3::R](R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOD_HWCFGR3 to value 0"]
impl crate::Resettable for GPIOD_HWCFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
