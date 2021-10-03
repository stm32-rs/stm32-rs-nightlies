#[doc = "Register `LPTIM1_HWCFGR` reader"]
pub struct R(crate::R<LPTIM1_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM1_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM1_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM1_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFG1` reader - CFG1"]
pub struct CFG1_R(crate::FieldReader<u8, u8>);
impl CFG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG2` reader - CFG2"]
pub struct CFG2_R(crate::FieldReader<u8, u8>);
impl CFG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG3` reader - CFG3"]
pub struct CFG3_R(crate::FieldReader<u8, u8>);
impl CFG3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG4` reader - CFG4"]
pub struct CFG4_R(crate::FieldReader<u8, u8>);
impl CFG4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "LPTIM 1 peripheral hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim1_hwcfgr](index.html) module"]
pub struct LPTIM1_HWCFGR_SPEC;
impl crate::RegisterSpec for LPTIM1_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim1_hwcfgr::R](R) reader structure"]
impl crate::Readable for LPTIM1_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPTIM1_HWCFGR to value 0x0001_0804"]
impl crate::Resettable for LPTIM1_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0804
    }
}
