#[doc = "Register `OTG_DOEPINT7` reader"]
pub struct R(crate::R<OTG_DOEPINT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DOEPINT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DOEPINT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DOEPINT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DOEPINT7` writer"]
pub struct W(crate::W<OTG_DOEPINT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DOEPINT7_SPEC>;
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
impl From<crate::W<OTG_DOEPINT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DOEPINT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - XFRC"]
pub struct XFRC_R(crate::FieldReader<bool, bool>);
impl XFRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRC` writer - XFRC"]
pub struct XFRC_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRC_W<'a> {
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
#[doc = "Field `EPDISD` reader - EPDISD"]
pub struct EPDISD_R(crate::FieldReader<bool, bool>);
impl EPDISD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDISD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDISD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDISD` writer - EPDISD"]
pub struct EPDISD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISD_W<'a> {
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
#[doc = "Field `AHBERR` reader - AHBERR"]
pub struct AHBERR_R(crate::FieldReader<bool, bool>);
impl AHBERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBERR` writer - AHBERR"]
pub struct AHBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERR_W<'a> {
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
#[doc = "Field `STUP` reader - STUP"]
pub struct STUP_R(crate::FieldReader<bool, bool>);
impl STUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUP` writer - STUP"]
pub struct STUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STUP_W<'a> {
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
#[doc = "Field `OTEPDIS` reader - OTEPDIS"]
pub struct OTEPDIS_R(crate::FieldReader<bool, bool>);
impl OTEPDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTEPDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTEPDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTEPDIS` writer - OTEPDIS"]
pub struct OTEPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTEPDIS_W<'a> {
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
#[doc = "Field `STSPHSRX` reader - STSPHSRX"]
pub struct STSPHSRX_R(crate::FieldReader<bool, bool>);
impl STSPHSRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        STSPHSRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSPHSRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSPHSRX` writer - STSPHSRX"]
pub struct STSPHSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> STSPHSRX_W<'a> {
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
#[doc = "Field `B2BSTUP` reader - B2BSTUP"]
pub struct B2BSTUP_R(crate::FieldReader<bool, bool>);
impl B2BSTUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2BSTUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2BSTUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2BSTUP` writer - B2BSTUP"]
pub struct B2BSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> B2BSTUP_W<'a> {
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
#[doc = "Field `OUTPKTERR` reader - OUTPKTERR"]
pub struct OUTPKTERR_R(crate::FieldReader<bool, bool>);
impl OUTPKTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTPKTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPKTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPKTERR` writer - OUTPKTERR"]
pub struct OUTPKTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPKTERR_W<'a> {
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
#[doc = "Field `BNA` reader - BNA"]
pub struct BNA_R(crate::FieldReader<bool, bool>);
impl BNA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNA` writer - BNA"]
pub struct BNA_W<'a> {
    w: &'a mut W,
}
impl<'a> BNA_W<'a> {
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
#[doc = "Field `BERR` reader - BERR"]
pub struct BERR_R(crate::FieldReader<bool, bool>);
impl BERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERR` writer - BERR"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
#[doc = "Field `NAK` reader - NAK"]
pub struct NAK_R(crate::FieldReader<bool, bool>);
impl NAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK` writer - NAK"]
pub struct NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_W<'a> {
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
#[doc = "Field `NYET` reader - NYET"]
pub struct NYET_R(crate::FieldReader<bool, bool>);
impl NYET_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYET` writer - NYET"]
pub struct NYET_W<'a> {
    w: &'a mut W,
}
impl<'a> NYET_W<'a> {
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
#[doc = "Field `STPKTRX` reader - STPKTRX"]
pub struct STPKTRX_R(crate::FieldReader<bool, bool>);
impl STPKTRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPKTRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPKTRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPKTRX` writer - STPKTRX"]
pub struct STPKTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> STPKTRX_W<'a> {
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
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHBERR"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - STSPHSRX"]
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OUTPKTERR"]
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNA"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BERR"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAK"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NYET"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - STPKTRX"]
    #[inline(always)]
    pub fn stpktrx(&self) -> STPKTRX_R {
        STPKTRX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W { w: self }
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W {
        EPDISD_W { w: self }
    }
    #[doc = "Bit 2 - AHBERR"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W { w: self }
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W {
        STUP_W { w: self }
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W {
        OTEPDIS_W { w: self }
    }
    #[doc = "Bit 5 - STSPHSRX"]
    #[inline(always)]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W {
        STSPHSRX_W { w: self }
    }
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W {
        B2BSTUP_W { w: self }
    }
    #[doc = "Bit 8 - OUTPKTERR"]
    #[inline(always)]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W {
        OUTPKTERR_W { w: self }
    }
    #[doc = "Bit 9 - BNA"]
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W {
        BNA_W { w: self }
    }
    #[doc = "Bit 12 - BERR"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
    #[doc = "Bit 13 - NAK"]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W { w: self }
    }
    #[doc = "Bit 14 - NYET"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W {
        NYET_W { w: self }
    }
    #[doc = "Bit 15 - STPKTRX"]
    #[inline(always)]
    pub fn stpktrx(&mut self) -> STPKTRX_W {
        STPKTRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint7](index.html) module"]
pub struct OTG_DOEPINT7_SPEC;
impl crate::RegisterSpec for OTG_DOEPINT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_doepint7::R](R) reader structure"]
impl crate::Readable for OTG_DOEPINT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_doepint7::W](W) writer structure"]
impl crate::Writable for OTG_DOEPINT7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DOEPINT7 to value 0x80"]
impl crate::Resettable for OTG_DOEPINT7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
