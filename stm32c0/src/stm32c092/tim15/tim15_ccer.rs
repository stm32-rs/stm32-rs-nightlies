///Register `TIM15_CCER` reader
pub type R = crate::R<TIM15_CCERrs>;
///Register `TIM15_CCER` writer
pub type W = crate::W<TIM15_CCERrs>;
/**Capture/Compare 1 output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E {
    ///0: Capture mode disabled / OC1 is not active (see below)
    B0x0 = 0,
    ///1: Capture mode enabled / OC1 signal is output on the corresponding output pin
    B0x1 = 1,
}
impl From<CC1E> for bool {
    #[inline(always)]
    fn from(variant: CC1E) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1E` reader - Capture/Compare 1 output enable
pub type CC1E_R = crate::BitReader<CC1E>;
impl CC1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1E {
        match self.bits {
            false => CC1E::B0x0,
            true => CC1E::B0x1,
        }
    }
    ///Capture mode disabled / OC1 is not active (see below)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1E::B0x0
    }
    ///Capture mode enabled / OC1 signal is output on the corresponding output pin
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1E::B0x1
    }
}
///Field `CC1E` writer - Capture/Compare 1 output enable
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG, CC1E>;
impl<'a, REG> CC1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture mode disabled / OC1 is not active (see below)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::B0x0)
    }
    ///Capture mode enabled / OC1 signal is output on the corresponding output pin
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::B0x1)
    }
}
/**Capture/Compare 1 output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P {
    ///0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    B0x0 = 0,
    ///1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    B0x1 = 1,
}
impl From<CC1P> for bool {
    #[inline(always)]
    fn from(variant: CC1P) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1P` reader - Capture/Compare 1 output polarity
pub type CC1P_R = crate::BitReader<CC1P>;
impl CC1P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1P {
        match self.bits {
            false => CC1P::B0x0,
            true => CC1P::B0x1,
        }
    }
    ///OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1P::B0x0
    }
    ///OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1P::B0x1
    }
}
///Field `CC1P` writer - Capture/Compare 1 output polarity
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG, CC1P>;
impl<'a, REG> CC1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::B0x0)
    }
    ///OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::B0x1)
    }
}
/**Capture/Compare 1 complementary output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NE {
    ///0: Off - OC1N is not active.
    B0x0 = 0,
    ///1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    B0x1 = 1,
}
impl From<CC1NE> for bool {
    #[inline(always)]
    fn from(variant: CC1NE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NE` reader - Capture/Compare 1 complementary output enable
pub type CC1NE_R = crate::BitReader<CC1NE>;
impl CC1NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1NE {
        match self.bits {
            false => CC1NE::B0x0,
            true => CC1NE::B0x1,
        }
    }
    ///Off - OC1N is not active.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1NE::B0x0
    }
    ///On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1NE::B0x1
    }
}
///Field `CC1NE` writer - Capture/Compare 1 complementary output enable
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG, CC1NE>;
impl<'a, REG> CC1NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Off - OC1N is not active.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::B0x0)
    }
    ///On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::B0x1)
    }
}
/**Capture/Compare 1 complementary output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NP {
    ///0: OC1N active high
    B0x0 = 0,
    ///1: OC1N active low
    B0x1 = 1,
}
impl From<CC1NP> for bool {
    #[inline(always)]
    fn from(variant: CC1NP) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NP` reader - Capture/Compare 1 complementary output polarity
pub type CC1NP_R = crate::BitReader<CC1NP>;
impl CC1NP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1NP {
        match self.bits {
            false => CC1NP::B0x0,
            true => CC1NP::B0x1,
        }
    }
    ///OC1N active high
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1NP::B0x0
    }
    ///OC1N active low
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1NP::B0x1
    }
}
///Field `CC1NP` writer - Capture/Compare 1 complementary output polarity
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG, CC1NP>;
impl<'a, REG> CC1NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC1N active high
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::B0x0)
    }
    ///OC1N active low
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::B0x1)
    }
}
///Field `CC2E` reader - Capture/Compare 2 output enable
pub type CC2E_R = crate::BitReader;
///Field `CC2E` writer - Capture/Compare 2 output enable
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - Capture/Compare 2 output polarity
pub type CC2P_R = crate::BitReader;
///Field `CC2P` writer - Capture/Compare 2 output polarity
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NP` reader - Capture/Compare 2 complementary output polarity
pub type CC2NP_R = crate::BitReader;
///Field `CC2NP` writer - Capture/Compare 2 complementary output polarity
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Capture/Compare 2 complementary output polarity
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .field("cc2e", &self.cc2e())
            .field("cc2p", &self.cc2p())
            .field("cc2np", &self.cc2np())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<'_, TIM15_CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 output polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<'_, TIM15_CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<'_, TIM15_CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<'_, TIM15_CCERrs> {
        CC1NP_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<'_, TIM15_CCERrs> {
        CC2E_W::new(self, 4)
    }
    ///Bit 5 - Capture/Compare 2 output polarity
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<'_, TIM15_CCERrs> {
        CC2P_W::new(self, 5)
    }
    ///Bit 7 - Capture/Compare 2 complementary output polarity
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<'_, TIM15_CCERrs> {
        CC2NP_W::new(self, 7)
    }
}
/**TIM15 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim15_ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_CCER)*/
pub struct TIM15_CCERrs;
impl crate::RegisterSpec for TIM15_CCERrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_ccer::R`](R) reader structure
impl crate::Readable for TIM15_CCERrs {}
///`write(|w| ..)` method takes [`tim15_ccer::W`](W) writer structure
impl crate::Writable for TIM15_CCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_CCER to value 0
impl crate::Resettable for TIM15_CCERrs {}
