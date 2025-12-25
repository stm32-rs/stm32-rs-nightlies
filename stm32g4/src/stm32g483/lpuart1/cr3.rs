///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///CTS enable
pub use crate::stm32g483::usart1::cr3::CTSE;
///Field `CTSE` reader - CTS enable
pub use crate::stm32g483::usart1::cr3::CTSE_R;
///Field `CTSE` writer - CTS enable
pub use crate::stm32g483::usart1::cr3::CTSE_W;
///CTS interrupt enable
pub use crate::stm32g483::usart1::cr3::CTSIE;
///Field `CTSIE` reader - CTS interrupt enable
pub use crate::stm32g483::usart1::cr3::CTSIE_R;
///Field `CTSIE` writer - CTS interrupt enable
pub use crate::stm32g483::usart1::cr3::CTSIE_W;
///DMA Disable on Reception Error
pub use crate::stm32g483::usart1::cr3::DDRE;
///Field `DDRE` reader - DMA Disable on Reception Error
pub use crate::stm32g483::usart1::cr3::DDRE_R;
///Field `DDRE` writer - DMA Disable on Reception Error
pub use crate::stm32g483::usart1::cr3::DDRE_W;
///Driver enable mode
pub use crate::stm32g483::usart1::cr3::DEM;
///Field `DEM` reader - Driver enable mode
pub use crate::stm32g483::usart1::cr3::DEM_R;
///Field `DEM` writer - Driver enable mode
pub use crate::stm32g483::usart1::cr3::DEM_W;
///Driver enable polarity selection
pub use crate::stm32g483::usart1::cr3::DEP;
///Field `DEP` reader - Driver enable polarity selection
pub use crate::stm32g483::usart1::cr3::DEP_R;
///Field `DEP` writer - Driver enable polarity selection
pub use crate::stm32g483::usart1::cr3::DEP_W;
///DMA enable receiver
pub use crate::stm32g483::usart1::cr3::DMAR;
///Field `DMAR` reader - DMA enable receiver
pub use crate::stm32g483::usart1::cr3::DMAR_R;
///Field `DMAR` writer - DMA enable receiver
pub use crate::stm32g483::usart1::cr3::DMAR_W;
///DMA enable transmitter
pub use crate::stm32g483::usart1::cr3::DMAT;
///Field `DMAT` reader - DMA enable transmitter
pub use crate::stm32g483::usart1::cr3::DMAT_R;
///Field `DMAT` writer - DMA enable transmitter
pub use crate::stm32g483::usart1::cr3::DMAT_W;
///Error interrupt enable
pub use crate::stm32g483::usart1::cr3::EIE;
///Field `EIE` reader - Error interrupt enable
pub use crate::stm32g483::usart1::cr3::EIE_R;
///Field `EIE` writer - Error interrupt enable
pub use crate::stm32g483::usart1::cr3::EIE_W;
///Half-duplex selection
pub use crate::stm32g483::usart1::cr3::HDSEL;
///Field `HDSEL` reader - Half-duplex selection
pub use crate::stm32g483::usart1::cr3::HDSEL_R;
///Field `HDSEL` writer - Half-duplex selection
pub use crate::stm32g483::usart1::cr3::HDSEL_W;
///Overrun Disable
pub use crate::stm32g483::usart1::cr3::OVRDIS;
///Field `OVRDIS` reader - Overrun Disable
pub use crate::stm32g483::usart1::cr3::OVRDIS_R;
///Field `OVRDIS` writer - Overrun Disable
pub use crate::stm32g483::usart1::cr3::OVRDIS_W;
///RTS enable
pub use crate::stm32g483::usart1::cr3::RTSE;
///Field `RTSE` reader - RTS enable
pub use crate::stm32g483::usart1::cr3::RTSE_R;
///Field `RTSE` writer - RTS enable
pub use crate::stm32g483::usart1::cr3::RTSE_W;
///RXFTCFG
pub use crate::stm32g483::usart1::cr3::RXFTCFG;
///Field `RXFTCFG` reader - RXFTCFG
pub use crate::stm32g483::usart1::cr3::RXFTCFG_R;
///Field `RXFTCFG` writer - RXFTCFG
pub use crate::stm32g483::usart1::cr3::RXFTCFG_W;
///RXFTIE
pub use crate::stm32g483::usart1::cr3::RXFTIE;
///Field `RXFTIE` reader - RXFTIE
pub use crate::stm32g483::usart1::cr3::RXFTIE_R;
///Field `RXFTIE` writer - RXFTIE
pub use crate::stm32g483::usart1::cr3::RXFTIE_W;
///TXFTCFG
pub use crate::stm32g483::usart1::cr3::TXFTCFG;
///Field `TXFTCFG` reader - TXFTCFG
pub use crate::stm32g483::usart1::cr3::TXFTCFG_R;
///Field `TXFTCFG` writer - TXFTCFG
pub use crate::stm32g483::usart1::cr3::TXFTCFG_W;
///TXFTIE
pub use crate::stm32g483::usart1::cr3::TXFTIE;
///Field `TXFTIE` reader - TXFTIE
pub use crate::stm32g483::usart1::cr3::TXFTIE_R;
///Field `TXFTIE` writer - TXFTIE
pub use crate::stm32g483::usart1::cr3::TXFTIE_W;
///Wakeup from Stop mode interrupt enable
pub use crate::stm32g483::usart1::cr3::WUFIE;
///Field `WUFIE` reader - Wakeup from Stop mode interrupt enable
pub use crate::stm32g483::usart1::cr3::WUFIE_R;
///Field `WUFIE` writer - Wakeup from Stop mode interrupt enable
pub use crate::stm32g483::usart1::cr3::WUFIE_W;
///Wakeup from Stop mode interrupt flag selection
pub use crate::stm32g483::usart1::cr3::WUS;
///Field `WUS` reader - Wakeup from Stop mode interrupt flag selection
pub use crate::stm32g483::usart1::cr3::WUS_R;
///Field `WUS` writer - Wakeup from Stop mode interrupt flag selection
pub use crate::stm32g483::usart1::cr3::WUS_W;
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFTIE
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:27 - RXFTCFG
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFTIE
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFTCFG
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("txftcfg", &self.txftcfg())
            .field("rxftie", &self.rxftie())
            .field("rxftcfg", &self.rxftcfg())
            .field("txftie", &self.txftie())
            .field("wufie", &self.wufie())
            .field("wus", &self.wus())
            .field("dep", &self.dep())
            .field("dem", &self.dem())
            .field("ddre", &self.ddre())
            .field("ovrdis", &self.ovrdis())
            .field("ctsie", &self.ctsie())
            .field("ctse", &self.ctse())
            .field("rtse", &self.rtse())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("hdsel", &self.hdsel())
            .field("eie", &self.eie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<'_, CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<'_, CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<'_, CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<'_, CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<'_, CR3rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<'_, CR3rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<'_, CR3rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<'_, CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<'_, CR3rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<'_, CR3rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<'_, CR3rs> {
        DEP_W::new(self, 15)
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<'_, CR3rs> {
        WUS_W::new(self, 20)
    }
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<'_, CR3rs> {
        WUFIE_W::new(self, 22)
    }
    ///Bit 23 - TXFTIE
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<'_, CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    ///Bits 25:27 - RXFTCFG
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<'_, CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    ///Bit 28 - RXFTIE
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<'_, CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    ///Bits 29:31 - TXFTCFG
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<'_, CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
/**Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#LPUART1:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
