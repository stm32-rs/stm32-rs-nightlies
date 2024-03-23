#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Cortex-M4 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLL {
    #[doc = "0: Cortex®-M4 LOCKUP output disconnected from TIM1/8/15/16/17 Break inputs"]
    Disconnected = 0,
    #[doc = "1: Cortex®-M4 LOCKUP output connected to TIM1/8/15/16/17 Break inputs"]
    Connected = 1,
}
impl From<CLL> for bool {
    #[inline(always)]
    fn from(variant: CLL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` writer - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLL>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cortex®-M4 LOCKUP output disconnected from TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Disconnected)
    }
    #[doc = "Cortex®-M4 LOCKUP output connected to TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Connected)
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPL {
    #[doc = "0: SRAM2 parity error signal disconnected from TIM1/8/15/16/17 Break inputs"]
    Disconnected = 0,
    #[doc = "1: SRAM2 parity error signal connected to TIM1/8/15/16/17 Break inputs"]
    Connected = 1,
}
impl From<SPL> for bool {
    #[inline(always)]
    fn from(variant: SPL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG, SPL>;
impl<'a, REG> SPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 parity error signal disconnected from TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(SPL::Disconnected)
    }
    #[doc = "SRAM2 parity error signal connected to TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(SPL::Connected)
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDL {
    #[doc = "0: PVD interrupt disconnected from TIM1/8/15/16/17 Break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    Disconnected = 0,
    #[doc = "1: PVD interrupt connected to TIM1/8/15/16/17 Break input, PVDE and PLS\\[2:0\\]
bits are read only"]
    Connected = 1,
}
impl From<PVDL> for bool {
    #[inline(always)]
    fn from(variant: PVDL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG, PVDL>;
impl<'a, REG> PVDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVD interrupt disconnected from TIM1/8/15/16/17 Break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL::Disconnected)
    }
    #[doc = "PVD interrupt connected to TIM1/8/15/16/17 Break input, PVDE and PLS\\[2:0\\]
bits are read only"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL::Connected)
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCL {
    #[doc = "0: ECC error disconnected from TIM1/8/15/16/17 Break input"]
    Disconnected = 0,
    #[doc = "1: ECC error connected to TIM1/8/15/16/17 Break input"]
    Connected = 1,
}
impl From<ECCL> for bool {
    #[inline(always)]
    fn from(variant: ECCL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG, ECCL>;
impl<'a, REG> ECCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC error disconnected from TIM1/8/15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(ECCL::Disconnected)
    }
    #[doc = "ECC error connected to TIM1/8/15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(ECCL::Connected)
    }
}
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPF {
    #[doc = "0: No SRAM2 parity error detected"]
    Cleared = 0,
    #[doc = "1: SRAM2 parity error detected"]
    Set = 1,
}
impl From<SPF> for bool {
    #[inline(always)]
    fn from(variant: SPF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SPF_R = crate::BitReader<SPF>;
impl SPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPF {
        match self.bits {
            false => SPF::Cleared,
            true => SPF::Set,
        }
    }
    #[doc = "No SRAM2 parity error detected"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SPF::Cleared
    }
    #[doc = "SRAM2 parity error detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SPF::Set
    }
}
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SPF_W<'a, REG> = crate::BitWriter<'a, REG, SPF>;
impl<'a, REG> SPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SRAM2 parity error detected"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(SPF::Cleared)
    }
    #[doc = "SRAM2 parity error detected"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(SPF::Set)
    }
}
impl R {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<CFGR2rs> {
        CLL_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<CFGR2rs> {
        SPL_W::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<CFGR2rs> {
        PVDL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> ECCL_W<CFGR2rs> {
        ECCL_W::new(self, 3)
    }
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn spf(&mut self) -> SPF_W<CFGR2rs> {
        SPF_W::new(self, 8)
    }
}
#[doc = "CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
