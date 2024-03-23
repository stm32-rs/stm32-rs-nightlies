#[doc = "Register `TIM2_CCER` reader"]
pub type R = crate::R<TIM2_CCERrs>;
#[doc = "Register `TIM2_CCER` writer"]
pub type W = crate::W<TIM2_CCERrs>;
#[doc = "Field `CC1E` reader - CC1E"]
pub type CC1E_R = crate::BitReader;
#[doc = "Field `CC1E` writer - CC1E"]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - CC1P"]
pub type CC1P_R = crate::BitReader;
#[doc = "Field `CC1P` writer - CC1P"]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NE` reader - CC1NE"]
pub type CC1NE_R = crate::BitReader;
#[doc = "Field `CC1NE` writer - CC1NE"]
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NP` reader - CC1NP"]
pub type CC1NP_R = crate::BitReader;
#[doc = "Field `CC1NP` writer - CC1NP"]
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - CC2E"]
pub type CC2E_R = crate::BitReader;
#[doc = "Field `CC2E` writer - CC2E"]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - CC2P"]
pub type CC2P_R = crate::BitReader;
#[doc = "Field `CC2P` writer - CC2P"]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NE` reader - CC2NE"]
pub type CC2NE_R = crate::BitReader;
#[doc = "Field `CC2NE` writer - CC2NE"]
pub type CC2NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - CC2NP"]
pub type CC2NP_R = crate::BitReader;
#[doc = "Field `CC2NP` writer - CC2NP"]
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - CC3E"]
pub type CC3E_R = crate::BitReader;
#[doc = "Field `CC3E` writer - CC3E"]
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - CC3P"]
pub type CC3P_R = crate::BitReader;
#[doc = "Field `CC3P` writer - CC3P"]
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NE` reader - CC3NE"]
pub type CC3NE_R = crate::BitReader;
#[doc = "Field `CC3NE` writer - CC3NE"]
pub type CC3NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - CC3NP"]
pub type CC3NP_R = crate::BitReader;
#[doc = "Field `CC3NP` writer - CC3NP"]
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - CC4E"]
pub type CC4E_R = crate::BitReader;
#[doc = "Field `CC4E` writer - CC4E"]
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - CC4P"]
pub type CC4P_R = crate::BitReader;
#[doc = "Field `CC4P` writer - CC4P"]
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - CC4NP"]
pub type CC4NP_R = crate::BitReader;
#[doc = "Field `CC4NP` writer - CC4NP"]
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5E` reader - CC5E"]
pub type CC5E_R = crate::BitReader;
#[doc = "Field `CC5E` writer - CC5E"]
pub type CC5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5P` reader - CC5P"]
pub type CC5P_R = crate::BitReader;
#[doc = "Field `CC5P` writer - CC5P"]
pub type CC5P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6E` reader - CC6E"]
pub type CC6E_R = crate::BitReader;
#[doc = "Field `CC6E` writer - CC6E"]
pub type CC6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6P` reader - CC6P"]
pub type CC6P_R = crate::BitReader;
#[doc = "Field `CC6P` writer - CC6P"]
pub type CC6P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2NE"]
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CC3E"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC3P"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC3NE"]
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3NP"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4E"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CC4P"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - CC4NP"]
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CC5E"]
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC5P"]
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - CC6E"]
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CC6P"]
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<TIM2_CCERrs> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<TIM2_CCERrs> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<TIM2_CCERrs> {
        CC1NE_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<TIM2_CCERrs> {
        CC1NP_W::new(self, 3)
    }
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<TIM2_CCERrs> {
        CC2E_W::new(self, 4)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<TIM2_CCERrs> {
        CC2P_W::new(self, 5)
    }
    #[doc = "Bit 6 - CC2NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ne(&mut self) -> CC2NE_W<TIM2_CCERrs> {
        CC2NE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<TIM2_CCERrs> {
        CC2NP_W::new(self, 7)
    }
    #[doc = "Bit 8 - CC3E"]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> CC3E_W<TIM2_CCERrs> {
        CC3E_W::new(self, 8)
    }
    #[doc = "Bit 9 - CC3P"]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> CC3P_W<TIM2_CCERrs> {
        CC3P_W::new(self, 9)
    }
    #[doc = "Bit 10 - CC3NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ne(&mut self) -> CC3NE_W<TIM2_CCERrs> {
        CC3NE_W::new(self, 10)
    }
    #[doc = "Bit 11 - CC3NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> CC3NP_W<TIM2_CCERrs> {
        CC3NP_W::new(self, 11)
    }
    #[doc = "Bit 12 - CC4E"]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> CC4E_W<TIM2_CCERrs> {
        CC4E_W::new(self, 12)
    }
    #[doc = "Bit 13 - CC4P"]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> CC4P_W<TIM2_CCERrs> {
        CC4P_W::new(self, 13)
    }
    #[doc = "Bit 15 - CC4NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc4np(&mut self) -> CC4NP_W<TIM2_CCERrs> {
        CC4NP_W::new(self, 15)
    }
    #[doc = "Bit 16 - CC5E"]
    #[inline(always)]
    #[must_use]
    pub fn cc5e(&mut self) -> CC5E_W<TIM2_CCERrs> {
        CC5E_W::new(self, 16)
    }
    #[doc = "Bit 17 - CC5P"]
    #[inline(always)]
    #[must_use]
    pub fn cc5p(&mut self) -> CC5P_W<TIM2_CCERrs> {
        CC5P_W::new(self, 17)
    }
    #[doc = "Bit 20 - CC6E"]
    #[inline(always)]
    #[must_use]
    pub fn cc6e(&mut self) -> CC6E_W<TIM2_CCERrs> {
        CC6E_W::new(self, 20)
    }
    #[doc = "Bit 21 - CC6P"]
    #[inline(always)]
    #[must_use]
    pub fn cc6p(&mut self) -> CC6P_W<TIM2_CCERrs> {
        CC6P_W::new(self, 21)
    }
}
#[doc = "TIM2 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim2_ccer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim2_ccer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM2_CCERrs;
impl crate::RegisterSpec for TIM2_CCERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim2_ccer::R`](R) reader structure"]
impl crate::Readable for TIM2_CCERrs {}
#[doc = "`write(|w| ..)` method takes [`tim2_ccer::W`](W) writer structure"]
impl crate::Writable for TIM2_CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM2_CCER to value 0"]
impl crate::Resettable for TIM2_CCERrs {
    const RESET_VALUE: u32 = 0;
}
