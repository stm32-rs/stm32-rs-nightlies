#[doc = "Register `FDCAN_TURCF` reader"]
pub struct R(crate::R<FDCAN_TURCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TURCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TURCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TURCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TURCF` writer"]
pub struct W(crate::W<FDCAN_TURCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TURCF_SPEC>;
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
impl From<crate::W<FDCAN_TURCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TURCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCL` reader - NCL"]
pub struct NCL_R(crate::FieldReader<u16, u16>);
impl NCL_R {
    pub(crate) fn new(bits: u16) -> Self {
        NCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCL` writer - NCL"]
pub struct NCL_W<'a> {
    w: &'a mut W,
}
impl<'a> NCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DC` reader - DC"]
pub struct DC_R(crate::FieldReader<u16, u16>);
impl DC_R {
    pub(crate) fn new(bits: u16) -> Self {
        DC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC` writer - DC"]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Field `ELT` reader - ELT"]
pub struct ELT_R(crate::FieldReader<bool, bool>);
impl ELT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELT` writer - ELT"]
pub struct ELT_W<'a> {
    w: &'a mut W,
}
impl<'a> ELT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - NCL"]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - DC"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - ELT"]
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - NCL"]
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W {
        NCL_W { w: self }
    }
    #[doc = "Bits 16:29 - DC"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bit 31 - ELT"]
    #[inline(always)]
    pub fn elt(&mut self) -> ELT_W {
        ELT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\\[17:16\\]
is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\\[15:0\\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_turcf](index.html) module"]
pub struct FDCAN_TURCF_SPEC;
impl crate::RegisterSpec for FDCAN_TURCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_turcf::R](R) reader structure"]
impl crate::Readable for FDCAN_TURCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_turcf::W](W) writer structure"]
impl crate::Writable for FDCAN_TURCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TURCF to value 0"]
impl crate::Resettable for FDCAN_TURCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
