///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**Cortex-M4 LOCKUP (Hardfault) output enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLL {
    ///0: Cortex®-M4 LOCKUP output disconnected from TIM1/8/15/16/17 Break inputs
    Disconnected = 0,
    ///1: Cortex®-M4 LOCKUP output connected to TIM1/8/15/16/17 Break inputs
    Connected = 1,
}
impl From<CLL> for bool {
    #[inline(always)]
    fn from(variant: CLL) -> Self {
        variant as u8 != 0
    }
}
///Field `CLL` writer - Cortex-M4 LOCKUP (Hardfault) output enable bit
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLL>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cortex®-M4 LOCKUP output disconnected from TIM1/8/15/16/17 Break inputs
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Disconnected)
    }
    ///Cortex®-M4 LOCKUP output connected to TIM1/8/15/16/17 Break inputs
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Connected)
    }
}
/**SRAM2 parity lock bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPL {
    ///0: SRAM2 parity error signal disconnected from TIM1/8/15/16/17 Break inputs
    Disconnected = 0,
    ///1: SRAM2 parity error signal connected to TIM1/8/15/16/17 Break inputs
    Connected = 1,
}
impl From<SPL> for bool {
    #[inline(always)]
    fn from(variant: SPL) -> Self {
        variant as u8 != 0
    }
}
///Field `SPL` writer - SRAM2 parity lock bit
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG, SPL>;
impl<'a, REG> SPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 parity error signal disconnected from TIM1/8/15/16/17 Break inputs
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(SPL::Disconnected)
    }
    ///SRAM2 parity error signal connected to TIM1/8/15/16/17 Break inputs
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(SPL::Connected)
    }
}
/**PVD lock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDL {
    ///0: PVD interrupt disconnected from TIM1/8/15/16/17 Break input. PVDE and PLS\[2:0\] bits can be programmed by the application
    Disconnected = 0,
    ///1: PVD interrupt connected to TIM1/8/15/16/17 Break input, PVDE and PLS\[2:0\] bits are read only
    Connected = 1,
}
impl From<PVDL> for bool {
    #[inline(always)]
    fn from(variant: PVDL) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDL` writer - PVD lock enable bit
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG, PVDL>;
impl<'a, REG> PVDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD interrupt disconnected from TIM1/8/15/16/17 Break input. PVDE and PLS\[2:0\] bits can be programmed by the application
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL::Disconnected)
    }
    ///PVD interrupt connected to TIM1/8/15/16/17 Break input, PVDE and PLS\[2:0\] bits are read only
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL::Connected)
    }
}
/**ECC Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCL {
    ///0: ECC error disconnected from TIM1/8/15/16/17 Break input
    Disconnected = 0,
    ///1: ECC error connected to TIM1/8/15/16/17 Break input
    Connected = 1,
}
impl From<ECCL> for bool {
    #[inline(always)]
    fn from(variant: ECCL) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCL` writer - ECC Lock
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG, ECCL>;
impl<'a, REG> ECCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECC error disconnected from TIM1/8/15/16/17 Break input
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(ECCL::Disconnected)
    }
    ///ECC error connected to TIM1/8/15/16/17 Break input
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(ECCL::Connected)
    }
}
/**SRAM2 parity error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPF {
    ///0: No SRAM2 parity error detected
    Cleared = 0,
    ///1: SRAM2 parity error detected
    Set = 1,
}
impl From<SPF> for bool {
    #[inline(always)]
    fn from(variant: SPF) -> Self {
        variant as u8 != 0
    }
}
///Field `SPF` reader - SRAM2 parity error flag
pub type SPF_R = crate::BitReader<SPF>;
impl SPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPF {
        match self.bits {
            false => SPF::Cleared,
            true => SPF::Set,
        }
    }
    ///No SRAM2 parity error detected
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SPF::Cleared
    }
    ///SRAM2 parity error detected
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SPF::Set
    }
}
///Field `SPF` writer - SRAM2 parity error flag
pub type SPF_W<'a, REG> = crate::BitWriter<'a, REG, SPF>;
impl<'a, REG> SPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No SRAM2 parity error detected
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(SPF::Cleared)
    }
    ///SRAM2 parity error detected
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(SPF::Set)
    }
}
impl R {
    ///Bit 8 - SRAM2 parity error flag
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2").field("spf", &self.spf()).finish()
    }
}
impl W {
    ///Bit 0 - Cortex-M4 LOCKUP (Hardfault) output enable bit
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W<'_, CFGR2rs> {
        CLL_W::new(self, 0)
    }
    ///Bit 1 - SRAM2 parity lock bit
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W<'_, CFGR2rs> {
        SPL_W::new(self, 1)
    }
    ///Bit 2 - PVD lock enable bit
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGR2rs> {
        PVDL_W::new(self, 2)
    }
    ///Bit 3 - ECC Lock
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W<'_, CFGR2rs> {
        ECCL_W::new(self, 3)
    }
    ///Bit 8 - SRAM2 parity error flag
    #[inline(always)]
    pub fn spf(&mut self) -> SPF_W<'_, CFGR2rs> {
        SPF_W::new(self, 8)
    }
}
/**CFGR2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
