#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWPF` reader - Swap Timer F outputs"]
pub struct SWPF_R(crate::FieldReader<bool, bool>);
impl SWPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPF` writer - Swap Timer F outputs"]
pub struct SWPF_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SWPE` reader - Swap Timer E outputs"]
pub struct SWPE_R(crate::FieldReader<bool, bool>);
impl SWPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPE` writer - Swap Timer E outputs"]
pub struct SWPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SWPD` reader - Swap Timer D outputs"]
pub struct SWPD_R(crate::FieldReader<bool, bool>);
impl SWPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPD` writer - Swap Timer D outputs"]
pub struct SWPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPD_W<'a> {
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
#[doc = "Field `SWPC` reader - Swap Timer C outputs"]
pub struct SWPC_R(crate::FieldReader<bool, bool>);
impl SWPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPC` writer - Swap Timer C outputs"]
pub struct SWPC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPC_W<'a> {
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
#[doc = "Field `SWPB` reader - Swap Timer B outputs"]
pub struct SWPB_R(crate::FieldReader<bool, bool>);
impl SWPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPB` writer - Swap Timer B outputs"]
pub struct SWPB_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPB_W<'a> {
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
#[doc = "Field `SWPA` reader - Swap Timer A outputs"]
pub struct SWPA_R(crate::FieldReader<bool, bool>);
impl SWPA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPA` writer - Swap Timer A outputs"]
pub struct SWPA_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TFRST` reader - Timer f counter software reset"]
pub struct TFRST_R(crate::FieldReader<bool, bool>);
impl TFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFRST` writer - Timer f counter software reset"]
pub struct TFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TFRST_W<'a> {
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
#[doc = "Field `TERST` reader - Timer E counter software reset"]
pub struct TERST_R(crate::FieldReader<bool, bool>);
impl TERST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERST` writer - Timer E counter software reset"]
pub struct TERST_W<'a> {
    w: &'a mut W,
}
impl<'a> TERST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TDRST` reader - Timer D counter software reset"]
pub struct TDRST_R(crate::FieldReader<bool, bool>);
impl TDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDRST` writer - Timer D counter software reset"]
pub struct TDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TCRST` reader - Timer C counter software reset"]
pub struct TCRST_R(crate::FieldReader<bool, bool>);
impl TCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCRST` writer - Timer C counter software reset"]
pub struct TCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TBRST` reader - Timer B counter software reset"]
pub struct TBRST_R(crate::FieldReader<bool, bool>);
impl TBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBRST` writer - Timer B counter software reset"]
pub struct TBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TBRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TARST` reader - Timer A counter software reset"]
pub struct TARST_R(crate::FieldReader<bool, bool>);
impl TARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARST` writer - Timer A counter software reset"]
pub struct TARST_W<'a> {
    w: &'a mut W,
}
impl<'a> TARST_W<'a> {
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
#[doc = "Field `MRST` reader - Master Counter software reset"]
pub struct MRST_R(crate::FieldReader<bool, bool>);
impl MRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRST` writer - Master Counter software reset"]
pub struct MRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MRST_W<'a> {
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
#[doc = "Field `TFSWU` reader - Timer f Software Update"]
pub struct TFSWU_R(crate::FieldReader<bool, bool>);
impl TFSWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFSWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFSWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFSWU` writer - Timer f Software Update"]
pub struct TFSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TFSWU_W<'a> {
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
#[doc = "Field `TESWU` reader - Timer E Software Update"]
pub struct TESWU_R(crate::FieldReader<bool, bool>);
impl TESWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TESWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TESWU` writer - Timer E Software Update"]
pub struct TESWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TESWU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TDSWU` reader - Timer D Software Update"]
pub struct TDSWU_R(crate::FieldReader<bool, bool>);
impl TDSWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDSWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDSWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDSWU` writer - Timer D Software Update"]
pub struct TDSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TDSWU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TCSWU` reader - Timer C Software Update"]
pub struct TCSWU_R(crate::FieldReader<bool, bool>);
impl TCSWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCSWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCSWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCSWU` writer - Timer C Software Update"]
pub struct TCSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSWU_W<'a> {
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
#[doc = "Field `TBSWU` reader - Timer B Software Update"]
pub struct TBSWU_R(crate::FieldReader<bool, bool>);
impl TBSWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBSWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBSWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBSWU` writer - Timer B Software Update"]
pub struct TBSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSWU_W<'a> {
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
#[doc = "Field `TASWU` reader - Timer A Software update"]
pub struct TASWU_R(crate::FieldReader<bool, bool>);
impl TASWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TASWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TASWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TASWU` writer - Timer A Software update"]
pub struct TASWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TASWU_W<'a> {
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
#[doc = "Field `MSWU` reader - Master Timer Software update"]
pub struct MSWU_R(crate::FieldReader<bool, bool>);
impl MSWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSWU` writer - Master Timer Software update"]
pub struct MSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> MSWU_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - Swap Timer F outputs"]
    #[inline(always)]
    pub fn swpf(&self) -> SWPF_R {
        SWPF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Swap Timer E outputs"]
    #[inline(always)]
    pub fn swpe(&self) -> SWPE_R {
        SWPE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Swap Timer D outputs"]
    #[inline(always)]
    pub fn swpd(&self) -> SWPD_R {
        SWPD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Swap Timer C outputs"]
    #[inline(always)]
    pub fn swpc(&self) -> SWPC_R {
        SWPC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Swap Timer B outputs"]
    #[inline(always)]
    pub fn swpb(&self) -> SWPB_R {
        SWPB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Swap Timer A outputs"]
    #[inline(always)]
    pub fn swpa(&self) -> SWPA_R {
        SWPA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timer f counter software reset"]
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer f Software Update"]
    #[inline(always)]
    pub fn tfswu(&self) -> TFSWU_R {
        TFSWU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&self) -> TESWU_R {
        TESWU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&self) -> TDSWU_R {
        TDSWU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&self) -> TCSWU_R {
        TCSWU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&self) -> TBSWU_R {
        TBSWU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&self) -> TASWU_R {
        TASWU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&self) -> MSWU_R {
        MSWU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Swap Timer F outputs"]
    #[inline(always)]
    pub fn swpf(&mut self) -> SWPF_W {
        SWPF_W { w: self }
    }
    #[doc = "Bit 20 - Swap Timer E outputs"]
    #[inline(always)]
    pub fn swpe(&mut self) -> SWPE_W {
        SWPE_W { w: self }
    }
    #[doc = "Bit 19 - Swap Timer D outputs"]
    #[inline(always)]
    pub fn swpd(&mut self) -> SWPD_W {
        SWPD_W { w: self }
    }
    #[doc = "Bit 18 - Swap Timer C outputs"]
    #[inline(always)]
    pub fn swpc(&mut self) -> SWPC_W {
        SWPC_W { w: self }
    }
    #[doc = "Bit 17 - Swap Timer B outputs"]
    #[inline(always)]
    pub fn swpb(&mut self) -> SWPB_W {
        SWPB_W { w: self }
    }
    #[doc = "Bit 16 - Swap Timer A outputs"]
    #[inline(always)]
    pub fn swpa(&mut self) -> SWPA_W {
        SWPA_W { w: self }
    }
    #[doc = "Bit 14 - Timer f counter software reset"]
    #[inline(always)]
    pub fn tfrst(&mut self) -> TFRST_W {
        TFRST_W { w: self }
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&mut self) -> TERST_W {
        TERST_W { w: self }
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&mut self) -> TDRST_W {
        TDRST_W { w: self }
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&mut self) -> TCRST_W {
        TCRST_W { w: self }
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&mut self) -> TBRST_W {
        TBRST_W { w: self }
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&mut self) -> TARST_W {
        TARST_W { w: self }
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&mut self) -> MRST_W {
        MRST_W { w: self }
    }
    #[doc = "Bit 6 - Timer f Software Update"]
    #[inline(always)]
    pub fn tfswu(&mut self) -> TFSWU_W {
        TFSWU_W { w: self }
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&mut self) -> TESWU_W {
        TESWU_W { w: self }
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&mut self) -> TDSWU_W {
        TDSWU_W { w: self }
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&mut self) -> TCSWU_W {
        TCSWU_W { w: self }
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&mut self) -> TBSWU_W {
        TBSWU_W { w: self }
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&mut self) -> TASWU_W {
        TASWU_W { w: self }
    }
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&mut self) -> MSWU_W {
        MSWU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
