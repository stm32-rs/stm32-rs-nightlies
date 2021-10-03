#[doc = "Register `DDRCTRL_DRAMTMG6` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG6` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG6_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_CKCSX` reader - T_CKCSX"]
pub struct T_CKCSX_R(crate::FieldReader<u8, u8>);
impl T_CKCSX_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKCSX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKCSX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKCSX` writer - T_CKCSX"]
pub struct T_CKCSX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKCSX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `T_CKDPDX` reader - T_CKDPDX"]
pub struct T_CKDPDX_R(crate::FieldReader<u8, u8>);
impl T_CKDPDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKDPDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKDPDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKDPDX` writer - T_CKDPDX"]
pub struct T_CKDPDX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKDPDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `T_CKDPDE` reader - T_CKDPDE"]
pub struct T_CKDPDE_R(crate::FieldReader<u8, u8>);
impl T_CKDPDE_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKDPDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKDPDE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKDPDE` writer - T_CKDPDE"]
pub struct T_CKDPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKDPDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - T_CKCSX"]
    #[inline(always)]
    pub fn t_ckcsx(&self) -> T_CKCSX_R {
        T_CKCSX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - T_CKDPDX"]
    #[inline(always)]
    pub fn t_ckdpdx(&self) -> T_CKDPDX_R {
        T_CKDPDX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - T_CKDPDE"]
    #[inline(always)]
    pub fn t_ckdpde(&self) -> T_CKDPDE_R {
        T_CKDPDE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - T_CKCSX"]
    #[inline(always)]
    pub fn t_ckcsx(&mut self) -> T_CKCSX_W {
        T_CKCSX_W { w: self }
    }
    #[doc = "Bits 16:19 - T_CKDPDX"]
    #[inline(always)]
    pub fn t_ckdpdx(&mut self) -> T_CKDPDX_W {
        T_CKDPDX_W { w: self }
    }
    #[doc = "Bits 24:27 - T_CKDPDE"]
    #[inline(always)]
    pub fn t_ckdpde(&mut self) -> T_CKDPDE_W {
        T_CKDPDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg6](index.html) module"]
pub struct DDRCTRL_DRAMTMG6_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg6::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg6::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG6 to value 0x0202_0005"]
impl crate::Resettable for DDRCTRL_DRAMTMG6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0202_0005
    }
}
