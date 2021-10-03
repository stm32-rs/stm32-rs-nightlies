#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC1` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC2` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC3` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC4` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC5` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC6` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC7` writer - I/O pin of Port x secure bit enable"]
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
#[doc = "Field `SEC8` writer - I/O pin of Port x secure bit enable"]
pub struct SEC8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC8_W<'a> {
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
#[doc = "Field `SEC9` writer - I/O pin of Port x secure bit enable"]
pub struct SEC9_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC9_W<'a> {
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
#[doc = "Field `SEC10` writer - I/O pin of Port x secure bit enable"]
pub struct SEC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC10_W<'a> {
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
#[doc = "Field `SEC11` writer - I/O pin of Port x secure bit enable"]
pub struct SEC11_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC11_W<'a> {
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
#[doc = "Field `SEC12` writer - I/O pin of Port x secure bit enable"]
pub struct SEC12_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC12_W<'a> {
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
#[doc = "Field `SEC13` writer - I/O pin of Port x secure bit enable"]
pub struct SEC13_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC13_W<'a> {
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
#[doc = "Field `SEC14` writer - I/O pin of Port x secure bit enable"]
pub struct SEC14_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC14_W<'a> {
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
#[doc = "Field `SEC15` writer - I/O pin of Port x secure bit enable"]
pub struct SEC15_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC15_W<'a> {
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
impl W {
    #[doc = "Bit 0 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W {
        SEC0_W { w: self }
    }
    #[doc = "Bit 1 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W {
        SEC1_W { w: self }
    }
    #[doc = "Bit 2 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W {
        SEC2_W { w: self }
    }
    #[doc = "Bit 3 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W {
        SEC3_W { w: self }
    }
    #[doc = "Bit 4 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W {
        SEC4_W { w: self }
    }
    #[doc = "Bit 5 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W {
        SEC5_W { w: self }
    }
    #[doc = "Bit 6 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W {
        SEC6_W { w: self }
    }
    #[doc = "Bit 7 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W {
        SEC7_W { w: self }
    }
    #[doc = "Bit 8 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec8(&mut self) -> SEC8_W {
        SEC8_W { w: self }
    }
    #[doc = "Bit 9 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec9(&mut self) -> SEC9_W {
        SEC9_W { w: self }
    }
    #[doc = "Bit 10 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W {
        SEC10_W { w: self }
    }
    #[doc = "Bit 11 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec11(&mut self) -> SEC11_W {
        SEC11_W { w: self }
    }
    #[doc = "Bit 12 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec12(&mut self) -> SEC12_W {
        SEC12_W { w: self }
    }
    #[doc = "Bit 13 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W {
        SEC13_W { w: self }
    }
    #[doc = "Bit 14 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W {
        SEC14_W { w: self }
    }
    #[doc = "Bit 15 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W {
        SEC15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO secure configuration register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
