#[doc = "Register `USBPHYC_PLL` reader"]
pub struct R(crate::R<USBPHYC_PLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYC_PLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYC_PLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYC_PLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYC_PLL` writer"]
pub struct W(crate::W<USBPHYC_PLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYC_PLL_SPEC>;
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
impl From<crate::W<USBPHYC_PLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYC_PLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLNDIV` reader - PLLNDIV"]
pub struct PLLNDIV_R(crate::FieldReader<u8, u8>);
impl PLLNDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLNDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLNDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLNDIV` writer - PLLNDIV"]
pub struct PLLNDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLNDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PLLODF` reader - PLLODF"]
pub struct PLLODF_R(crate::FieldReader<u8, u8>);
impl PLLODF_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLODF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLODF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLODF` writer - PLLODF"]
pub struct PLLODF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLODF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `PLLFRACIN` reader - PLLFRACIN"]
pub struct PLLFRACIN_R(crate::FieldReader<u16, u16>);
impl PLLFRACIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PLLFRACIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLFRACIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLFRACIN` writer - PLLFRACIN"]
pub struct PLLFRACIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFRACIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 10)) | ((value as u32 & 0xffff) << 10);
        self.w
    }
}
#[doc = "Field `PLLEN` reader - PLLEN"]
pub struct PLLEN_R(crate::FieldReader<bool, bool>);
impl PLLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLEN` writer - PLLEN"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
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
#[doc = "Field `PLLSTRB` reader - PLLSTRB"]
pub struct PLLSTRB_R(crate::FieldReader<bool, bool>);
impl PLLSTRB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSTRB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSTRB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSTRB` writer - PLLSTRB"]
pub struct PLLSTRB_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTRB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PLLSTRBYP` reader - PLLSTRBYP"]
pub struct PLLSTRBYP_R(crate::FieldReader<bool, bool>);
impl PLLSTRBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSTRBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSTRBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSTRBYP` writer - PLLSTRBYP"]
pub struct PLLSTRBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTRBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PLLFRACCTL` reader - PLLFRACCTL"]
pub struct PLLFRACCTL_R(crate::FieldReader<bool, bool>);
impl PLLFRACCTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLFRACCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLFRACCTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLFRACCTL` writer - PLLFRACCTL"]
pub struct PLLFRACCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFRACCTL_W<'a> {
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
#[doc = "Field `PLLDITHEN0` reader - PLLDITHEN0"]
pub struct PLLDITHEN0_R(crate::FieldReader<bool, bool>);
impl PLLDITHEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLDITHEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLDITHEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLDITHEN0` writer - PLLDITHEN0"]
pub struct PLLDITHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDITHEN0_W<'a> {
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
#[doc = "Field `PLLDITHEN1` reader - PLLDITHEN1"]
pub struct PLLDITHEN1_R(crate::FieldReader<bool, bool>);
impl PLLDITHEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLDITHEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLDITHEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLDITHEN1` writer - PLLDITHEN1"]
pub struct PLLDITHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDITHEN1_W<'a> {
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
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    pub fn pllndiv(&self) -> PLLNDIV_R {
        PLLNDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    pub fn pllodf(&self) -> PLLODF_R {
        PLLODF_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    pub fn pllfracin(&self) -> PLLFRACIN_R {
        PLLFRACIN_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    pub fn pllstrb(&self) -> PLLSTRB_R {
        PLLSTRB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    pub fn pllstrbyp(&self) -> PLLSTRBYP_R {
        PLLSTRBYP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    pub fn pllfracctl(&self) -> PLLFRACCTL_R {
        PLLFRACCTL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    pub fn plldithen0(&self) -> PLLDITHEN0_R {
        PLLDITHEN0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    pub fn plldithen1(&self) -> PLLDITHEN1_R {
        PLLDITHEN1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    pub fn pllndiv(&mut self) -> PLLNDIV_W {
        PLLNDIV_W { w: self }
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    pub fn pllodf(&mut self) -> PLLODF_W {
        PLLODF_W { w: self }
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    pub fn pllfracin(&mut self) -> PLLFRACIN_W {
        PLLFRACIN_W { w: self }
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    pub fn pllstrb(&mut self) -> PLLSTRB_W {
        PLLSTRB_W { w: self }
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    pub fn pllstrbyp(&mut self) -> PLLSTRBYP_W {
        PLLSTRBYP_W { w: self }
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    pub fn pllfracctl(&mut self) -> PLLFRACCTL_W {
        PLLFRACCTL_W { w: self }
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    pub fn plldithen0(&mut self) -> PLLDITHEN0_W {
        PLLDITHEN0_W { w: self }
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    pub fn plldithen1(&mut self) -> PLLDITHEN1_W {
        PLLDITHEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the PLL of the HS PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_pll](index.html) module"]
pub struct USBPHYC_PLL_SPEC;
impl crate::RegisterSpec for USBPHYC_PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphyc_pll::R](R) reader structure"]
impl crate::Readable for USBPHYC_PLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphyc_pll::W](W) writer structure"]
impl crate::Writable for USBPHYC_PLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPHYC_PLL to value 0xc000_0000"]
impl crate::Resettable for USBPHYC_PLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
