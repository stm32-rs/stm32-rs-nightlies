///Register `EXTI_RTSR2` reader
pub type R = crate::R<EXTI_RTSR2rs>;
///Register `EXTI_RTSR2` writer
pub type W = crate::W<EXTI_RTSR2rs>;
/**Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT34 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT34> for bool {
    #[inline(always)]
    fn from(variant: RT34) -> Self {
        variant as u8 != 0
    }
}
///Field `RT34` reader - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT34_R = crate::BitReader<RT34>;
impl RT34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT34 {
        match self.bits {
            false => RT34::B0x0,
            true => RT34::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT34::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT34::B0x1
    }
}
///Field `RT34` writer - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT34_W<'a, REG> = crate::BitWriter<'a, REG, RT34>;
impl<'a, REG> RT34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT34::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT34::B0x1)
    }
}
impl R {
    ///Bit 2 - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt34(&self) -> RT34_R {
        RT34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_RTSR2")
            .field("rt34", &self.rt34())
            .finish()
    }
}
impl W {
    ///Bit 2 - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt34(&mut self) -> RT34_W<'_, EXTI_RTSR2rs> {
        RT34_W::new(self, 2)
    }
}
/**EXTI rising trigger selection register 2

You can [`read`](crate::Reg::read) this register and get [`exti_rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#EXTI:EXTI_RTSR2)*/
pub struct EXTI_RTSR2rs;
impl crate::RegisterSpec for EXTI_RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_rtsr2::R`](R) reader structure
impl crate::Readable for EXTI_RTSR2rs {}
///`write(|w| ..)` method takes [`exti_rtsr2::W`](W) writer structure
impl crate::Writable for EXTI_RTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_RTSR2 to value 0
impl crate::Resettable for EXTI_RTSR2rs {}
