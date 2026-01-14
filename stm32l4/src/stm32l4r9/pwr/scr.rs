///Register `SCR` writer
pub type W = crate::W<SCRrs>;
/**Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1 {
    ///1: Setting this bit clears the WUFx flag in the PWR_SR1 register
    Clear = 1,
}
impl From<CWUF1> for bool {
    #[inline(always)]
    fn from(variant: CWUF1) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF1` writer - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register.
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the WUFx flag in the PWR_SR1 register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1::Clear)
    }
}
///Field `CWUF2` writer - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register.
pub use CWUF1_W as CWUF2_W;
///Field `CWUF3` writer - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register.
pub use CWUF1_W as CWUF3_W;
///Field `CWUF4` writer - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register.
pub use CWUF1_W as CWUF4_W;
///Field `CWUF5` writer - Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register.
pub use CWUF1_W as CWUF5_W;
/**Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBF {
    ///1: Setting this bit clears the SBF flag in the PWR_SR1 register
    Clear = 1,
}
impl From<CSBF> for bool {
    #[inline(always)]
    fn from(variant: CSBF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` writer - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register.
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG, CSBF>;
impl<'a, REG> CSBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the SBF flag in the PWR_SR1 register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSBF::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W<'_, SCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W<'_, SCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W<'_, SCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 3 - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf4(&mut self) -> CWUF4_W<'_, SCRrs> {
        CWUF4_W::new(self, 3)
    }
    ///Bit 4 - Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf5(&mut self) -> CWUF5_W<'_, SCRrs> {
        CWUF5_W::new(self, 4)
    }
    ///Bit 8 - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, SCRrs> {
        CSBF_W::new(self, 8)
    }
}
/**Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#PWR:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
