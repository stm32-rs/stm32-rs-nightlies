#[doc = "Register `DDRPHYC_ZQ0SR0` reader"]
pub struct R(crate::R<DDRPHYC_ZQ0SR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ZQ0SR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ZQ0SR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ZQ0SR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZCTRL` reader - ZCTRL"]
pub struct ZCTRL_R(crate::FieldReader<u32, u32>);
impl ZCTRL_R {
    pub(crate) fn new(bits: u32) -> Self {
        ZCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZCTRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZERR` reader - ZERR"]
pub struct ZERR_R(crate::FieldReader<bool, bool>);
impl ZERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZDONE` reader - ZDONE"]
pub struct ZDONE_R(crate::FieldReader<bool, bool>);
impl ZDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19 - ZCTRL"]
    #[inline(always)]
    pub fn zctrl(&self) -> ZCTRL_R {
        ZCTRL_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 30 - ZERR"]
    #[inline(always)]
    pub fn zerr(&self) -> ZERR_R {
        ZERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ZDONE"]
    #[inline(always)]
    pub fn zdone(&self) -> ZDONE_R {
        ZDONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "DDRPHYC ZQ0S register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr0](index.html) module"]
pub struct DDRPHYC_ZQ0SR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_ZQ0SR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_zq0sr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0SR0 to value 0x014a"]
impl crate::Resettable for DDRPHYC_ZQ0SR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x014a
    }
}
