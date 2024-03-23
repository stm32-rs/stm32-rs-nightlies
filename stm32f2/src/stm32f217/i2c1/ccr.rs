#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_R = crate::FieldReader<u16>;
#[doc = "Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Fast mode duty cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUTY {
    #[doc = "0: Duty cycle t_low/t_high = 2/1"]
    Duty2_1 = 0,
    #[doc = "1: Duty cycle t_low/t_high = 16/9"]
    Duty16_9 = 1,
}
impl From<DUTY> for bool {
    #[inline(always)]
    fn from(variant: DUTY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUTY` reader - Fast mode duty cycle"]
pub type DUTY_R = crate::BitReader<DUTY>;
impl DUTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DUTY {
        match self.bits {
            false => DUTY::Duty2_1,
            true => DUTY::Duty16_9,
        }
    }
    #[doc = "Duty cycle t_low/t_high = 2/1"]
    #[inline(always)]
    pub fn is_duty2_1(&self) -> bool {
        *self == DUTY::Duty2_1
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        *self == DUTY::Duty16_9
    }
}
#[doc = "Field `DUTY` writer - Fast mode duty cycle"]
pub type DUTY_W<'a, REG> = crate::BitWriter<'a, REG, DUTY>;
impl<'a, REG> DUTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Duty cycle t_low/t_high = 2/1"]
    #[inline(always)]
    pub fn duty2_1(self) -> &'a mut crate::W<REG> {
        self.variant(DUTY::Duty2_1)
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut crate::W<REG> {
        self.variant(DUTY::Duty16_9)
    }
}
#[doc = "I2C master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F_S {
    #[doc = "0: Standard mode I2C"]
    Standard = 0,
    #[doc = "1: Fast mode I2C"]
    Fast = 1,
}
impl From<F_S> for bool {
    #[inline(always)]
    fn from(variant: F_S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_S` reader - I2C master mode selection"]
pub type F_S_R = crate::BitReader<F_S>;
impl F_S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F_S {
        match self.bits {
            false => F_S::Standard,
            true => F_S::Fast,
        }
    }
    #[doc = "Standard mode I2C"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == F_S::Standard
    }
    #[doc = "Fast mode I2C"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == F_S::Fast
    }
}
#[doc = "Field `F_S` writer - I2C master mode selection"]
pub type F_S_W<'a, REG> = crate::BitWriter<'a, REG, F_S>;
impl<'a, REG> F_S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard mode I2C"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(F_S::Standard)
    }
    #[doc = "Fast mode I2C"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(F_S::Fast)
    }
}
impl R {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<CCRrs> {
        CCR_W::new(self, 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CCRrs> {
        DUTY_W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn f_s(&mut self) -> F_S_W<CCRrs> {
        F_S_W::new(self, 15)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
