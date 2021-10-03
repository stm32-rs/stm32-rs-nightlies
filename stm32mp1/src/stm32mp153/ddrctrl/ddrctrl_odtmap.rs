#[doc = "Register `DDRCTRL_ODTMAP` reader"]
pub struct R(crate::R<DDRCTRL_ODTMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ODTMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ODTMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ODTMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ODTMAP` writer"]
pub struct W(crate::W<DDRCTRL_ODTMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ODTMAP_SPEC>;
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
impl From<crate::W<DDRCTRL_ODTMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ODTMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RANK0_WR_ODT` reader - RANK0_WR_ODT"]
pub struct RANK0_WR_ODT_R(crate::FieldReader<bool, bool>);
impl RANK0_WR_ODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANK0_WR_ODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANK0_WR_ODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANK0_WR_ODT` writer - RANK0_WR_ODT"]
pub struct RANK0_WR_ODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RANK0_WR_ODT_W<'a> {
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
#[doc = "Field `RANK0_RD_ODT` reader - RANK0_RD_ODT"]
pub struct RANK0_RD_ODT_R(crate::FieldReader<bool, bool>);
impl RANK0_RD_ODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANK0_RD_ODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANK0_RD_ODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANK0_RD_ODT` writer - RANK0_RD_ODT"]
pub struct RANK0_RD_ODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RANK0_RD_ODT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RANK0_WR_ODT"]
    #[inline(always)]
    pub fn rank0_wr_odt(&self) -> RANK0_WR_ODT_R {
        RANK0_WR_ODT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - RANK0_RD_ODT"]
    #[inline(always)]
    pub fn rank0_rd_odt(&self) -> RANK0_RD_ODT_R {
        RANK0_RD_ODT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RANK0_WR_ODT"]
    #[inline(always)]
    pub fn rank0_wr_odt(&mut self) -> RANK0_WR_ODT_W {
        RANK0_WR_ODT_W { w: self }
    }
    #[doc = "Bit 4 - RANK0_RD_ODT"]
    #[inline(always)]
    pub fn rank0_rd_odt(&mut self) -> RANK0_RD_ODT_W {
        RANK0_RD_ODT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL ODT/Rank map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_odtmap](index.html) module"]
pub struct DDRCTRL_ODTMAP_SPEC;
impl crate::RegisterSpec for DDRCTRL_ODTMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_odtmap::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ODTMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_odtmap::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ODTMAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_ODTMAP to value 0x11"]
impl crate::Resettable for DDRCTRL_ODTMAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
