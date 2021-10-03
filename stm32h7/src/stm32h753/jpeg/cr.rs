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
#[doc = "Field `JCEN` reader - JPEG Core Enable Enable the JPEG codec Core."]
pub struct JCEN_R(crate::FieldReader<bool, bool>);
impl JCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JCEN` writer - JPEG Core Enable Enable the JPEG codec Core."]
pub struct JCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JCEN_W<'a> {
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
#[doc = "Field `IFTIE` reader - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
pub struct IFTIE_R(crate::FieldReader<bool, bool>);
impl IFTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFTIE` writer - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
pub struct IFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFTIE_W<'a> {
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
#[doc = "Field `IFNFIE` reader - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
pub struct IFNFIE_R(crate::FieldReader<bool, bool>);
impl IFNFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFNFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFNFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFNFIE` writer - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
pub struct IFNFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFNFIE_W<'a> {
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
#[doc = "Field `OFTIE` reader - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
pub struct OFTIE_R(crate::FieldReader<bool, bool>);
impl OFTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFTIE` writer - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
pub struct OFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFTIE_W<'a> {
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
#[doc = "Field `OFNEIE` reader - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
pub struct OFNEIE_R(crate::FieldReader<bool, bool>);
impl OFNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFNEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFNEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFNEIE` writer - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
pub struct OFNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFNEIE_W<'a> {
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
#[doc = "Field `EOCIE` reader - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
pub struct EOCIE_R(crate::FieldReader<bool, bool>);
impl EOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCIE` writer - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
pub struct EOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCIE_W<'a> {
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
#[doc = "Field `HPDIE` reader - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
pub struct HPDIE_R(crate::FieldReader<bool, bool>);
impl HPDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPDIE` writer - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
pub struct HPDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPDIE_W<'a> {
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
#[doc = "Field `IDMAEN` reader - Input DMA Enable Enable the DMA request generation for the input FIFO."]
pub struct IDMAEN_R(crate::FieldReader<bool, bool>);
impl IDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMAEN` writer - Input DMA Enable Enable the DMA request generation for the input FIFO."]
pub struct IDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMAEN_W<'a> {
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
#[doc = "Field `ODMAEN` reader - Output DMA Enable Enable the DMA request generation for the output FIFO."]
pub struct ODMAEN_R(crate::FieldReader<bool, bool>);
impl ODMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODMAEN` writer - Output DMA Enable Enable the DMA request generation for the output FIFO."]
pub struct ODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODMAEN_W<'a> {
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
#[doc = "Field `IFF` reader - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
pub struct IFF_R(crate::FieldReader<bool, bool>);
impl IFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFF` writer - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
pub struct IFF_W<'a> {
    w: &'a mut W,
}
impl<'a> IFF_W<'a> {
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
#[doc = "Field `OFF` reader - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
pub struct OFF_R(crate::FieldReader<bool, bool>);
impl OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF` writer - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
pub struct OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - JPEG Core Enable Enable the JPEG codec Core."]
    #[inline(always)]
    pub fn jcen(&self) -> JCEN_R {
        JCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
    #[inline(always)]
    pub fn iftie(&self) -> IFTIE_R {
        IFTIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
    #[inline(always)]
    pub fn ifnfie(&self) -> IFNFIE_R {
        IFNFIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
    #[inline(always)]
    pub fn oftie(&self) -> OFTIE_R {
        OFTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
    #[inline(always)]
    pub fn ofneie(&self) -> OFNEIE_R {
        OFNEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
    #[inline(always)]
    pub fn hpdie(&self) -> HPDIE_R {
        HPDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input DMA Enable Enable the DMA request generation for the input FIFO."]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output DMA Enable Enable the DMA request generation for the output FIFO."]
    #[inline(always)]
    pub fn odmaen(&self) -> ODMAEN_R {
        ODMAEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn iff(&self) -> IFF_R {
        IFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - JPEG Core Enable Enable the JPEG codec Core."]
    #[inline(always)]
    pub fn jcen(&mut self) -> JCEN_W {
        JCEN_W { w: self }
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
    #[inline(always)]
    pub fn iftie(&mut self) -> IFTIE_W {
        IFTIE_W { w: self }
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
    #[inline(always)]
    pub fn ifnfie(&mut self) -> IFNFIE_W {
        IFNFIE_W { w: self }
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
    #[inline(always)]
    pub fn oftie(&mut self) -> OFTIE_W {
        OFTIE_W { w: self }
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
    #[inline(always)]
    pub fn ofneie(&mut self) -> OFNEIE_W {
        OFNEIE_W { w: self }
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W { w: self }
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
    #[inline(always)]
    pub fn hpdie(&mut self) -> HPDIE_W {
        HPDIE_W { w: self }
    }
    #[doc = "Bit 11 - Input DMA Enable Enable the DMA request generation for the input FIFO."]
    #[inline(always)]
    pub fn idmaen(&mut self) -> IDMAEN_W {
        IDMAEN_W { w: self }
    }
    #[doc = "Bit 12 - Output DMA Enable Enable the DMA request generation for the output FIFO."]
    #[inline(always)]
    pub fn odmaen(&mut self) -> ODMAEN_W {
        ODMAEN_W { w: self }
    }
    #[doc = "Bit 13 - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn iff(&mut self) -> IFF_W {
        IFF_W { w: self }
    }
    #[doc = "Bit 14 - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W {
        OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
