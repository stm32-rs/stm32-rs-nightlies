#[doc = "Register `CCER` reader"]
pub type R = crate::R<CCERrs>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CCERrs>;
#[doc = "Field `CCE(1-1)` reader - Capture/Compare %s output enable"]
pub type CCE_R = crate::BitReader;
#[doc = "Field `CCE(1-1)` writer - Capture/Compare %s output enable"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCP(1-1)` reader - Capture/Compare %s output Polarity"]
pub type CCP_R = crate::BitReader;
#[doc = "Field `CCP(1-1)` writer - Capture/Compare %s output Polarity"]
pub type CCP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCNP(1-1)` reader - Capture/Compare %s output Polarity"]
pub type CCNP_R = crate::BitReader;
#[doc = "Field `CCNP(1-1)` writer - Capture/Compare %s output Polarity"]
pub type CCNP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Capture/Compare (1-1) output enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1E` field"]
    #[inline(always)]
    pub fn cce(&self, n: u8) -> CCE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCE_R::new(((self.bits >> (n * 0)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) output enable"]
    #[inline(always)]
    pub fn cce_iter(&self) -> impl Iterator<Item = CCE_R> + '_ {
        (0..1).map(move |n| CCE_R::new(((self.bits >> (n * 0)) & 1) != 0))
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> CCE_R {
        CCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Capture/Compare (1-1) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1P` field"]
    #[inline(always)]
    pub fn ccp(&self, n: u8) -> CCP_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) output Polarity"]
    #[inline(always)]
    pub fn ccp_iter(&self) -> impl Iterator<Item = CCP_R> + '_ {
        (0..1).map(move |n| CCP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Capture/Compare (1-1) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1NP` field"]
    #[inline(always)]
    pub fn ccnp(&self, n: u8) -> CCNP_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCNP_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) output Polarity"]
    #[inline(always)]
    pub fn ccnp_iter(&self) -> impl Iterator<Item = CCNP_R> + '_ {
        (0..1).map(move |n| CCNP_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Capture/Compare (1-1) output enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1E` field"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self, n: u8) -> CCE_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCE_W::new(self, n * 0)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 0)
    }
    #[doc = "Capture/Compare (1-1) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1P` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccp(&mut self, n: u8) -> CCP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCP_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 1)
    }
    #[doc = "Capture/Compare (1-1) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1NP` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccnp(&mut self, n: u8) -> CCNP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCNP_W::new(self, n * 0 + 3)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 3)
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
