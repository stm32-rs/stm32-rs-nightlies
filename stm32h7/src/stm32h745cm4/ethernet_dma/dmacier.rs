///Register `DMACIER` reader
pub type R = crate::R<DMACIERrs>;
///Register `DMACIER` writer
pub type W = crate::W<DMACIERrs>;
///Field `TIE` reader - Transmit Interrupt Enable
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - Transmit Interrupt Enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSE` reader - Transmit Stopped Enable
pub type TXSE_R = crate::BitReader;
///Field `TXSE` writer - Transmit Stopped Enable
pub type TXSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBUE` reader - Transmit Buffer Unavailable Enable
pub type TBUE_R = crate::BitReader;
///Field `TBUE` writer - Transmit Buffer Unavailable Enable
pub type TBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIE` reader - Receive Interrupt Enable
pub type RIE_R = crate::BitReader;
///Field `RIE` writer - Receive Interrupt Enable
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBUE` reader - Receive Buffer Unavailable Enable
pub type RBUE_R = crate::BitReader;
///Field `RBUE` writer - Receive Buffer Unavailable Enable
pub type RBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSE` reader - Receive Stopped Enable
pub type RSE_R = crate::BitReader;
///Field `RSE` writer - Receive Stopped Enable
pub type RSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWTE` reader - Receive Watchdog Timeout Enable
pub type RWTE_R = crate::BitReader;
///Field `RWTE` writer - Receive Watchdog Timeout Enable
pub type RWTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETIE` reader - Early Transmit Interrupt Enable
pub type ETIE_R = crate::BitReader;
///Field `ETIE` writer - Early Transmit Interrupt Enable
pub type ETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERIE` reader - Early Receive Interrupt Enable
pub type ERIE_R = crate::BitReader;
///Field `ERIE` writer - Early Receive Interrupt Enable
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FBEE` reader - Fatal Bus Error Enable
pub type FBEE_R = crate::BitReader;
///Field `FBEE` writer - Fatal Bus Error Enable
pub type FBEE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDEE` reader - Context Descriptor Error Enable
pub type CDEE_R = crate::BitReader;
///Field `CDEE` writer - Context Descriptor Error Enable
pub type CDEE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AIE` reader - Abnormal Interrupt Summary Enable
pub type AIE_R = crate::BitReader;
///Field `AIE` writer - Abnormal Interrupt Summary Enable
pub type AIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NIE` reader - Normal Interrupt Summary Enable
pub type NIE_R = crate::BitReader;
///Field `NIE` writer - Normal Interrupt Summary Enable
pub type NIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit Stopped Enable
    #[inline(always)]
    pub fn txse(&self) -> TXSE_R {
        TXSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit Buffer Unavailable Enable
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive Buffer Unavailable Enable
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Receive Stopped Enable
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Receive Watchdog Timeout Enable
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Early Transmit Interrupt Enable
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Early Receive Interrupt Enable
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Fatal Bus Error Enable
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Context Descriptor Error Enable
    #[inline(always)]
    pub fn cdee(&self) -> CDEE_R {
        CDEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Abnormal Interrupt Summary Enable
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Normal Interrupt Summary Enable
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACIER")
            .field("nie", &self.nie())
            .field("aie", &self.aie())
            .field("cdee", &self.cdee())
            .field("fbee", &self.fbee())
            .field("erie", &self.erie())
            .field("etie", &self.etie())
            .field("rwte", &self.rwte())
            .field("rse", &self.rse())
            .field("rbue", &self.rbue())
            .field("rie", &self.rie())
            .field("tbue", &self.tbue())
            .field("txse", &self.txse())
            .field("tie", &self.tie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<DMACIERrs> {
        TIE_W::new(self, 0)
    }
    ///Bit 1 - Transmit Stopped Enable
    #[inline(always)]
    pub fn txse(&mut self) -> TXSE_W<DMACIERrs> {
        TXSE_W::new(self, 1)
    }
    ///Bit 2 - Transmit Buffer Unavailable Enable
    #[inline(always)]
    pub fn tbue(&mut self) -> TBUE_W<DMACIERrs> {
        TBUE_W::new(self, 2)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<DMACIERrs> {
        RIE_W::new(self, 6)
    }
    ///Bit 7 - Receive Buffer Unavailable Enable
    #[inline(always)]
    pub fn rbue(&mut self) -> RBUE_W<DMACIERrs> {
        RBUE_W::new(self, 7)
    }
    ///Bit 8 - Receive Stopped Enable
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W<DMACIERrs> {
        RSE_W::new(self, 8)
    }
    ///Bit 9 - Receive Watchdog Timeout Enable
    #[inline(always)]
    pub fn rwte(&mut self) -> RWTE_W<DMACIERrs> {
        RWTE_W::new(self, 9)
    }
    ///Bit 10 - Early Transmit Interrupt Enable
    #[inline(always)]
    pub fn etie(&mut self) -> ETIE_W<DMACIERrs> {
        ETIE_W::new(self, 10)
    }
    ///Bit 11 - Early Receive Interrupt Enable
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W<DMACIERrs> {
        ERIE_W::new(self, 11)
    }
    ///Bit 12 - Fatal Bus Error Enable
    #[inline(always)]
    pub fn fbee(&mut self) -> FBEE_W<DMACIERrs> {
        FBEE_W::new(self, 12)
    }
    ///Bit 13 - Context Descriptor Error Enable
    #[inline(always)]
    pub fn cdee(&mut self) -> CDEE_W<DMACIERrs> {
        CDEE_W::new(self, 13)
    }
    ///Bit 14 - Abnormal Interrupt Summary Enable
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W<DMACIERrs> {
        AIE_W::new(self, 14)
    }
    ///Bit 15 - Normal Interrupt Summary Enable
    #[inline(always)]
    pub fn nie(&mut self) -> NIE_W<DMACIERrs> {
        NIE_W::new(self, 15)
    }
}
/**Channel interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmacier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#Ethernet_DMA:DMACIER)*/
pub struct DMACIERrs;
impl crate::RegisterSpec for DMACIERrs {
    type Ux = u32;
}
///`read()` method returns [`dmacier::R`](R) reader structure
impl crate::Readable for DMACIERrs {}
///`write(|w| ..)` method takes [`dmacier::W`](W) writer structure
impl crate::Writable for DMACIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACIER to value 0
impl crate::Resettable for DMACIERrs {}
