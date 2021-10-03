#[doc = "Register `CLIDR` reader"]
pub struct R(crate::R<CLIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CL1` reader - CL1"]
pub struct CL1_R(crate::FieldReader<u8, u8>);
impl CL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL2` reader - CL2"]
pub struct CL2_R(crate::FieldReader<u8, u8>);
impl CL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL3` reader - CL3"]
pub struct CL3_R(crate::FieldReader<u8, u8>);
impl CL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL4` reader - CL4"]
pub struct CL4_R(crate::FieldReader<u8, u8>);
impl CL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL5` reader - CL5"]
pub struct CL5_R(crate::FieldReader<u8, u8>);
impl CL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL6` reader - CL6"]
pub struct CL6_R(crate::FieldReader<u8, u8>);
impl CL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CL7` reader - CL7"]
pub struct CL7_R(crate::FieldReader<u8, u8>);
impl CL7_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CL7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoUIS` reader - LoUIS"]
pub struct LOUIS_R(crate::FieldReader<u8, u8>);
impl LOUIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOUIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOUIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoC` reader - LoC"]
pub struct LOC_R(crate::FieldReader<u8, u8>);
impl LOC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoU` reader - LoU"]
pub struct LOU_R(crate::FieldReader<u8, u8>);
impl LOU_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - CL1"]
    #[inline(always)]
    pub fn cl1(&self) -> CL1_R {
        CL1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - CL2"]
    #[inline(always)]
    pub fn cl2(&self) -> CL2_R {
        CL2_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - CL3"]
    #[inline(always)]
    pub fn cl3(&self) -> CL3_R {
        CL3_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - CL4"]
    #[inline(always)]
    pub fn cl4(&self) -> CL4_R {
        CL4_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - CL5"]
    #[inline(always)]
    pub fn cl5(&self) -> CL5_R {
        CL5_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - CL6"]
    #[inline(always)]
    pub fn cl6(&self) -> CL6_R {
        CL6_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - CL7"]
    #[inline(always)]
    pub fn cl7(&self) -> CL7_R {
        CL7_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - LoUIS"]
    #[inline(always)]
    pub fn lo_uis(&self) -> LOUIS_R {
        LOUIS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - LoC"]
    #[inline(always)]
    pub fn lo_c(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29 - LoU"]
    #[inline(always)]
    pub fn lo_u(&self) -> LOU_R {
        LOU_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
#[doc = "Cache Level ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clidr](index.html) module"]
pub struct CLIDR_SPEC;
impl crate::RegisterSpec for CLIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clidr::R](R) reader structure"]
impl crate::Readable for CLIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLIDR to value 0x0900_0003"]
impl crate::Resettable for CLIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0900_0003
    }
}
