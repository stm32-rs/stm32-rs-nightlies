///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader;
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IREN` reader - Ir mode enable
pub type IREN_R = crate::BitReader;
///Field `IREN` writer - Ir mode enable
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRLP` reader - Ir low-power
pub type IRLP_R = crate::BitReader;
///Field `IRLP` writer - Ir low-power
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader;
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - Smartcard NACK enable
pub type NACK_R = crate::BitReader;
///Field `NACK` writer - Smartcard NACK enable
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCEN` reader - Smartcard mode enable
pub type SCEN_R = crate::BitReader;
///Field `SCEN` writer - Smartcard mode enable
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAR` reader - DMA enable receiver
pub type DMAR_R = crate::BitReader;
///Field `DMAR` writer - DMA enable receiver
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAT` reader - DMA enable transmitter
pub type DMAT_R = crate::BitReader;
///Field `DMAT` writer - DMA enable transmitter
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTSE` reader - RTS enable
pub type RTSE_R = crate::BitReader;
///Field `RTSE` writer - RTS enable
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSE` reader - CTS enable
pub type CTSE_R = crate::BitReader;
///Field `CTSE` writer - CTS enable
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSIE` reader - CTS interrupt enable
pub type CTSIE_R = crate::BitReader;
///Field `CTSIE` writer - CTS interrupt enable
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ONEBIT` reader - One sample bit method enable
pub type ONEBIT_R = crate::BitReader;
///Field `ONEBIT` writer - One sample bit method enable
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRDIS` reader - Overrun Disable
pub type OVRDIS_R = crate::BitReader;
///Field `OVRDIS` writer - Overrun Disable
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRE` reader - DMA Disable on Reception Error
pub type DDRE_R = crate::BitReader;
///Field `DDRE` writer - DMA Disable on Reception Error
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEM` reader - Driver enable mode
pub type DEM_R = crate::BitReader;
///Field `DEM` writer - Driver enable mode
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEP` reader - Driver enable polarity selection
pub type DEP_R = crate::BitReader;
///Field `DEP` writer - Driver enable polarity selection
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCARCNT` reader - Smartcard auto-retry count
pub type SCARCNT_R = crate::FieldReader;
///Field `SCARCNT` writer - Smartcard auto-retry count
pub type SCARCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WUS` reader - Wakeup from Stop mode interrupt flag selection
pub type WUS_R = crate::FieldReader;
///Field `WUS` writer - Wakeup from Stop mode interrupt flag selection
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUFIE` reader - Wakeup from Stop mode interrupt enable
pub type WUFIE_R = crate::BitReader;
///Field `WUFIE` writer - Wakeup from Stop mode interrupt enable
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFTIE` reader - threshold interrupt enable
pub type TXFTIE_R = crate::BitReader;
///Field `TXFTIE` writer - threshold interrupt enable
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCBGTIE` reader - Tr Complete before guard time, interrupt enable
pub type TCBGTIE_R = crate::BitReader;
///Field `TCBGTIE` writer - Tr Complete before guard time, interrupt enable
pub type TCBGTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFTCFG` reader - Receive FIFO threshold configuration
pub type RXFTCFG_R = crate::FieldReader;
///Field `RXFTCFG` writer - Receive FIFO threshold configuration
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXFTIE` reader - RXFIFO threshold interrupt enable
pub type RXFTIE_R = crate::BitReader;
///Field `RXFTIE` writer - RXFIFO threshold interrupt enable
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFTCFG` reader - TXFIFO threshold configuration
pub type TXFTCFG_R = crate::FieldReader;
///Field `TXFTCFG` writer - TXFIFO threshold configuration
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
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
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
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
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
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
    ///Bit 23 - threshold interrupt enable
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Tr Complete before guard time, interrupt enable
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFIFO threshold configuration
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
            .field("tcbgtie", &self.tcbgtie())
            .field("txftie", &self.txftie())
            .field("wufie", &self.wufie())
            .field("wus", &self.wus())
            .field("scarcnt", &self.scarcnt())
            .field("dep", &self.dep())
            .field("dem", &self.dem())
            .field("ddre", &self.ddre())
            .field("ovrdis", &self.ovrdis())
            .field("onebit", &self.onebit())
            .field("ctsie", &self.ctsie())
            .field("ctse", &self.ctse())
            .field("rtse", &self.rtse())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("scen", &self.scen())
            .field("nack", &self.nack())
            .field("hdsel", &self.hdsel())
            .field("irlp", &self.irlp())
            .field("iren", &self.iren())
            .field("eie", &self.eie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<CR3rs> {
        IREN_W::new(self, 1)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W<CR3rs> {
        IRLP_W::new(self, 2)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<CR3rs> {
        NACK_W::new(self, 4)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W<CR3rs> {
        SCEN_W::new(self, 5)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W<CR3rs> {
        ONEBIT_W::new(self, 11)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<CR3rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<CR3rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<CR3rs> {
        DEP_W::new(self, 15)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W<CR3rs> {
        SCARCNT_W::new(self, 17)
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<CR3rs> {
        WUS_W::new(self, 20)
    }
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<CR3rs> {
        WUFIE_W::new(self, 22)
    }
    ///Bit 23 - threshold interrupt enable
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    ///Bit 24 - Tr Complete before guard time, interrupt enable
    #[inline(always)]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W<CR3rs> {
        TCBGTIE_W::new(self, 24)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    ///Bits 29:31 - TXFIFO threshold configuration
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
/**Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USART1:CR3)*/
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
