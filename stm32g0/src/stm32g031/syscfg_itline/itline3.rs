#[doc = "Register `ITLINE3` reader"]
pub struct R(crate::R<ITLINE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLASH_ITF` reader - FLASH_ITF"]
pub struct FLASH_ITF_R(crate::FieldReader<bool, bool>);
impl FLASH_ITF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_ITF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ITF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_ECC` reader - FLASH_ECC"]
pub struct FLASH_ECC_R(crate::FieldReader<bool, bool>);
impl FLASH_ECC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ECC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - FLASH_ITF"]
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLASH_ECC"]
    #[inline(always)]
    pub fn flash_ecc(&self) -> FLASH_ECC_R {
        FLASH_ECC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "interrupt line 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline3](index.html) module"]
pub struct ITLINE3_SPEC;
impl crate::RegisterSpec for ITLINE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline3::R](R) reader structure"]
impl crate::Readable for ITLINE3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE3 to value 0"]
impl crate::Resettable for ITLINE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
