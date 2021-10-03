#[doc = "Register `FNR` reader"]
pub struct R(crate::R<FNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FN` reader - Frame number"]
pub struct FN_R(crate::FieldReader<u16, u16>);
impl FN_R {
    pub(crate) fn new(bits: u16) -> Self {
        FN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSOF` reader - Lost SOF"]
pub struct LSOF_R(crate::FieldReader<u8, u8>);
impl LSOF_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSOF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Locked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCK_A {
    #[doc = "1: the frame timer remains in this state until an USB reset or USB suspend event occurs"]
    LOCKED = 1,
}
impl From<LCK_A> for bool {
    #[inline(always)]
    fn from(variant: LCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK` reader - Locked"]
pub struct LCK_R(crate::FieldReader<bool, LCK_A>);
impl LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCK_A> {
        match self.bits {
            true => Some(LCK_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LCK_A::LOCKED
    }
}
impl core::ops::Deref for LCK_R {
    type Target = crate::FieldReader<bool, LCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive data - line status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDM_A {
    #[doc = "1: received data minus upstream port data line"]
    RECEIVED = 1,
}
impl From<RXDM_A> for bool {
    #[inline(always)]
    fn from(variant: RXDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDM` reader - Receive data - line status"]
pub struct RXDM_R(crate::FieldReader<bool, RXDM_A>);
impl RXDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXDM_A> {
        match self.bits {
            true => Some(RXDM_A::RECEIVED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        **self == RXDM_A::RECEIVED
    }
}
impl core::ops::Deref for RXDM_R {
    type Target = crate::FieldReader<bool, RXDM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive data + line status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDP_A {
    #[doc = "1: received data plus upstream port data line"]
    RECEIVED = 1,
}
impl From<RXDP_A> for bool {
    #[inline(always)]
    fn from(variant: RXDP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDP` reader - Receive data + line status"]
pub struct RXDP_R(crate::FieldReader<bool, RXDP_A>);
impl RXDP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXDP_A> {
        match self.bits {
            true => Some(RXDP_A::RECEIVED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        **self == RXDP_A::RECEIVED
    }
}
impl core::ops::Deref for RXDP_R {
    type Target = crate::FieldReader<bool, RXDP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - Lost SOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Locked"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive data - line status"]
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive data + line status"]
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "frame number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnr](index.html) module"]
pub struct FNR_SPEC;
impl crate::RegisterSpec for FNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fnr::R](R) reader structure"]
impl crate::Readable for FNR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FNR to value 0"]
impl crate::Resettable for FNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
