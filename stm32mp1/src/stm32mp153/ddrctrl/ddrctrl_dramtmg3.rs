#[doc = "Register `DDRCTRL_DRAMTMG3` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG3` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG3_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_MOD` reader - T_MOD"]
pub struct T_MOD_R(crate::FieldReader<u16, u16>);
impl T_MOD_R {
    pub(crate) fn new(bits: u16) -> Self {
        T_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_MOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_MOD` writer - T_MOD"]
pub struct T_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> T_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `T_MRD` reader - T_MRD"]
pub struct T_MRD_R(crate::FieldReader<u8, u8>);
impl T_MRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_MRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_MRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_MRD` writer - T_MRD"]
pub struct T_MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> T_MRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `T_MRW` reader - T_MRW"]
pub struct T_MRW_R(crate::FieldReader<u16, u16>);
impl T_MRW_R {
    pub(crate) fn new(bits: u16) -> Self {
        T_MRW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_MRW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_MRW` writer - T_MRW"]
pub struct T_MRW_W<'a> {
    w: &'a mut W,
}
impl<'a> T_MRW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    pub fn t_mod(&self) -> T_MOD_R {
        T_MOD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    pub fn t_mrd(&self) -> T_MRD_R {
        T_MRD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    pub fn t_mrw(&self) -> T_MRW_R {
        T_MRW_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    pub fn t_mod(&mut self) -> T_MOD_W {
        T_MOD_W { w: self }
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    pub fn t_mrd(&mut self) -> T_MRD_W {
        T_MRD_W { w: self }
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    pub fn t_mrw(&mut self) -> T_MRW_W {
        T_MRW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg3](index.html) module"]
pub struct DDRCTRL_DRAMTMG3_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg3::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg3::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG3 to value 0x0050_400c"]
impl crate::Resettable for DDRCTRL_DRAMTMG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0050_400c
    }
}
