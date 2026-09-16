///Register `EXTI_IMR2` reader
pub type R = crate::R<EXTI_IMR2rs>;
///Register `EXTI_IMR2` writer
pub type W = crate::W<EXTI_IMR2rs>;
/**CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM34 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM34> for bool {
    #[inline(always)]
    fn from(variant: IM34) -> Self {
        variant as u8 != 0
    }
}
///Field `IM34` reader - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
pub type IM34_R = crate::BitReader<IM34>;
impl IM34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM34 {
        match self.bits {
            false => IM34::B0x0,
            true => IM34::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM34::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM34::B0x1
    }
}
///Field `IM34` writer - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
pub type IM34_W<'a, REG> = crate::BitWriter<'a, REG, IM34>;
impl<'a, REG> IM34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM34::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM34::B0x1)
    }
}
impl R {
    ///Bit 2 - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_IMR2")
            .field("im34", &self.im34())
            .finish()
    }
}
impl W {
    ///Bit 2 - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<'_, EXTI_IMR2rs> {
        IM34_W::new(self, 2)
    }
}
/**EXTI CPU wakeup with interrupt mask register 2

You can [`read`](crate::Reg::read) this register and get [`exti_imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#EXTI:EXTI_IMR2)*/
pub struct EXTI_IMR2rs;
impl crate::RegisterSpec for EXTI_IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_imr2::R`](R) reader structure
impl crate::Readable for EXTI_IMR2rs {}
///`write(|w| ..)` method takes [`exti_imr2::W`](W) writer structure
impl crate::Writable for EXTI_IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_IMR2 to value 0
impl crate::Resettable for EXTI_IMR2rs {}
