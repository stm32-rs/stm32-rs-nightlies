#[doc = "Register `ETH_MTLTxQ1ECR` reader"]
pub struct R(crate::R<ETH_MTLTXQ1ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQ1ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQ1ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQ1ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTxQ1ECR` writer"]
pub struct W(crate::W<ETH_MTLTXQ1ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTXQ1ECR_SPEC>;
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
impl From<crate::W<ETH_MTLTXQ1ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTXQ1ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVALG` reader - AVALG"]
pub struct AVALG_R(crate::FieldReader<bool, bool>);
impl AVALG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVALG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVALG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVALG` writer - AVALG"]
pub struct AVALG_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALG_W<'a> {
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
#[doc = "Field `CC` reader - CC"]
pub struct CC_R(crate::FieldReader<bool, bool>);
impl CC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` writer - CC"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
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
#[doc = "Field `SLC` reader - SLC"]
pub struct SLC_R(crate::FieldReader<u8, u8>);
impl SLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC` writer - SLC"]
pub struct SLC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - AVALG"]
    #[inline(always)]
    pub fn avalg(&self) -> AVALG_R {
        AVALG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CC"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - SLC"]
    #[inline(always)]
    pub fn slc(&self) -> SLC_R {
        SLC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - AVALG"]
    #[inline(always)]
    pub fn avalg(&mut self) -> AVALG_W {
        AVALG_W { w: self }
    }
    #[doc = "Bit 3 - CC"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Bits 4:6 - SLC"]
    #[inline(always)]
    pub fn slc(&mut self) -> SLC_W {
        SLC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Queue ETS Control register controls the enhanced transmission selection operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1ecr](index.html) module"]
pub struct ETH_MTLTXQ1ECR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQ1ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q1ecr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1ecr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1ECR to value 0"]
impl crate::Resettable for ETH_MTLTXQ1ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
