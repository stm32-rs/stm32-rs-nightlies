#[doc = "Register `IER1` reader"]
pub type R = crate::R<IER1rs>;
#[doc = "Register `IER1` writer"]
pub type W = crate::W<IER1rs>;
#[doc = "Field `TZICIE` reader - TZICIE"]
pub type TZICIE_R = crate::BitReader;
#[doc = "Field `TZICIE` writer - TZICIE"]
pub type TZICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSCIE` reader - TZSCIE"]
pub type TZSCIE_R = crate::BitReader;
#[doc = "Field `TZSCIE` writer - TZSCIE"]
pub type TZSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESIE` reader - AESIE"]
pub type AESIE_R = crate::BitReader;
#[doc = "Field `AESIE` writer - AESIE"]
pub type AESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGIE` reader - RNGIE"]
pub type RNGIE_R = crate::BitReader;
#[doc = "Field `RNGIE` writer - RNGIE"]
pub type RNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUBGHZSPIIE` reader - SUBGHZSPIIE"]
pub type SUBGHZSPIIE_R = crate::BitReader;
#[doc = "Field `SUBGHZSPIIE` writer - SUBGHZSPIIE"]
pub type SUBGHZSPIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRIE` reader - PWRIE"]
pub type PWRIE_R = crate::BitReader;
#[doc = "Field `PWRIE` writer - PWRIE"]
pub type PWRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHIFIE` reader - FLASHIFIE"]
pub type FLASHIFIE_R = crate::BitReader;
#[doc = "Field `FLASHIFIE` writer - FLASHIFIE"]
pub type FLASHIFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1IE` reader - DMA1IE"]
pub type DMA1IE_R = crate::BitReader;
#[doc = "Field `DMA1IE` writer - DMA1IE"]
pub type DMA1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2IE` reader - DMA2IE"]
pub type DMA2IE_R = crate::BitReader;
#[doc = "Field `DMA2IE` writer - DMA2IE"]
pub type DMA2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUX1IE` reader - DMAMUX1IE"]
pub type DMAMUX1IE_R = crate::BitReader;
#[doc = "Field `DMAMUX1IE` writer - DMAMUX1IE"]
pub type DMAMUX1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHIE` reader - FLASHIE"]
pub type FLASHIE_R = crate::BitReader;
#[doc = "Field `FLASHIE` writer - FLASHIE"]
pub type FLASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1IE` reader - SRAM1IE"]
pub type SRAM1IE_R = crate::BitReader;
#[doc = "Field `SRAM1IE` writer - SRAM1IE"]
pub type SRAM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2IE` reader - SRAM2IE"]
pub type SRAM2IE_R = crate::BitReader;
#[doc = "Field `SRAM2IE` writer - SRAM2IE"]
pub type SRAM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAIE` reader - PKAIE"]
pub type PKAIE_R = crate::BitReader;
#[doc = "Field `PKAIE` writer - PKAIE"]
pub type PKAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    pub fn subghzspiie(&self) -> SUBGHZSPIIE_R {
        SUBGHZSPIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    pub fn flashifie(&self) -> FLASHIFIE_R {
        FLASHIFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzicie(&mut self) -> TZICIE_W<IER1rs> {
        TZICIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzscie(&mut self) -> TZSCIE_W<IER1rs> {
        TZSCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    #[must_use]
    pub fn aesie(&mut self) -> AESIE_W<IER1rs> {
        AESIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    #[must_use]
    pub fn rngie(&mut self) -> RNGIE_W<IER1rs> {
        RNGIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspiie(&mut self) -> SUBGHZSPIIE_W<IER1rs> {
        SUBGHZSPIIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    #[must_use]
    pub fn pwrie(&mut self) -> PWRIE_W<IER1rs> {
        PWRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    #[must_use]
    pub fn flashifie(&mut self) -> FLASHIFIE_W<IER1rs> {
        FLASHIFIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ie(&mut self) -> DMA1IE_W<IER1rs> {
        DMA1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ie(&mut self) -> DMA2IE_W<IER1rs> {
        DMA2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W<IER1rs> {
        DMAMUX1IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    #[must_use]
    pub fn flashie(&mut self) -> FLASHIE_W<IER1rs> {
        FLASHIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    #[must_use]
    pub fn sram1ie(&mut self) -> SRAM1IE_W<IER1rs> {
        SRAM1IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    #[must_use]
    pub fn sram2ie(&mut self) -> SRAM2IE_W<IER1rs> {
        SRAM2IE_W::new(self, 12)
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    #[must_use]
    pub fn pkaie(&mut self) -> PKAIE_W<IER1rs> {
        PKAIE_W::new(self, 13)
    }
}
#[doc = "TZIC interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier1::R`](R) reader structure"]
impl crate::Readable for IER1rs {}
#[doc = "`write(|w| ..)` method takes [`ier1::W`](W) writer structure"]
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER1 to value 0xffff_ffff"]
impl crate::Resettable for IER1rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
