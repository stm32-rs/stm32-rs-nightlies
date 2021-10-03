#[doc = "Register `DDRCTRL_MRCTRL1` reader"]
pub struct R(crate::R<DDRCTRL_MRCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MRCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MRCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MRCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_MRCTRL1` writer"]
pub struct W(crate::W<DDRCTRL_MRCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_MRCTRL1_SPEC>;
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
impl From<crate::W<DDRCTRL_MRCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_MRCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR_DATA` reader - MR_DATA"]
pub struct MR_DATA_R(crate::FieldReader<u16, u16>);
impl MR_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        MR_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR_DATA` writer - MR_DATA"]
pub struct MR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MR_DATA"]
    #[inline(always)]
    pub fn mr_data(&self) -> MR_DATA_R {
        MR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MR_DATA"]
    #[inline(always)]
    pub fn mr_data(&mut self) -> MR_DATA_W {
        MR_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL mode register read/write control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mrctrl1](index.html) module"]
pub struct DDRCTRL_MRCTRL1_SPEC;
impl crate::RegisterSpec for DDRCTRL_MRCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_mrctrl1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_MRCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mrctrl1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_MRCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_MRCTRL1 to value 0"]
impl crate::Resettable for DDRCTRL_MRCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
