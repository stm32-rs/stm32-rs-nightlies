///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `EN` reader - I3C enable (whatever I3C acts as controller/target)
pub type EN_R = crate::BitReader;
///Field `EN` writer - I3C enable (whatever I3C acts as controller/target)
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRINIT` reader - Initial controller/target role
pub type CRINIT_R = crate::BitReader;
///Field `CRINIT` writer - Initial controller/target role
pub type CRINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOARBH` reader - No arbitrable header after a start (when I3C acts as a controller)
pub type NOARBH_R = crate::BitReader;
///Field `NOARBH` writer - No arbitrable header after a start (when I3C acts as a controller)
pub type NOARBH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPTRN` reader - HDR reset pattern enable (when I3C acts as a controller)
pub type RSTPTRN_R = crate::BitReader;
///Field `RSTPTRN` writer - HDR reset pattern enable (when I3C acts as a controller)
pub type RSTPTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXITPTRN` reader - HDR exit pattern enable (when I3C acts as a controller)
pub type EXITPTRN_R = crate::BitReader;
///Field `EXITPTRN` writer - HDR exit pattern enable (when I3C acts as a controller)
pub type EXITPTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HKSDAEN` reader - High-keeper enable on SDA line (when I3C acts as a controller)
pub type HKSDAEN_R = crate::BitReader;
///Field `HKSDAEN` writer - High-keeper enable on SDA line (when I3C acts as a controller)
pub type HKSDAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HJACK` reader - Hot-join request acknowledge (when I3C acts as a controller)
pub type HJACK_R = crate::BitReader;
///Field `HJACK` writer - Hot-join request acknowledge (when I3C acts as a controller)
pub type HJACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDMAEN` reader - RX-FIFO DMA request enable (whatever I3C acts as controller/target)
pub type RXDMAEN_R = crate::BitReader;
///Field `RXDMAEN` writer - RX-FIFO DMA request enable (whatever I3C acts as controller/target)
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLUSH` writer - RX-FIFO flush (whatever I3C acts as controller/target)
pub type RXFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXTHRES` reader - RX-FIFO threshold (whatever I3C acts as controller/target)
pub type RXTHRES_R = crate::BitReader;
///Field `RXTHRES` writer - RX-FIFO threshold (whatever I3C acts as controller/target)
pub type RXTHRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - TX-FIFO DMA request enable (whatever I3C acts as controller/target)
pub type TXDMAEN_R = crate::BitReader;
///Field `TXDMAEN` writer - TX-FIFO DMA request enable (whatever I3C acts as controller/target)
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFLUSH` writer - TX-FIFO flush (whatever I3C acts as controller/target)
pub type TXFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTHRES` reader - TX-FIFO threshold (whatever I3C acts as controller/target)
pub type TXTHRES_R = crate::BitReader;
///Field `TXTHRES` writer - TX-FIFO threshold (whatever I3C acts as controller/target)
pub type TXTHRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMAEN` reader - S-FIFO DMA request enable (when I3C acts as controller)
pub type SDMAEN_R = crate::BitReader;
///Field `SDMAEN` writer - S-FIFO DMA request enable (when I3C acts as controller)
pub type SDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFLUSH` writer - S-FIFO flush (when I3C acts as controller)
pub type SFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMODE` reader - S-FIFO enable / status receive mode (when I3C acts as controller)
pub type SMODE_R = crate::BitReader;
///Field `SMODE` writer - S-FIFO enable / status receive mode (when I3C acts as controller)
pub type SMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TMODE` reader - Transmit mode (when I3C acts as controller)
pub type TMODE_R = crate::BitReader;
///Field `TMODE` writer - Transmit mode (when I3C acts as controller)
pub type TMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMAEN` reader - C-FIFO DMA request enable (when I3C acts as controller)
pub type CDMAEN_R = crate::BitReader;
///Field `CDMAEN` writer - C-FIFO DMA request enable (when I3C acts as controller)
pub type CDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFLUSH` writer - C-FIFO flush (when I3C acts as controller)
pub type CFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSFSET` writer - Frame transfer set (software trigger) (when I3C acts as controller)
pub type TSFSET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I3C enable (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Initial controller/target role
    #[inline(always)]
    pub fn crinit(&self) -> CRINIT_R {
        CRINIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - No arbitrable header after a start (when I3C acts as a controller)
    #[inline(always)]
    pub fn noarbh(&self) -> NOARBH_R {
        NOARBH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HDR reset pattern enable (when I3C acts as a controller)
    #[inline(always)]
    pub fn rstptrn(&self) -> RSTPTRN_R {
        RSTPTRN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HDR exit pattern enable (when I3C acts as a controller)
    #[inline(always)]
    pub fn exitptrn(&self) -> EXITPTRN_R {
        EXITPTRN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - High-keeper enable on SDA line (when I3C acts as a controller)
    #[inline(always)]
    pub fn hksdaen(&self) -> HKSDAEN_R {
        HKSDAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Hot-join request acknowledge (when I3C acts as a controller)
    #[inline(always)]
    pub fn hjack(&self) -> HJACK_R {
        HJACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RX-FIFO DMA request enable (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - RX-FIFO threshold (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn rxthres(&self) -> RXTHRES_R {
        RXTHRES_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - TX-FIFO DMA request enable (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TX-FIFO threshold (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn txthres(&self) -> TXTHRES_R {
        TXTHRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - S-FIFO DMA request enable (when I3C acts as controller)
    #[inline(always)]
    pub fn sdmaen(&self) -> SDMAEN_R {
        SDMAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - S-FIFO enable / status receive mode (when I3C acts as controller)
    #[inline(always)]
    pub fn smode(&self) -> SMODE_R {
        SMODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transmit mode (when I3C acts as controller)
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - C-FIFO DMA request enable (when I3C acts as controller)
    #[inline(always)]
    pub fn cdmaen(&self) -> CDMAEN_R {
        CDMAEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("en", &self.en())
            .field("crinit", &self.crinit())
            .field("noarbh", &self.noarbh())
            .field("rstptrn", &self.rstptrn())
            .field("exitptrn", &self.exitptrn())
            .field("hksdaen", &self.hksdaen())
            .field("hjack", &self.hjack())
            .field("rxdmaen", &self.rxdmaen())
            .field("rxthres", &self.rxthres())
            .field("txdmaen", &self.txdmaen())
            .field("txthres", &self.txthres())
            .field("sdmaen", &self.sdmaen())
            .field("smode", &self.smode())
            .field("tmode", &self.tmode())
            .field("cdmaen", &self.cdmaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I3C enable (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CFGRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Initial controller/target role
    #[inline(always)]
    pub fn crinit(&mut self) -> CRINIT_W<'_, CFGRrs> {
        CRINIT_W::new(self, 1)
    }
    ///Bit 2 - No arbitrable header after a start (when I3C acts as a controller)
    #[inline(always)]
    pub fn noarbh(&mut self) -> NOARBH_W<'_, CFGRrs> {
        NOARBH_W::new(self, 2)
    }
    ///Bit 3 - HDR reset pattern enable (when I3C acts as a controller)
    #[inline(always)]
    pub fn rstptrn(&mut self) -> RSTPTRN_W<'_, CFGRrs> {
        RSTPTRN_W::new(self, 3)
    }
    ///Bit 4 - HDR exit pattern enable (when I3C acts as a controller)
    #[inline(always)]
    pub fn exitptrn(&mut self) -> EXITPTRN_W<'_, CFGRrs> {
        EXITPTRN_W::new(self, 4)
    }
    ///Bit 5 - High-keeper enable on SDA line (when I3C acts as a controller)
    #[inline(always)]
    pub fn hksdaen(&mut self) -> HKSDAEN_W<'_, CFGRrs> {
        HKSDAEN_W::new(self, 5)
    }
    ///Bit 7 - Hot-join request acknowledge (when I3C acts as a controller)
    #[inline(always)]
    pub fn hjack(&mut self) -> HJACK_W<'_, CFGRrs> {
        HJACK_W::new(self, 7)
    }
    ///Bit 8 - RX-FIFO DMA request enable (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CFGRrs> {
        RXDMAEN_W::new(self, 8)
    }
    ///Bit 9 - RX-FIFO flush (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn rxflush(&mut self) -> RXFLUSH_W<'_, CFGRrs> {
        RXFLUSH_W::new(self, 9)
    }
    ///Bit 10 - RX-FIFO threshold (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn rxthres(&mut self) -> RXTHRES_W<'_, CFGRrs> {
        RXTHRES_W::new(self, 10)
    }
    ///Bit 12 - TX-FIFO DMA request enable (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, CFGRrs> {
        TXDMAEN_W::new(self, 12)
    }
    ///Bit 13 - TX-FIFO flush (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn txflush(&mut self) -> TXFLUSH_W<'_, CFGRrs> {
        TXFLUSH_W::new(self, 13)
    }
    ///Bit 14 - TX-FIFO threshold (whatever I3C acts as controller/target)
    #[inline(always)]
    pub fn txthres(&mut self) -> TXTHRES_W<'_, CFGRrs> {
        TXTHRES_W::new(self, 14)
    }
    ///Bit 16 - S-FIFO DMA request enable (when I3C acts as controller)
    #[inline(always)]
    pub fn sdmaen(&mut self) -> SDMAEN_W<'_, CFGRrs> {
        SDMAEN_W::new(self, 16)
    }
    ///Bit 17 - S-FIFO flush (when I3C acts as controller)
    #[inline(always)]
    pub fn sflush(&mut self) -> SFLUSH_W<'_, CFGRrs> {
        SFLUSH_W::new(self, 17)
    }
    ///Bit 18 - S-FIFO enable / status receive mode (when I3C acts as controller)
    #[inline(always)]
    pub fn smode(&mut self) -> SMODE_W<'_, CFGRrs> {
        SMODE_W::new(self, 18)
    }
    ///Bit 19 - Transmit mode (when I3C acts as controller)
    #[inline(always)]
    pub fn tmode(&mut self) -> TMODE_W<'_, CFGRrs> {
        TMODE_W::new(self, 19)
    }
    ///Bit 20 - C-FIFO DMA request enable (when I3C acts as controller)
    #[inline(always)]
    pub fn cdmaen(&mut self) -> CDMAEN_W<'_, CFGRrs> {
        CDMAEN_W::new(self, 20)
    }
    ///Bit 21 - C-FIFO flush (when I3C acts as controller)
    #[inline(always)]
    pub fn cflush(&mut self) -> CFLUSH_W<'_, CFGRrs> {
        CFLUSH_W::new(self, 21)
    }
    ///Bit 30 - Frame transfer set (software trigger) (when I3C acts as controller)
    #[inline(always)]
    pub fn tsfset(&mut self) -> TSFSET_W<'_, CFGRrs> {
        TSFSET_W::new(self, 30)
    }
}
/**I3C configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#I3C1:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
