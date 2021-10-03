#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUCKSEL` reader - WUCKSEL"]
pub struct WUCKSEL_R(crate::FieldReader<u8, u8>);
impl WUCKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUCKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUCKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUCKSEL` writer - WUCKSEL"]
pub struct WUCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `TSEDGE` reader - TSEDGE"]
pub struct TSEDGE_R(crate::FieldReader<bool, bool>);
impl TSEDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEDGE` writer - TSEDGE"]
pub struct TSEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEDGE_W<'a> {
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
#[doc = "Field `REFCKON` reader - REFCKON"]
pub struct REFCKON_R(crate::FieldReader<bool, bool>);
impl REFCKON_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFCKON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCKON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCKON` writer - REFCKON"]
pub struct REFCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCKON_W<'a> {
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
#[doc = "Field `BYPSHAD` reader - BYPSHAD"]
pub struct BYPSHAD_R(crate::FieldReader<bool, bool>);
impl BYPSHAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPSHAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPSHAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPSHAD` writer - BYPSHAD"]
pub struct BYPSHAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPSHAD_W<'a> {
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
#[doc = "Field `FMT` reader - FMT"]
pub struct FMT_R(crate::FieldReader<bool, bool>);
impl FMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMT` writer - FMT"]
pub struct FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> FMT_W<'a> {
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
#[doc = "Field `ALRAE` reader - ALRAE"]
pub struct ALRAE_R(crate::FieldReader<bool, bool>);
impl ALRAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRAE` writer - ALRAE"]
pub struct ALRAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAE_W<'a> {
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
#[doc = "Field `ALRBE` reader - ALRBE"]
pub struct ALRBE_R(crate::FieldReader<bool, bool>);
impl ALRBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBE` writer - ALRBE"]
pub struct ALRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBE_W<'a> {
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
#[doc = "Field `WUTE` reader - WUTE"]
pub struct WUTE_R(crate::FieldReader<bool, bool>);
impl WUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTE` writer - WUTE"]
pub struct WUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTE_W<'a> {
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
#[doc = "Field `TSE` reader - TSE"]
pub struct TSE_R(crate::FieldReader<bool, bool>);
impl TSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSE` writer - TSE"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
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
#[doc = "Field `ALRAIE` reader - ALRAIE"]
pub struct ALRAIE_R(crate::FieldReader<bool, bool>);
impl ALRAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRAIE` writer - ALRAIE"]
pub struct ALRAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAIE_W<'a> {
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
#[doc = "Field `ALRBIE` reader - ALRBIE"]
pub struct ALRBIE_R(crate::FieldReader<bool, bool>);
impl ALRBIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBIE` writer - ALRBIE"]
pub struct ALRBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBIE_W<'a> {
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
#[doc = "Field `WUTIE` reader - WUTIE"]
pub struct WUTIE_R(crate::FieldReader<bool, bool>);
impl WUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTIE` writer - WUTIE"]
pub struct WUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTIE_W<'a> {
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
#[doc = "Field `TSIE` reader - TSIE"]
pub struct TSIE_R(crate::FieldReader<bool, bool>);
impl TSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIE` writer - TSIE"]
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ADD1H` reader - ADD1H"]
pub struct ADD1H_R(crate::FieldReader<bool, bool>);
impl ADD1H_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD1H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD1H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD1H` writer - ADD1H"]
pub struct ADD1H_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1H_W<'a> {
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
#[doc = "Field `SUB1H` reader - SUB1H"]
pub struct SUB1H_R(crate::FieldReader<bool, bool>);
impl SUB1H_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUB1H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUB1H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUB1H` writer - SUB1H"]
pub struct SUB1H_W<'a> {
    w: &'a mut W,
}
impl<'a> SUB1H_W<'a> {
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
#[doc = "Field `BKP` reader - BKP"]
pub struct BKP_R(crate::FieldReader<bool, bool>);
impl BKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKP` writer - BKP"]
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
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
#[doc = "Field `COSEL` reader - COSEL"]
pub struct COSEL_R(crate::FieldReader<bool, bool>);
impl COSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COSEL` writer - COSEL"]
pub struct COSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COSEL_W<'a> {
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
#[doc = "Field `POL` reader - POL"]
pub struct POL_R(crate::FieldReader<bool, bool>);
impl POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL` writer - POL"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
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
#[doc = "Field `OSEL` reader - OSEL"]
pub struct OSEL_R(crate::FieldReader<u8, u8>);
impl OSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSEL` writer - OSEL"]
pub struct OSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `COE` reader - COE"]
pub struct COE_R(crate::FieldReader<bool, bool>);
impl COE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COE` writer - COE"]
pub struct COE_W<'a> {
    w: &'a mut W,
}
impl<'a> COE_W<'a> {
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
#[doc = "Field `ITSE` reader - ITSE"]
pub struct ITSE_R(crate::FieldReader<bool, bool>);
impl ITSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITSE` writer - ITSE"]
pub struct ITSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `TAMPTS` reader - TAMPTS"]
pub struct TAMPTS_R(crate::FieldReader<bool, bool>);
impl TAMPTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPTS` writer - TAMPTS"]
pub struct TAMPTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `TAMPOE` reader - TAMPOE"]
pub struct TAMPOE_R(crate::FieldReader<bool, bool>);
impl TAMPOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPOE` writer - TAMPOE"]
pub struct TAMPOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM_PU"]
pub struct TAMPALRM_PU_R(crate::FieldReader<bool, bool>);
impl TAMPALRM_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPALRM_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPALRM_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM_PU"]
pub struct TAMPALRM_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPALRM_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM_TYPE"]
pub struct TAMPALRM_TYPE_R(crate::FieldReader<bool, bool>);
impl TAMPALRM_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPALRM_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPALRM_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM_TYPE"]
pub struct TAMPALRM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPALRM_TYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `OUT2EN` reader - OUT2EN"]
pub struct OUT2EN_R(crate::FieldReader<bool, bool>);
impl OUT2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT2EN` writer - OUT2EN"]
pub struct OUT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - WUCKSEL"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - TSEDGE"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - REFCKON"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BYPSHAD"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FMT"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ALRAE"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ALRBE"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WUTE"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ALRAIE"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ALRBIE"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WUTIE"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADD1H"]
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SUB1H"]
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - COSEL"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - OSEL"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - COE"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ITSE"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TAMPTS"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TAMPOE"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM_PU"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM_TYPE"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - OUT2EN"]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WUCKSEL"]
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W {
        WUCKSEL_W { w: self }
    }
    #[doc = "Bit 3 - TSEDGE"]
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W {
        TSEDGE_W { w: self }
    }
    #[doc = "Bit 4 - REFCKON"]
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W {
        REFCKON_W { w: self }
    }
    #[doc = "Bit 5 - BYPSHAD"]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W {
        BYPSHAD_W { w: self }
    }
    #[doc = "Bit 6 - FMT"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W {
        FMT_W { w: self }
    }
    #[doc = "Bit 8 - ALRAE"]
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W {
        ALRAE_W { w: self }
    }
    #[doc = "Bit 9 - ALRBE"]
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W {
        ALRBE_W { w: self }
    }
    #[doc = "Bit 10 - WUTE"]
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W {
        WUTE_W { w: self }
    }
    #[doc = "Bit 11 - TSE"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bit 12 - ALRAIE"]
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W {
        ALRAIE_W { w: self }
    }
    #[doc = "Bit 13 - ALRBIE"]
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W {
        ALRBIE_W { w: self }
    }
    #[doc = "Bit 14 - WUTIE"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W {
        WUTIE_W { w: self }
    }
    #[doc = "Bit 15 - TSIE"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    #[doc = "Bit 16 - ADD1H"]
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W {
        ADD1H_W { w: self }
    }
    #[doc = "Bit 17 - SUB1H"]
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W {
        SUB1H_W { w: self }
    }
    #[doc = "Bit 18 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    #[doc = "Bit 19 - COSEL"]
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W {
        COSEL_W { w: self }
    }
    #[doc = "Bit 20 - POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Bits 21:22 - OSEL"]
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W {
        OSEL_W { w: self }
    }
    #[doc = "Bit 23 - COE"]
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W {
        COE_W { w: self }
    }
    #[doc = "Bit 24 - ITSE"]
    #[inline(always)]
    pub fn itse(&mut self) -> ITSE_W {
        ITSE_W { w: self }
    }
    #[doc = "Bit 25 - TAMPTS"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W {
        TAMPTS_W { w: self }
    }
    #[doc = "Bit 26 - TAMPOE"]
    #[inline(always)]
    pub fn tampoe(&mut self) -> TAMPOE_W {
        TAMPOE_W { w: self }
    }
    #[doc = "Bit 29 - TAMPALRM_PU"]
    #[inline(always)]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W {
        TAMPALRM_PU_W { w: self }
    }
    #[doc = "Bit 30 - TAMPALRM_TYPE"]
    #[inline(always)]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W {
        TAMPALRM_TYPE_W { w: self }
    }
    #[doc = "Bit 31 - OUT2EN"]
    #[inline(always)]
    pub fn out2en(&mut self) -> OUT2EN_W {
        OUT2EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
