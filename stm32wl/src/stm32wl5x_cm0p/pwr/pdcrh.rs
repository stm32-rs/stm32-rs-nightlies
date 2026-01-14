///Register `PDCRH` reader
pub type R = crate::R<PDCRHrs>;
///Register `PDCRH` writer
pub type W = crate::W<PDCRHrs>;
/**pull-down

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD3 {
    ///0: Disable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    Enabled = 1,
}
impl From<PD3> for bool {
    #[inline(always)]
    fn from(variant: PD3) -> Self {
        variant as u8 != 0
    }
}
///Field `PD3` reader - pull-down
pub type PD3_R = crate::BitReader<PD3>;
impl PD3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PD3 {
        match self.bits {
            false => PD3::Disabled,
            true => PD3::Enabled,
        }
    }
    ///Disable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD3::Disabled
    }
    ///Enable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD3::Enabled
    }
}
///Field `PD3` writer - pull-down
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG, PD3>;
impl<'a, REG> PD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD3::Disabled)
    }
    ///Enable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PD3::Enabled)
    }
}
impl R {
    ///Bit 3 - pull-down
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRH").field("pd3", &self.pd3()).finish()
    }
}
impl W {
    ///Bit 3 - pull-down
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<'_, PDCRHrs> {
        PD3_W::new(self, 3)
    }
}
/**Power Port H pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#PWR:PDCRH)*/
pub struct PDCRHrs;
impl crate::RegisterSpec for PDCRHrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrh::R`](R) reader structure
impl crate::Readable for PDCRHrs {}
///`write(|w| ..)` method takes [`pdcrh::W`](W) writer structure
impl crate::Writable for PDCRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRH to value 0
impl crate::Resettable for PDCRHrs {}
