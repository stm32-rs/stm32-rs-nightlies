#[doc = "Register `TIM13_SR` reader"]
pub type R = crate::R<TIM13_SRrs>;
#[doc = "Register `TIM13_SR` writer"]
pub type W = crate::W<TIM13_SRrs>;
#[doc = "Field `UIF` reader - UIF"]
pub type UIF_R = crate::BitReader;
#[doc = "Field `UIF` writer - UIF"]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - CC1IF"]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - CC1IF"]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn uif(&mut self) -> UIF_W<TIM13_SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<TIM13_SRrs> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<TIM13_SRrs> {
        CC1OF_W::new(self, 9)
    }
}
#[doc = "TIM13 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim13_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim13_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM13_SRrs;
impl crate::RegisterSpec for TIM13_SRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim13_sr::R`](R) reader structure"]
impl crate::Readable for TIM13_SRrs {}
#[doc = "`write(|w| ..)` method takes [`tim13_sr::W`](W) writer structure"]
impl crate::Writable for TIM13_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM13_SR to value 0"]
impl crate::Resettable for TIM13_SRrs {
    const RESET_VALUE: u16 = 0;
}
