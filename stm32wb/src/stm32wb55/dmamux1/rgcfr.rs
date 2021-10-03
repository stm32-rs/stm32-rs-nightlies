#[doc = "Register `RGCFR` reader"]
pub struct R(crate::R<RGCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSOF0` reader - Generator Clear Overrun Flag 0"]
pub struct CSOF0_R(crate::FieldReader<bool, bool>);
impl CSOF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF1` reader - Generator Clear Overrun Flag 1"]
pub struct CSOF1_R(crate::FieldReader<bool, bool>);
impl CSOF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF2` reader - Generator Clear Overrun Flag 2"]
pub struct CSOF2_R(crate::FieldReader<bool, bool>);
impl CSOF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF3` reader - Generator Clear Overrun Flag 3"]
pub struct CSOF3_R(crate::FieldReader<bool, bool>);
impl CSOF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Generator Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&self) -> CSOF0_R {
        CSOF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Generator Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&self) -> CSOF1_R {
        CSOF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generator Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&self) -> CSOF2_R {
        CSOF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generator Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&self) -> CSOF3_R {
        CSOF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "DMA Request Generator Clear Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcfr](index.html) module"]
pub struct RGCFR_SPEC;
impl crate::RegisterSpec for RGCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgcfr::R](R) reader structure"]
impl crate::Readable for RGCFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
