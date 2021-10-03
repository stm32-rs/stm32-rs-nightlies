#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PD` reader - PHY Direction"]
pub struct PD_R(crate::FieldReader<bool, bool>);
impl PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSSC` reader - PHY Stop State Clock lane"]
pub struct PSSC_R(crate::FieldReader<bool, bool>);
impl PSSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UANC` reader - ULPS Active Not Clock lane"]
pub struct UANC_R(crate::FieldReader<bool, bool>);
impl UANC_R {
    pub(crate) fn new(bits: bool) -> Self {
        UANC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UANC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS0` reader - PHY Stop State lane 0"]
pub struct PSS0_R(crate::FieldReader<bool, bool>);
impl PSS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UAN0` reader - ULPS Active Not lane 1"]
pub struct UAN0_R(crate::FieldReader<bool, bool>);
impl UAN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UAN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UAN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUE0` reader - RX ULPS Escape lane 0"]
pub struct RUE0_R(crate::FieldReader<bool, bool>);
impl RUE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS1` reader - PHY Stop State lane 1"]
pub struct PSS1_R(crate::FieldReader<bool, bool>);
impl PSS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UAN1` reader - ULPS Active Not lane 1"]
pub struct UAN1_R(crate::FieldReader<bool, bool>);
impl UAN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UAN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - PHY Direction"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PHY Stop State Clock lane"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ULPS Active Not Clock lane"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHY Stop State lane 0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX ULPS Escape lane 0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PHY Stop State lane 1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "DSI Host PHY Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
