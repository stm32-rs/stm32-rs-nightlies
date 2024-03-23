#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Clear wakeup flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1W {
    #[doc = "1: Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWUF1W> for bool {
    #[inline(always)]
    fn from(variant: CWUF1W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1W>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1W::Clear)
    }
}
#[doc = "Clear wakeup flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF2W {
    #[doc = "1: Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWUF2W> for bool {
    #[inline(always)]
    fn from(variant: CWUF2W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG, CWUF2W>;
impl<'a, REG> CWUF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF2W::Clear)
    }
}
#[doc = "Clear wakeup flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF3W {
    #[doc = "1: Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWUF3W> for bool {
    #[inline(always)]
    fn from(variant: CWUF3W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG, CWUF3W>;
impl<'a, REG> CWUF3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF3W::Clear)
    }
}
#[doc = "Clear wakeup PVD interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWPVDFW {
    #[doc = "1: Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWPVDFW> for bool {
    #[inline(always)]
    fn from(variant: CWPVDFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWPVDF` writer - Clear wakeup PVD interrupt flag"]
pub type CWPVDF_W<'a, REG> = crate::BitWriter<'a, REG, CWPVDFW>;
impl<'a, REG> CWPVDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWPVDFW::Clear)
    }
}
#[doc = "Clear wakeup Radio BUSY flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWRFBUSYFW {
    #[doc = "1: Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0."]
    Clear = 1,
}
impl From<CWRFBUSYFW> for bool {
    #[inline(always)]
    fn from(variant: CWRFBUSYFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWRFBUSYF` writer - Clear wakeup Radio BUSY flag"]
pub type CWRFBUSYF_W<'a, REG> = crate::BitWriter<'a, REG, CWRFBUSYFW>;
impl<'a, REG> CWRFBUSYF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWRFBUSYFW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<SCRrs> {
        CWUF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<SCRrs> {
        CWUF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<SCRrs> {
        CWUF3_W::new(self, 2)
    }
    #[doc = "Bit 8 - Clear wakeup PVD interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwpvdf(&mut self) -> CWPVDF_W<SCRrs> {
        CWPVDF_W::new(self, 8)
    }
    #[doc = "Bit 11 - Clear wakeup Radio BUSY flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwrfbusyf(&mut self) -> CWRFBUSYF_W<SCRrs> {
        CWRFBUSYF_W::new(self, 11)
    }
}
#[doc = "Power status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
