///Register `IER1` reader
pub type R = crate::R<IER1rs>;
///Register `IER1` writer
pub type W = crate::W<IER1rs>;
///Field `TZICIE` reader - TZICIE
pub type TZICIE_R = crate::BitReader;
///Field `TZICIE` writer - TZICIE
pub type TZICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZSCIE` reader - TZSCIE
pub type TZSCIE_R = crate::BitReader;
///Field `TZSCIE` writer - TZSCIE
pub type TZSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESIE` reader - AESIE
pub type AESIE_R = crate::BitReader;
///Field `AESIE` writer - AESIE
pub type AESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGIE` reader - RNGIE
pub type RNGIE_R = crate::BitReader;
///Field `RNGIE` writer - RNGIE
pub type RNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUBGHZSPIIE` reader - SUBGHZSPIIE
pub type SUBGHZSPIIE_R = crate::BitReader;
///Field `SUBGHZSPIIE` writer - SUBGHZSPIIE
pub type SUBGHZSPIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRIE` reader - PWRIE
pub type PWRIE_R = crate::BitReader;
///Field `PWRIE` writer - PWRIE
pub type PWRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHIFIE` reader - FLASHIFIE
pub type FLASHIFIE_R = crate::BitReader;
///Field `FLASHIFIE` writer - FLASHIFIE
pub type FLASHIFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1IE` reader - DMA1IE
pub type DMA1IE_R = crate::BitReader;
///Field `DMA1IE` writer - DMA1IE
pub type DMA1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2IE` reader - DMA2IE
pub type DMA2IE_R = crate::BitReader;
///Field `DMA2IE` writer - DMA2IE
pub type DMA2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAMUX1IE` reader - DMAMUX1IE
pub type DMAMUX1IE_R = crate::BitReader;
///Field `DMAMUX1IE` writer - DMAMUX1IE
pub type DMAMUX1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHIE` reader - FLASHIE
pub type FLASHIE_R = crate::BitReader;
///Field `FLASHIE` writer - FLASHIE
pub type FLASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1IE` reader - SRAM1IE
pub type SRAM1IE_R = crate::BitReader;
///Field `SRAM1IE` writer - SRAM1IE
pub type SRAM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2IE` reader - SRAM2IE
pub type SRAM2IE_R = crate::BitReader;
///Field `SRAM2IE` writer - SRAM2IE
pub type SRAM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAIE` reader - PKAIE
pub type PKAIE_R = crate::BitReader;
///Field `PKAIE` writer - PKAIE
pub type PKAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TZICIE
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZSCIE
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AESIE
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGIE
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SUBGHZSPIIE
    #[inline(always)]
    pub fn subghzspiie(&self) -> SUBGHZSPIIE_R {
        SUBGHZSPIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PWRIE
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FLASHIFIE
    #[inline(always)]
    pub fn flashifie(&self) -> FLASHIFIE_R {
        FLASHIFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA1IE
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DMA2IE
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMAMUX1IE
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FLASHIE
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM1IE
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM2IE
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PKAIE
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER1")
            .field("tzicie", &self.tzicie())
            .field("tzscie", &self.tzscie())
            .field("aesie", &self.aesie())
            .field("rngie", &self.rngie())
            .field("subghzspiie", &self.subghzspiie())
            .field("pwrie", &self.pwrie())
            .field("flashifie", &self.flashifie())
            .field("dma1ie", &self.dma1ie())
            .field("dma2ie", &self.dma2ie())
            .field("dmamux1ie", &self.dmamux1ie())
            .field("flashie", &self.flashie())
            .field("sram1ie", &self.sram1ie())
            .field("sram2ie", &self.sram2ie())
            .field("pkaie", &self.pkaie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TZICIE
    #[inline(always)]
    pub fn tzicie(&mut self) -> TZICIE_W<'_, IER1rs> {
        TZICIE_W::new(self, 0)
    }
    ///Bit 1 - TZSCIE
    #[inline(always)]
    pub fn tzscie(&mut self) -> TZSCIE_W<'_, IER1rs> {
        TZSCIE_W::new(self, 1)
    }
    ///Bit 2 - AESIE
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W<'_, IER1rs> {
        AESIE_W::new(self, 2)
    }
    ///Bit 3 - RNGIE
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W<'_, IER1rs> {
        RNGIE_W::new(self, 3)
    }
    ///Bit 4 - SUBGHZSPIIE
    #[inline(always)]
    pub fn subghzspiie(&mut self) -> SUBGHZSPIIE_W<'_, IER1rs> {
        SUBGHZSPIIE_W::new(self, 4)
    }
    ///Bit 5 - PWRIE
    #[inline(always)]
    pub fn pwrie(&mut self) -> PWRIE_W<'_, IER1rs> {
        PWRIE_W::new(self, 5)
    }
    ///Bit 6 - FLASHIFIE
    #[inline(always)]
    pub fn flashifie(&mut self) -> FLASHIFIE_W<'_, IER1rs> {
        FLASHIFIE_W::new(self, 6)
    }
    ///Bit 7 - DMA1IE
    #[inline(always)]
    pub fn dma1ie(&mut self) -> DMA1IE_W<'_, IER1rs> {
        DMA1IE_W::new(self, 7)
    }
    ///Bit 8 - DMA2IE
    #[inline(always)]
    pub fn dma2ie(&mut self) -> DMA2IE_W<'_, IER1rs> {
        DMA2IE_W::new(self, 8)
    }
    ///Bit 9 - DMAMUX1IE
    #[inline(always)]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W<'_, IER1rs> {
        DMAMUX1IE_W::new(self, 9)
    }
    ///Bit 10 - FLASHIE
    #[inline(always)]
    pub fn flashie(&mut self) -> FLASHIE_W<'_, IER1rs> {
        FLASHIE_W::new(self, 10)
    }
    ///Bit 11 - SRAM1IE
    #[inline(always)]
    pub fn sram1ie(&mut self) -> SRAM1IE_W<'_, IER1rs> {
        SRAM1IE_W::new(self, 11)
    }
    ///Bit 12 - SRAM2IE
    #[inline(always)]
    pub fn sram2ie(&mut self) -> SRAM2IE_W<'_, IER1rs> {
        SRAM2IE_W::new(self, 12)
    }
    ///Bit 13 - PKAIE
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W<'_, IER1rs> {
        PKAIE_W::new(self, 13)
    }
}
/**TZIC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TZIC:IER1)*/
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
///`read()` method returns [`ier1::R`](R) reader structure
impl crate::Readable for IER1rs {}
///`write(|w| ..)` method takes [`ier1::W`](W) writer structure
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER1 to value 0xffff_ffff
impl crate::Resettable for IER1rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
