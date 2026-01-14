///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)
pub type CCR_R = crate::FieldReader<u16>;
///Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
/**Fast mode duty cycle

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUTY {
    ///0: Duty cycle t_low/t_high = 2/1
    Duty2_1 = 0,
    ///1: Duty cycle t_low/t_high = 16/9
    Duty16_9 = 1,
}
impl From<DUTY> for bool {
    #[inline(always)]
    fn from(variant: DUTY) -> Self {
        variant as u8 != 0
    }
}
///Field `DUTY` reader - Fast mode duty cycle
pub type DUTY_R = crate::BitReader<DUTY>;
impl DUTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DUTY {
        match self.bits {
            false => DUTY::Duty2_1,
            true => DUTY::Duty16_9,
        }
    }
    ///Duty cycle t_low/t_high = 2/1
    #[inline(always)]
    pub fn is_duty2_1(&self) -> bool {
        *self == DUTY::Duty2_1
    }
    ///Duty cycle t_low/t_high = 16/9
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        *self == DUTY::Duty16_9
    }
}
///Field `DUTY` writer - Fast mode duty cycle
pub type DUTY_W<'a, REG> = crate::BitWriter<'a, REG, DUTY>;
impl<'a, REG> DUTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Duty cycle t_low/t_high = 2/1
    #[inline(always)]
    pub fn duty2_1(self) -> &'a mut crate::W<REG> {
        self.variant(DUTY::Duty2_1)
    }
    ///Duty cycle t_low/t_high = 16/9
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut crate::W<REG> {
        self.variant(DUTY::Duty16_9)
    }
}
/**I2C master mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F_S {
    ///0: Standard mode I2C
    Standard = 0,
    ///1: Fast mode I2C
    Fast = 1,
}
impl From<F_S> for bool {
    #[inline(always)]
    fn from(variant: F_S) -> Self {
        variant as u8 != 0
    }
}
///Field `F_S` reader - I2C master mode selection
pub type F_S_R = crate::BitReader<F_S>;
impl F_S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> F_S {
        match self.bits {
            false => F_S::Standard,
            true => F_S::Fast,
        }
    }
    ///Standard mode I2C
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == F_S::Standard
    }
    ///Fast mode I2C
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == F_S::Fast
    }
}
///Field `F_S` writer - I2C master mode selection
pub type F_S_W<'a, REG> = crate::BitWriter<'a, REG, F_S>;
impl<'a, REG> F_S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Standard mode I2C
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(F_S::Standard)
    }
    ///Fast mode I2C
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(F_S::Fast)
    }
}
impl R {
    ///Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new(self.bits & 0x0fff)
    }
    ///Bit 14 - Fast mode duty cycle
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - I2C master mode selection
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("f_s", &self.f_s())
            .field("duty", &self.duty())
            .field("ccr", &self.ccr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W<'_, CCRrs> {
        CCR_W::new(self, 0)
    }
    ///Bit 14 - Fast mode duty cycle
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W<'_, CCRrs> {
        DUTY_W::new(self, 14)
    }
    ///Bit 15 - I2C master mode selection
    #[inline(always)]
    pub fn f_s(&mut self) -> F_S_W<'_, CCRrs> {
        F_S_W::new(self, 15)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F401.html#I2C1:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u16;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
