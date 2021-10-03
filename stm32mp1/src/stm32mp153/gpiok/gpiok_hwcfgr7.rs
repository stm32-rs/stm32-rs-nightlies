#[doc = "Register `GPIOK_HWCFGR7` reader"]
pub struct R(crate::R<GPIOK_HWCFGR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOK_HWCFGR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOK_HWCFGR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOK_HWCFGR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AF_PRIO0` reader - AF_PRIO0"]
pub struct AF_PRIO0_R(crate::FieldReader<u8, u8>);
impl AF_PRIO0_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO1` reader - AF_PRIO1"]
pub struct AF_PRIO1_R(crate::FieldReader<u8, u8>);
impl AF_PRIO1_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO2` reader - AF_PRIO2"]
pub struct AF_PRIO2_R(crate::FieldReader<u8, u8>);
impl AF_PRIO2_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO3` reader - AF_PRIO3"]
pub struct AF_PRIO3_R(crate::FieldReader<u8, u8>);
impl AF_PRIO3_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO4` reader - AF_PRIO4"]
pub struct AF_PRIO4_R(crate::FieldReader<u8, u8>);
impl AF_PRIO4_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO5` reader - AF_PRIO5"]
pub struct AF_PRIO5_R(crate::FieldReader<u8, u8>);
impl AF_PRIO5_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO6` reader - AF_PRIO6"]
pub struct AF_PRIO6_R(crate::FieldReader<u8, u8>);
impl AF_PRIO6_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO7` reader - AF_PRIO7"]
pub struct AF_PRIO7_R(crate::FieldReader<u8, u8>);
impl AF_PRIO7_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - AF_PRIO0"]
    #[inline(always)]
    pub fn af_prio0(&self) -> AF_PRIO0_R {
        AF_PRIO0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_PRIO1"]
    #[inline(always)]
    pub fn af_prio1(&self) -> AF_PRIO1_R {
        AF_PRIO1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AF_PRIO2"]
    #[inline(always)]
    pub fn af_prio2(&self) -> AF_PRIO2_R {
        AF_PRIO2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AF_PRIO3"]
    #[inline(always)]
    pub fn af_prio3(&self) -> AF_PRIO3_R {
        AF_PRIO3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AF_PRIO4"]
    #[inline(always)]
    pub fn af_prio4(&self) -> AF_PRIO4_R {
        AF_PRIO4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AF_PRIO5"]
    #[inline(always)]
    pub fn af_prio5(&self) -> AF_PRIO5_R {
        AF_PRIO5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AF_PRIO6"]
    #[inline(always)]
    pub fn af_prio6(&self) -> AF_PRIO6_R {
        AF_PRIO6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AF_PRIO7"]
    #[inline(always)]
    pub fn af_prio7(&self) -> AF_PRIO7_R {
        AF_PRIO7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr7](index.html) module"]
pub struct GPIOK_HWCFGR7_SPEC;
impl crate::RegisterSpec for GPIOK_HWCFGR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiok_hwcfgr7::R](R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOK_HWCFGR7 to value 0xffff_ffff"]
impl crate::Resettable for GPIOK_HWCFGR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
