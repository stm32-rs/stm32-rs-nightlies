#[doc = "Register `DMA_S2CR` reader"]
pub struct R(crate::R<DMA_S2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_S2CR` writer"]
pub struct W(crate::W<DMA_S2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S2CR_SPEC>;
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
impl From<crate::W<DMA_S2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EN"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `DMEIE` reader - DMEIE"]
pub struct DMEIE_R(crate::FieldReader<bool, bool>);
impl DMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIE` writer - DMEIE"]
pub struct DMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMEIE_W<'a> {
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
#[doc = "Field `TEIE` reader - TEIE"]
pub struct TEIE_R(crate::FieldReader<bool, bool>);
impl TEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIE` writer - TEIE"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
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
#[doc = "Field `HTIE` reader - HTIE"]
pub struct HTIE_R(crate::FieldReader<bool, bool>);
impl HTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIE` writer - HTIE"]
pub struct HTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE_W<'a> {
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
#[doc = "Field `TCIE` reader - TCIE"]
pub struct TCIE_R(crate::FieldReader<bool, bool>);
impl TCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIE` writer - TCIE"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
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
#[doc = "Field `PFCTRL` reader - PFCTRL"]
pub struct PFCTRL_R(crate::FieldReader<bool, bool>);
impl PFCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PFCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFCTRL` writer - PFCTRL"]
pub struct PFCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFCTRL_W<'a> {
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
#[doc = "Field `DIR` reader - DIR"]
pub struct DIR_R(crate::FieldReader<u8, u8>);
impl DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - DIR"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CIRC` reader - CIRC"]
pub struct CIRC_R(crate::FieldReader<bool, bool>);
impl CIRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIRC` writer - CIRC"]
pub struct CIRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CIRC_W<'a> {
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
#[doc = "Field `PINC` reader - PINC"]
pub struct PINC_R(crate::FieldReader<bool, bool>);
impl PINC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINC` writer - PINC"]
pub struct PINC_W<'a> {
    w: &'a mut W,
}
impl<'a> PINC_W<'a> {
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
#[doc = "Field `MINC` reader - MINC"]
pub struct MINC_R(crate::FieldReader<bool, bool>);
impl MINC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINC` writer - MINC"]
pub struct MINC_W<'a> {
    w: &'a mut W,
}
impl<'a> MINC_W<'a> {
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
#[doc = "Field `PSIZE` reader - PSIZE"]
pub struct PSIZE_R(crate::FieldReader<u8, u8>);
impl PSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSIZE` writer - PSIZE"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `MSIZE` reader - MSIZE"]
pub struct MSIZE_R(crate::FieldReader<u8, u8>);
impl MSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIZE` writer - MSIZE"]
pub struct MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `PINCOS` reader - PINCOS"]
pub struct PINCOS_R(crate::FieldReader<bool, bool>);
impl PINCOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINCOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINCOS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINCOS` writer - PINCOS"]
pub struct PINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCOS_W<'a> {
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
#[doc = "Field `PL` reader - PL"]
pub struct PL_R(crate::FieldReader<u8, u8>);
impl PL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PL` writer - PL"]
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DBM` reader - DBM"]
pub struct DBM_R(crate::FieldReader<bool, bool>);
impl DBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBM` writer - DBM"]
pub struct DBM_W<'a> {
    w: &'a mut W,
}
impl<'a> DBM_W<'a> {
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
#[doc = "Field `CT` reader - CT"]
pub struct CT_R(crate::FieldReader<bool, bool>);
impl CT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CT` writer - CT"]
pub struct CT_W<'a> {
    w: &'a mut W,
}
impl<'a> CT_W<'a> {
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
#[doc = "Field `PBURST` reader - PBURST"]
pub struct PBURST_R(crate::FieldReader<u8, u8>);
impl PBURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBURST` writer - PBURST"]
pub struct PBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> PBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `MBURST` reader - MBURST"]
pub struct MBURST_R(crate::FieldReader<u8, u8>);
impl MBURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        MBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBURST` writer - MBURST"]
pub struct MBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> MBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMEIE"]
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HTIE"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PFCTRL"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - CIRC"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PINC"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MINC"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - PSIZE"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - MSIZE"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - PINCOS"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - PL"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - DBM"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - PBURST"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - MBURST"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - DMEIE"]
    #[inline(always)]
    pub fn dmeie(&mut self) -> DMEIE_W {
        DMEIE_W { w: self }
    }
    #[doc = "Bit 2 - TEIE"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Bit 3 - HTIE"]
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W {
        HTIE_W { w: self }
    }
    #[doc = "Bit 4 - TCIE"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 5 - PFCTRL"]
    #[inline(always)]
    pub fn pfctrl(&mut self) -> PFCTRL_W {
        PFCTRL_W { w: self }
    }
    #[doc = "Bits 6:7 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 8 - CIRC"]
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W {
        CIRC_W { w: self }
    }
    #[doc = "Bit 9 - PINC"]
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W {
        PINC_W { w: self }
    }
    #[doc = "Bit 10 - MINC"]
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W {
        MINC_W { w: self }
    }
    #[doc = "Bits 11:12 - PSIZE"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    #[doc = "Bits 13:14 - MSIZE"]
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W {
        MSIZE_W { w: self }
    }
    #[doc = "Bit 15 - PINCOS"]
    #[inline(always)]
    pub fn pincos(&mut self) -> PINCOS_W {
        PINCOS_W { w: self }
    }
    #[doc = "Bits 16:17 - PL"]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
    #[doc = "Bit 18 - DBM"]
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W {
        DBM_W { w: self }
    }
    #[doc = "Bit 19 - CT"]
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W {
        CT_W { w: self }
    }
    #[doc = "Bits 21:22 - PBURST"]
    #[inline(always)]
    pub fn pburst(&mut self) -> PBURST_W {
        PBURST_W { w: self }
    }
    #[doc = "Bits 23:24 - MBURST"]
    #[inline(always)]
    pub fn mburst(&mut self) -> MBURST_W {
        MBURST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2cr](index.html) module"]
pub struct DMA_S2CR_SPEC;
impl crate::RegisterSpec for DMA_S2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_s2cr::R](R) reader structure"]
impl crate::Readable for DMA_S2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_s2cr::W](W) writer structure"]
impl crate::Writable for DMA_S2CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_S2CR to value 0"]
impl crate::Resettable for DMA_S2CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
