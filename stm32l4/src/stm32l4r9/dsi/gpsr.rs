#[doc = "Register `GPSR` reader"]
pub struct R(crate::R<GPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDFE` reader - Command FIFO Empty"]
pub struct CMDFE_R(crate::FieldReader<bool, bool>);
impl CMDFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDFF` reader - Command FIFO Full"]
pub struct CMDFF_R(crate::FieldReader<bool, bool>);
impl CMDFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRFE` reader - Payload Write FIFO Empty"]
pub struct PWRFE_R(crate::FieldReader<bool, bool>);
impl PWRFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRFF` reader - Payload Write FIFO Full"]
pub struct PWRFF_R(crate::FieldReader<bool, bool>);
impl PWRFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDFE` reader - Payload Read FIFO Empty"]
pub struct PRDFE_R(crate::FieldReader<bool, bool>);
impl PRDFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRDFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRDFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDFF` reader - Payload Read FIFO Full"]
pub struct PRDFF_R(crate::FieldReader<bool, bool>);
impl PRDFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRDFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRDFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB` reader - Read Command Busy"]
pub struct RCB_R(crate::FieldReader<bool, bool>);
impl RCB_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Command FIFO Empty"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command FIFO Full"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Payload Write FIFO Empty"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Payload Write FIFO Full"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Payload Read FIFO Empty"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Payload Read FIFO Full"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read Command Busy"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "DSI Host Generic Packet Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpsr](index.html) module"]
pub struct GPSR_SPEC;
impl crate::RegisterSpec for GPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpsr::R](R) reader structure"]
impl crate::Readable for GPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPSR to value 0x15"]
impl crate::Resettable for GPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
