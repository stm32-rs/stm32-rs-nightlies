///Register `PUCRH` reader
pub type R = crate::R<PUCRHrs>;
///Register `PUCRH` writer
pub type W = crate::W<PUCRHrs>;
/**pull-up

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU3 {
    ///0: Disable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\[y\] bit is also set
    Enabled = 1,
}
impl From<PU3> for bool {
    #[inline(always)]
    fn from(variant: PU3) -> Self {
        variant as u8 != 0
    }
}
///Field `PU3` reader - pull-up
pub type PU3_R = crate::BitReader<PU3>;
impl PU3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PU3 {
        match self.bits {
            false => PU3::Disabled,
            true => PU3::Enabled,
        }
    }
    ///Disable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU3::Disabled
    }
    ///Enable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\[y\] bit is also set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU3::Enabled
    }
}
///Field `PU3` writer - pull-up
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG, PU3>;
impl<'a, REG> PU3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU3::Disabled)
    }
    ///Enable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\[y\] bit is also set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU3::Enabled)
    }
}
impl R {
    ///Bit 3 - pull-up
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRH").field("pu3", &self.pu3()).finish()
    }
}
impl W {
    ///Bit 3 - pull-up
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<'_, PUCRHrs> {
        PU3_W::new(self, 3)
    }
}
/**Power Port H pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#PWR:PUCRH)*/
pub struct PUCRHrs;
impl crate::RegisterSpec for PUCRHrs {
    type Ux = u32;
}
///`read()` method returns [`pucrh::R`](R) reader structure
impl crate::Readable for PUCRHrs {}
///`write(|w| ..)` method takes [`pucrh::W`](W) writer structure
impl crate::Writable for PUCRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRH to value 0
impl crate::Resettable for PUCRHrs {}
