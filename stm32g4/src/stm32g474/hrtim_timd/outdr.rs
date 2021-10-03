#[doc = "Register `OUTDR` reader"]
pub struct R(crate::R<OUTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTDR` writer"]
pub struct W(crate::W<OUTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OUTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry"]
pub struct DIDL2_R(crate::FieldReader<bool, bool>);
impl DIDL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIDL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIDL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry"]
pub struct DIDL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIDL2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CHP2` reader - Output 2 Chopper enable"]
pub struct CHP2_R(crate::FieldReader<bool, bool>);
impl CHP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHP2` writer - Output 2 Chopper enable"]
pub struct CHP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `FAULT2` reader - Output 2 Fault state"]
pub struct FAULT2_R(crate::FieldReader<u8, u8>);
impl FAULT2_R {
    pub(crate) fn new(bits: u8) -> Self {
        FAULT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT2` writer - Output 2 Fault state"]
pub struct FAULT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `IDLES2` reader - Output 2 Idle State"]
pub struct IDLES2_R(crate::FieldReader<bool, bool>);
impl IDLES2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLES2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLES2` writer - Output 2 Idle State"]
pub struct IDLES2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLES2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `IDLEM2` reader - Output 2 Idle mode"]
pub struct IDLEM2_R(crate::FieldReader<bool, bool>);
impl IDLEM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLEM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLEM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLEM2` writer - Output 2 Idle mode"]
pub struct IDLEM2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEM2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `POL2` reader - Output 2 polarity"]
pub struct POL2_R(crate::FieldReader<bool, bool>);
impl POL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL2` writer - Output 2 polarity"]
pub struct POL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `BIAR` reader - Balanced Idle Automatic Resume"]
pub struct BIAR_R(crate::FieldReader<bool, bool>);
impl BIAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAR` writer - Balanced Idle Automatic Resume"]
pub struct BIAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `DLYPRT` reader - Delayed Protection"]
pub struct DLYPRT_R(crate::FieldReader<u8, u8>);
impl DLYPRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYPRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYPRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYPRT` writer - Delayed Protection"]
pub struct DLYPRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `DLYPRTEN` reader - Delayed Protection Enable"]
pub struct DLYPRTEN_R(crate::FieldReader<bool, bool>);
impl DLYPRTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLYPRTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYPRTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYPRTEN` writer - Delayed Protection Enable"]
pub struct DLYPRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DTEN` reader - Deadtime enable"]
pub struct DTEN_R(crate::FieldReader<bool, bool>);
impl DTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN` writer - Deadtime enable"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry"]
pub struct DIDL1_R(crate::FieldReader<bool, bool>);
impl DIDL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIDL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIDL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry"]
pub struct DIDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIDL1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CHP1` reader - Output 1 Chopper enable"]
pub struct CHP1_R(crate::FieldReader<bool, bool>);
impl CHP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHP1` writer - Output 1 Chopper enable"]
pub struct CHP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FAULT1` reader - Output 1 Fault state"]
pub struct FAULT1_R(crate::FieldReader<u8, u8>);
impl FAULT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        FAULT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1` writer - Output 1 Fault state"]
pub struct FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `IDLES1` reader - Output 1 Idle State"]
pub struct IDLES1_R(crate::FieldReader<bool, bool>);
impl IDLES1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLES1` writer - Output 1 Idle State"]
pub struct IDLES1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLES1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `IDLEM1` reader - Output 1 Idle mode"]
pub struct IDLEM1_R(crate::FieldReader<bool, bool>);
impl IDLEM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLEM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLEM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLEM1` writer - Output 1 Idle mode"]
pub struct IDLEM1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEM1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `POL1` reader - Output 1 polarity"]
pub struct POL1_R(crate::FieldReader<bool, bool>);
impl POL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL1` writer - Output 1 polarity"]
pub struct POL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Balanced Idle Automatic Resume"]
    #[inline(always)]
    pub fn biar(&self) -> BIAR_R {
        BIAR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&mut self) -> DIDL2_W {
        DIDL2_W { w: self }
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&mut self) -> CHP2_W {
        CHP2_W { w: self }
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&mut self) -> FAULT2_W {
        FAULT2_W { w: self }
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&mut self) -> IDLES2_W {
        IDLES2_W { w: self }
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&mut self) -> IDLEM2_W {
        IDLEM2_W { w: self }
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W {
        POL2_W { w: self }
    }
    #[doc = "Bit 14 - Balanced Idle Automatic Resume"]
    #[inline(always)]
    pub fn biar(&mut self) -> BIAR_W {
        BIAR_W { w: self }
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&mut self) -> DLYPRT_W {
        DLYPRT_W { w: self }
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W {
        DLYPRTEN_W { w: self }
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&mut self) -> DIDL1_W {
        DIDL1_W { w: self }
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&mut self) -> CHP1_W {
        CHP1_W { w: self }
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&mut self) -> IDLES1_W {
        IDLES1_W { w: self }
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&mut self) -> IDLEM1_W {
        IDLEM1_W { w: self }
    }
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W {
        POL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdr](index.html) module"]
pub struct OUTDR_SPEC;
impl crate::RegisterSpec for OUTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdr::R](R) reader structure"]
impl crate::Readable for OUTDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outdr::W](W) writer structure"]
impl crate::Writable for OUTDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTDR to value 0"]
impl crate::Resettable for OUTDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
