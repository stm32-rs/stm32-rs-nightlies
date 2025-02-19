///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
/**Capture/Compare %s output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E {
    ///0: Capture disabled
    Disabled = 0,
    ///1: Capture enabled
    Enabled = 1,
}
impl From<CC1E> for bool {
    #[inline(always)]
    fn from(variant: CC1E) -> Self {
        variant as u8 != 0
    }
}
///Field `CCE(1-1)` reader - Capture/Compare %s output enable
pub type CCE_R = crate::BitReader<CC1E>;
impl CCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1E {
        match self.bits {
            false => CC1E::Disabled,
            true => CC1E::Enabled,
        }
    }
    ///Capture disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1E::Disabled
    }
    ///Capture enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1E::Enabled
    }
}
///Field `CCE(1-1)` writer - Capture/Compare %s output enable
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG, CC1E>;
impl<'a, REG> CCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::Disabled)
    }
    ///Capture enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::Enabled)
    }
}
/**Capture/Compare %s output Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P {
    ///0: Noninverted/rising edge
    RisingEdge = 0,
    ///1: Inverted/falling edge
    FallingEdge = 1,
}
impl From<CC1P> for bool {
    #[inline(always)]
    fn from(variant: CC1P) -> Self {
        variant as u8 != 0
    }
}
///Field `CCP(1-1)` reader - Capture/Compare %s output Polarity
pub type CCP_R = crate::BitReader<CC1P>;
impl CCP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1P {
        match self.bits {
            false => CC1P::RisingEdge,
            true => CC1P::FallingEdge,
        }
    }
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CC1P::RisingEdge
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CC1P::FallingEdge
    }
}
///Field `CCP(1-1)` writer - Capture/Compare %s output Polarity
pub type CCP_W<'a, REG> = crate::BitWriter<'a, REG, CC1P>;
impl<'a, REG> CCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::RisingEdge)
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::FallingEdge)
    }
}
///Field `CCNP(1-1)` reader - Capture/Compare %s output Polarity
pub type CCNP_R = crate::BitReader;
///Field `CCNP(1-1)` writer - Capture/Compare %s output Polarity
pub type CCNP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Capture/Compare (1-1) output enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1E` field.</div>
    #[inline(always)]
    pub fn cce(&self, n: u8) -> CCE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCE_R::new(((self.bits >> (n * 0)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) output enable
    #[inline(always)]
    pub fn cce_iter(&self) -> impl Iterator<Item = CCE_R> + '_ {
        (0..1).map(move |n| CCE_R::new(((self.bits >> (n * 0)) & 1) != 0))
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CCE_R {
        CCE_R::new((self.bits & 1) != 0)
    }
    ///Capture/Compare (1-1) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1P` field.</div>
    #[inline(always)]
    pub fn ccp(&self, n: u8) -> CCP_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) output Polarity
    #[inline(always)]
    pub fn ccp_iter(&self) -> impl Iterator<Item = CCP_R> + '_ {
        (0..1).map(move |n| CCP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Capture/Compare (1-1) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1NP` field.</div>
    #[inline(always)]
    pub fn ccnp(&self, n: u8) -> CCNP_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCNP_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) output Polarity
    #[inline(always)]
    pub fn ccnp_iter(&self) -> impl Iterator<Item = CCNP_R> + '_ {
        (0..1).map(move |n| CCNP_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CCNP_R {
        CCNP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1np", &self.cc1np())
            .finish()
    }
}
impl W {
    ///Capture/Compare (1-1) output enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1E` field.</div>
    #[inline(always)]
    pub fn cce(&mut self, n: u8) -> CCE_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCE_W::new(self, n * 0)
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CCE_W<CCERrs> {
        CCE_W::new(self, 0)
    }
    ///Capture/Compare (1-1) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1P` field.</div>
    #[inline(always)]
    pub fn ccp(&mut self, n: u8) -> CCP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCP_W::new(self, n * 0 + 1)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CCP_W<CCERrs> {
        CCP_W::new(self, 1)
    }
    ///Capture/Compare (1-1) output Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1NP` field.</div>
    #[inline(always)]
    pub fn ccnp(&mut self, n: u8) -> CCNP_W<CCERrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCNP_W::new(self, n * 0 + 3)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&mut self) -> CCNP_W<CCERrs> {
        CCNP_W::new(self, 3)
    }
}
/**TIM14 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#TIM14:CCER)*/
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u16;
}
///`read()` method returns [`ccer::R`](R) reader structure
impl crate::Readable for CCERrs {}
///`write(|w| ..)` method takes [`ccer::W`](W) writer structure
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCERrs {
    const RESET_VALUE: u16 = 0;
}
