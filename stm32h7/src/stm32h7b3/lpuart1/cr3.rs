#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader;
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader;
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader;
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader;
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRDIS` reader - Overrun Disable"]
pub type OVRDIS_R = crate::BitReader;
#[doc = "Field `OVRDIS` writer - Overrun Disable"]
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error"]
pub type DDRE_R = crate::BitReader;
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error"]
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DEM_R = crate::BitReader;
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - Driver enable polarity selection"]
pub type DEP_R = crate::BitReader;
#[doc = "Field `DEP` writer - Driver enable polarity selection"]
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUS` reader - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_R = crate::FieldReader;
#[doc = "Field `WUS` writer - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WUFIE` reader - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_R = crate::BitReader;
#[doc = "Field `WUFIE` writer - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTIE` reader - TXFIFO threshold interrupt enable"]
pub type TXFTIE_R = crate::BitReader;
#[doc = "Field `TXFTIE` writer - TXFIFO threshold interrupt enable"]
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTCFG` reader - Receive FIFO threshold configuration"]
pub type RXFTCFG_R = crate::FieldReader;
#[doc = "Field `RXFTCFG` writer - Receive FIFO threshold configuration"]
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFTIE` reader - RXFIFO threshold interrupt enable"]
pub type RXFTIE_R = crate::BitReader;
#[doc = "Field `RXFTIE` writer - RXFIFO threshold interrupt enable"]
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTCFG` reader - TXFIFO threshold configuration"]
pub type TXFTCFG_R = crate::FieldReader;
#[doc = "Field `TXFTCFG` writer - TXFIFO threshold configuration"]
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration"]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration"]
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrdis(&mut self) -> OVRDIS_W<CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DDRE_W<CR3rs> {
        DDRE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DEM_W<CR3rs> {
        DEM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<CR3rs> {
        DEP_W::new(self, 15)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    #[must_use]
    pub fn wus(&mut self) -> WUS_W<CR3rs> {
        WUS_W::new(self, 20)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wufie(&mut self) -> WUFIE_W<CR3rs> {
        WUFIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txftie(&mut self) -> TXFTIE_W<CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxftie(&mut self) -> RXFTIE_W<CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration"]
    #[inline(always)]
    #[must_use]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
