#[doc = "Register `OTG_FS_GAHBCFG` reader"]
pub struct R(crate::R<OTG_FS_GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GAHBCFG` writer"]
pub struct W(crate::W<OTG_FS_GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GAHBCFG_SPEC>;
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
impl From<crate::W<OTG_FS_GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub struct GINT_R(crate::FieldReader<bool, bool>);
impl GINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub struct GINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_W<'a> {
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
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub struct TXFELVL_R(crate::FieldReader<bool, bool>);
impl TXFELVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFELVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFELVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub struct TXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFELVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub struct PTXFELVL_R(crate::FieldReader<bool, bool>);
impl PTXFELVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFELVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFELVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub struct PTXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFELVL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W {
        GINT_W { w: self }
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W {
        TXFELVL_W { w: self }
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W {
        PTXFELVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gahbcfg](index.html) module"]
pub struct OTG_FS_GAHBCFG_SPEC;
impl crate::RegisterSpec for OTG_FS_GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_gahbcfg::R](R) reader structure"]
impl crate::Readable for OTG_FS_GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_gahbcfg::W](W) writer structure"]
impl crate::Writable for OTG_FS_GAHBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GAHBCFG to value 0"]
impl crate::Resettable for OTG_FS_GAHBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
