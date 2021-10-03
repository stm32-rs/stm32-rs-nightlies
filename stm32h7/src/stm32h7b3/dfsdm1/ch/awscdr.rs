#[doc = "Register `AWSCDR` reader"]
pub struct R(crate::R<AWSCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWSCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWSCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWSCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWSCDR` writer"]
pub struct W(crate::W<AWSCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWSCDR_SPEC>;
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
impl From<crate::W<AWSCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWSCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDT` reader - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
pub struct SCDT_R(crate::FieldReader<u8, u8>);
impl SCDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDT` writer - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
pub struct SCDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BKSCD` reader - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
pub struct BKSCD_R(crate::FieldReader<u8, u8>);
impl BKSCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        BKSCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKSCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKSCD` writer - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
pub struct BKSCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BKSCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `AWFOSR` reader - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
pub struct AWFOSR_R(crate::FieldReader<u8, u8>);
impl AWFOSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWFOSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWFOSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWFOSR` writer - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
pub struct AWFOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFOSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AWFORD_A {
    #[doc = "0: FastSinc filter type"]
    B_0X0 = 0,
    #[doc = "1: Sinc1 filter type"]
    B_0X1 = 1,
}
impl From<AWFORD_A> for u8 {
    #[inline(always)]
    fn from(variant: AWFORD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AWFORD` reader - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct AWFORD_R(crate::FieldReader<u8, AWFORD_A>);
impl AWFORD_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWFORD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AWFORD_A> {
        match self.bits {
            0 => Some(AWFORD_A::B_0X0),
            1 => Some(AWFORD_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AWFORD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AWFORD_A::B_0X1
    }
}
impl core::ops::Deref for AWFORD_R {
    type Target = crate::FieldReader<u8, AWFORD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWFORD` writer - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct AWFORD_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWFORD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWFORD_A::B_0X0)
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWFORD_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W {
        SCDT_W { w: self }
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W {
        BKSCD_W { w: self }
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W {
        AWFOSR_W { w: self }
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W {
        AWFORD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel 0 analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awscdr](index.html) module"]
pub struct AWSCDR_SPEC;
impl crate::RegisterSpec for AWSCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awscdr::R](R) reader structure"]
impl crate::Readable for AWSCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awscdr::W](W) writer structure"]
impl crate::Writable for AWSCDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWSCDR to value 0"]
impl crate::Resettable for AWSCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
