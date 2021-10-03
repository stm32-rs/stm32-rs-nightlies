#[doc = "Register `MACRxTxSR` reader"]
pub struct R(crate::R<MACRXTXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRXTXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRXTXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRXTXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub struct TJT_R(crate::FieldReader<bool, bool>);
impl TJT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TJT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TJT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCARR` reader - No Carrier"]
pub struct NCARR_R(crate::FieldReader<bool, bool>);
impl NCARR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NCARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCARR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCARR` reader - Loss of Carrier"]
pub struct LCARR_R(crate::FieldReader<bool, bool>);
impl LCARR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCARR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXDEF` reader - Excessive Deferral"]
pub struct EXDEF_R(crate::FieldReader<bool, bool>);
impl EXDEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXDEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCOL` reader - Late Collision"]
pub struct LCOL_R(crate::FieldReader<bool, bool>);
impl LCOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCOL` reader - Excessive Collisions"]
pub struct EXCOL_R(crate::FieldReader<bool, bool>);
impl EXCOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub struct RWT_R(crate::FieldReader<bool, bool>);
impl RWT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - No Carrier"]
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loss of Carrier"]
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Excessive Deferral"]
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Late Collision"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Rx Tx status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macrx_tx_sr](index.html) module"]
pub struct MACRXTXSR_SPEC;
impl crate::RegisterSpec for MACRXTXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macrx_tx_sr::R](R) reader structure"]
impl crate::Readable for MACRXTXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACRxTxSR to value 0"]
impl crate::Resettable for MACRXTXSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
