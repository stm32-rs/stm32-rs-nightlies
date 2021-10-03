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
#[doc = "Field `ALRAF` reader - ALRAF"]
pub struct ALRAF_R(crate::FieldReader<bool, bool>);
impl ALRAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBF` reader - ALRBF"]
pub struct ALRBF_R(crate::FieldReader<bool, bool>);
impl ALRBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTF` reader - WUTF"]
pub struct WUTF_R(crate::FieldReader<bool, bool>);
impl WUTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSF` reader - TSF"]
pub struct TSF_R(crate::FieldReader<bool, bool>);
impl TSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOVF` reader - TSOVF"]
pub struct TSOVF_R(crate::FieldReader<bool, bool>);
impl TSOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITSF` reader - ITSF"]
pub struct ITSF_R(crate::FieldReader<bool, bool>);
impl ITSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ALRAF"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBF"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTF"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TSOVF"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ITSF"]
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
