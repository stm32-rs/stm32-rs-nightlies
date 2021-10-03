#[doc = "Register `HWCFGR1` reader"]
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
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
#[doc = "Field `CFG5` reader - CFG5"]
pub struct CFG5_R(crate::FieldReader<u8, u8>);
impl CFG5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG6` reader - CFG6"]
pub struct CFG6_R(crate::FieldReader<u8, u8>);
impl CFG6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG7` reader - CFG7"]
pub struct CFG7_R(crate::FieldReader<u8, u8>);
impl CFG7_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG8` reader - CFG8"]
pub struct CFG8_R(crate::FieldReader<u8, u8>);
impl CFG8_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG5"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG6"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG7"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG8"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "USART Hardware Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](index.html) module"]
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr1::R](R) reader structure"]
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR1 to value 0x14"]
impl crate::Resettable for HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
