#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `RXBFIE` reader - Receive buffer full interrupt enable"]
pub type RXBFIE_R = crate::BitReader;
#[doc = "Field `RXBFIE` writer - Receive buffer full interrupt enable"]
pub type RXBFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEIE` reader - Transmit buffer empty interrupt enable"]
pub type TXBEIE_R = crate::BitReader;
#[doc = "Field `TXBEIE` writer - Transmit buffer empty interrupt enable"]
pub type TXBEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBERIE` reader - Receive CRC error interrupt enable"]
pub type RXBERIE_R = crate::BitReader;
#[doc = "Field `RXBERIE` writer - Receive CRC error interrupt enable"]
pub type RXBERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRIE` reader - Receive overrun error interrupt enable"]
pub type RXOVRIE_R = crate::BitReader;
#[doc = "Field `RXOVRIE` writer - Receive overrun error interrupt enable"]
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNRIE` reader - Transmit underrun error interrupt enable"]
pub type TXUNRIE_R = crate::BitReader;
#[doc = "Field `TXUNRIE` writer - Transmit underrun error interrupt enable"]
pub type TXUNRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transmit complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmit complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRIE` reader - Slave resume interrupt enable"]
pub type SRIE_R = crate::BitReader;
#[doc = "Field `SRIE` writer - Slave resume interrupt enable"]
pub type SRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDYIE` reader - Transceiver ready interrupt enable"]
pub type RDYIE_R = crate::BitReader;
#[doc = "Field `RDYIE` writer - Transceiver ready interrupt enable"]
pub type RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbfie(&self) -> RXBFIE_R {
        RXBFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeie(&self) -> TXBEIE_R {
        TXBEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    pub fn rxberie(&self) -> RXBERIE_R {
        RXBERIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunrie(&self) -> TXUNRIE_R {
        TXUNRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    pub fn srie(&self) -> SRIE_R {
        SRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Transceiver ready interrupt enable"]
    #[inline(always)]
    pub fn rdyie(&self) -> RDYIE_R {
        RDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbfie(&mut self) -> RXBFIE_W<IERrs> {
        RXBFIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbeie(&mut self) -> TXBEIE_W<IERrs> {
        TXBEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxberie(&mut self) -> RXBERIE_W<IERrs> {
        RXBERIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<IERrs> {
        RXOVRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txunrie(&mut self) -> TXUNRIE_W<IERrs> {
        TXUNRIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<IERrs> {
        RIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<IERrs> {
        TIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<IERrs> {
        TCIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn srie(&mut self) -> SRIE_W<IERrs> {
        SRIE_W::new(self, 8)
    }
    #[doc = "Bit 11 - Transceiver ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdyie(&mut self) -> RDYIE_W<IERrs> {
        RDYIE_W::new(self, 11)
    }
}
#[doc = "SWPMI Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
