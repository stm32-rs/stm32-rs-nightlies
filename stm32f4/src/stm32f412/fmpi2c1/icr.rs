#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "ADDRCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRCF {
    #[doc = "1: Clears the ADDR flag in ISR register"]
    Clear = 1,
}
impl From<ADDRCF> for bool {
    #[inline(always)]
    fn from(variant: ADDRCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRCF` writer - ADDRCF"]
pub type ADDRCF_W<'a, REG> = crate::BitWriter<'a, REG, ADDRCF>;
impl<'a, REG> ADDRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ADDR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRCF::Clear)
    }
}
#[doc = "NACKCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKCF {
    #[doc = "1: Clears the NACK flag in ISR register"]
    Clear = 1,
}
impl From<NACKCF> for bool {
    #[inline(always)]
    fn from(variant: NACKCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKCF` writer - NACKCF"]
pub type NACKCF_W<'a, REG> = crate::BitWriter<'a, REG, NACKCF>;
impl<'a, REG> NACKCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the NACK flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NACKCF::Clear)
    }
}
#[doc = "STOPCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPCF {
    #[doc = "1: Clears the STOP flag in ISR register"]
    Clear = 1,
}
impl From<STOPCF> for bool {
    #[inline(always)]
    fn from(variant: STOPCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPCF` writer - STOPCF"]
pub type STOPCF_W<'a, REG> = crate::BitWriter<'a, REG, STOPCF>;
impl<'a, REG> STOPCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the STOP flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STOPCF::Clear)
    }
}
#[doc = "BERRCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRCF {
    #[doc = "1: Clears the BERR flag in ISR register"]
    Clear = 1,
}
impl From<BERRCF> for bool {
    #[inline(always)]
    fn from(variant: BERRCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERRCF` writer - BERRCF"]
pub type BERRCF_W<'a, REG> = crate::BitWriter<'a, REG, BERRCF>;
impl<'a, REG> BERRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the BERR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BERRCF::Clear)
    }
}
#[doc = "ARLOCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOCF {
    #[doc = "1: Clears the ARLO flag in ISR register"]
    Clear = 1,
}
impl From<ARLOCF> for bool {
    #[inline(always)]
    fn from(variant: ARLOCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLOCF` writer - ARLOCF"]
pub type ARLOCF_W<'a, REG> = crate::BitWriter<'a, REG, ARLOCF>;
impl<'a, REG> ARLOCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ARLO flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ARLOCF::Clear)
    }
}
#[doc = "OVRCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCF {
    #[doc = "1: Clears the OVR flag in ISR register"]
    Clear = 1,
}
impl From<OVRCF> for bool {
    #[inline(always)]
    fn from(variant: OVRCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRCF` writer - OVRCF"]
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG, OVRCF>;
impl<'a, REG> OVRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the OVR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCF::Clear)
    }
}
#[doc = "PECCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECCF {
    #[doc = "1: Clears the PEC flag in ISR register"]
    Clear = 1,
}
impl From<PECCF> for bool {
    #[inline(always)]
    fn from(variant: PECCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECCF` writer - PECCF"]
pub type PECCF_W<'a, REG> = crate::BitWriter<'a, REG, PECCF>;
impl<'a, REG> PECCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the PEC flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PECCF::Clear)
    }
}
#[doc = "TIMOUTCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTCF {
    #[doc = "1: Clears the TIMOUT flag in ISR register"]
    Clear = 1,
}
impl From<TIMOUTCF> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUTCF` writer - TIMOUTCF"]
pub type TIMOUTCF_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUTCF>;
impl<'a, REG> TIMOUTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the TIMOUT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTCF::Clear)
    }
}
#[doc = "ALERTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTCF {
    #[doc = "1: Clears the ALERT flag in ISR register"]
    Clear = 1,
}
impl From<ALERTCF> for bool {
    #[inline(always)]
    fn from(variant: ALERTCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTCF` writer - ALERTC"]
pub type ALERTCF_W<'a, REG> = crate::BitWriter<'a, REG, ALERTCF>;
impl<'a, REG> ALERTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ALERT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTCF::Clear)
    }
}
impl W {
    #[doc = "Bit 3 - ADDRCF"]
    #[inline(always)]
    #[must_use]
    pub fn addrcf(&mut self) -> ADDRCF_W<ICRrs> {
        ADDRCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - NACKCF"]
    #[inline(always)]
    #[must_use]
    pub fn nackcf(&mut self) -> NACKCF_W<ICRrs> {
        NACKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - STOPCF"]
    #[inline(always)]
    #[must_use]
    pub fn stopcf(&mut self) -> STOPCF_W<ICRrs> {
        STOPCF_W::new(self, 5)
    }
    #[doc = "Bit 8 - BERRCF"]
    #[inline(always)]
    #[must_use]
    pub fn berrcf(&mut self) -> BERRCF_W<ICRrs> {
        BERRCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - ARLOCF"]
    #[inline(always)]
    #[must_use]
    pub fn arlocf(&mut self) -> ARLOCF_W<ICRrs> {
        ARLOCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - OVRCF"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<ICRrs> {
        OVRCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - PECCF"]
    #[inline(always)]
    #[must_use]
    pub fn peccf(&mut self) -> PECCF_W<ICRrs> {
        PECCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - TIMOUTCF"]
    #[inline(always)]
    #[must_use]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<ICRrs> {
        TIMOUTCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - ALERTC"]
    #[inline(always)]
    #[must_use]
    pub fn alertcf(&mut self) -> ALERTCF_W<ICRrs> {
        ALERTCF_W::new(self, 13)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
