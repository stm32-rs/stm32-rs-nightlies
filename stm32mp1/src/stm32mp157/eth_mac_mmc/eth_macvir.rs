#[doc = "Register `ETH_MACVIR` reader"]
pub struct R(crate::R<ETH_MACVIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACVIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACVIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACVIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACVIR` writer"]
pub struct W(crate::W<ETH_MACVIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACVIR_SPEC>;
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
impl From<crate::W<ETH_MACVIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACVIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLT` reader - VLT"]
pub struct VLT_R(crate::FieldReader<u16, u16>);
impl VLT_R {
    pub(crate) fn new(bits: u16) -> Self {
        VLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLT` writer - VLT"]
pub struct VLT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `VLC` reader - VLC"]
pub struct VLC_R(crate::FieldReader<u8, u8>);
impl VLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        VLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLC` writer - VLC"]
pub struct VLC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `VLP` reader - VLP"]
pub struct VLP_R(crate::FieldReader<bool, bool>);
impl VLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLP` writer - VLP"]
pub struct VLP_W<'a> {
    w: &'a mut W,
}
impl<'a> VLP_W<'a> {
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
#[doc = "Field `CSVL` reader - CSVL"]
pub struct CSVL_R(crate::FieldReader<bool, bool>);
impl CSVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSVL` writer - CSVL"]
pub struct CSVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSVL_W<'a> {
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
#[doc = "Field `VLTI` reader - VLTI"]
pub struct VLTI_R(crate::FieldReader<bool, bool>);
impl VLTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLTI` writer - VLTI"]
pub struct VLTI_W<'a> {
    w: &'a mut W,
}
impl<'a> VLTI_W<'a> {
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
impl R {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W {
        VLT_W { w: self }
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&mut self) -> VLC_W {
        VLC_W { w: self }
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&mut self) -> VLP_W {
        VLP_W { w: self }
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W {
        CSVL_W { w: self }
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&mut self) -> VLTI_W {
        VLTI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvir](index.html) module"]
pub struct ETH_MACVIR_SPEC;
impl crate::RegisterSpec for ETH_MACVIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macvir::R](R) reader structure"]
impl crate::Readable for ETH_MACVIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macvir::W](W) writer structure"]
impl crate::Writable for ETH_MACVIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACVIR to value 0"]
impl crate::Resettable for ETH_MACVIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
