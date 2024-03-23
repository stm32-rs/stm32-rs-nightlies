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
#[doc = "Field `WAVE2` reader - WAVE2"]
pub type WAVE2_R = crate::BitReader;
#[doc = "Field `WAVE2` writer - WAVE2"]
pub type WAVE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::BitReader;
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP10` reader - MAMP10"]
pub type MAMP10_R = crate::BitReader;
#[doc = "Field `MAMP10` writer - MAMP10"]
pub type MAMP10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP11` reader - MAMP11"]
pub type MAMP11_R = crate::BitReader;
#[doc = "Field `MAMP11` writer - MAMP11"]
pub type MAMP11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP12` reader - MAMP12"]
pub type MAMP12_R = crate::BitReader;
#[doc = "Field `MAMP12` writer - MAMP12"]
pub type MAMP12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP13` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP13_R = crate::BitReader;
#[doc = "Field `MAMP13` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub type DMAEN1_R = crate::BitReader;
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub type DMAEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_R = crate::BitReader;
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - WAVE2"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MAMP10"]
    #[inline(always)]
    pub fn mamp10(&self) -> MAMP10_R {
        MAMP10_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MAMP11"]
    #[inline(always)]
    pub fn mamp11(&self) -> MAMP11_R {
        MAMP11_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MAMP12"]
    #[inline(always)]
    pub fn mamp12(&self) -> MAMP12_R {
        MAMP12_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp13(&self) -> MAMP13_R {
        MAMP13_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 6 - WAVE2"]
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<CRrs> {
        WAVE2_W::new(self, 6)
    }
    #[doc = "Bit 7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<CRrs> {
        WAVE1_W::new(self, 7)
    }
    #[doc = "Bit 8 - MAMP10"]
    #[inline(always)]
    #[must_use]
    pub fn mamp10(&mut self) -> MAMP10_W<CRrs> {
        MAMP10_W::new(self, 8)
    }
    #[doc = "Bit 9 - MAMP11"]
    #[inline(always)]
    #[must_use]
    pub fn mamp11(&mut self) -> MAMP11_W<CRrs> {
        MAMP11_W::new(self, 9)
    }
    #[doc = "Bit 10 - MAMP12"]
    #[inline(always)]
    #[must_use]
    pub fn mamp12(&mut self) -> MAMP12_W<CRrs> {
        MAMP12_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp13(&mut self) -> MAMP13_W<CRrs> {
        MAMP13_W::new(self, 11)
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
