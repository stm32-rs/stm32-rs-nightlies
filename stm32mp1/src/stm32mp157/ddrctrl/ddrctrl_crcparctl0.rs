#[doc = "Register `DDRCTRL_CRCPARCTL0` reader"]
pub struct R(crate::R<DDRCTRL_CRCPARCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_CRCPARCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_CRCPARCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_CRCPARCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_CRCPARCTL0` writer"]
pub struct W(crate::W<DDRCTRL_CRCPARCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_CRCPARCTL0_SPEC>;
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
impl From<crate::W<DDRCTRL_CRCPARCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_CRCPARCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_ALERT_ERR_INT_EN` reader - DFI_ALERT_ERR_INT_EN"]
pub struct DFI_ALERT_ERR_INT_EN_R(crate::FieldReader<bool, bool>);
impl DFI_ALERT_ERR_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_ALERT_ERR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_ALERT_ERR_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_ALERT_ERR_INT_EN` writer - DFI_ALERT_ERR_INT_EN"]
pub struct DFI_ALERT_ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_ALERT_ERR_INT_EN_W<'a> {
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
#[doc = "Field `DFI_ALERT_ERR_INT_CLR` reader - DFI_ALERT_ERR_INT_CLR"]
pub struct DFI_ALERT_ERR_INT_CLR_R(crate::FieldReader<bool, bool>);
impl DFI_ALERT_ERR_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_ALERT_ERR_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_ALERT_ERR_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_ALERT_ERR_INT_CLR` writer - DFI_ALERT_ERR_INT_CLR"]
pub struct DFI_ALERT_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_ALERT_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `DFI_ALERT_ERR_CNT_CLR` reader - DFI_ALERT_ERR_CNT_CLR"]
pub struct DFI_ALERT_ERR_CNT_CLR_R(crate::FieldReader<bool, bool>);
impl DFI_ALERT_ERR_CNT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_ALERT_ERR_CNT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_ALERT_ERR_CNT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_ALERT_ERR_CNT_CLR` writer - DFI_ALERT_ERR_CNT_CLR"]
pub struct DFI_ALERT_ERR_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_ALERT_ERR_CNT_CLR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DFI_ALERT_ERR_INT_EN"]
    #[inline(always)]
    pub fn dfi_alert_err_int_en(&self) -> DFI_ALERT_ERR_INT_EN_R {
        DFI_ALERT_ERR_INT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DFI_ALERT_ERR_INT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_int_clr(&self) -> DFI_ALERT_ERR_INT_CLR_R {
        DFI_ALERT_ERR_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFI_ALERT_ERR_CNT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt_clr(&self) -> DFI_ALERT_ERR_CNT_CLR_R {
        DFI_ALERT_ERR_CNT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_ALERT_ERR_INT_EN"]
    #[inline(always)]
    pub fn dfi_alert_err_int_en(&mut self) -> DFI_ALERT_ERR_INT_EN_W {
        DFI_ALERT_ERR_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - DFI_ALERT_ERR_INT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_int_clr(&mut self) -> DFI_ALERT_ERR_INT_CLR_W {
        DFI_ALERT_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - DFI_ALERT_ERR_CNT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt_clr(&mut self) -> DFI_ALERT_ERR_CNT_CLR_W {
        DFI_ALERT_ERR_CNT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL CRC parity control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_crcparctl0](index.html) module"]
pub struct DDRCTRL_CRCPARCTL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_CRCPARCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_crcparctl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_CRCPARCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_crcparctl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_CRCPARCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_CRCPARCTL0 to value 0"]
impl crate::Resettable for DDRCTRL_CRCPARCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
