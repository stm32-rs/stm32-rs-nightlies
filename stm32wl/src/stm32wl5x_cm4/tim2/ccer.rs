#[doc = "Register `CCER` reader"]
pub type R = crate::R<CCERrs>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CCERrs>;
#[doc = "Capture/Compare %s output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E {
    #[doc = "0: Capture disabled"]
    Disabled = 0,
    #[doc = "1: Capture enabled"]
    Enabled = 1,
}
impl From<CC1E> for bool {
    #[inline(always)]
    fn from(variant: CC1E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE(1-4)` reader - Capture/Compare %s output enable"]
pub type CCE_R = crate::BitReader<CC1E>;
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1E {
        match self.bits {
            false => CC1E::Disabled,
            true => CC1E::Enabled,
        }
    }
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1E::Disabled
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1E::Enabled
    }
}
#[doc = "Field `CCE(1-4)` writer - Capture/Compare %s output enable"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG, CC1E>;
impl<'a, REG> CCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::Disabled)
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::Enabled)
    }
}
#[doc = "Capture/Compare %s output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P {
    #[doc = "0: Noninverted/rising edge"]
    RisingEdge = 0,
    #[doc = "1: Inverted/falling edge"]
    FallingEdge = 1,
}
impl From<CC1P> for bool {
    #[inline(always)]
    fn from(variant: CC1P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCP(1-4)` reader - Capture/Compare %s output Polarity"]
pub type CCP_R = crate::BitReader<CC1P>;
impl CCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1P {
        match self.bits {
            false => CC1P::RisingEdge,
            true => CC1P::FallingEdge,
        }
    }
    #[doc = "Noninverted/rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CC1P::RisingEdge
    }
    #[doc = "Inverted/falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CC1P::FallingEdge
    }
}
#[doc = "Field `CCP(1-4)` writer - Capture/Compare %s output Polarity"]
pub type CCP_W<'a, REG> = crate::BitWriter<'a, REG, CC1P>;
impl<'a, REG> CCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noninverted/rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::RisingEdge)
    }
    #[doc = "Inverted/falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::FallingEdge)
    }
}
#[doc = "Capture/Compare %s output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NP {
    #[doc = "0: OCxN active high"]
    ActiveHigh = 0,
    #[doc = "1: OCxN active low"]
    ActiveLow = 1,
}
impl From<CC1NP> for bool {
    #[inline(always)]
    fn from(variant: CC1NP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCNP(1-4)` reader - Capture/Compare %s output Polarity"]
pub type CCNP_R = crate::BitReader<CC1NP>;
impl CCNP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1NP {
        match self.bits {
            false => CC1NP::ActiveHigh,
            true => CC1NP::ActiveLow,
        }
    }
    #[doc = "OCxN active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == CC1NP::ActiveHigh
    }
    #[doc = "OCxN active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == CC1NP::ActiveLow
    }
}
#[doc = "Field `CCNP(1-4)` writer - Capture/Compare %s output Polarity"]
pub type CCNP_W<'a, REG> = crate::BitWriter<'a, REG, CC1NP>;
impl<'a, REG> CCNP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCxN active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::ActiveHigh)
    }
    #[doc = "OCxN active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::ActiveLow)
    }
}
impl R {
    #[doc = "Capture/Compare (1-4) output enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1E` field"]
    #[inline(always)]
    pub fn cce(&self, n: u8) -> CCE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCE_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) output enable"]
    #[inline(always)]
    pub fn cce_iter(&self) -> impl Iterator<Item = CCE_R> + '_ {
        (0..4).map(move |n| CCE_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> CCE_R {
        CCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Capture/Compare (1-4) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1P` field"]
    #[inline(always)]
    pub fn ccp(&self, n: u8) -> CCP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCP_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) output Polarity"]
    #[inline(always)]
    pub fn ccp_iter(&self) -> impl Iterator<Item = CCP_R> + '_ {
        (0..4).map(move |n| CCP_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Capture/Compare (1-4) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1NP` field"]
    #[inline(always)]
    pub fn ccnp(&self, n: u8) -> CCNP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCNP_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) output Polarity"]
    #[inline(always)]
    pub fn ccnp_iter(&self) -> impl Iterator<Item = CCNP_R> + '_ {
        (0..4).map(move |n| CCNP_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Capture/Compare (1-4) output enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1E` field"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self, n: u8) -> CCE_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCE_W::new(self, n * 4)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 8)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 12)
    }
    #[doc = "Capture/Compare (1-4) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1P` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccp(&mut self, n: u8) -> CCP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCP_W::new(self, n * 4 + 1)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 1)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 5)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 9)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 13)
    }
    #[doc = "Capture/Compare (1-4) output Polarity"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1NP` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccnp(&mut self, n: u8) -> CCNP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCNP_W::new(self, n * 4 + 3)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 3)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 7)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 11)
    }
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cc4np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 15)
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
