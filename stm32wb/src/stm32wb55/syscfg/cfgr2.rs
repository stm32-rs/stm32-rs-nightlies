///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**Cortex-M4 LOCKUP (Hardfault) output enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLLR {
    ///0: CPU LOCKUP output disconnected from TIM1/16/17 break input
    Disconnected = 0,
    ///1: CPU LOCKUP output connected to TIM1/16/17 break input
    Connected = 1,
}
impl From<CLLR> for bool {
    #[inline(always)]
    fn from(variant: CLLR) -> Self {
        variant as u8 != 0
    }
}
///Field `CLL` reader - Cortex-M4 LOCKUP (Hardfault) output enable bit
pub type CLL_R = crate::BitReader<CLLR>;
impl CLL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLLR {
        match self.bits {
            false => CLLR::Disconnected,
            true => CLLR::Connected,
        }
    }
    ///CPU LOCKUP output disconnected from TIM1/16/17 break input
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CLLR::Disconnected
    }
    ///CPU LOCKUP output connected to TIM1/16/17 break input
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CLLR::Connected
    }
}
/**Cortex-M4 LOCKUP (Hardfault) output enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLLW {
    ///1: Connect CPU LOCKUP output to TIM1/16/17 break input
    Connect = 1,
}
impl From<CLLW> for bool {
    #[inline(always)]
    fn from(variant: CLLW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLL` writer - Cortex-M4 LOCKUP (Hardfault) output enable bit
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLLW>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Connect CPU LOCKUP output to TIM1/16/17 break input
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(CLLW::Connect)
    }
}
/**SRAM2 parity lock bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLR {
    ///0: SRAM2 parity error signal disconnected from TIM1/16/17 break input
    Disconnected = 0,
    ///1: SRAM2 parity error signal connected to TIM1/16/17 break input
    Connected = 1,
}
impl From<SPLR> for bool {
    #[inline(always)]
    fn from(variant: SPLR) -> Self {
        variant as u8 != 0
    }
}
///Field `SPL` reader - SRAM2 parity lock bit
pub type SPL_R = crate::BitReader<SPLR>;
impl SPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPLR {
        match self.bits {
            false => SPLR::Disconnected,
            true => SPLR::Connected,
        }
    }
    ///SRAM2 parity error signal disconnected from TIM1/16/17 break input
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == SPLR::Disconnected
    }
    ///SRAM2 parity error signal connected to TIM1/16/17 break input
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == SPLR::Connected
    }
}
/**SRAM2 parity lock bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLW {
    ///1: Connect SRAM2 parity error signal to TIM1/16/17 break input
    Connect = 1,
}
impl From<SPLW> for bool {
    #[inline(always)]
    fn from(variant: SPLW) -> Self {
        variant as u8 != 0
    }
}
///Field `SPL` writer - SRAM2 parity lock bit
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG, SPLW>;
impl<'a, REG> SPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Connect SRAM2 parity error signal to TIM1/16/17 break input
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(SPLW::Connect)
    }
}
/**PVD lock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDLR {
    ///0: PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\[2:0\] bits can be programmed by the application
    Disconnected = 0,
    ///1: PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\[2:0\] bits are read only
    Connected = 1,
}
impl From<PVDLR> for bool {
    #[inline(always)]
    fn from(variant: PVDLR) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDL` reader - PVD lock enable bit
pub type PVDL_R = crate::BitReader<PVDLR>;
impl PVDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDLR {
        match self.bits {
            false => PVDLR::Disconnected,
            true => PVDLR::Connected,
        }
    }
    ///PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\[2:0\] bits can be programmed by the application
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PVDLR::Disconnected
    }
    ///PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\[2:0\] bits are read only
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVDLR::Connected
    }
}
/**PVD lock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDLW {
    ///1: Connect PVD interretup to TIM1/16/17 break input
    Connect = 1,
}
impl From<PVDLW> for bool {
    #[inline(always)]
    fn from(variant: PVDLW) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDL` writer - PVD lock enable bit
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG, PVDLW>;
impl<'a, REG> PVDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Connect PVD interretup to TIM1/16/17 break input
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLW::Connect)
    }
}
/**ECC Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCLR {
    ///0: ECC error disconnected from TIM1/16/17 break input
    Disconnected = 0,
    ///1: ECC error connected to TIM1/16/17 break input
    Connected = 1,
}
impl From<ECCLR> for bool {
    #[inline(always)]
    fn from(variant: ECCLR) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCL` reader - ECC Lock
pub type ECCL_R = crate::BitReader<ECCLR>;
impl ECCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCLR {
        match self.bits {
            false => ECCLR::Disconnected,
            true => ECCLR::Connected,
        }
    }
    ///ECC error disconnected from TIM1/16/17 break input
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == ECCLR::Disconnected
    }
    ///ECC error connected to TIM1/16/17 break input
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == ECCLR::Connected
    }
}
/**ECC Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCLW {
    ///1: Connect ECC error to TIM1/16/17 break input
    Connect = 1,
}
impl From<ECCLW> for bool {
    #[inline(always)]
    fn from(variant: ECCLW) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCL` writer - ECC Lock
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG, ECCLW>;
impl<'a, REG> ECCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Connect ECC error to TIM1/16/17 break input
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(ECCLW::Connect)
    }
}
/**SRAM2 parity error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPFR {
    ///0: No SRAM2 parity error detected
    Nominal = 0,
    ///1: SRAM2 parity error detected
    Error = 1,
}
impl From<SPFR> for bool {
    #[inline(always)]
    fn from(variant: SPFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SPF` reader - SRAM2 parity error flag
pub type SPF_R = crate::BitReader<SPFR>;
impl SPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPFR {
        match self.bits {
            false => SPFR::Nominal,
            true => SPFR::Error,
        }
    }
    ///No SRAM2 parity error detected
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        *self == SPFR::Nominal
    }
    ///SRAM2 parity error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SPFR::Error
    }
}
/**SRAM2 parity error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPFW {
    ///1: Clear SRAM2 parity error flag
    Clear = 1,
}
impl From<SPFW> for bool {
    #[inline(always)]
    fn from(variant: SPFW) -> Self {
        variant as u8 != 0
    }
}
///Field `SPF` writer - SRAM2 parity error flag
pub type SPF_W<'a, REG> = crate::BitWriter<'a, REG, SPFW>;
impl<'a, REG> SPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear SRAM2 parity error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SPFW::Clear)
    }
}
impl R {
    ///Bit 0 - Cortex-M4 LOCKUP (Hardfault) output enable bit
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 parity lock bit
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD lock enable bit
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC Lock
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SRAM2 parity error flag
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("spf", &self.spf())
            .field("eccl", &self.eccl())
            .field("pvdl", &self.pvdl())
            .field("spl", &self.spl())
            .field("cll", &self.cll())
            .finish()
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:CFGR2)*/
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
