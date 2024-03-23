#[doc = "Register `TIM12_SR` reader"]
pub type R = crate::R<TIM12_SRrs>;
#[doc = "Register `TIM12_SR` writer"]
pub type W = crate::W<TIM12_SRrs>;
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
#[doc = "Field `TIF` reader - TIF"]
pub type TIF_R = crate::BitReader;
#[doc = "Field `TIF` writer - TIF"]
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - CC1OF"]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC1OF` writer - CC1OF"]
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - CC2OF"]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `CC2OF` writer - CC2OF"]
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<TIM12_SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<TIM12_SRrs> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC2IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<TIM12_SRrs> {
        CC2IF_W::new(self, 2)
    }
    #[doc = "Bit 6 - TIF"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<TIM12_SRrs> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<TIM12_SRrs> {
        CC1OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - CC2OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<TIM12_SRrs> {
        CC2OF_W::new(self, 10)
    }
}
#[doc = "TIM12 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim12_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim12_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM12_SRrs;
impl crate::RegisterSpec for TIM12_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim12_sr::R`](R) reader structure"]
impl crate::Readable for TIM12_SRrs {}
#[doc = "`write(|w| ..)` method takes [`tim12_sr::W`](W) writer structure"]
impl crate::Writable for TIM12_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM12_SR to value 0"]
impl crate::Resettable for TIM12_SRrs {
    const RESET_VALUE: u32 = 0;
}
