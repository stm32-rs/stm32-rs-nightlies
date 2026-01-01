///Register `CR3_ALTERNATE1` reader
pub type R = crate::R<CR3_ALTERNATE1rs>;
///Register `CR3_ALTERNATE1` writer
pub type W = crate::W<CR3_ALTERNATE1rs>;
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader;
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader;
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `OVRDIS` reader - Overrun Disable
pub type OVRDIS_R = crate::BitReader;
///Field `OVRDIS` writer - Overrun Disable
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRE` reader - DMA Disable on reception Error
pub type DDRE_R = crate::BitReader;
///Field `DDRE` writer - DMA Disable on reception Error
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEM` reader - Driver enable mode
pub type DEM_R = crate::BitReader;
///Field `DEM` writer - Driver enable mode
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEP` reader - Driver enable polarity selection
pub type DEP_R = crate::BitReader;
///Field `DEP` writer - Driver enable polarity selection
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUS0` reader - Wake-up from low-power mode interrupt flag selection
pub type WUS0_R = crate::BitReader;
///Field `WUS0` writer - Wake-up from low-power mode interrupt flag selection
pub type WUS0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUS1` reader - Wake-up from low-power mode interrupt flag selection
pub type WUS1_R = crate::BitReader;
///Field `WUS1` writer - Wake-up from low-power mode interrupt flag selection
pub type WUS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUFIE` reader - Wake-up from low-power mode interrupt enable
pub type WUFIE_R = crate::BitReader;
///Field `WUFIE` writer - Wake-up from low-power mode interrupt enable
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 13 - DMA Disable on reception Error
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
    ///Bit 20 - Wake-up from low-power mode interrupt flag selection
    #[inline(always)]
    pub fn wus0(&self) -> WUS0_R {
        WUS0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Wake-up from low-power mode interrupt flag selection
    #[inline(always)]
    pub fn wus1(&self) -> WUS1_R {
        WUS1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Wake-up from low-power mode interrupt enable
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3_ALTERNATE1")
            .field("eie", &self.eie())
            .field("hdsel", &self.hdsel())
            .field("dmar", &self.dmar())
            .field("dmat", &self.dmat())
            .field("rtse", &self.rtse())
            .field("ctse", &self.ctse())
            .field("ctsie", &self.ctsie())
            .field("ovrdis", &self.ovrdis())
            .field("ddre", &self.ddre())
            .field("dem", &self.dem())
            .field("dep", &self.dep())
            .field("wus0", &self.wus0())
            .field("wus1", &self.wus1())
            .field("wufie", &self.wufie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<'_, CR3_ALTERNATE1rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<'_, CR3_ALTERNATE1rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<'_, CR3_ALTERNATE1rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<'_, CR3_ALTERNATE1rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<'_, CR3_ALTERNATE1rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<'_, CR3_ALTERNATE1rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<'_, CR3_ALTERNATE1rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<'_, CR3_ALTERNATE1rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DMA Disable on reception Error
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<'_, CR3_ALTERNATE1rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<'_, CR3_ALTERNATE1rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<'_, CR3_ALTERNATE1rs> {
        DEP_W::new(self, 15)
    }
    ///Bit 20 - Wake-up from low-power mode interrupt flag selection
    #[inline(always)]
    pub fn wus0(&mut self) -> WUS0_W<'_, CR3_ALTERNATE1rs> {
        WUS0_W::new(self, 20)
    }
    ///Bit 21 - Wake-up from low-power mode interrupt flag selection
    #[inline(always)]
    pub fn wus1(&mut self) -> WUS1_W<'_, CR3_ALTERNATE1rs> {
        WUS1_W::new(self, 21)
    }
    ///Bit 22 - Wake-up from low-power mode interrupt enable
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<'_, CR3_ALTERNATE1rs> {
        WUFIE_W::new(self, 22)
    }
}
/**LPUART control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#LPUART:CR3_ALTERNATE1)*/
pub struct CR3_ALTERNATE1rs;
impl crate::RegisterSpec for CR3_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`cr3_alternate1::R`](R) reader structure
impl crate::Readable for CR3_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`cr3_alternate1::W`](W) writer structure
impl crate::Writable for CR3_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3_ALTERNATE1 to value 0
impl crate::Resettable for CR3_ALTERNATE1rs {}
