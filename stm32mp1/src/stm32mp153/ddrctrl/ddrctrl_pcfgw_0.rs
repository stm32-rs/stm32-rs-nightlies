#[doc = "Register `DDRCTRL_PCFGW_0` reader"]
pub struct R(crate::R<DDRCTRL_PCFGW_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGW_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGW_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGW_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCFGW_0` writer"]
pub struct W(crate::W<DDRCTRL_PCFGW_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGW_0_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGW_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGW_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_PORT_PRIORITY` reader - WR_PORT_PRIORITY"]
pub struct WR_PORT_PRIORITY_R(crate::FieldReader<u16, u16>);
impl WR_PORT_PRIORITY_R {
    pub(crate) fn new(bits: u16) -> Self {
        WR_PORT_PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PORT_PRIORITY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PORT_PRIORITY` writer - WR_PORT_PRIORITY"]
pub struct WR_PORT_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `WR_PORT_AGING_EN` reader - WR_PORT_AGING_EN"]
pub struct WR_PORT_AGING_EN_R(crate::FieldReader<bool, bool>);
impl WR_PORT_AGING_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_PORT_AGING_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PORT_AGING_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PORT_AGING_EN` writer - WR_PORT_AGING_EN"]
pub struct WR_PORT_AGING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_AGING_EN_W<'a> {
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
#[doc = "Field `WR_PORT_URGENT_EN` reader - WR_PORT_URGENT_EN"]
pub struct WR_PORT_URGENT_EN_R(crate::FieldReader<bool, bool>);
impl WR_PORT_URGENT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_PORT_URGENT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PORT_URGENT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PORT_URGENT_EN` writer - WR_PORT_URGENT_EN"]
pub struct WR_PORT_URGENT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_URGENT_EN_W<'a> {
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
#[doc = "Field `WR_PORT_PAGEMATCH_EN` reader - WR_PORT_PAGEMATCH_EN"]
pub struct WR_PORT_PAGEMATCH_EN_R(crate::FieldReader<bool, bool>);
impl WR_PORT_PAGEMATCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_PORT_PAGEMATCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PORT_PAGEMATCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PORT_PAGEMATCH_EN` writer - WR_PORT_PAGEMATCH_EN"]
pub struct WR_PORT_PAGEMATCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_PAGEMATCH_EN_W<'a> {
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
    #[doc = "Bits 0:9 - WR_PORT_PRIORITY"]
    #[inline(always)]
    pub fn wr_port_priority(&self) -> WR_PORT_PRIORITY_R {
        WR_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - WR_PORT_AGING_EN"]
    #[inline(always)]
    pub fn wr_port_aging_en(&self) -> WR_PORT_AGING_EN_R {
        WR_PORT_AGING_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WR_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn wr_port_urgent_en(&self) -> WR_PORT_URGENT_EN_R {
        WR_PORT_URGENT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WR_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn wr_port_pagematch_en(&self) -> WR_PORT_PAGEMATCH_EN_R {
        WR_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - WR_PORT_PRIORITY"]
    #[inline(always)]
    pub fn wr_port_priority(&mut self) -> WR_PORT_PRIORITY_W {
        WR_PORT_PRIORITY_W { w: self }
    }
    #[doc = "Bit 12 - WR_PORT_AGING_EN"]
    #[inline(always)]
    pub fn wr_port_aging_en(&mut self) -> WR_PORT_AGING_EN_W {
        WR_PORT_AGING_EN_W { w: self }
    }
    #[doc = "Bit 13 - WR_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn wr_port_urgent_en(&mut self) -> WR_PORT_URGENT_EN_W {
        WR_PORT_URGENT_EN_W { w: self }
    }
    #[doc = "Bit 14 - WR_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn wr_port_pagematch_en(&mut self) -> WR_PORT_PAGEMATCH_EN_W {
        WR_PORT_PAGEMATCH_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port 0 configuration write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgw_0](index.html) module"]
pub struct DDRCTRL_PCFGW_0_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGW_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pcfgw_0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGW_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgw_0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGW_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGW_0 to value 0x4000"]
impl crate::Resettable for DDRCTRL_PCFGW_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
