#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub type EN1_R = crate::BitReader;
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFF1` reader - DAC channel1 output buffer disable"]
pub type BOFF1_R = crate::BitReader;
#[doc = "Field `BOFF1` writer - DAC channel1 output buffer disable"]
pub type BOFF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type TEN1_R = crate::BitReader;
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type TEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub type TSEL1_R = crate::FieldReader;
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub type TSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::FieldReader;
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP1_R = crate::FieldReader;
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub type DMAEN1_R = crate::BitReader;
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub type DMAEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_R = crate::BitReader;
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` reader - DAC channel2 enable"]
pub type EN2_R = crate::BitReader;
#[doc = "Field `EN2` writer - DAC channel2 enable"]
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFF2` reader - DAC channel2 output buffer disable"]
pub type BOFF2_R = crate::BitReader;
#[doc = "Field `BOFF2` writer - DAC channel2 output buffer disable"]
pub type BOFF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN2` reader - DAC channel2 trigger enable"]
pub type TEN2_R = crate::BitReader;
#[doc = "Field `TEN2` writer - DAC channel2 trigger enable"]
pub type TEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL2` reader - DAC channel2 trigger selection"]
pub type TSEL2_R = crate::FieldReader;
#[doc = "Field `TSEL2` writer - DAC channel2 trigger selection"]
pub type TSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable"]
pub type WAVE2_R = crate::FieldReader;
#[doc = "Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable"]
pub type WAVE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP2` reader - DAC channel2 mask/amplitude selector"]
pub type MAMP2_R = crate::FieldReader;
#[doc = "Field `MAMP2` writer - DAC channel2 mask/amplitude selector"]
pub type MAMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN2` reader - DAC channel2 DMA enable"]
pub type DMAEN2_R = crate::BitReader;
#[doc = "Field `DMAEN2` writer - DAC channel2 DMA enable"]
pub type DMAEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable"]
pub type DMAUDRIE2_R = crate::BitReader;
#[doc = "Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable"]
pub type DMAUDRIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline(always)]
    pub fn boff2(&self) -> BOFF2_R {
        BOFF2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CRrs> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn boff1(&mut self) -> BOFF1_W<CRrs> {
        BOFF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<CRrs> {
        TEN1_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<CRrs> {
        TSEL1_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<CRrs> {
        WAVE1_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> MAMP1_W<CRrs> {
        MAMP1_W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<CRrs> {
        DMAEN1_W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<CRrs> {
        DMAUDRIE1_W::new(self, 13)
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<CRrs> {
        EN2_W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn boff2(&mut self) -> BOFF2_W<CRrs> {
        BOFF2_W::new(self, 17)
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten2(&mut self) -> TEN2_W<CRrs> {
        TEN2_W::new(self, 18)
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel2(&mut self) -> TSEL2_W<CRrs> {
        TSEL2_W::new(self, 19)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<CRrs> {
        WAVE2_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp2(&mut self) -> MAMP2_W<CRrs> {
        MAMP2_W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen2(&mut self) -> DMAEN2_W<CRrs> {
        DMAEN2_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W<CRrs> {
        DMAUDRIE2_W::new(self, 29)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
