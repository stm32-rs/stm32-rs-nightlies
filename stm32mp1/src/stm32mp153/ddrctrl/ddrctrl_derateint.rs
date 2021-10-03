#[doc = "Register `DDRCTRL_DERATEINT` reader"]
pub struct R(crate::R<DDRCTRL_DERATEINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DERATEINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DERATEINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DERATEINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DERATEINT` writer"]
pub struct W(crate::W<DDRCTRL_DERATEINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DERATEINT_SPEC>;
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
impl From<crate::W<DDRCTRL_DERATEINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DERATEINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR4_READ_INTERVAL` reader - MR4_READ_INTERVAL"]
pub struct MR4_READ_INTERVAL_R(crate::FieldReader<u32, u32>);
impl MR4_READ_INTERVAL_R {
    pub(crate) fn new(bits: u32) -> Self {
        MR4_READ_INTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR4_READ_INTERVAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR4_READ_INTERVAL` writer - MR4_READ_INTERVAL"]
pub struct MR4_READ_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MR4_READ_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MR4_READ_INTERVAL"]
    #[inline(always)]
    pub fn mr4_read_interval(&self) -> MR4_READ_INTERVAL_R {
        MR4_READ_INTERVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MR4_READ_INTERVAL"]
    #[inline(always)]
    pub fn mr4_read_interval(&mut self) -> MR4_READ_INTERVAL_W {
        MR4_READ_INTERVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL temperature derate interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_derateint](index.html) module"]
pub struct DDRCTRL_DERATEINT_SPEC;
impl crate::RegisterSpec for DDRCTRL_DERATEINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_derateint::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DERATEINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_derateint::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DERATEINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DERATEINT to value 0x0080_0000"]
impl crate::Resettable for DDRCTRL_DERATEINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
