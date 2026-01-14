///Register `SCR` writer
pub type W = crate::W<SCRrs>;
/**Clear wakeup flag 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1W {
    ///1: Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0.
    Clear = 1,
}
impl From<CWUF1W> for bool {
    #[inline(always)]
    fn from(variant: CWUF1W) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF1` writer - Clear wakeup flag 1
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1W>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0.
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1W::Clear)
    }
}
/**Clear wakeup flag 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF2W {
    ///1: Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0.
    Clear = 1,
}
impl From<CWUF2W> for bool {
    #[inline(always)]
    fn from(variant: CWUF2W) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF2` writer - Clear wakeup flag 2
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG, CWUF2W>;
impl<'a, REG> CWUF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0.
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF2W::Clear)
    }
}
/**Clear wakeup flag 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF3W {
    ///1: Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0.
    Clear = 1,
}
impl From<CWUF3W> for bool {
    #[inline(always)]
    fn from(variant: CWUF3W) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF3` writer - Clear wakeup flag 3
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG, CWUF3W>;
impl<'a, REG> CWUF3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0.
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF3W::Clear)
    }
}
/**Clear wakeup PVD interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWPVDFW {
    ///1: Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0.
    Clear = 1,
}
impl From<CWPVDFW> for bool {
    #[inline(always)]
    fn from(variant: CWPVDFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWPVDF` writer - Clear wakeup PVD interrupt flag
pub type CWPVDF_W<'a, REG> = crate::BitWriter<'a, REG, CWPVDFW>;
impl<'a, REG> CWPVDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0.
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWPVDFW::Clear)
    }
}
/**Clear wakeup Radio BUSY flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWRFBUSYFW {
    ///1: Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0.
    Clear = 1,
}
impl From<CWRFBUSYFW> for bool {
    #[inline(always)]
    fn from(variant: CWRFBUSYFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWRFBUSYF` writer - Clear wakeup Radio BUSY flag
pub type CWRFBUSYF_W<'a, REG> = crate::BitWriter<'a, REG, CWRFBUSYFW>;
impl<'a, REG> CWRFBUSYF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0.
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWRFBUSYFW::Clear)
    }
}
///Field `CC2HF` writer - lear CPU2 Hold interrupt flag
pub type CC2HF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear wakeup flag 1
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W<'_, SCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - Clear wakeup flag 2
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W<'_, SCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag 3
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W<'_, SCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 8 - Clear wakeup PVD interrupt flag
    #[inline(always)]
    pub fn cwpvdf(&mut self) -> CWPVDF_W<'_, SCRrs> {
        CWPVDF_W::new(self, 8)
    }
    ///Bit 11 - Clear wakeup Radio BUSY flag
    #[inline(always)]
    pub fn cwrfbusyf(&mut self) -> CWRFBUSYF_W<'_, SCRrs> {
        CWRFBUSYF_W::new(self, 11)
    }
    ///Bit 14 - lear CPU2 Hold interrupt flag
    #[inline(always)]
    pub fn cc2hf(&mut self) -> CC2HF_W<'_, SCRrs> {
        CC2HF_W::new(self, 14)
    }
}
/**Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#PWR:SCR)*/
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
