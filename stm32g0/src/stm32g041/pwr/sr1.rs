#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WUF1` reader - Wakeup flag 1"]
pub struct WUF1_R(crate::FieldReader<bool, bool>);
impl WUF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF2` reader - Wakeup flag 2"]
pub struct WUF2_R(crate::FieldReader<bool, bool>);
impl WUF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF4` reader - Wakeup flag 4"]
pub struct WUF4_R(crate::FieldReader<bool, bool>);
impl WUF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF5` reader - Wakeup flag 5"]
pub struct WUF5_R(crate::FieldReader<bool, bool>);
impl WUF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF6` reader - Wakeup flag 6"]
pub struct WUF6_R(crate::FieldReader<bool, bool>);
impl WUF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBF` reader - Standby flag"]
pub struct SBF_R(crate::FieldReader<bool, bool>);
impl SBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUFI` reader - Wakeup flag internal"]
pub struct WUFI_R(crate::FieldReader<bool, bool>);
impl WUFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag 6"]
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Power status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
