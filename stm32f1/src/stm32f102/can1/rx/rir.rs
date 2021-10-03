#[doc = "Register `RIR` reader"]
pub struct R(crate::R<RIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STID` reader - STID"]
pub struct STID_R(crate::FieldReader<u16, u16>);
impl STID_R {
    pub(crate) fn new(bits: u16) -> Self {
        STID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXID` reader - EXID"]
pub struct EXID_R(crate::FieldReader<u32, u32>);
impl EXID_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDE` reader - IDE"]
pub struct IDE_R(crate::FieldReader<bool, bool>);
impl IDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTR` reader - RTR"]
pub struct RTR_R(crate::FieldReader<bool, bool>);
impl RTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "CAN_RI0R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rir](index.html) module"]
pub struct RIR_SPEC;
impl crate::RegisterSpec for RIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rir::R](R) reader structure"]
impl crate::Readable for RIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIR to value 0"]
impl crate::Resettable for RIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
