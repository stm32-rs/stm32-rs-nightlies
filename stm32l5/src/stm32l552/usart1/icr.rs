#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `PECF` writer - Parity error clear flag"]
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - Framing error clear flag"]
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCF` writer - Noise detected clear flag"]
pub type NCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - Overrun error clear flag"]
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - Idle line detected clear flag"]
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFECF` writer - TXFECF"]
pub type TXFECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - Transmission complete clear flag"]
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBGTCF` writer - TCBGTCF"]
pub type TCBGTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDCF` writer - LIN break detection clear flag"]
pub type LBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTS clear flag"]
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOCF` writer - Receiver timeout clear flag"]
pub type RTOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBCF` writer - End of block clear flag"]
pub type EOBCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRCF` writer - UDRCF"]
pub type UDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - Character match clear flag"]
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - Wakeup from Stop mode clear flag"]
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PECF_W<ICRrs> {
        PECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FECF_W<ICRrs> {
        FECF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncf(&mut self) -> NCF_W<ICRrs> {
        NCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> ORECF_W<ICRrs> {
        ORECF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IDLECF_W<ICRrs> {
        IDLECF_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFECF"]
    #[inline(always)]
    #[must_use]
    pub fn txfecf(&mut self) -> TXFECF_W<ICRrs> {
        TXFECF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TCCF_W<ICRrs> {
        TCCF_W::new(self, 6)
    }
    #[doc = "Bit 7 - TCBGTCF"]
    #[inline(always)]
    #[must_use]
    pub fn tcbgtcf(&mut self) -> TCBGTCF_W<ICRrs> {
        TCBGTCF_W::new(self, 7)
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbdcf(&mut self) -> LBDCF_W<ICRrs> {
        LBDCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<ICRrs> {
        CTSCF_W::new(self, 9)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtocf(&mut self) -> RTOCF_W<ICRrs> {
        RTOCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - End of block clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn eobcf(&mut self) -> EOBCF_W<ICRrs> {
        EOBCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - UDRCF"]
    #[inline(always)]
    #[must_use]
    pub fn udrcf(&mut self) -> UDRCF_W<ICRrs> {
        UDRCF_W::new(self, 13)
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CMCF_W<ICRrs> {
        CMCF_W::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WUCF_W<ICRrs> {
        WUCF_W::new(self, 20)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
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
