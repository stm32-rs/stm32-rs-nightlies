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
#[doc = "Field `TXIS` reader - TXIS"]
pub struct TXIS_R(crate::FieldReader<bool, bool>);
impl TXIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGDISC` reader - TXMSGDISC"]
pub struct TXMSGDISC_R(crate::FieldReader<bool, bool>);
impl TXMSGDISC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGDISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGDISC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGSENT` reader - TXMSGSENT"]
pub struct TXMSGSENT_R(crate::FieldReader<bool, bool>);
impl TXMSGSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGSENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGSENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGABT` reader - TXMSGABT"]
pub struct TXMSGABT_R(crate::FieldReader<bool, bool>);
impl TXMSGABT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGABT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGABT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRSTDISC` reader - HRSTDISC"]
pub struct HRSTDISC_R(crate::FieldReader<bool, bool>);
impl HRSTDISC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTDISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRSTDISC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRSTSENT` reader - HRSTSENT"]
pub struct HRSTSENT_R(crate::FieldReader<bool, bool>);
impl HRSTSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTSENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRSTSENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUND` reader - TXUND"]
pub struct TXUND_R(crate::FieldReader<bool, bool>);
impl TXUND_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNE` reader - RXNE"]
pub struct RXNE_R(crate::FieldReader<bool, bool>);
impl RXNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXORDDET` reader - RXORDDET"]
pub struct RXORDDET_R(crate::FieldReader<bool, bool>);
impl RXORDDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXORDDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORDDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXHRSTDET` reader - RXHRSTDET"]
pub struct RXHRSTDET_R(crate::FieldReader<bool, bool>);
impl RXHRSTDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXHRSTDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXHRSTDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVR` reader - RXOVR"]
pub struct RXOVR_R(crate::FieldReader<bool, bool>);
impl RXOVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMSGEND` reader - RXMSGEND"]
pub struct RXMSGEND_R(crate::FieldReader<bool, bool>);
impl RXMSGEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMSGEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMSGEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERR` reader - RXERR"]
pub struct RXERR_R(crate::FieldReader<bool, bool>);
impl RXERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPECEVT1` reader - TYPECEVT1"]
pub struct TYPECEVT1_R(crate::FieldReader<bool, bool>);
impl TYPECEVT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPECEVT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPECEVT2` reader - TYPECEVT2"]
pub struct TYPECEVT2_R(crate::FieldReader<bool, bool>);
impl TYPECEVT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPECEVT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPEC_VSTATE_CC1` reader - TYPEC_VSTATE_CC1"]
pub struct TYPEC_VSTATE_CC1_R(crate::FieldReader<u8, u8>);
impl TYPEC_VSTATE_CC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TYPEC_VSTATE_CC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPEC_VSTATE_CC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPEC_VSTATE_CC2` reader - TYPEC_VSTATE_CC2"]
pub struct TYPEC_VSTATE_CC2_R(crate::FieldReader<u8, u8>);
impl TYPEC_VSTATE_CC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TYPEC_VSTATE_CC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPEC_VSTATE_CC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSEVT` reader - FRSEVT"]
pub struct FRSEVT_R(crate::FieldReader<bool, bool>);
impl FRSEVT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSEVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSEVT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TXIS"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISC"]
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENT"]
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXMSGABT"]
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRSTDISC"]
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRSTSENT"]
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXUND"]
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXORDDET"]
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDET"]
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RXMSGEND"]
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RXERR"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1"]
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2"]
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - FRSEVT"]
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
#[doc = "UCPD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
