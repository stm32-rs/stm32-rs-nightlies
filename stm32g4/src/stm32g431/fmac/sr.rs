#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `YEMPTY` reader - YEMPTY"]
pub struct YEMPTY_R(crate::FieldReader<bool, bool>);
impl YEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        YEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X1FULL` reader - X1FULL"]
pub struct X1FULL_R(crate::FieldReader<bool, bool>);
impl X1FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        X1FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X1FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFL` reader - OVFL"]
pub struct OVFL_R(crate::FieldReader<bool, bool>);
impl OVFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNFL` reader - UNFL"]
pub struct UNFL_R(crate::FieldReader<bool, bool>);
impl UNFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAT` reader - SAT"]
pub struct SAT_R(crate::FieldReader<bool, bool>);
impl SAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - YEMPTY"]
    #[inline(always)]
    pub fn yempty(&self) -> YEMPTY_R {
        YEMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - X1FULL"]
    #[inline(always)]
    pub fn x1full(&self) -> X1FULL_R {
        X1FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OVFL"]
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UNFL"]
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SAT"]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "FMAC Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
