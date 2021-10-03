#[doc = "Register `PRAR_CUR` reader"]
pub struct R(crate::R<PRAR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRAR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRAR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRAR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROT_AREA_START` reader - Bank 1 lowest PCROP protected address"]
pub struct PROT_AREA_START_R(crate::FieldReader<u16, u16>);
impl PROT_AREA_START_R {
    pub(crate) fn new(bits: u16) -> Self {
        PROT_AREA_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_AREA_START_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT_AREA_END` reader - Bank 1 highest PCROP protected address"]
pub struct PROT_AREA_END_R(crate::FieldReader<u16, u16>);
impl PROT_AREA_END_R {
    pub(crate) fn new(bits: u16) -> Self {
        PROT_AREA_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_AREA_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEP` reader - Bank 1 PCROP protected erase enable option status bit"]
pub struct DMEP_R(crate::FieldReader<bool, bool>);
impl DMEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "FLASH protection address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prar_cur](index.html) module"]
pub struct PRAR_CUR_SPEC;
impl crate::RegisterSpec for PRAR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prar_cur::R](R) reader structure"]
impl crate::Readable for PRAR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRAR_CUR to value 0"]
impl crate::Resettable for PRAR_CUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
