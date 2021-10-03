#[doc = "Register `GPIOZ_SECCFGR` writer"]
pub struct W(crate::W<GPIOZ_SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOZ_SECCFGR_SPEC>;
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
impl From<crate::W<GPIOZ_SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOZ_SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` writer - SEC0"]
pub struct SEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC0_W<'a> {
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
#[doc = "Field `SEC1` writer - SEC1"]
pub struct SEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC1_W<'a> {
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
#[doc = "Field `SEC2` writer - SEC2"]
pub struct SEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC2_W<'a> {
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
#[doc = "Field `SEC3` writer - SEC3"]
pub struct SEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC3_W<'a> {
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
#[doc = "Field `SEC4` writer - SEC4"]
pub struct SEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC4_W<'a> {
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
#[doc = "Field `SEC5` writer - SEC5"]
pub struct SEC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC5_W<'a> {
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
#[doc = "Field `SEC6` writer - SEC6"]
pub struct SEC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC6_W<'a> {
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
#[doc = "Field `SEC7` writer - SEC7"]
pub struct SEC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC7_W<'a> {
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
impl W {
    #[doc = "Bit 0 - SEC0"]
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W {
        SEC0_W { w: self }
    }
    #[doc = "Bit 1 - SEC1"]
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W {
        SEC1_W { w: self }
    }
    #[doc = "Bit 2 - SEC2"]
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W {
        SEC2_W { w: self }
    }
    #[doc = "Bit 3 - SEC3"]
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W {
        SEC3_W { w: self }
    }
    #[doc = "Bit 4 - SEC4"]
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W {
        SEC4_W { w: self }
    }
    #[doc = "Bit 5 - SEC5"]
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W {
        SEC5_W { w: self }
    }
    #[doc = "Bit 6 - SEC6"]
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W {
        SEC6_W { w: self }
    }
    #[doc = "Bit 7 - SEC7"]
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W {
        SEC7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_seccfgr](index.html) module"]
pub struct GPIOZ_SECCFGR_SPEC;
impl crate::RegisterSpec for GPIOZ_SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpioz_seccfgr::W](W) writer structure"]
impl crate::Writable for GPIOZ_SECCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOZ_SECCFGR to value 0xff"]
impl crate::Resettable for GPIOZ_SECCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
