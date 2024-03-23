#[doc = "Register `C2IMR2` reader"]
pub type R = crate::R<C2IMR2rs>;
#[doc = "Register `C2IMR2` writer"]
pub type W = crate::W<C2IMR2rs>;
#[doc = "Field `DMA1CH1IM` reader - DMA1CH1IM"]
pub type DMA1CH1IM_R = crate::BitReader;
#[doc = "Field `DMA1CH1IM` writer - DMA1CH1IM"]
pub type DMA1CH1IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1CH2IM` reader - DMA1CH2IM"]
pub type DMA1CH2IM_R = crate::BitReader;
#[doc = "Field `DMA1CH2IM` writer - DMA1CH2IM"]
pub type DMA1CH2IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1CH3IM` reader - DMA1CH3IM"]
pub type DMA1CH3IM_R = crate::BitReader;
#[doc = "Field `DMA1CH3IM` writer - DMA1CH3IM"]
pub type DMA1CH3IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1CH4IM` reader - DMA1CH4IM"]
pub type DMA1CH4IM_R = crate::BitReader;
#[doc = "Field `DMA1CH4IM` writer - DMA1CH4IM"]
pub type DMA1CH4IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1CH5IM` reader - DMA1CH5IM"]
pub type DMA1CH5IM_R = crate::BitReader;
#[doc = "Field `DMA1CH5IM` writer - DMA1CH5IM"]
pub type DMA1CH5IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1CH6IM` reader - DMA1CH6IM"]
pub type DMA1CH6IM_R = crate::BitReader;
#[doc = "Field `DMA1CH6IM` writer - DMA1CH6IM"]
pub type DMA1CH6IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1CH7IM` reader - DMA1CH7IM"]
pub type DMA1CH7IM_R = crate::BitReader;
#[doc = "Field `DMA1CH7IM` writer - DMA1CH7IM"]
pub type DMA1CH7IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CH1IM` reader - DMA2CH1IM"]
pub type DMA2CH1IM_R = crate::BitReader;
#[doc = "Field `DMA2CH1IM` writer - DMA2CH1IM"]
pub type DMA2CH1IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CH2IM` reader - DMA2CH2IM"]
pub type DMA2CH2IM_R = crate::BitReader;
#[doc = "Field `DMA2CH2IM` writer - DMA2CH2IM"]
pub type DMA2CH2IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CH3IM` reader - DMA2CH3IM"]
pub type DMA2CH3IM_R = crate::BitReader;
#[doc = "Field `DMA2CH3IM` writer - DMA2CH3IM"]
pub type DMA2CH3IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CH4IM` reader - DMA2CH4IM"]
pub type DMA2CH4IM_R = crate::BitReader;
#[doc = "Field `DMA2CH4IM` writer - DMA2CH4IM"]
pub type DMA2CH4IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CH5IM` reader - DMA2CH5IM"]
pub type DMA2CH5IM_R = crate::BitReader;
#[doc = "Field `DMA2CH5IM` writer - DMA2CH5IM"]
pub type DMA2CH5IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CH6IM` reader - DMA2CH6IM"]
pub type DMA2CH6IM_R = crate::BitReader;
#[doc = "Field `DMA2CH6IM` writer - DMA2CH6IM"]
pub type DMA2CH6IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2CH7IM` reader - DMA2CH7IM"]
pub type DMA2CH7IM_R = crate::BitReader;
#[doc = "Field `DMA2CH7IM` writer - DMA2CH7IM"]
pub type DMA2CH7IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUX1IM` reader - DMAMUX1IM"]
pub type DMAMUX1IM_R = crate::BitReader;
#[doc = "Field `DMAMUX1IM` writer - DMAMUX1IM"]
pub type DMAMUX1IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVM3IM` reader - PVM3IM"]
pub type PVM3IM_R = crate::BitReader;
#[doc = "Field `PVM3IM` writer - PVM3IM"]
pub type PVM3IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDIM` reader - PVDIM"]
pub type PVDIM_R = crate::BitReader;
#[doc = "Field `PVDIM` writer - PVDIM"]
pub type PVDIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1CH1IM"]
    #[inline(always)]
    pub fn dma1ch1im(&self) -> DMA1CH1IM_R {
        DMA1CH1IM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1CH2IM"]
    #[inline(always)]
    pub fn dma1ch2im(&self) -> DMA1CH2IM_R {
        DMA1CH2IM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA1CH3IM"]
    #[inline(always)]
    pub fn dma1ch3im(&self) -> DMA1CH3IM_R {
        DMA1CH3IM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA1CH4IM"]
    #[inline(always)]
    pub fn dma1ch4im(&self) -> DMA1CH4IM_R {
        DMA1CH4IM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA1CH5IM"]
    #[inline(always)]
    pub fn dma1ch5im(&self) -> DMA1CH5IM_R {
        DMA1CH5IM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA1CH6IM"]
    #[inline(always)]
    pub fn dma1ch6im(&self) -> DMA1CH6IM_R {
        DMA1CH6IM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA1CH7IM"]
    #[inline(always)]
    pub fn dma1ch7im(&self) -> DMA1CH7IM_R {
        DMA1CH7IM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA2CH1IM"]
    #[inline(always)]
    pub fn dma2ch1im(&self) -> DMA2CH1IM_R {
        DMA2CH1IM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA2CH2IM"]
    #[inline(always)]
    pub fn dma2ch2im(&self) -> DMA2CH2IM_R {
        DMA2CH2IM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA2CH3IM"]
    #[inline(always)]
    pub fn dma2ch3im(&self) -> DMA2CH3IM_R {
        DMA2CH3IM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA2CH4IM"]
    #[inline(always)]
    pub fn dma2ch4im(&self) -> DMA2CH4IM_R {
        DMA2CH4IM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA2CH5IM"]
    #[inline(always)]
    pub fn dma2ch5im(&self) -> DMA2CH5IM_R {
        DMA2CH5IM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA2CH6IM"]
    #[inline(always)]
    pub fn dma2ch6im(&self) -> DMA2CH6IM_R {
        DMA2CH6IM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA2CH7IM"]
    #[inline(always)]
    pub fn dma2ch7im(&self) -> DMA2CH7IM_R {
        DMA2CH7IM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMAMUX1IM"]
    #[inline(always)]
    pub fn dmamux1im(&self) -> DMAMUX1IM_R {
        DMAMUX1IM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - PVM3IM"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - PVDIM"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1CH1IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ch1im(&mut self) -> DMA1CH1IM_W<C2IMR2rs> {
        DMA1CH1IM_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA1CH2IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ch2im(&mut self) -> DMA1CH2IM_W<C2IMR2rs> {
        DMA1CH2IM_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA1CH3IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ch3im(&mut self) -> DMA1CH3IM_W<C2IMR2rs> {
        DMA1CH3IM_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA1CH4IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ch4im(&mut self) -> DMA1CH4IM_W<C2IMR2rs> {
        DMA1CH4IM_W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA1CH5IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ch5im(&mut self) -> DMA1CH5IM_W<C2IMR2rs> {
        DMA1CH5IM_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA1CH6IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ch6im(&mut self) -> DMA1CH6IM_W<C2IMR2rs> {
        DMA1CH6IM_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA1CH7IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ch7im(&mut self) -> DMA1CH7IM_W<C2IMR2rs> {
        DMA1CH7IM_W::new(self, 6)
    }
    #[doc = "Bit 8 - DMA2CH1IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ch1im(&mut self) -> DMA2CH1IM_W<C2IMR2rs> {
        DMA2CH1IM_W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA2CH2IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ch2im(&mut self) -> DMA2CH2IM_W<C2IMR2rs> {
        DMA2CH2IM_W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA2CH3IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ch3im(&mut self) -> DMA2CH3IM_W<C2IMR2rs> {
        DMA2CH3IM_W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA2CH4IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ch4im(&mut self) -> DMA2CH4IM_W<C2IMR2rs> {
        DMA2CH4IM_W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA2CH5IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ch5im(&mut self) -> DMA2CH5IM_W<C2IMR2rs> {
        DMA2CH5IM_W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA2CH6IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ch6im(&mut self) -> DMA2CH6IM_W<C2IMR2rs> {
        DMA2CH6IM_W::new(self, 13)
    }
    #[doc = "Bit 14 - DMA2CH7IM"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ch7im(&mut self) -> DMA2CH7IM_W<C2IMR2rs> {
        DMA2CH7IM_W::new(self, 14)
    }
    #[doc = "Bit 15 - DMAMUX1IM"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1im(&mut self) -> DMAMUX1IM_W<C2IMR2rs> {
        DMAMUX1IM_W::new(self, 15)
    }
    #[doc = "Bit 18 - PVM3IM"]
    #[inline(always)]
    #[must_use]
    pub fn pvm3im(&mut self) -> PVM3IM_W<C2IMR2rs> {
        PVM3IM_W::new(self, 18)
    }
    #[doc = "Bit 20 - PVDIM"]
    #[inline(always)]
    #[must_use]
    pub fn pvdim(&mut self) -> PVDIM_W<C2IMR2rs> {
        PVDIM_W::new(self, 20)
    }
}
#[doc = "SYSCFG CPU2 interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2IMR2rs;
impl crate::RegisterSpec for C2IMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2imr2::R`](R) reader structure"]
impl crate::Readable for C2IMR2rs {}
#[doc = "`write(|w| ..)` method takes [`c2imr2::W`](W) writer structure"]
impl crate::Writable for C2IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2IMR2 to value 0"]
impl crate::Resettable for C2IMR2rs {
    const RESET_VALUE: u32 = 0;
}
