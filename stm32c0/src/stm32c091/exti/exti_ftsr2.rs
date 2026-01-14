///Register `EXTI_FTSR2` reader
pub type R = crate::R<EXTI_FTSR2rs>;
///Register `EXTI_FTSR2` writer
pub type W = crate::W<EXTI_FTSR2rs>;
/**Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT34 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<FT34> for bool {
    #[inline(always)]
    fn from(variant: FT34) -> Self {
        variant as u8 != 0
    }
}
///Field `FT34` reader - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
pub type FT34_R = crate::BitReader<FT34>;
impl FT34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FT34 {
        match self.bits {
            false => FT34::B0x0,
            true => FT34::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FT34::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FT34::B0x1
    }
}
///Field `FT34` writer - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
pub type FT34_W<'a, REG> = crate::BitWriter<'a, REG, FT34>;
impl<'a, REG> FT34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT34::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT34::B0x1)
    }
}
impl R {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn ft34(&self) -> FT34_R {
        FT34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_FTSR2")
            .field("ft34", &self.ft34())
            .finish()
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn ft34(&mut self) -> FT34_W<'_, EXTI_FTSR2rs> {
        FT34_W::new(self, 2)
    }
}
/**EXTI falling trigger selection register 2

You can [`read`](crate::Reg::read) this register and get [`exti_ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#EXTI:EXTI_FTSR2)*/
pub struct EXTI_FTSR2rs;
impl crate::RegisterSpec for EXTI_FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_ftsr2::R`](R) reader structure
impl crate::Readable for EXTI_FTSR2rs {}
///`write(|w| ..)` method takes [`exti_ftsr2::W`](W) writer structure
impl crate::Writable for EXTI_FTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_FTSR2 to value 0
impl crate::Resettable for EXTI_FTSR2rs {}
