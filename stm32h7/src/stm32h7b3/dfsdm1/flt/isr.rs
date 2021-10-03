#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOCF_A {
    #[doc = "0: No injected conversion has completed"]
    B_0X0 = 0,
    #[doc = "1: An injected conversion has completed and its data may be read"]
    B_0X1 = 1,
}
impl From<JEOCF_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOCF` reader - End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR."]
pub struct JEOCF_R(crate::FieldReader<bool, JEOCF_A>);
impl JEOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCF_A {
        match self.bits {
            false => JEOCF_A::B_0X0,
            true => JEOCF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JEOCF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JEOCF_A::B_0X1
    }
}
impl core::ops::Deref for JEOCF_R {
    type Target = crate::FieldReader<bool, JEOCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REOCF_A {
    #[doc = "0: No regular conversion has completed"]
    B_0X0 = 0,
    #[doc = "1: A regular conversion has completed and its data may be read"]
    B_0X1 = 1,
}
impl From<REOCF_A> for bool {
    #[inline(always)]
    fn from(variant: REOCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REOCF` reader - End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR."]
pub struct REOCF_R(crate::FieldReader<bool, REOCF_A>);
impl REOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REOCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REOCF_A {
        match self.bits {
            false => REOCF_A::B_0X0,
            true => REOCF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == REOCF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == REOCF_A::B_0X1
    }
}
impl core::ops::Deref for REOCF_R {
    type Target = crate::FieldReader<bool, REOCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JOVRF_A {
    #[doc = "0: No injected conversion overrun has occurred"]
    B_0X0 = 0,
    #[doc = "1: An injected conversion overrun has occurred, which means that an injected conversion finished while JEOCF was already '1â\u{80}\u{99}. JDATAR is not affected by overruns"]
    B_0X1 = 1,
}
impl From<JOVRF_A> for bool {
    #[inline(always)]
    fn from(variant: JOVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register."]
pub struct JOVRF_R(crate::FieldReader<bool, JOVRF_A>);
impl JOVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        JOVRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVRF_A {
        match self.bits {
            false => JOVRF_A::B_0X0,
            true => JOVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JOVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JOVRF_A::B_0X1
    }
}
impl core::ops::Deref for JOVRF_R {
    type Target = crate::FieldReader<bool, JOVRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVRF_A {
    #[doc = "0: No regular conversion overrun has occurred"]
    B_0X0 = 0,
    #[doc = "1: A regular conversion overrun has occurred, which means that a regular conversion finished while REOCF was already '1â\u{80}\u{99}. RDATAR is not affected by overruns"]
    B_0X1 = 1,
}
impl From<ROVRF_A> for bool {
    #[inline(always)]
    fn from(variant: ROVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register."]
pub struct ROVRF_R(crate::FieldReader<bool, ROVRF_A>);
impl ROVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVRF_A {
        match self.bits {
            false => ROVRF_A::B_0X0,
            true => ROVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ROVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ROVRF_A::B_0X1
    }
}
impl core::ops::Deref for ROVRF_R {
    type Target = crate::FieldReader<bool, ROVRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\\[7:0\\]
and AWLTF\\[7:0\\]
in DFSDM_FLTxAWSR register (by writing '1â\u{80}\u{99} into the clear bits in DFSDM_FLTxAWCFR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDF_A {
    #[doc = "0: No Analog watchdog event occurred"]
    B_0X0 = 0,
    #[doc = "1: The analog watchdog block detected voltage which crosses the value programmed in the DFSDM_FLTxAWLTR or DFSDM_FLTxAWHTR registers."]
    B_0X1 = 1,
}
impl From<AWDF_A> for bool {
    #[inline(always)]
    fn from(variant: AWDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDF` reader - Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\\[7:0\\]
and AWLTF\\[7:0\\]
in DFSDM_FLTxAWSR register (by writing '1â\u{80}\u{99} into the clear bits in DFSDM_FLTxAWCFR register)."]
pub struct AWDF_R(crate::FieldReader<bool, AWDF_A>);
impl AWDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDF_A {
        match self.bits {
            false => AWDF_A::B_0X0,
            true => AWDF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AWDF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AWDF_A::B_0X1
    }
}
impl core::ops::Deref for AWDF_R {
    type Target = crate::FieldReader<bool, AWDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JCIP_A {
    #[doc = "0: No request to convert the injected channel group (neither by software nor by trigger) has been issued"]
    B_0X0 = 0,
    #[doc = "1: The conversion of the injected channel group is in progress or a request for a injected conversion is pending, due either to '1â\u{80}\u{99} being written to JSWSTART or to a trigger detection"]
    B_0X1 = 1,
}
impl From<JCIP_A> for bool {
    #[inline(always)]
    fn from(variant: JCIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JCIP` reader - Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1."]
pub struct JCIP_R(crate::FieldReader<bool, JCIP_A>);
impl JCIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        JCIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JCIP_A {
        match self.bits {
            false => JCIP_A::B_0X0,
            true => JCIP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JCIP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JCIP_A::B_0X1
    }
}
impl core::ops::Deref for JCIP_R {
    type Target = crate::FieldReader<bool, JCIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCIP_A {
    #[doc = "0: No request to convert the regular channel has been issued"]
    B_0X0 = 0,
    #[doc = "1: The conversion of the regular channel is in progress or a request for a regular conversion is pending"]
    B_0X1 = 1,
}
impl From<RCIP_A> for bool {
    #[inline(always)]
    fn from(variant: RCIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCIP` reader - Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1."]
pub struct RCIP_R(crate::FieldReader<bool, RCIP_A>);
impl RCIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCIP_A {
        match self.bits {
            false => RCIP_A::B_0X0,
            true => RCIP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RCIP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RCIP_A::B_0X1
    }
}
impl core::ops::Deref for RCIP_R {
    type Target = crate::FieldReader<bool, RCIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKABF` reader - Clock absence flag CKABF\\[y\\]=0: Clock signal on channel y is present. CKABF\\[y\\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\\[y\\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\\[y\\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\\[y\\]
bit in the DFSDM_FLTxICR register. Note: CKABF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)"]
pub struct CKABF_R(crate::FieldReader<u8, u8>);
impl CKABF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKABF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKABF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDF` reader - short-circuit detector flag SDCF\\[y\\]=0: No short-circuit detector event occurred on channel y SDCF\\[y\\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\\[y\\]
bit in the DFSDM_FLTxICR register. SCDF\\[y\\]
is cleared also by hardware when CHEN\\[y\\]
= 0 (given channel is disabled). Note: SCDF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)"]
pub struct SCDF_R(crate::FieldReader<u8, u8>);
impl SCDF_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR."]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR."]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register."]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register."]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\\[7:0\\]
and AWLTF\\[7:0\\]
in DFSDM_FLTxAWSR register (by writing '1â\u{80}\u{99} into the clear bits in DFSDM_FLTxAWCFR register)."]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1."]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1."]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Clock absence flag CKABF\\[y\\]=0: Clock signal on channel y is present. CKABF\\[y\\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\\[y\\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\\[y\\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\\[y\\]
bit in the DFSDM_FLTxICR register. Note: CKABF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - short-circuit detector flag SDCF\\[y\\]=0: No short-circuit detector event occurred on channel y SDCF\\[y\\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\\[y\\]
bit in the DFSDM_FLTxICR register. SCDF\\[y\\]
is cleared also by hardware when CHEN\\[y\\]
= 0 (given channel is disabled). Note: SCDF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0x00ff_0000"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_0000
    }
}
