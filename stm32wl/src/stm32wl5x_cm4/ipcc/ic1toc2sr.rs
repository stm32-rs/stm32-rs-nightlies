#[doc = "Register `IC1TOC2SR` reader"]
pub struct R(crate::R<IC1TOC2SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC1TOC2SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC1TOC2SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC1TOC2SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH1F` reader - CH1F"]
pub struct CH1F_R(crate::FieldReader<bool, bool>);
impl CH1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2F` reader - CH2F"]
pub struct CH2F_R(crate::FieldReader<bool, bool>);
impl CH2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3F` reader - CH3F"]
pub struct CH3F_R(crate::FieldReader<bool, bool>);
impl CH3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4F` reader - CH4F"]
pub struct CH4F_R(crate::FieldReader<bool, bool>);
impl CH4F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5F` reader - CH5F"]
pub struct CH5F_R(crate::FieldReader<bool, bool>);
impl CH5F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6F` reader - CH6F"]
pub struct CH6F_R(crate::FieldReader<bool, bool>);
impl CH6F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH3F"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH4F"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH5F"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH6F"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "IPCC processor 1 to processor 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1toc2sr](index.html) module"]
pub struct IC1TOC2SR_SPEC;
impl crate::RegisterSpec for IC1TOC2SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic1toc2sr::R](R) reader structure"]
impl crate::Readable for IC1TOC2SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC1TOC2SR to value 0"]
impl crate::Resettable for IC1TOC2SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
