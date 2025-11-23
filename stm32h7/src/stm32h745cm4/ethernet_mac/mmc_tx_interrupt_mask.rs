///Register `MMC_TX_INTERRUPT_MASK` reader
pub type R = crate::R<MMC_TX_INTERRUPT_MASKrs>;
///Register `MMC_TX_INTERRUPT_MASK` writer
pub type W = crate::W<MMC_TX_INTERRUPT_MASKrs>;
///Field `TXSCOLGPIM` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Mask
pub type TXSCOLGPIM_R = crate::BitReader;
///Field `TXSCOLGPIM` writer - MMC Transmit Single Collision Good Packet Counter Interrupt Mask
pub type TXSCOLGPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMCOLGPIM` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask
pub type TXMCOLGPIM_R = crate::BitReader;
///Field `TXMCOLGPIM` writer - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask
pub type TXMCOLGPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXGPKTIM` reader - MMC Transmit Good Packet Counter Interrupt Mask
pub type TXGPKTIM_R = crate::BitReader;
///Field `TXGPKTIM` writer - MMC Transmit Good Packet Counter Interrupt Mask
pub type TXGPKTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXLPIUSCIM` reader - MMC Transmit LPI microsecond counter interrupt Mask
pub type TXLPIUSCIM_R = crate::BitReader;
///Field `TXLPIUSCIM` writer - MMC Transmit LPI microsecond counter interrupt Mask
pub type TXLPIUSCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXLPITRCIM` reader - MMC Transmit LPI transition counter interrupt Mask
pub type TXLPITRCIM_R = crate::BitReader;
///Field `TXLPITRCIM` writer - MMC Transmit LPI transition counter interrupt Mask
pub type TXLPITRCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask
    #[inline(always)]
    pub fn txscolgpim(&self) -> TXSCOLGPIM_R {
        TXSCOLGPIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TXMCOLGPIM_R {
        TXMCOLGPIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask
    #[inline(always)]
    pub fn txgpktim(&self) -> TXGPKTIM_R {
        TXGPKTIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TXLPIUSCIM_R {
        TXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MMC Transmit LPI transition counter interrupt Mask
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TXLPITRCIM_R {
        TXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_TX_INTERRUPT_MASK")
            .field("txscolgpim", &self.txscolgpim())
            .field("txmcolgpim", &self.txmcolgpim())
            .field("txgpktim", &self.txgpktim())
            .field("txlpiuscim", &self.txlpiuscim())
            .field("txlpitrcim", &self.txlpitrcim())
            .finish()
    }
}
impl W {
    ///Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask
    #[inline(always)]
    pub fn txscolgpim(&mut self) -> TXSCOLGPIM_W<'_, MMC_TX_INTERRUPT_MASKrs> {
        TXSCOLGPIM_W::new(self, 14)
    }
    ///Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask
    #[inline(always)]
    pub fn txmcolgpim(&mut self) -> TXMCOLGPIM_W<'_, MMC_TX_INTERRUPT_MASKrs> {
        TXMCOLGPIM_W::new(self, 15)
    }
    ///Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask
    #[inline(always)]
    pub fn txgpktim(&mut self) -> TXGPKTIM_W<'_, MMC_TX_INTERRUPT_MASKrs> {
        TXGPKTIM_W::new(self, 21)
    }
    ///Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask
    #[inline(always)]
    pub fn txlpiuscim(&mut self) -> TXLPIUSCIM_W<'_, MMC_TX_INTERRUPT_MASKrs> {
        TXLPIUSCIM_W::new(self, 26)
    }
    ///Bit 27 - MMC Transmit LPI transition counter interrupt Mask
    #[inline(always)]
    pub fn txlpitrcim(&mut self) -> TXLPITRCIM_W<'_, MMC_TX_INTERRUPT_MASKrs> {
        TXLPITRCIM_W::new(self, 27)
    }
}
/**MMC Tx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#Ethernet_MAC:MMC_TX_INTERRUPT_MASK)*/
pub struct MMC_TX_INTERRUPT_MASKrs;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_MASKrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_tx_interrupt_mask::R`](R) reader structure
impl crate::Readable for MMC_TX_INTERRUPT_MASKrs {}
///`write(|w| ..)` method takes [`mmc_tx_interrupt_mask::W`](W) writer structure
impl crate::Writable for MMC_TX_INTERRUPT_MASKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_TX_INTERRUPT_MASK to value 0
impl crate::Resettable for MMC_TX_INTERRUPT_MASKrs {}
