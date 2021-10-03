#[doc = "Register `STGENR_PIDR2` reader"]
pub struct R(crate::R<STGENR_PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DES_1` reader - DES_1"]
pub struct DES_1_R(crate::FieldReader<u8, u8>);
impl DES_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DES_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DES_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEDEC` reader - JEDEC"]
pub struct JEDEC_R(crate::FieldReader<bool, bool>);
impl JEDEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEDEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` reader - REVISION"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - DES_1"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - JEDEC"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - REVISION"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENR peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr2](index.html) module"]
pub struct STGENR_PIDR2_SPEC;
impl crate::RegisterSpec for STGENR_PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_pidr2::R](R) reader structure"]
impl crate::Readable for STGENR_PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_PIDR2 to value 0x1b"]
impl crate::Resettable for STGENR_PIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1b
    }
}
