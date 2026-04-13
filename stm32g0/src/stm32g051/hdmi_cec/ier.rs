///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `RXBRIE` reader - Rx-byte received interrupt enable The RXBRIE bit is set and cleared by software.
pub type RXBRIE_R = crate::BitReader;
///Field `RXBRIE` writer - Rx-byte received interrupt enable The RXBRIE bit is set and cleared by software.
pub type RXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXENDIE` reader - End of reception interrupt enable The RXENDIE bit is set and cleared by software.
pub type RXENDIE_R = crate::BitReader;
///Field `RXENDIE` writer - End of reception interrupt enable The RXENDIE bit is set and cleared by software.
pub type RXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVRIE` reader - Rx-buffer overrun interrupt enable The RXOVRIE bit is set and cleared by software.
pub type RXOVRIE_R = crate::BitReader;
///Field `RXOVRIE` writer - Rx-buffer overrun interrupt enable The RXOVRIE bit is set and cleared by software.
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREIE` reader - Bit rising error interrupt enable The BREIE bit is set and cleared by software.
pub type BREIE_R = crate::BitReader;
///Field `BREIE` writer - Bit rising error interrupt enable The BREIE bit is set and cleared by software.
pub type BREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBPEIE` reader - Short bit period error interrupt enable The SBPEIE bit is set and cleared by software.
pub type SBPEIE_R = crate::BitReader;
///Field `SBPEIE` writer - Short bit period error interrupt enable The SBPEIE bit is set and cleared by software.
pub type SBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBPEIE` reader - Long bit period error interrupt enable The LBPEIE bit is set and cleared by software.
pub type LBPEIE_R = crate::BitReader;
///Field `LBPEIE` writer - Long bit period error interrupt enable The LBPEIE bit is set and cleared by software.
pub type LBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXACKIE` reader - Rx-missing acknowledge error interrupt enable The RXACKIE bit is set and cleared by software.
pub type RXACKIE_R = crate::BitReader;
///Field `RXACKIE` writer - Rx-missing acknowledge error interrupt enable The RXACKIE bit is set and cleared by software.
pub type RXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLSTIE` reader - Arbitration lost interrupt enable The ARBLSTIE bit is set and cleared by software.
pub type ARBLSTIE_R = crate::BitReader;
///Field `ARBLSTIE` writer - Arbitration lost interrupt enable The ARBLSTIE bit is set and cleared by software.
pub type ARBLSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXBRIE` reader - Tx-byte request interrupt enable The TXBRIE bit is set and cleared by software.
pub type TXBRIE_R = crate::BitReader;
///Field `TXBRIE` writer - Tx-byte request interrupt enable The TXBRIE bit is set and cleared by software.
pub type TXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXENDIE` reader - Tx-end of message interrupt enable The TXENDIE bit is set and cleared by software.
pub type TXENDIE_R = crate::BitReader;
///Field `TXENDIE` writer - Tx-end of message interrupt enable The TXENDIE bit is set and cleared by software.
pub type TXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUDRIE` reader - Tx-underrun interrupt enable The TXUDRIE bit is set and cleared by software.
pub type TXUDRIE_R = crate::BitReader;
///Field `TXUDRIE` writer - Tx-underrun interrupt enable The TXUDRIE bit is set and cleared by software.
pub type TXUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERRIE` reader - Tx-error interrupt enable The TXERRIE bit is set and cleared by software.
pub type TXERRIE_R = crate::BitReader;
///Field `TXERRIE` writer - Tx-error interrupt enable The TXERRIE bit is set and cleared by software.
pub type TXERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXACKIE` reader - Tx-missing acknowledge error interrupt enable The TXACKEIE bit is set and cleared by software.
pub type TXACKIE_R = crate::BitReader;
///Field `TXACKIE` writer - Tx-missing acknowledge error interrupt enable The TXACKEIE bit is set and cleared by software.
pub type TXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx-byte received interrupt enable The RXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of reception interrupt enable The RXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx-buffer overrun interrupt enable The RXOVRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bit rising error interrupt enable The BREIE bit is set and cleared by software.
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Short bit period error interrupt enable The SBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Long bit period error interrupt enable The LBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx-missing acknowledge error interrupt enable The RXACKIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Arbitration lost interrupt enable The ARBLSTIE bit is set and cleared by software.
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Tx-byte request interrupt enable The TXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx-end of message interrupt enable The TXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx-underrun interrupt enable The TXUDRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx-error interrupt enable The TXERRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx-missing acknowledge error interrupt enable The TXACKEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rxbrie", &self.rxbrie())
            .field("rxendie", &self.rxendie())
            .field("rxovrie", &self.rxovrie())
            .field("breie", &self.breie())
            .field("sbpeie", &self.sbpeie())
            .field("lbpeie", &self.lbpeie())
            .field("rxackie", &self.rxackie())
            .field("arblstie", &self.arblstie())
            .field("txbrie", &self.txbrie())
            .field("txendie", &self.txendie())
            .field("txudrie", &self.txudrie())
            .field("txerrie", &self.txerrie())
            .field("txackie", &self.txackie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx-byte received interrupt enable The RXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxbrie(&mut self) -> RXBRIE_W<'_, IERrs> {
        RXBRIE_W::new(self, 0)
    }
    ///Bit 1 - End of reception interrupt enable The RXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxendie(&mut self) -> RXENDIE_W<'_, IERrs> {
        RXENDIE_W::new(self, 1)
    }
    ///Bit 2 - Rx-buffer overrun interrupt enable The RXOVRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<'_, IERrs> {
        RXOVRIE_W::new(self, 2)
    }
    ///Bit 3 - Bit rising error interrupt enable The BREIE bit is set and cleared by software.
    #[inline(always)]
    pub fn breie(&mut self) -> BREIE_W<'_, IERrs> {
        BREIE_W::new(self, 3)
    }
    ///Bit 4 - Short bit period error interrupt enable The SBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn sbpeie(&mut self) -> SBPEIE_W<'_, IERrs> {
        SBPEIE_W::new(self, 4)
    }
    ///Bit 5 - Long bit period error interrupt enable The LBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn lbpeie(&mut self) -> LBPEIE_W<'_, IERrs> {
        LBPEIE_W::new(self, 5)
    }
    ///Bit 6 - Rx-missing acknowledge error interrupt enable The RXACKIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxackie(&mut self) -> RXACKIE_W<'_, IERrs> {
        RXACKIE_W::new(self, 6)
    }
    ///Bit 7 - Arbitration lost interrupt enable The ARBLSTIE bit is set and cleared by software.
    #[inline(always)]
    pub fn arblstie(&mut self) -> ARBLSTIE_W<'_, IERrs> {
        ARBLSTIE_W::new(self, 7)
    }
    ///Bit 8 - Tx-byte request interrupt enable The TXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txbrie(&mut self) -> TXBRIE_W<'_, IERrs> {
        TXBRIE_W::new(self, 8)
    }
    ///Bit 9 - Tx-end of message interrupt enable The TXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txendie(&mut self) -> TXENDIE_W<'_, IERrs> {
        TXENDIE_W::new(self, 9)
    }
    ///Bit 10 - Tx-underrun interrupt enable The TXUDRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txudrie(&mut self) -> TXUDRIE_W<'_, IERrs> {
        TXUDRIE_W::new(self, 10)
    }
    ///Bit 11 - Tx-error interrupt enable The TXERRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txerrie(&mut self) -> TXERRIE_W<'_, IERrs> {
        TXERRIE_W::new(self, 11)
    }
    ///Bit 12 - Tx-missing acknowledge error interrupt enable The TXACKEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txackie(&mut self) -> TXACKIE_W<'_, IERrs> {
        TXACKIE_W::new(self, 12)
    }
}
/**CEC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#HDMI_CEC:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
