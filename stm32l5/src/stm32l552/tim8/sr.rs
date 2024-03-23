#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UIF_R = crate::BitReader;
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag"]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag"]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag"]
pub type CC2IF_R = crate::BitReader;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag"]
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag"]
pub type CC3IF_R = crate::BitReader;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag"]
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag"]
pub type CC4IF_R = crate::BitReader;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag"]
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type COMIF_R = crate::BitReader;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - Trigger interrupt flag"]
pub type TIF_R = crate::BitReader;
#[doc = "Field `TIF` writer - Trigger interrupt flag"]
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BIF_R = crate::BitReader;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag"]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag"]
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag"]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag"]
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag"]
pub type CC3OF_R = crate::BitReader;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag"]
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag"]
pub type CC4OF_R = crate::BitReader;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag"]
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIF` reader - System Break interrupt flag"]
pub type SBIF_R = crate::BitReader;
#[doc = "Field `SBIF` writer - System Break interrupt flag"]
pub type SBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IF` reader - Compare 5 interrupt flag"]
pub type CC5IF_R = crate::BitReader;
#[doc = "Field `CC5IF` writer - Compare 5 interrupt flag"]
pub type CC5IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IF` reader - Compare 6 interrupt flag"]
pub type CC6IF_R = crate::BitReader;
#[doc = "Field `CC6IF` writer - Compare 6 interrupt flag"]
pub type CC6IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - System Break interrupt flag"]
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag"]
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<SRrs> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<SRrs> {
        CC2IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<SRrs> {
        CC3IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<SRrs> {
        CC4IF_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<SRrs> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<SRrs> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<SRrs> {
        BIF_W::new(self, 7)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<SRrs> {
        CC1OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<SRrs> {
        CC2OF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<SRrs> {
        CC3OF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<SRrs> {
        CC4OF_W::new(self, 12)
    }
    #[doc = "Bit 13 - System Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn sbif(&mut self) -> SBIF_W<SRrs> {
        SBIF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc5if(&mut self) -> CC5IF_W<SRrs> {
        CC5IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc6if(&mut self) -> CC6IF_W<SRrs> {
        CC6IF_W::new(self, 17)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
