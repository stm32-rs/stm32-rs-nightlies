#[doc = "Register `ICR1` reader"]
pub type R = crate::R<ICR1rs>;
#[doc = "Register `ICR1` writer"]
pub type W = crate::W<ICR1rs>;
#[doc = "Field `TZICCF` reader - TZICCF"]
pub type TZICCF_R = crate::BitReader;
#[doc = "Field `TZICCF` writer - TZICCF"]
pub type TZICCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSCCF` reader - TZSCCF"]
pub type TZSCCF_R = crate::BitReader;
#[doc = "Field `TZSCCF` writer - TZSCCF"]
pub type TZSCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESCF` reader - AESCF"]
pub type AESCF_R = crate::BitReader;
#[doc = "Field `AESCF` writer - AESCF"]
pub type AESCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGCF` reader - RNGCF"]
pub type RNGCF_R = crate::BitReader;
#[doc = "Field `RNGCF` writer - RNGCF"]
pub type RNGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUBGHZSPICF` reader - SUBGHZSPICF"]
pub type SUBGHZSPICF_R = crate::BitReader;
#[doc = "Field `SUBGHZSPICF` writer - SUBGHZSPICF"]
pub type SUBGHZSPICF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRCF` reader - PWRCF"]
pub type PWRCF_R = crate::BitReader;
#[doc = "Field `PWRCF` writer - PWRCF"]
pub type PWRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHIFCF` reader - FLASHIFCF"]
pub type FLASHIFCF_R = crate::BitReader;
#[doc = "Field `FLASHIFCF` writer - FLASHIFCF"]
pub type FLASHIFCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1CF` reader - DMA1CF"]
pub type DMA1CF_R = crate::BitReader;
#[doc = "Field `DMA1CF` writer - DMA1CF"]
pub type DMA1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CF` reader - DMA2CF"]
pub type DMA2CF_R = crate::BitReader;
#[doc = "Field `DMA2CF` writer - DMA2CF"]
pub type DMA2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUX1CF` reader - DMAMUX1CF"]
pub type DMAMUX1CF_R = crate::BitReader;
#[doc = "Field `DMAMUX1CF` writer - DMAMUX1CF"]
pub type DMAMUX1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHCF` reader - FLASHCF"]
pub type FLASHCF_R = crate::BitReader;
#[doc = "Field `FLASHCF` writer - FLASHCF"]
pub type FLASHCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1CF` reader - SRAM1CF"]
pub type SRAM1CF_R = crate::BitReader;
#[doc = "Field `SRAM1CF` writer - SRAM1CF"]
pub type SRAM1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2CF` reader - SRAM2CF"]
pub type SRAM2CF_R = crate::BitReader;
#[doc = "Field `SRAM2CF` writer - SRAM2CF"]
pub type SRAM2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKACF` reader - PKACF"]
pub type PKACF_R = crate::BitReader;
#[doc = "Field `PKACF` writer - PKACF"]
pub type PKACF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    pub fn tziccf(&self) -> TZICCF_R {
        TZICCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    pub fn tzsccf(&self) -> TZSCCF_R {
        TZSCCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    pub fn aescf(&self) -> AESCF_R {
        AESCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    pub fn rngcf(&self) -> RNGCF_R {
        RNGCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    pub fn subghzspicf(&self) -> SUBGHZSPICF_R {
        SUBGHZSPICF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    pub fn pwrcf(&self) -> PWRCF_R {
        PWRCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    pub fn flashifcf(&self) -> FLASHIFCF_R {
        FLASHIFCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    pub fn dma1cf(&self) -> DMA1CF_R {
        DMA1CF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    pub fn dma2cf(&self) -> DMA2CF_R {
        DMA2CF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    pub fn dmamux1cf(&self) -> DMAMUX1CF_R {
        DMAMUX1CF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    pub fn flashcf(&self) -> FLASHCF_R {
        FLASHCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    pub fn sram1cf(&self) -> SRAM1CF_R {
        SRAM1CF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    pub fn sram2cf(&self) -> SRAM2CF_R {
        SRAM2CF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    pub fn pkacf(&self) -> PKACF_R {
        PKACF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    #[must_use]
    pub fn tziccf(&mut self) -> TZICCF_W<ICR1rs> {
        TZICCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    #[must_use]
    pub fn tzsccf(&mut self) -> TZSCCF_W<ICR1rs> {
        TZSCCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    #[must_use]
    pub fn aescf(&mut self) -> AESCF_W<ICR1rs> {
        AESCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    #[must_use]
    pub fn rngcf(&mut self) -> RNGCF_W<ICR1rs> {
        RNGCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspicf(&mut self) -> SUBGHZSPICF_W<ICR1rs> {
        SUBGHZSPICF_W::new(self, 4)
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrcf(&mut self) -> PWRCF_W<ICR1rs> {
        PWRCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    #[must_use]
    pub fn flashifcf(&mut self) -> FLASHIFCF_W<ICR1rs> {
        FLASHIFCF_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    #[must_use]
    pub fn dma1cf(&mut self) -> DMA1CF_W<ICR1rs> {
        DMA1CF_W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    #[must_use]
    pub fn dma2cf(&mut self) -> DMA2CF_W<ICR1rs> {
        DMA2CF_W::new(self, 8)
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1cf(&mut self) -> DMAMUX1CF_W<ICR1rs> {
        DMAMUX1CF_W::new(self, 9)
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    #[must_use]
    pub fn flashcf(&mut self) -> FLASHCF_W<ICR1rs> {
        FLASHCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    #[must_use]
    pub fn sram1cf(&mut self) -> SRAM1CF_W<ICR1rs> {
        SRAM1CF_W::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    #[must_use]
    pub fn sram2cf(&mut self) -> SRAM2CF_W<ICR1rs> {
        SRAM2CF_W::new(self, 12)
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    #[must_use]
    pub fn pkacf(&mut self) -> PKACF_W<ICR1rs> {
        PKACF_W::new(self, 13)
    }
}
#[doc = "TZIC interrupt status clear register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR1rs;
impl crate::RegisterSpec for ICR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr1::R`](R) reader structure"]
impl crate::Readable for ICR1rs {}
#[doc = "`write(|w| ..)` method takes [`icr1::W`](W) writer structure"]
impl crate::Writable for ICR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR1 to value 0"]
impl crate::Resettable for ICR1rs {
    const RESET_VALUE: u32 = 0;
}
