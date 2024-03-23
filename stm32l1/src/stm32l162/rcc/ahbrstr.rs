#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTRrs>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTRrs>;
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GPIOARST_R = crate::BitReader;
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub type GPIOBRST_R = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub type GPIOCRST_R = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub type GPIODRST_R = crate::BitReader;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub type GPIOERST_R = crate::BitReader;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub type GPIOHRST_R = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub type GPIOFRST_R = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGRST` reader - IO port G reset"]
pub type GPIOGRST_R = crate::BitReader;
#[doc = "Field `GPIOGRST` writer - IO port G reset"]
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITFRST` reader - FLITF reset"]
pub type FLITFRST_R = crate::BitReader;
#[doc = "Field `FLITFRST` writer - FLITF reset"]
pub type FLITFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type DMA1RST_R = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub type DMA2RST_R = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSMCRST` reader - FSMC reset"]
pub type FSMCRST_R = crate::BitReader;
#[doc = "Field `FSMCRST` writer - FSMC reset"]
pub type FSMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - FLITF reset"]
    #[inline(always)]
    pub fn flitfrst(&self) -> FLITFRST_R {
        FLITFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - FSMC reset"]
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHBRSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHBRSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHBRSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHBRSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<AHBRSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHBRSTRrs> {
        GPIOHRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<AHBRSTRrs> {
        GPIOFRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<AHBRSTRrs> {
        GPIOGRST_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
    #[doc = "Bit 15 - FLITF reset"]
    #[inline(always)]
    #[must_use]
    pub fn flitfrst(&mut self) -> FLITFRST_W<AHBRSTRrs> {
        FLITFRST_W::new(self, 15)
    }
    #[doc = "Bit 24 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHBRSTRrs> {
        DMA1RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<AHBRSTRrs> {
        DMA2RST_W::new(self, 25)
    }
    #[doc = "Bit 30 - FSMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn fsmcrst(&mut self) -> FSMCRST_W<AHBRSTRrs> {
        FSMCRST_W::new(self, 30)
    }
}
#[doc = "AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTRrs {
    const RESET_VALUE: u32 = 0;
}
