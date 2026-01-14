///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `RXBFIE` reader - Receive buffer full interrupt enable
pub type RXBFIE_R = crate::BitReader;
///Field `RXBFIE` writer - Receive buffer full interrupt enable
pub type RXBFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXBEIE` reader - Transmit buffer empty interrupt enable
pub type TXBEIE_R = crate::BitReader;
///Field `TXBEIE` writer - Transmit buffer empty interrupt enable
pub type TXBEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXBERIE` reader - Receive CRC error interrupt enable
pub type RXBERIE_R = crate::BitReader;
///Field `RXBERIE` writer - Receive CRC error interrupt enable
pub type RXBERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVRIE` reader - Receive overrun error interrupt enable
pub type RXOVRIE_R = crate::BitReader;
///Field `RXOVRIE` writer - Receive overrun error interrupt enable
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUNRIE` reader - Transmit underrun error interrupt enable
pub type TXUNRIE_R = crate::BitReader;
///Field `TXUNRIE` writer - Transmit underrun error interrupt enable
pub type TXUNRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIE` reader - Receive interrupt enable
pub type RIE_R = crate::BitReader;
///Field `RIE` writer - Receive interrupt enable
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - Transmit interrupt enable
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - Transmit interrupt enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transmit complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transmit complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRIE` reader - Slave resume interrupt enable
pub type SRIE_R = crate::BitReader;
///Field `SRIE` writer - Slave resume interrupt enable
pub type SRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDYIE` reader - Transceiver ready interrupt enable
pub type RDYIE_R = crate::BitReader;
///Field `RDYIE` writer - Transceiver ready interrupt enable
pub type RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Receive buffer full interrupt enable
    #[inline(always)]
    pub fn rxbfie(&self) -> RXBFIE_R {
        RXBFIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty interrupt enable
    #[inline(always)]
    pub fn txbeie(&self) -> TXBEIE_R {
        TXBEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive CRC error interrupt enable
    #[inline(always)]
    pub fn rxberie(&self) -> RXBERIE_R {
        RXBERIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receive overrun error interrupt enable
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit underrun error interrupt enable
    #[inline(always)]
    pub fn txunrie(&self) -> TXUNRIE_R {
        TXUNRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Slave resume interrupt enable
    #[inline(always)]
    pub fn srie(&self) -> SRIE_R {
        SRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Transceiver ready interrupt enable
    #[inline(always)]
    pub fn rdyie(&self) -> RDYIE_R {
        RDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rxbfie", &self.rxbfie())
            .field("txbeie", &self.txbeie())
            .field("rxberie", &self.rxberie())
            .field("rxovrie", &self.rxovrie())
            .field("txunrie", &self.txunrie())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .field("tcie", &self.tcie())
            .field("srie", &self.srie())
            .field("rdyie", &self.rdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Receive buffer full interrupt enable
    #[inline(always)]
    pub fn rxbfie(&mut self) -> RXBFIE_W<'_, IERrs> {
        RXBFIE_W::new(self, 0)
    }
    ///Bit 1 - Transmit buffer empty interrupt enable
    #[inline(always)]
    pub fn txbeie(&mut self) -> TXBEIE_W<'_, IERrs> {
        TXBEIE_W::new(self, 1)
    }
    ///Bit 2 - Receive CRC error interrupt enable
    #[inline(always)]
    pub fn rxberie(&mut self) -> RXBERIE_W<'_, IERrs> {
        RXBERIE_W::new(self, 2)
    }
    ///Bit 3 - Receive overrun error interrupt enable
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<'_, IERrs> {
        RXOVRIE_W::new(self, 3)
    }
    ///Bit 4 - Transmit underrun error interrupt enable
    #[inline(always)]
    pub fn txunrie(&mut self) -> TXUNRIE_W<'_, IERrs> {
        TXUNRIE_W::new(self, 4)
    }
    ///Bit 5 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<'_, IERrs> {
        RIE_W::new(self, 5)
    }
    ///Bit 6 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, IERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 7 - Transmit complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, IERrs> {
        TCIE_W::new(self, 7)
    }
    ///Bit 8 - Slave resume interrupt enable
    #[inline(always)]
    pub fn srie(&mut self) -> SRIE_W<'_, IERrs> {
        SRIE_W::new(self, 8)
    }
    ///Bit 11 - Transceiver ready interrupt enable
    #[inline(always)]
    pub fn rdyie(&mut self) -> RDYIE_W<'_, IERrs> {
        RDYIE_W::new(self, 11)
    }
}
/**SWPMI Interrupt Enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#SWPMI:IER)*/
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
