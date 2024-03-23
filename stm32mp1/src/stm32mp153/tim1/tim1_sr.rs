#[doc = "Register `TIM1_SR` reader"]
pub type R = crate::R<TIM1_SRrs>;
#[doc = "Register `TIM1_SR` writer"]
pub type W = crate::W<TIM1_SRrs>;
#[doc = "Field `UIF` reader - UIF"]
pub type UIF_R = crate::BitReader;
#[doc = "Field `UIF` writer - UIF"]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - CC1IF"]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - CC1IF"]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - CC2IF"]
pub type CC2IF_R = crate::BitReader;
#[doc = "Field `CC2IF` writer - CC2IF"]
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - CC3IF"]
pub type CC3IF_R = crate::BitReader;
#[doc = "Field `CC3IF` writer - CC3IF"]
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - CC4IF"]
pub type CC4IF_R = crate::BitReader;
#[doc = "Field `CC4IF` writer - CC4IF"]
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - COMIF"]
pub type COMIF_R = crate::BitReader;
#[doc = "Field `COMIF` writer - COMIF"]
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - TIF"]
pub type TIF_R = crate::BitReader;
#[doc = "Field `TIF` writer - TIF"]
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - BIF"]
pub type BIF_R = crate::BitReader;
#[doc = "Field `BIF` writer - BIF"]
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2IF` reader - B2IF"]
pub type B2IF_R = crate::BitReader;
#[doc = "Field `B2IF` writer - B2IF"]
pub type B2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - CC1OF"]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC1OF` writer - CC1OF"]
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - CC2OF"]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `CC2OF` writer - CC2OF"]
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - CC3OF"]
pub type CC3OF_R = crate::BitReader;
#[doc = "Field `CC3OF` writer - CC3OF"]
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - CC4OF"]
pub type CC4OF_R = crate::BitReader;
#[doc = "Field `CC4OF` writer - CC4OF"]
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIF` reader - SBIF"]
pub type SBIF_R = crate::BitReader;
#[doc = "Field `SBIF` writer - SBIF"]
pub type SBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IF` reader - CC5IF"]
pub type CC5IF_R = crate::BitReader;
#[doc = "Field `CC5IF` writer - CC5IF"]
pub type CC5IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IF` reader - CC6IF"]
pub type CC6IF_R = crate::BitReader;
#[doc = "Field `CC6IF` writer - CC6IF"]
pub type CC6IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1IF"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2IF"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC3IF"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4IF"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMIF"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - B2IF"]
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2OF"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3OF"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4OF"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SBIF"]
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CC5IF"]
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC6IF"]
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<TIM1_SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<TIM1_SRrs> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC2IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<TIM1_SRrs> {
        CC2IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC3IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<TIM1_SRrs> {
        CC3IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CC4IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<TIM1_SRrs> {
        CC4IF_W::new(self, 4)
    }
    #[doc = "Bit 5 - COMIF"]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<TIM1_SRrs> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - TIF"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<TIM1_SRrs> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - BIF"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<TIM1_SRrs> {
        BIF_W::new(self, 7)
    }
    #[doc = "Bit 8 - B2IF"]
    #[inline(always)]
    #[must_use]
    pub fn b2if(&mut self) -> B2IF_W<TIM1_SRrs> {
        B2IF_W::new(self, 8)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<TIM1_SRrs> {
        CC1OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - CC2OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<TIM1_SRrs> {
        CC2OF_W::new(self, 10)
    }
    #[doc = "Bit 11 - CC3OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<TIM1_SRrs> {
        CC3OF_W::new(self, 11)
    }
    #[doc = "Bit 12 - CC4OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<TIM1_SRrs> {
        CC4OF_W::new(self, 12)
    }
    #[doc = "Bit 13 - SBIF"]
    #[inline(always)]
    #[must_use]
    pub fn sbif(&mut self) -> SBIF_W<TIM1_SRrs> {
        SBIF_W::new(self, 13)
    }
    #[doc = "Bit 16 - CC5IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc5if(&mut self) -> CC5IF_W<TIM1_SRrs> {
        CC5IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - CC6IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc6if(&mut self) -> CC6IF_W<TIM1_SRrs> {
        CC6IF_W::new(self, 17)
    }
}
#[doc = "TIM1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_SRrs;
impl crate::RegisterSpec for TIM1_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_sr::R`](R) reader structure"]
impl crate::Readable for TIM1_SRrs {}
#[doc = "`write(|w| ..)` method takes [`tim1_sr::W`](W) writer structure"]
impl crate::Writable for TIM1_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_SR to value 0"]
impl crate::Resettable for TIM1_SRrs {
    const RESET_VALUE: u32 = 0;
}
