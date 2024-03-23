#[doc = "Register `ETH_MACTSCR` reader"]
pub type R = crate::R<ETH_MACTSCRrs>;
#[doc = "Register `ETH_MACTSCR` writer"]
pub type W = crate::W<ETH_MACTSCRrs>;
#[doc = "Field `TSENA` reader - TSENA"]
pub type TSENA_R = crate::BitReader;
#[doc = "Field `TSENA` writer - TSENA"]
pub type TSENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCFUPDT` reader - TSCFUPDT"]
pub type TSCFUPDT_R = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - TSCFUPDT"]
pub type TSCFUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINIT` reader - TSINIT"]
pub type TSINIT_R = crate::BitReader;
#[doc = "Field `TSINIT` writer - TSINIT"]
pub type TSINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUPDT` reader - TSUPDT"]
pub type TSUPDT_R = crate::BitReader;
#[doc = "Field `TSUPDT` writer - TSUPDT"]
pub type TSUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSADDREG` reader - TSADDREG"]
pub type TSADDREG_R = crate::BitReader;
#[doc = "Field `TSADDREG` writer - TSADDREG"]
pub type TSADDREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENALL` reader - TSENALL"]
pub type TSENALL_R = crate::BitReader;
#[doc = "Field `TSENALL` writer - TSENALL"]
pub type TSENALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRLSSR` reader - TSCTRLSSR"]
pub type TSCTRLSSR_R = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - TSCTRLSSR"]
pub type TSCTRLSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVER2ENA` reader - TSVER2ENA"]
pub type TSVER2ENA_R = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - TSVER2ENA"]
pub type TSVER2ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPENA` reader - TSIPENA"]
pub type TSIPENA_R = crate::BitReader;
#[doc = "Field `TSIPENA` writer - TSIPENA"]
pub type TSIPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV6ENA` reader - TSIPV6ENA"]
pub type TSIPV6ENA_R = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - TSIPV6ENA"]
pub type TSIPV6ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV4ENA` reader - TSIPV4ENA"]
pub type TSIPV4ENA_R = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - TSIPV4ENA"]
pub type TSIPV4ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEVNTENA` reader - TSEVNTENA"]
pub type TSEVNTENA_R = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - TSEVNTENA"]
pub type TSEVNTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMSTRENA` reader - TSMSTRENA"]
pub type TSMSTRENA_R = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - TSMSTRENA"]
pub type TSMSTRENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPTYPSEL` reader - SNAPTYPSEL"]
pub type SNAPTYPSEL_R = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - SNAPTYPSEL"]
pub type SNAPTYPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMACADDR` reader - TSENMACADDR"]
pub type TSENMACADDR_R = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - TSENMACADDR"]
pub type TSENMACADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - CSC"]
pub type CSC_R = crate::BitReader;
#[doc = "Field `TXTSSTSM` reader - TXTSSTSM"]
pub type TXTSSTSM_R = crate::BitReader;
#[doc = "Field `TXTSSTSM` writer - TXTSSTSM"]
pub type TXTSSTSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AV8021ASMEN` reader - AV8021ASMEN"]
pub type AV8021ASMEN_R = crate::BitReader;
#[doc = "Field `AV8021ASMEN` writer - AV8021ASMEN"]
pub type AV8021ASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    pub fn txtsstsm(&self) -> TXTSSTSM_R {
        TXTSSTSM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - AV8021ASMEN"]
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TSENA_W<ETH_MACTSCRrs> {
        TSENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    #[must_use]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<ETH_MACTSCRrs> {
        TSCFUPDT_W::new(self, 1)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    #[must_use]
    pub fn tsinit(&mut self) -> TSINIT_W<ETH_MACTSCRrs> {
        TSINIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    #[must_use]
    pub fn tsupdt(&mut self) -> TSUPDT_W<ETH_MACTSCRrs> {
        TSUPDT_W::new(self, 3)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    #[must_use]
    pub fn tsaddreg(&mut self) -> TSADDREG_W<ETH_MACTSCRrs> {
        TSADDREG_W::new(self, 5)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    #[must_use]
    pub fn tsenall(&mut self) -> TSENALL_W<ETH_MACTSCRrs> {
        TSENALL_W::new(self, 8)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<ETH_MACTSCRrs> {
        TSCTRLSSR_W::new(self, 9)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<ETH_MACTSCRrs> {
        TSVER2ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipena(&mut self) -> TSIPENA_W<ETH_MACTSCRrs> {
        TSIPENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<ETH_MACTSCRrs> {
        TSIPV6ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<ETH_MACTSCRrs> {
        TSIPV4ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W<ETH_MACTSCRrs> {
        TSEVNTENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<ETH_MACTSCRrs> {
        TSMSTRENA_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<ETH_MACTSCRrs> {
        SNAPTYPSEL_W::new(self, 16)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    #[must_use]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<ETH_MACTSCRrs> {
        TSENMACADDR_W::new(self, 18)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    #[must_use]
    pub fn txtsstsm(&mut self) -> TXTSSTSM_W<ETH_MACTSCRrs> {
        TXTSSTSM_W::new(self, 24)
    }
    #[doc = "Bit 28 - AV8021ASMEN"]
    #[inline(always)]
    #[must_use]
    pub fn av8021asmen(&mut self) -> AV8021ASMEN_W<ETH_MACTSCRrs> {
        AV8021ASMEN_W::new(self, 28)
    }
}
#[doc = "This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACTSCRrs;
impl crate::RegisterSpec for ETH_MACTSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mactscr::R`](R) reader structure"]
impl crate::Readable for ETH_MACTSCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mactscr::W`](W) writer structure"]
impl crate::Writable for ETH_MACTSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACTSCR to value 0x2000"]
impl crate::Resettable for ETH_MACTSCRrs {
    const RESET_VALUE: u32 = 0x2000;
}
