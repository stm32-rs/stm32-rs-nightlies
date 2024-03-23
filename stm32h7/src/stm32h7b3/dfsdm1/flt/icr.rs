#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_R = crate::BitReader;
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type CLRROVRF_R = crate::BitReader;
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCKABF` reader - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub type CLRCKABF_R = crate::FieldReader;
#[doc = "Field `CLRCKABF` writer - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub type CLRCKABF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLRSCDF` reader - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub type CLRSCDF_R = crate::FieldReader;
#[doc = "Field `CLRSCDF` writer - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub type CLRSCDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    #[must_use]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    #[must_use]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
