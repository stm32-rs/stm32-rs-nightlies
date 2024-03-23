#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `JEOCF` reader - End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR."]
pub type JEOCF_R = crate::BitReader;
#[doc = "Field `REOCF` reader - End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR."]
pub type REOCF_R = crate::BitReader;
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register."]
pub type JOVRF_R = crate::BitReader;
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register."]
pub type ROVRF_R = crate::BitReader;
#[doc = "Field `AWDF` reader - Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\\[7:0\\]
and AWLTF\\[7:0\\]
in DFSDM_FLTxAWSR register (by writing '1â\u{80}\u{99} into the clear bits in DFSDM_FLTxAWCFR register)."]
pub type AWDF_R = crate::BitReader;
#[doc = "Field `JCIP` reader - Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1."]
pub type JCIP_R = crate::BitReader;
#[doc = "Field `RCIP` reader - Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1."]
pub type RCIP_R = crate::BitReader;
#[doc = "Field `CKABF` reader - Clock absence flag CKABF\\[y\\]=0: Clock signal on channel y is present. CKABF\\[y\\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\\[y\\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\\[y\\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\\[y\\]
bit in the DFSDM_FLTxICR register. Note: CKABF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)"]
pub type CKABF_R = crate::FieldReader;
#[doc = "Field `SCDF` reader - short-circuit detector flag SDCF\\[y\\]=0: No short-circuit detector event occurred on channel y SDCF\\[y\\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\\[y\\]
bit in the DFSDM_FLTxICR register. SCDF\\[y\\]
is cleared also by hardware when CHEN\\[y\\]
= 0 (given channel is disabled). Note: SCDF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)"]
pub type SCDF_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR."]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR."]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register."]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register."]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\\[7:0\\]
and AWLTF\\[7:0\\]
in DFSDM_FLTxAWSR register (by writing '1â\u{80}\u{99} into the clear bits in DFSDM_FLTxAWCFR register)."]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1."]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1."]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0x00ff_0000"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
