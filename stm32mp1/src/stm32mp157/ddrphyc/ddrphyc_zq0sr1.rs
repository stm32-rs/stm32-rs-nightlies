#[doc = "Register `DDRPHYC_ZQ0SR1` reader"]
pub struct R(crate::R<DDRPHYC_ZQ0SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ZQ0SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ZQ0SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ZQ0SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZPD` reader - ZPD"]
pub struct ZPD_R(crate::FieldReader<u8, u8>);
impl ZPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPU` reader - ZPU"]
pub struct ZPU_R(crate::FieldReader<u8, u8>);
impl ZPU_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZPU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPD` reader - OPD"]
pub struct OPD_R(crate::FieldReader<u8, u8>);
impl OPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPU` reader - OPU"]
pub struct OPU_R(crate::FieldReader<u8, u8>);
impl OPU_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - ZPD"]
    #[inline(always)]
    pub fn zpd(&self) -> ZPD_R {
        ZPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ZPU"]
    #[inline(always)]
    pub fn zpu(&self) -> ZPU_R {
        ZPU_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - OPD"]
    #[inline(always)]
    pub fn opd(&self) -> OPD_R {
        OPD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - OPU"]
    #[inline(always)]
    pub fn opu(&self) -> OPU_R {
        OPU_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "DDRPHYC ZQ0S register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr1](index.html) module"]
pub struct DDRPHYC_ZQ0SR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_ZQ0SR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddrphyc_zq0sr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0SR1 to value 0"]
impl crate::Resettable for DDRPHYC_ZQ0SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
