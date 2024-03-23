#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLLR {
    #[doc = "0: CPU LOCKUP output disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: CPU LOCKUP output connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<CLLR> for bool {
    #[inline(always)]
    fn from(variant: CLLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` reader - CPU1 LOCKUP (Hardfault) output enable bit"]
pub type CLL_R = crate::BitReader<CLLR>;
impl CLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLLR {
        match self.bits {
            false => CLLR::Disconnected,
            true => CLLR::Connected,
        }
    }
    #[doc = "CPU LOCKUP output disconnected from TIM1/16/17 break input"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CLLR::Disconnected
    }
    #[doc = "CPU LOCKUP output connected to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CLLR::Connected
    }
}
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLLW {
    #[doc = "1: Connect CPU LOCKUP output to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<CLLW> for bool {
    #[inline(always)]
    fn from(variant: CLLW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` writer - CPU1 LOCKUP (Hardfault) output enable bit"]
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLLW>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect CPU LOCKUP output to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(CLLW::Connect)
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLR {
    #[doc = "0: SRAM2 parity error signal disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: SRAM2 parity error signal connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<SPLR> for bool {
    #[inline(always)]
    fn from(variant: SPLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` reader - SRAM2 parity lock bit"]
pub type SPL_R = crate::BitReader<SPLR>;
impl SPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPLR {
        match self.bits {
            false => SPLR::Disconnected,
            true => SPLR::Connected,
        }
    }
    #[doc = "SRAM2 parity error signal disconnected from TIM1/16/17 break input"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == SPLR::Disconnected
    }
    #[doc = "SRAM2 parity error signal connected to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == SPLR::Connected
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLW {
    #[doc = "1: Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<SPLW> for bool {
    #[inline(always)]
    fn from(variant: SPLW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG, SPLW>;
impl<'a, REG> SPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(SPLW::Connect)
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDLR {
    #[doc = "0: PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    Disconnected = 0,
    #[doc = "1: PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits are read only"]
    Connected = 1,
}
impl From<PVDLR> for bool {
    #[inline(always)]
    fn from(variant: PVDLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` reader - PVD lock enable bit"]
pub type PVDL_R = crate::BitReader<PVDLR>;
impl PVDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDLR {
        match self.bits {
            false => PVDLR::Disconnected,
            true => PVDLR::Connected,
        }
    }
    #[doc = "PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PVDLR::Disconnected
    }
    #[doc = "PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits are read only"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVDLR::Connected
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDLW {
    #[doc = "1: Connect PVD interretup to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<PVDLW> for bool {
    #[inline(always)]
    fn from(variant: PVDLW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG, PVDLW>;
impl<'a, REG> PVDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect PVD interretup to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLW::Connect)
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCLR {
    #[doc = "0: ECC error disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: ECC error connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<ECCLR> for bool {
    #[inline(always)]
    fn from(variant: ECCLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` reader - ECC Lock"]
pub type ECCL_R = crate::BitReader<ECCLR>;
impl ECCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCLR {
        match self.bits {
            false => ECCLR::Disconnected,
            true => ECCLR::Connected,
        }
    }
    #[doc = "ECC error disconnected from TIM1/16/17 break input"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == ECCLR::Disconnected
    }
    #[doc = "ECC error connected to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == ECCLR::Connected
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCLW {
    #[doc = "1: Connect ECC error to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<ECCLW> for bool {
    #[inline(always)]
    fn from(variant: ECCLW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG, ECCLW>;
impl<'a, REG> ECCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect ECC error to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(ECCLW::Connect)
    }
}
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPFR {
    #[doc = "0: No SRAM2 parity error detected"]
    Nominal = 0,
    #[doc = "1: SRAM2 parity error detected"]
    Error = 1,
}
impl From<SPFR> for bool {
    #[inline(always)]
    fn from(variant: SPFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SPF_R = crate::BitReader<SPFR>;
impl SPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPFR {
        match self.bits {
            false => SPFR::Nominal,
            true => SPFR::Error,
        }
    }
    #[doc = "No SRAM2 parity error detected"]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        *self == SPFR::Nominal
    }
    #[doc = "SRAM2 parity error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SPFR::Error
    }
}
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPFW {
    #[doc = "1: Clear SRAM2 parity error flag"]
    Clear = 1,
}
impl From<SPFW> for bool {
    #[inline(always)]
    fn from(variant: SPFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SPF_W<'a, REG> = crate::BitWriter<'a, REG, SPFW>;
impl<'a, REG> SPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear SRAM2 parity error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SPFW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
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
