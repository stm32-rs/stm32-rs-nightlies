///Register `EXTI_SWIER2` reader
pub type R = crate::R<EXTI_SWIER2rs>;
///Register `EXTI_SWIER2` writer
pub type W = crate::W<EXTI_SWIER2rs>;
/**Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI34 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI34> for bool {
    #[inline(always)]
    fn from(variant: SWI34) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI34` reader - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI34_R = crate::BitReader<SWI34>;
impl SWI34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI34 {
        match self.bits {
            false => SWI34::B0x0,
            true => SWI34::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI34::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI34::B0x1
    }
}
///Field `SWI34` writer - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI34_W<'a, REG> = crate::BitWriter<'a, REG, SWI34>;
impl<'a, REG> SWI34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI34::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI34::B0x1)
    }
}
impl R {
    ///Bit 2 - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi34(&self) -> SWI34_R {
        SWI34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_SWIER2")
            .field("swi34", &self.swi34())
            .finish()
    }
}
impl W {
    ///Bit 2 - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi34(&mut self) -> SWI34_W<'_, EXTI_SWIER2rs> {
        SWI34_W::new(self, 2)
    }
}
/**EXTI software interrupt event register 2

You can [`read`](crate::Reg::read) this register and get [`exti_swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#EXTI:EXTI_SWIER2)*/
pub struct EXTI_SWIER2rs;
impl crate::RegisterSpec for EXTI_SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_swier2::R`](R) reader structure
impl crate::Readable for EXTI_SWIER2rs {}
///`write(|w| ..)` method takes [`exti_swier2::W`](W) writer structure
impl crate::Writable for EXTI_SWIER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_SWIER2 to value 0
impl crate::Resettable for EXTI_SWIER2rs {}
