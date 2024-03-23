#[doc = "Register `TIMx_SR` reader"]
pub type R = crate::R<TIMX_SRrs>;
#[doc = "Register `TIMx_SR` writer"]
pub type W = crate::W<TIMX_SRrs>;
#[doc = "Field `UIF` reader - UIF"]
pub type UIF_R = crate::BitReader;
#[doc = "Field `UIF` writer - UIF"]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - CC1IF"]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - CC1IF"]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - COMIF"]
pub type COMIF_R = crate::BitReader;
#[doc = "Field `COMIF` writer - COMIF"]
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - BIF"]
pub type BIF_R = crate::BitReader;
#[doc = "Field `BIF` writer - BIF"]
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - CC1OF"]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC1OF` writer - CC1OF"]
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - COMIF"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<TIMX_SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<TIMX_SRrs> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 5 - COMIF"]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<TIMX_SRrs> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 7 - BIF"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<TIMX_SRrs> {
        BIF_W::new(self, 7)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<TIMX_SRrs> {
        CC1OF_W::new(self, 9)
    }
}
#[doc = "TIM16/TIM17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMX_SRrs;
impl crate::RegisterSpec for TIMX_SRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`timx_sr::R`](R) reader structure"]
impl crate::Readable for TIMX_SRrs {}
#[doc = "`write(|w| ..)` method takes [`timx_sr::W`](W) writer structure"]
impl crate::Writable for TIMX_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIMx_SR to value 0"]
impl crate::Resettable for TIMX_SRrs {
    const RESET_VALUE: u16 = 0;
}
