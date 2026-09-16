///Register `CEC_IER` reader
pub type R = crate::R<CEC_IERrs>;
///Register `CEC_IER` writer
pub type W = crate::W<CEC_IERrs>;
///Field `RXBRIE` reader - Rx-Byte Received Interrupt Enable
pub type RXBRIE_R = crate::BitReader;
///Field `RXBRIE` writer - Rx-Byte Received Interrupt Enable
pub type RXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXENDIE` reader - End Of Reception Interrupt Enable
pub type RXENDIE_R = crate::BitReader;
///Field `RXENDIE` writer - End Of Reception Interrupt Enable
pub type RXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVRIE` reader - Rx-Buffer Overrun Interrupt Enable
pub type RXOVRIE_R = crate::BitReader;
///Field `RXOVRIE` writer - Rx-Buffer Overrun Interrupt Enable
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREIE` reader - Bit Rising Error Interrupt Enable
pub type BREIE_R = crate::BitReader;
///Field `BREIE` writer - Bit Rising Error Interrupt Enable
pub type BREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBPEIE` reader - Short Bit Period Error Interrupt Enable
pub type SBPEIE_R = crate::BitReader;
///Field `SBPEIE` writer - Short Bit Period Error Interrupt Enable
pub type SBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBPEIE` reader - Long Bit Period Error Interrupt Enable
pub type LBPEIE_R = crate::BitReader;
///Field `LBPEIE` writer - Long Bit Period Error Interrupt Enable
pub type LBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXACKIE` reader - Rx-Missing Acknowledge Error Interrupt Enable
pub type RXACKIE_R = crate::BitReader;
///Field `RXACKIE` writer - Rx-Missing Acknowledge Error Interrupt Enable
pub type RXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLSTIE` reader - Arbitration Lost Interrupt Enable
pub type ARBLSTIE_R = crate::BitReader;
///Field `ARBLSTIE` writer - Arbitration Lost Interrupt Enable
pub type ARBLSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXBRIE` reader - Tx-Byte Request Interrupt Enable
pub type TXBRIE_R = crate::BitReader;
///Field `TXBRIE` writer - Tx-Byte Request Interrupt Enable
pub type TXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXENDIE` reader - Tx-End Of Message Interrupt Enable
pub type TXENDIE_R = crate::BitReader;
///Field `TXENDIE` writer - Tx-End Of Message Interrupt Enable
pub type TXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUDRIE` reader - Tx-Underrun Interrupt Enable
pub type TXUDRIE_R = crate::BitReader;
///Field `TXUDRIE` writer - Tx-Underrun Interrupt Enable
pub type TXUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERRIE` reader - Tx-Error Interrupt Enable
pub type TXERRIE_R = crate::BitReader;
///Field `TXERRIE` writer - Tx-Error Interrupt Enable
pub type TXERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXACKIE` reader - Tx-Missing Acknowledge Error Interrupt Enable
pub type TXACKIE_R = crate::BitReader;
///Field `TXACKIE` writer - Tx-Missing Acknowledge Error Interrupt Enable
pub type TXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx-Byte Received Interrupt Enable
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End Of Reception Interrupt Enable
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx-Buffer Overrun Interrupt Enable
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bit Rising Error Interrupt Enable
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Short Bit Period Error Interrupt Enable
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Long Bit Period Error Interrupt Enable
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Arbitration Lost Interrupt Enable
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Tx-Byte Request Interrupt Enable
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx-End Of Message Interrupt Enable
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx-Underrun Interrupt Enable
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx-Error Interrupt Enable
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEC_IER")
            .field("txackie", &self.txackie())
            .field("txerrie", &self.txerrie())
            .field("txudrie", &self.txudrie())
            .field("txendie", &self.txendie())
            .field("txbrie", &self.txbrie())
            .field("arblstie", &self.arblstie())
            .field("rxackie", &self.rxackie())
            .field("lbpeie", &self.lbpeie())
            .field("sbpeie", &self.sbpeie())
            .field("breie", &self.breie())
            .field("rxovrie", &self.rxovrie())
            .field("rxendie", &self.rxendie())
            .field("rxbrie", &self.rxbrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx-Byte Received Interrupt Enable
    #[inline(always)]
    pub fn rxbrie(&mut self) -> RXBRIE_W<'_, CEC_IERrs> {
        RXBRIE_W::new(self, 0)
    }
    ///Bit 1 - End Of Reception Interrupt Enable
    #[inline(always)]
    pub fn rxendie(&mut self) -> RXENDIE_W<'_, CEC_IERrs> {
        RXENDIE_W::new(self, 1)
    }
    ///Bit 2 - Rx-Buffer Overrun Interrupt Enable
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<'_, CEC_IERrs> {
        RXOVRIE_W::new(self, 2)
    }
    ///Bit 3 - Bit Rising Error Interrupt Enable
    #[inline(always)]
    pub fn breie(&mut self) -> BREIE_W<'_, CEC_IERrs> {
        BREIE_W::new(self, 3)
    }
    ///Bit 4 - Short Bit Period Error Interrupt Enable
    #[inline(always)]
    pub fn sbpeie(&mut self) -> SBPEIE_W<'_, CEC_IERrs> {
        SBPEIE_W::new(self, 4)
    }
    ///Bit 5 - Long Bit Period Error Interrupt Enable
    #[inline(always)]
    pub fn lbpeie(&mut self) -> LBPEIE_W<'_, CEC_IERrs> {
        LBPEIE_W::new(self, 5)
    }
    ///Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable
    #[inline(always)]
    pub fn rxackie(&mut self) -> RXACKIE_W<'_, CEC_IERrs> {
        RXACKIE_W::new(self, 6)
    }
    ///Bit 7 - Arbitration Lost Interrupt Enable
    #[inline(always)]
    pub fn arblstie(&mut self) -> ARBLSTIE_W<'_, CEC_IERrs> {
        ARBLSTIE_W::new(self, 7)
    }
    ///Bit 8 - Tx-Byte Request Interrupt Enable
    #[inline(always)]
    pub fn txbrie(&mut self) -> TXBRIE_W<'_, CEC_IERrs> {
        TXBRIE_W::new(self, 8)
    }
    ///Bit 9 - Tx-End Of Message Interrupt Enable
    #[inline(always)]
    pub fn txendie(&mut self) -> TXENDIE_W<'_, CEC_IERrs> {
        TXENDIE_W::new(self, 9)
    }
    ///Bit 10 - Tx-Underrun Interrupt Enable
    #[inline(always)]
    pub fn txudrie(&mut self) -> TXUDRIE_W<'_, CEC_IERrs> {
        TXUDRIE_W::new(self, 10)
    }
    ///Bit 11 - Tx-Error Interrupt Enable
    #[inline(always)]
    pub fn txerrie(&mut self) -> TXERRIE_W<'_, CEC_IERrs> {
        TXERRIE_W::new(self, 11)
    }
    ///Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable
    #[inline(always)]
    pub fn txackie(&mut self) -> TXACKIE_W<'_, CEC_IERrs> {
        TXACKIE_W::new(self, 12)
    }
}
/**CEC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cec_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_IER)*/
pub struct CEC_IERrs;
impl crate::RegisterSpec for CEC_IERrs {
    type Ux = u32;
}
///`read()` method returns [`cec_ier::R`](R) reader structure
impl crate::Readable for CEC_IERrs {}
///`write(|w| ..)` method takes [`cec_ier::W`](W) writer structure
impl crate::Writable for CEC_IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEC_IER to value 0
impl crate::Resettable for CEC_IERrs {}
