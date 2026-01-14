///Register `MACTSCR` reader
pub type R = crate::R<MACTSCRrs>;
///Register `MACTSCR` writer
pub type W = crate::W<MACTSCRrs>;
///Field `TSENA` reader - TSENA
pub type TSENA_R = crate::BitReader;
///Field `TSENA` writer - TSENA
pub type TSENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCFUPDT` reader - TSCFUPDT
pub type TSCFUPDT_R = crate::BitReader;
///Field `TSCFUPDT` writer - TSCFUPDT
pub type TSCFUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSINIT` reader - TSINIT
pub type TSINIT_R = crate::BitReader;
///Field `TSINIT` writer - TSINIT
pub type TSINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSUPDT` reader - TSUPDT
pub type TSUPDT_R = crate::BitReader;
///Field `TSUPDT` writer - TSUPDT
pub type TSUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSADDREG` reader - TSADDREG
pub type TSADDREG_R = crate::BitReader;
///Field `TSADDREG` writer - TSADDREG
pub type TSADDREG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENALL` reader - TSENALL
pub type TSENALL_R = crate::BitReader;
///Field `TSENALL` writer - TSENALL
pub type TSENALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCTRLSSR` reader - TSCTRLSSR
pub type TSCTRLSSR_R = crate::BitReader;
///Field `TSCTRLSSR` writer - TSCTRLSSR
pub type TSCTRLSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSVER2ENA` reader - TSVER2ENA
pub type TSVER2ENA_R = crate::BitReader;
///Field `TSVER2ENA` writer - TSVER2ENA
pub type TSVER2ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIPENA` reader - TSIPENA
pub type TSIPENA_R = crate::BitReader;
///Field `TSIPENA` writer - TSIPENA
pub type TSIPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIPV6ENA` reader - TSIPV6ENA
pub type TSIPV6ENA_R = crate::BitReader;
///Field `TSIPV6ENA` writer - TSIPV6ENA
pub type TSIPV6ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIPV4ENA` reader - TSIPV4ENA
pub type TSIPV4ENA_R = crate::BitReader;
///Field `TSIPV4ENA` writer - TSIPV4ENA
pub type TSIPV4ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEVNTENA` reader - TSEVNTENA
pub type TSEVNTENA_R = crate::BitReader;
///Field `TSEVNTENA` writer - TSEVNTENA
pub type TSEVNTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSMSTRENA` reader - TSMSTRENA
pub type TSMSTRENA_R = crate::BitReader;
///Field `TSMSTRENA` writer - TSMSTRENA
pub type TSMSTRENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAPTYPSEL` reader - SNAPTYPSEL
pub type SNAPTYPSEL_R = crate::FieldReader;
///Field `SNAPTYPSEL` writer - SNAPTYPSEL
pub type SNAPTYPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TSENMACADDR` reader - TSENMACADDR
pub type TSENMACADDR_R = crate::BitReader;
///Field `TSENMACADDR` writer - TSENMACADDR
pub type TSENMACADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSC` reader - CSC
pub type CSC_R = crate::BitReader;
///Field `TXTSSTSM` reader - TXTSSTSM
pub type TXTSSTSM_R = crate::BitReader;
///Field `TXTSSTSM` writer - TXTSSTSM
pub type TXTSSTSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AV8021ASMEN` reader - AV8021ASMEN
pub type AV8021ASMEN_R = crate::BitReader;
///Field `AV8021ASMEN` writer - AV8021ASMEN
pub type AV8021ASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TSENA
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSCFUPDT
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TSINIT
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSUPDT
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - TSADDREG
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - TSENALL
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TSCTRLSSR
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TSVER2ENA
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TSIPENA
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TSIPV6ENA
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TSIPV4ENA
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TSEVNTENA
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TSMSTRENA
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - SNAPTYPSEL
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - TSENMACADDR
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CSC
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - TXTSSTSM
    #[inline(always)]
    pub fn txtsstsm(&self) -> TXTSSTSM_R {
        TXTSSTSM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - AV8021ASMEN
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSCR")
            .field("tsena", &self.tsena())
            .field("tscfupdt", &self.tscfupdt())
            .field("tsinit", &self.tsinit())
            .field("tsupdt", &self.tsupdt())
            .field("tsaddreg", &self.tsaddreg())
            .field("tsenall", &self.tsenall())
            .field("tsctrlssr", &self.tsctrlssr())
            .field("tsver2ena", &self.tsver2ena())
            .field("tsipena", &self.tsipena())
            .field("tsipv6ena", &self.tsipv6ena())
            .field("tsipv4ena", &self.tsipv4ena())
            .field("tsevntena", &self.tsevntena())
            .field("tsmstrena", &self.tsmstrena())
            .field("snaptypsel", &self.snaptypsel())
            .field("tsenmacaddr", &self.tsenmacaddr())
            .field("csc", &self.csc())
            .field("txtsstsm", &self.txtsstsm())
            .field("av8021asmen", &self.av8021asmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TSENA
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W<'_, MACTSCRrs> {
        TSENA_W::new(self, 0)
    }
    ///Bit 1 - TSCFUPDT
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<'_, MACTSCRrs> {
        TSCFUPDT_W::new(self, 1)
    }
    ///Bit 2 - TSINIT
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W<'_, MACTSCRrs> {
        TSINIT_W::new(self, 2)
    }
    ///Bit 3 - TSUPDT
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W<'_, MACTSCRrs> {
        TSUPDT_W::new(self, 3)
    }
    ///Bit 5 - TSADDREG
    #[inline(always)]
    pub fn tsaddreg(&mut self) -> TSADDREG_W<'_, MACTSCRrs> {
        TSADDREG_W::new(self, 5)
    }
    ///Bit 8 - TSENALL
    #[inline(always)]
    pub fn tsenall(&mut self) -> TSENALL_W<'_, MACTSCRrs> {
        TSENALL_W::new(self, 8)
    }
    ///Bit 9 - TSCTRLSSR
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<'_, MACTSCRrs> {
        TSCTRLSSR_W::new(self, 9)
    }
    ///Bit 10 - TSVER2ENA
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<'_, MACTSCRrs> {
        TSVER2ENA_W::new(self, 10)
    }
    ///Bit 11 - TSIPENA
    #[inline(always)]
    pub fn tsipena(&mut self) -> TSIPENA_W<'_, MACTSCRrs> {
        TSIPENA_W::new(self, 11)
    }
    ///Bit 12 - TSIPV6ENA
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<'_, MACTSCRrs> {
        TSIPV6ENA_W::new(self, 12)
    }
    ///Bit 13 - TSIPV4ENA
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<'_, MACTSCRrs> {
        TSIPV4ENA_W::new(self, 13)
    }
    ///Bit 14 - TSEVNTENA
    #[inline(always)]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W<'_, MACTSCRrs> {
        TSEVNTENA_W::new(self, 14)
    }
    ///Bit 15 - TSMSTRENA
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<'_, MACTSCRrs> {
        TSMSTRENA_W::new(self, 15)
    }
    ///Bits 16:17 - SNAPTYPSEL
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<'_, MACTSCRrs> {
        SNAPTYPSEL_W::new(self, 16)
    }
    ///Bit 18 - TSENMACADDR
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<'_, MACTSCRrs> {
        TSENMACADDR_W::new(self, 18)
    }
    ///Bit 24 - TXTSSTSM
    #[inline(always)]
    pub fn txtsstsm(&mut self) -> TXTSSTSM_W<'_, MACTSCRrs> {
        TXTSSTSM_W::new(self, 24)
    }
    ///Bit 28 - AV8021ASMEN
    #[inline(always)]
    pub fn av8021asmen(&mut self) -> AV8021ASMEN_W<'_, MACTSCRrs> {
        AV8021ASMEN_W::new(self, 28)
    }
}
/**This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.

You can [`read`](crate::Reg::read) this register and get [`mactscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSCR)*/
pub struct MACTSCRrs;
impl crate::RegisterSpec for MACTSCRrs {
    type Ux = u32;
}
///`read()` method returns [`mactscr::R`](R) reader structure
impl crate::Readable for MACTSCRrs {}
///`write(|w| ..)` method takes [`mactscr::W`](W) writer structure
impl crate::Writable for MACTSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSCR to value 0x2000
impl crate::Resettable for MACTSCRrs {
    const RESET_VALUE: u32 = 0x2000;
}
