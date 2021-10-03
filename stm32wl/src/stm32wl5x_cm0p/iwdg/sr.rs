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
#[doc = "Watchdog counter window value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WVU_A {
    #[doc = "0: No update on-going"]
    IDLE = 0,
    #[doc = "1: Update on-going"]
    BUSY = 1,
}
impl From<WVU_A> for bool {
    #[inline(always)]
    fn from(variant: WVU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WVU` reader - Watchdog counter window value update"]
pub struct WVU_R(crate::FieldReader<bool, WVU_A>);
impl WVU_R {
    pub(crate) fn new(bits: bool) -> Self {
        WVU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WVU_A {
        match self.bits {
            false => WVU_A::IDLE,
            true => WVU_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == WVU_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == WVU_A::BUSY
    }
}
impl core::ops::Deref for WVU_R {
    type Target = crate::FieldReader<bool, WVU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog counter reload value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RVU_A {
    #[doc = "0: No update on-going"]
    IDLE = 0,
    #[doc = "1: Update on-going"]
    BUSY = 1,
}
impl From<RVU_A> for bool {
    #[inline(always)]
    fn from(variant: RVU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RVU` reader - Watchdog counter reload value update"]
pub struct RVU_R(crate::FieldReader<bool, RVU_A>);
impl RVU_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RVU_A {
        match self.bits {
            false => RVU_A::IDLE,
            true => RVU_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == RVU_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == RVU_A::BUSY
    }
}
impl core::ops::Deref for RVU_R {
    type Target = crate::FieldReader<bool, RVU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog prescaler value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVU_A {
    #[doc = "0: No update on-going"]
    IDLE = 0,
    #[doc = "1: Update on-going"]
    BUSY = 1,
}
impl From<PVU_A> for bool {
    #[inline(always)]
    fn from(variant: PVU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVU` reader - Watchdog prescaler value update"]
pub struct PVU_R(crate::FieldReader<bool, PVU_A>);
impl PVU_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVU_A {
        match self.bits {
            false => PVU_A::IDLE,
            true => PVU_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == PVU_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == PVU_A::BUSY
    }
}
impl core::ops::Deref for PVU_R {
    type Target = crate::FieldReader<bool, PVU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Watchdog counter window value update"]
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update"]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
