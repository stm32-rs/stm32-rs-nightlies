///Register `ICR1` reader
pub type R = crate::R<ICR1rs>;
///Register `ICR1` writer
pub type W = crate::W<ICR1rs>;
///Field `TZICCF` reader - TZICCF
pub type TZICCF_R = crate::BitReader;
///Field `TZICCF` writer - TZICCF
pub type TZICCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZSCCF` reader - TZSCCF
pub type TZSCCF_R = crate::BitReader;
///Field `TZSCCF` writer - TZSCCF
pub type TZSCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESCF` reader - AESCF
pub type AESCF_R = crate::BitReader;
///Field `AESCF` writer - AESCF
pub type AESCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGCF` reader - RNGCF
pub type RNGCF_R = crate::BitReader;
///Field `RNGCF` writer - RNGCF
pub type RNGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUBGHZSPICF` reader - SUBGHZSPICF
pub type SUBGHZSPICF_R = crate::BitReader;
///Field `SUBGHZSPICF` writer - SUBGHZSPICF
pub type SUBGHZSPICF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRCF` reader - PWRCF
pub type PWRCF_R = crate::BitReader;
///Field `PWRCF` writer - PWRCF
pub type PWRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHIFCF` reader - FLASHIFCF
pub type FLASHIFCF_R = crate::BitReader;
///Field `FLASHIFCF` writer - FLASHIFCF
pub type FLASHIFCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1CF` reader - DMA1CF
pub type DMA1CF_R = crate::BitReader;
///Field `DMA1CF` writer - DMA1CF
pub type DMA1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2CF` reader - DMA2CF
pub type DMA2CF_R = crate::BitReader;
///Field `DMA2CF` writer - DMA2CF
pub type DMA2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAMUX1CF` reader - DMAMUX1CF
pub type DMAMUX1CF_R = crate::BitReader;
///Field `DMAMUX1CF` writer - DMAMUX1CF
pub type DMAMUX1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHCF` reader - FLASHCF
pub type FLASHCF_R = crate::BitReader;
///Field `FLASHCF` writer - FLASHCF
pub type FLASHCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1CF` reader - SRAM1CF
pub type SRAM1CF_R = crate::BitReader;
///Field `SRAM1CF` writer - SRAM1CF
pub type SRAM1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2CF` reader - SRAM2CF
pub type SRAM2CF_R = crate::BitReader;
///Field `SRAM2CF` writer - SRAM2CF
pub type SRAM2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKACF` reader - PKACF
pub type PKACF_R = crate::BitReader;
///Field `PKACF` writer - PKACF
pub type PKACF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TZICCF
    #[inline(always)]
    pub fn tziccf(&self) -> TZICCF_R {
        TZICCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZSCCF
    #[inline(always)]
    pub fn tzsccf(&self) -> TZSCCF_R {
        TZSCCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AESCF
    #[inline(always)]
    pub fn aescf(&self) -> AESCF_R {
        AESCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGCF
    #[inline(always)]
    pub fn rngcf(&self) -> RNGCF_R {
        RNGCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SUBGHZSPICF
    #[inline(always)]
    pub fn subghzspicf(&self) -> SUBGHZSPICF_R {
        SUBGHZSPICF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PWRCF
    #[inline(always)]
    pub fn pwrcf(&self) -> PWRCF_R {
        PWRCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FLASHIFCF
    #[inline(always)]
    pub fn flashifcf(&self) -> FLASHIFCF_R {
        FLASHIFCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA1CF
    #[inline(always)]
    pub fn dma1cf(&self) -> DMA1CF_R {
        DMA1CF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DMA2CF
    #[inline(always)]
    pub fn dma2cf(&self) -> DMA2CF_R {
        DMA2CF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMAMUX1CF
    #[inline(always)]
    pub fn dmamux1cf(&self) -> DMAMUX1CF_R {
        DMAMUX1CF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FLASHCF
    #[inline(always)]
    pub fn flashcf(&self) -> FLASHCF_R {
        FLASHCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM1CF
    #[inline(always)]
    pub fn sram1cf(&self) -> SRAM1CF_R {
        SRAM1CF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM2CF
    #[inline(always)]
    pub fn sram2cf(&self) -> SRAM2CF_R {
        SRAM2CF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PKACF
    #[inline(always)]
    pub fn pkacf(&self) -> PKACF_R {
        PKACF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR1")
            .field("tziccf", &self.tziccf())
            .field("tzsccf", &self.tzsccf())
            .field("aescf", &self.aescf())
            .field("rngcf", &self.rngcf())
            .field("subghzspicf", &self.subghzspicf())
            .field("pwrcf", &self.pwrcf())
            .field("flashifcf", &self.flashifcf())
            .field("dma1cf", &self.dma1cf())
            .field("dma2cf", &self.dma2cf())
            .field("dmamux1cf", &self.dmamux1cf())
            .field("flashcf", &self.flashcf())
            .field("sram1cf", &self.sram1cf())
            .field("sram2cf", &self.sram2cf())
            .field("pkacf", &self.pkacf())
            .finish()
    }
}
impl W {
    ///Bit 0 - TZICCF
    #[inline(always)]
    pub fn tziccf(&mut self) -> TZICCF_W<'_, ICR1rs> {
        TZICCF_W::new(self, 0)
    }
    ///Bit 1 - TZSCCF
    #[inline(always)]
    pub fn tzsccf(&mut self) -> TZSCCF_W<'_, ICR1rs> {
        TZSCCF_W::new(self, 1)
    }
    ///Bit 2 - AESCF
    #[inline(always)]
    pub fn aescf(&mut self) -> AESCF_W<'_, ICR1rs> {
        AESCF_W::new(self, 2)
    }
    ///Bit 3 - RNGCF
    #[inline(always)]
    pub fn rngcf(&mut self) -> RNGCF_W<'_, ICR1rs> {
        RNGCF_W::new(self, 3)
    }
    ///Bit 4 - SUBGHZSPICF
    #[inline(always)]
    pub fn subghzspicf(&mut self) -> SUBGHZSPICF_W<'_, ICR1rs> {
        SUBGHZSPICF_W::new(self, 4)
    }
    ///Bit 5 - PWRCF
    #[inline(always)]
    pub fn pwrcf(&mut self) -> PWRCF_W<'_, ICR1rs> {
        PWRCF_W::new(self, 5)
    }
    ///Bit 6 - FLASHIFCF
    #[inline(always)]
    pub fn flashifcf(&mut self) -> FLASHIFCF_W<'_, ICR1rs> {
        FLASHIFCF_W::new(self, 6)
    }
    ///Bit 7 - DMA1CF
    #[inline(always)]
    pub fn dma1cf(&mut self) -> DMA1CF_W<'_, ICR1rs> {
        DMA1CF_W::new(self, 7)
    }
    ///Bit 8 - DMA2CF
    #[inline(always)]
    pub fn dma2cf(&mut self) -> DMA2CF_W<'_, ICR1rs> {
        DMA2CF_W::new(self, 8)
    }
    ///Bit 9 - DMAMUX1CF
    #[inline(always)]
    pub fn dmamux1cf(&mut self) -> DMAMUX1CF_W<'_, ICR1rs> {
        DMAMUX1CF_W::new(self, 9)
    }
    ///Bit 10 - FLASHCF
    #[inline(always)]
    pub fn flashcf(&mut self) -> FLASHCF_W<'_, ICR1rs> {
        FLASHCF_W::new(self, 10)
    }
    ///Bit 11 - SRAM1CF
    #[inline(always)]
    pub fn sram1cf(&mut self) -> SRAM1CF_W<'_, ICR1rs> {
        SRAM1CF_W::new(self, 11)
    }
    ///Bit 12 - SRAM2CF
    #[inline(always)]
    pub fn sram2cf(&mut self) -> SRAM2CF_W<'_, ICR1rs> {
        SRAM2CF_W::new(self, 12)
    }
    ///Bit 13 - PKACF
    #[inline(always)]
    pub fn pkacf(&mut self) -> PKACF_W<'_, ICR1rs> {
        PKACF_W::new(self, 13)
    }
}
/**TZIC interrupt status clear register 1

You can [`read`](crate::Reg::read) this register and get [`icr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TZIC:ICR1)*/
pub struct ICR1rs;
impl crate::RegisterSpec for ICR1rs {
    type Ux = u32;
}
///`read()` method returns [`icr1::R`](R) reader structure
impl crate::Readable for ICR1rs {}
///`write(|w| ..)` method takes [`icr1::W`](W) writer structure
impl crate::Writable for ICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR1 to value 0
impl crate::Resettable for ICR1rs {}
