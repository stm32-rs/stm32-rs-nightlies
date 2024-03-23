#[doc = "Register `CCER` reader"]
pub type R = crate::R<CCERrs>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CCERrs>;
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable"]
pub type CC1E_R = crate::BitReader;
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable"]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - Capture/Compare 1 output Polarity"]
pub type CC1P_R = crate::BitReader;
#[doc = "Field `CC1P` writer - Capture/Compare 1 output Polarity"]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NE` reader - Capture/Compare 1 complementary output enable"]
pub type CC1NE_R = crate::BitReader;
#[doc = "Field `CC1NE` writer - Capture/Compare 1 complementary output enable"]
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NP` reader - Capture/Compare 1 output Polarity"]
pub type CC1NP_R = crate::BitReader;
#[doc = "Field `CC1NP` writer - Capture/Compare 1 output Polarity"]
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable"]
pub type CC2E_R = crate::BitReader;
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable"]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Capture/Compare 2 output polarity"]
pub type CC2P_R = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output polarity"]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 complementary output polarity"]
pub type CC2NP_R = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 complementary output polarity"]
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<CCERrs> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<CCERrs> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<CCERrs> {
        CC1NE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<CCERrs> {
        CC1NP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<CCERrs> {
        CC2E_W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<CCERrs> {
        CC2P_W::new(self, 5)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<CCERrs> {
        CC2NP_W::new(self, 7)
    }
}
#[doc = "capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CCERrs {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CCERrs {
    const RESET_VALUE: u32 = 0;
}
