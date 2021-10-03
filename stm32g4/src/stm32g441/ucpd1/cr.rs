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
#[doc = "Field `TXMODE` reader - TXMODE"]
pub struct TXMODE_R(crate::FieldReader<u8, u8>);
impl TXMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMODE` writer - TXMODE"]
pub struct TXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TXSEND` reader - TXSEND"]
pub struct TXSEND_R(crate::FieldReader<bool, bool>);
impl TXSEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSEND` writer - TXSEND"]
pub struct TXSEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSEND_W<'a> {
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
#[doc = "Field `TXHRST` reader - TXHRST"]
pub struct TXHRST_R(crate::FieldReader<bool, bool>);
impl TXHRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXHRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXHRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXHRST` writer - TXHRST"]
pub struct TXHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXHRST_W<'a> {
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
#[doc = "Field `RXMODE` reader - RXMODE"]
pub struct RXMODE_R(crate::FieldReader<bool, bool>);
impl RXMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMODE` writer - RXMODE"]
pub struct RXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMODE_W<'a> {
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
#[doc = "Field `PHYRXEN` reader - PHYRXEN"]
pub struct PHYRXEN_R(crate::FieldReader<bool, bool>);
impl PHYRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYRXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYRXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYRXEN` writer - PHYRXEN"]
pub struct PHYRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYRXEN_W<'a> {
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
#[doc = "Field `PHYCCSEL` reader - PHYCCSEL"]
pub struct PHYCCSEL_R(crate::FieldReader<bool, bool>);
impl PHYCCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYCCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYCCSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYCCSEL` writer - PHYCCSEL"]
pub struct PHYCCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYCCSEL_W<'a> {
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
#[doc = "Field `ANASUBMODE` reader - ANASUBMODE"]
pub struct ANASUBMODE_R(crate::FieldReader<u8, u8>);
impl ANASUBMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANASUBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANASUBMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANASUBMODE` writer - ANASUBMODE"]
pub struct ANASUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANASUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `ANAMODE` reader - ANAMODE"]
pub struct ANAMODE_R(crate::FieldReader<bool, bool>);
impl ANAMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANAMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANAMODE` writer - ANAMODE"]
pub struct ANAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAMODE_W<'a> {
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
#[doc = "Field `CCENABLE` reader - CCENABLE"]
pub struct CCENABLE_R(crate::FieldReader<u8, u8>);
impl CCENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCENABLE` writer - CCENABLE"]
pub struct CCENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `FRSRXEN` reader - FRSRXEN"]
pub struct FRSRXEN_R(crate::FieldReader<bool, bool>);
impl FRSRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSRXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSRXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSRXEN` writer - FRSRXEN"]
pub struct FRSRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSRXEN_W<'a> {
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
#[doc = "Field `FRSTX` reader - FRSTX"]
pub struct FRSTX_R(crate::FieldReader<bool, bool>);
impl FRSTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSTX` writer - FRSTX"]
pub struct FRSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTX_W<'a> {
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
#[doc = "Field `RDCH` reader - RDCH"]
pub struct RDCH_R(crate::FieldReader<bool, bool>);
impl RDCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDCH` writer - RDCH"]
pub struct RDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCH_W<'a> {
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
#[doc = "Field `CC1TCDIS` reader - CC1TCDIS"]
pub struct CC1TCDIS_R(crate::FieldReader<bool, bool>);
impl CC1TCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1TCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1TCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1TCDIS` writer - CC1TCDIS"]
pub struct CC1TCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1TCDIS_W<'a> {
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
#[doc = "Field `CC2TCDIS` reader - CC2TCDIS"]
pub struct CC2TCDIS_R(crate::FieldReader<bool, bool>);
impl CC2TCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2TCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2TCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2TCDIS` writer - CC2TCDIS"]
pub struct CC2TCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2TCDIS_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W {
        TXMODE_W { w: self }
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&mut self) -> TXSEND_W {
        TXSEND_W { w: self }
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&mut self) -> TXHRST_W {
        TXHRST_W { w: self }
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W {
        RXMODE_W { w: self }
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&mut self) -> PHYRXEN_W {
        PHYRXEN_W { w: self }
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W {
        PHYCCSEL_W { w: self }
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W {
        ANASUBMODE_W { w: self }
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&mut self) -> ANAMODE_W {
        ANAMODE_W { w: self }
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&mut self) -> CCENABLE_W {
        CCENABLE_W { w: self }
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&mut self) -> FRSRXEN_W {
        FRSRXEN_W { w: self }
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&mut self) -> FRSTX_W {
        FRSTX_W { w: self }
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&mut self) -> RDCH_W {
        RDCH_W { w: self }
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W {
        CC1TCDIS_W { w: self }
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W {
        CC2TCDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
