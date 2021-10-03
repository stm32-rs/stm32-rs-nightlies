#[doc = "Register `BOOT_CURR` reader"]
pub struct R(crate::R<BOOT_CURR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_CURR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_CURR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_CURR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BOOT_ADD0` reader - Boot address 0"]
pub struct BOOT_ADD0_R(crate::FieldReader<u16, u16>);
impl BOOT_ADD0_R {
    pub(crate) fn new(bits: u16) -> Self {
        BOOT_ADD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_ADD0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_ADD1` reader - Boot address 1"]
pub struct BOOT_ADD1_R(crate::FieldReader<u16, u16>);
impl BOOT_ADD1_R {
    pub(crate) fn new(bits: u16) -> Self {
        BOOT_ADD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_ADD1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FLASH register with boot address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_curr](index.html) module"]
pub struct BOOT_CURR_SPEC;
impl crate::RegisterSpec for BOOT_CURR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_curr::R](R) reader structure"]
impl crate::Readable for BOOT_CURR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BOOT_CURR to value 0"]
impl crate::Resettable for BOOT_CURR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
