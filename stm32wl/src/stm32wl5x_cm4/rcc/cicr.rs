///Register `CICR` writer
pub type W = crate::W<CICRrs>;
/**LSI ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYC>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC::Clear)
    }
}
/**LSE ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSERDYC> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYC` writer - LSE ready interrupt clear
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYC>;
impl<'a, REG> LSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYC::Clear)
    }
}
/**MSI ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<MSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub type MSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, MSIRDYC>;
impl<'a, REG> MSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYC::Clear)
    }
}
/**HSI16 ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<HSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYC` writer - HSI16 ready interrupt clear
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYC>;
impl<'a, REG> HSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYC::Clear)
    }
}
/**HSE32 ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<HSERDYC> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYC` writer - HSE32 ready interrupt clear
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYC>;
impl<'a, REG> HSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYC::Clear)
    }
}
/**PLL ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<PLLRDYC> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub type PLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLLRDYC>;
impl<'a, REG> PLLRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYC::Clear)
    }
}
/**HSE32 Clock security system interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<CSSC> for bool {
    #[inline(always)]
    fn from(variant: CSSC) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSC` writer - HSE32 Clock security system interrupt clear
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG, CSSC>;
impl<'a, REG> CSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSSC::Clear)
    }
}
/**LSE Clock security system interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSC {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSECSSC> for bool {
    #[inline(always)]
    fn from(variant: LSECSSC) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSC` writer - LSE Clock security system interrupt clear
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSC>;
impl<'a, REG> LSECSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSC::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<CICRrs> {
        MSIRDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE32 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 5 - PLL ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CICRrs> {
        PLLRDYC_W::new(self, 5)
    }
    ///Bit 8 - HSE32 Clock security system interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<CICRrs> {
        CSSC_W::new(self, 8)
    }
    ///Bit 9 - LSE Clock security system interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<CICRrs> {
        LSECSSC_W::new(self, 9)
    }
}
/**Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RCC:CICR)*/
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cicr::W`](W) writer structure
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {
    const RESET_VALUE: u32 = 0;
}
