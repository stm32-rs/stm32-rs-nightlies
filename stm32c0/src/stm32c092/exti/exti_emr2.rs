///Register `EXTI_EMR2` reader
pub type R = crate::R<EXTI_EMR2rs>;
///Register `EXTI_EMR2` writer
pub type W = crate::W<EXTI_EMR2rs>;
/**CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM34 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM34> for bool {
    #[inline(always)]
    fn from(variant: EM34) -> Self {
        variant as u8 != 0
    }
}
///Field `EM34` reader - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
pub type EM34_R = crate::BitReader<EM34>;
impl EM34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM34 {
        match self.bits {
            false => EM34::B0x0,
            true => EM34::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM34::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM34::B0x1
    }
}
///Field `EM34` writer - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
pub type EM34_W<'a, REG> = crate::BitWriter<'a, REG, EM34>;
impl<'a, REG> EM34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM34::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM34::B0x1)
    }
}
impl R {
    ///Bit 2 - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EMR2")
            .field("em34", &self.em34())
            .finish()
    }
}
impl W {
    ///Bit 2 - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
    #[inline(always)]
    pub fn em34(&mut self) -> EM34_W<'_, EXTI_EMR2rs> {
        EM34_W::new(self, 2)
    }
}
/**EXTI CPU wakeup with event mask register 2

You can [`read`](crate::Reg::read) this register and get [`exti_emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#EXTI:EXTI_EMR2)*/
pub struct EXTI_EMR2rs;
impl crate::RegisterSpec for EXTI_EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_emr2::R`](R) reader structure
impl crate::Readable for EXTI_EMR2rs {}
///`write(|w| ..)` method takes [`exti_emr2::W`](W) writer structure
impl crate::Writable for EXTI_EMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_EMR2 to value 0
impl crate::Resettable for EXTI_EMR2rs {}
