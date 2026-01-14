///Register `TIM14_DIER` reader
pub type R = crate::R<TIM14_DIERrs>;
///Register `TIM14_DIER` writer
pub type W = crate::W<TIM14_DIERrs>;
/**Update interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE {
    ///0: Update interrupt disabled
    B0x0 = 0,
    ///1: Update interrupt enabled
    B0x1 = 1,
}
impl From<UIE> for bool {
    #[inline(always)]
    fn from(variant: UIE) -> Self {
        variant as u8 != 0
    }
}
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader<UIE>;
impl UIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIE {
        match self.bits {
            false => UIE::B0x0,
            true => UIE::B0x1,
        }
    }
    ///Update interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UIE::B0x0
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UIE::B0x1
    }
}
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::B0x0)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::B0x1)
    }
}
/**Capture/Compare 1 interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IE {
    ///0: CC1 interrupt disabled
    B0x0 = 0,
    ///1: CC1 interrupt enabled
    B0x1 = 1,
}
impl From<CC1IE> for bool {
    #[inline(always)]
    fn from(variant: CC1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable
pub type CC1IE_R = crate::BitReader<CC1IE>;
impl CC1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1IE {
        match self.bits {
            false => CC1IE::B0x0,
            true => CC1IE::B0x1,
        }
    }
    ///CC1 interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1IE::B0x0
    }
    ///CC1 interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1IE::B0x1
    }
}
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE>;
impl<'a, REG> CC1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC1 interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::B0x0)
    }
    ///CC1 interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::B0x1)
    }
}
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM14_DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, TIM14_DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, TIM14_DIERrs> {
        CC1IE_W::new(self, 1)
    }
}
/**TIM14 Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim14_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM14:TIM14_DIER)*/
pub struct TIM14_DIERrs;
impl crate::RegisterSpec for TIM14_DIERrs {
    type Ux = u16;
}
///`read()` method returns [`tim14_dier::R`](R) reader structure
impl crate::Readable for TIM14_DIERrs {}
///`write(|w| ..)` method takes [`tim14_dier::W`](W) writer structure
impl crate::Writable for TIM14_DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM14_DIER to value 0
impl crate::Resettable for TIM14_DIERrs {}
