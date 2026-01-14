///Register `CEC_ISR` reader
pub type R = crate::R<CEC_ISRrs>;
///Register `CEC_ISR` writer
pub type W = crate::W<CEC_ISRrs>;
///Field `RXBR` reader - Rx-Byte Received
pub type RXBR_R = crate::BitReader;
///Field `RXBR` writer - Rx-Byte Received
pub type RXBR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXEND` reader - End Of Reception
pub type RXEND_R = crate::BitReader;
///Field `RXEND` writer - End Of Reception
pub type RXEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVR` reader - Rx-Overrun
pub type RXOVR_R = crate::BitReader;
///Field `RXOVR` writer - Rx-Overrun
pub type RXOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRE` reader - Rx-Bit Rising Error
pub type BRE_R = crate::BitReader;
///Field `BRE` writer - Rx-Bit Rising Error
pub type BRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBPE` reader - Rx-Short Bit Period Error
pub type SBPE_R = crate::BitReader;
///Field `SBPE` writer - Rx-Short Bit Period Error
pub type SBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBPE` reader - Rx-Long Bit Period Error
pub type LBPE_R = crate::BitReader;
///Field `LBPE` writer - Rx-Long Bit Period Error
pub type LBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXACKE` reader - Rx-Missing Acknowledge
pub type RXACKE_R = crate::BitReader;
///Field `RXACKE` writer - Rx-Missing Acknowledge
pub type RXACKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLST` reader - Arbitration Lost
pub type ARBLST_R = crate::BitReader;
///Field `ARBLST` writer - Arbitration Lost
pub type ARBLST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXBR` reader - Tx-Byte Request
pub type TXBR_R = crate::BitReader;
///Field `TXBR` writer - Tx-Byte Request
pub type TXBR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEND` reader - End of Transmission
pub type TXEND_R = crate::BitReader;
///Field `TXEND` writer - End of Transmission
pub type TXEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUDR` reader - Tx-Buffer Underrun
pub type TXUDR_R = crate::BitReader;
///Field `TXUDR` writer - Tx-Buffer Underrun
pub type TXUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERR` reader - Tx-Error
pub type TXERR_R = crate::BitReader;
///Field `TXERR` writer - Tx-Error
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXACKE` reader - Tx-Missing Acknowledge Error
pub type TXACKE_R = crate::BitReader;
///Field `TXACKE` writer - Tx-Missing Acknowledge Error
pub type TXACKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx-Byte Received
    #[inline(always)]
    pub fn rxbr(&self) -> RXBR_R {
        RXBR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End Of Reception
    #[inline(always)]
    pub fn rxend(&self) -> RXEND_R {
        RXEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx-Overrun
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx-Bit Rising Error
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx-Short Bit Period Error
    #[inline(always)]
    pub fn sbpe(&self) -> SBPE_R {
        SBPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx-Long Bit Period Error
    #[inline(always)]
    pub fn lbpe(&self) -> LBPE_R {
        LBPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx-Missing Acknowledge
    #[inline(always)]
    pub fn rxacke(&self) -> RXACKE_R {
        RXACKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Arbitration Lost
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Tx-Byte Request
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - End of Transmission
    #[inline(always)]
    pub fn txend(&self) -> TXEND_R {
        TXEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx-Buffer Underrun
    #[inline(always)]
    pub fn txudr(&self) -> TXUDR_R {
        TXUDR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx-Error
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx-Missing Acknowledge Error
    #[inline(always)]
    pub fn txacke(&self) -> TXACKE_R {
        TXACKE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEC_ISR")
            .field("txacke", &self.txacke())
            .field("txerr", &self.txerr())
            .field("txudr", &self.txudr())
            .field("txend", &self.txend())
            .field("txbr", &self.txbr())
            .field("arblst", &self.arblst())
            .field("rxacke", &self.rxacke())
            .field("lbpe", &self.lbpe())
            .field("sbpe", &self.sbpe())
            .field("bre", &self.bre())
            .field("rxovr", &self.rxovr())
            .field("rxend", &self.rxend())
            .field("rxbr", &self.rxbr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx-Byte Received
    #[inline(always)]
    pub fn rxbr(&mut self) -> RXBR_W<'_, CEC_ISRrs> {
        RXBR_W::new(self, 0)
    }
    ///Bit 1 - End Of Reception
    #[inline(always)]
    pub fn rxend(&mut self) -> RXEND_W<'_, CEC_ISRrs> {
        RXEND_W::new(self, 1)
    }
    ///Bit 2 - Rx-Overrun
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W<'_, CEC_ISRrs> {
        RXOVR_W::new(self, 2)
    }
    ///Bit 3 - Rx-Bit Rising Error
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W<'_, CEC_ISRrs> {
        BRE_W::new(self, 3)
    }
    ///Bit 4 - Rx-Short Bit Period Error
    #[inline(always)]
    pub fn sbpe(&mut self) -> SBPE_W<'_, CEC_ISRrs> {
        SBPE_W::new(self, 4)
    }
    ///Bit 5 - Rx-Long Bit Period Error
    #[inline(always)]
    pub fn lbpe(&mut self) -> LBPE_W<'_, CEC_ISRrs> {
        LBPE_W::new(self, 5)
    }
    ///Bit 6 - Rx-Missing Acknowledge
    #[inline(always)]
    pub fn rxacke(&mut self) -> RXACKE_W<'_, CEC_ISRrs> {
        RXACKE_W::new(self, 6)
    }
    ///Bit 7 - Arbitration Lost
    #[inline(always)]
    pub fn arblst(&mut self) -> ARBLST_W<'_, CEC_ISRrs> {
        ARBLST_W::new(self, 7)
    }
    ///Bit 8 - Tx-Byte Request
    #[inline(always)]
    pub fn txbr(&mut self) -> TXBR_W<'_, CEC_ISRrs> {
        TXBR_W::new(self, 8)
    }
    ///Bit 9 - End of Transmission
    #[inline(always)]
    pub fn txend(&mut self) -> TXEND_W<'_, CEC_ISRrs> {
        TXEND_W::new(self, 9)
    }
    ///Bit 10 - Tx-Buffer Underrun
    #[inline(always)]
    pub fn txudr(&mut self) -> TXUDR_W<'_, CEC_ISRrs> {
        TXUDR_W::new(self, 10)
    }
    ///Bit 11 - Tx-Error
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W<'_, CEC_ISRrs> {
        TXERR_W::new(self, 11)
    }
    ///Bit 12 - Tx-Missing Acknowledge Error
    #[inline(always)]
    pub fn txacke(&mut self) -> TXACKE_W<'_, CEC_ISRrs> {
        TXACKE_W::new(self, 12)
    }
}
/**CEC Interrupt and Status Register

You can [`read`](crate::Reg::read) this register and get [`cec_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_ISR)*/
pub struct CEC_ISRrs;
impl crate::RegisterSpec for CEC_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`cec_isr::R`](R) reader structure
impl crate::Readable for CEC_ISRrs {}
///`write(|w| ..)` method takes [`cec_isr::W`](W) writer structure
impl crate::Writable for CEC_ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEC_ISR to value 0
impl crate::Resettable for CEC_ISRrs {}
