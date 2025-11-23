///Register `CICR` writer
pub type W = crate::W<CICRrs>;
/**LSI ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC {
    ///1: Clear the LSIRDYF flag
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
    ///Clear the LSIRDYF flag
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
    ///1: Clear the LSERDYF flag
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
    ///Clear the LSERDYF flag
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
    ///1: Clear the MSIRDYF flag
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
    ///Clear the MSIRDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYC::Clear)
    }
}
/**HSI ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC {
    ///1: Clear HSIRDYF flag
    Clear = 1,
}
impl From<HSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYC` writer - HSI ready interrupt clear
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYC>;
impl<'a, REG> HSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear HSIRDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYC::Clear)
    }
}
/**HSE ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC {
    ///1: Clear HSERDYF flag
    Clear = 1,
}
impl From<HSERDYC> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYC` writer - HSE ready interrupt clear
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYC>;
impl<'a, REG> HSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear HSERDYF flag
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
    ///1: Clear PLLRDYF flag
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
    ///Clear PLLRDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYC::Clear)
    }
}
/**PLLSAI1 ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYC {
    ///1: Clear PLLSAI1RDYF flag
    Clear = 1,
}
impl From<PLLSAI1RDYC> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear
pub type PLLSAI1RDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1RDYC>;
impl<'a, REG> PLLSAI1RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear PLLSAI1RDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1RDYC::Clear)
    }
}
/**PLLSAI2 ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYC {
    ///1: Clear PLLSAI2RDYF flag
    Clear = 1,
}
impl From<PLLSAI2RDYC> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI2RDYC` writer - PLLSAI2 ready interrupt clear
pub type PLLSAI2RDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2RDYC>;
impl<'a, REG> PLLSAI2RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear PLLSAI2RDYF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2RDYC::Clear)
    }
}
/**Clock security system interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSC {
    ///1: Clear the CSSF flag
    Clear = 1,
}
impl From<CSSC> for bool {
    #[inline(always)]
    fn from(variant: CSSC) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSC` writer - Clock security system interrupt clear
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG, CSSC>;
impl<'a, REG> CSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the CSSF flag
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
    ///1: Clear the LSECSSF flag
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
    ///Clear the LSECSSF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSC::Clear)
    }
}
/**HSI48 oscillator ready interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYC {
    ///1: Clear the HSI48RDYC flag
    Clear = 1,
}
impl From<HSI48RDYC> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear
pub type HSI48RDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSI48RDYC>;
impl<'a, REG> HSI48RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the HSI48RDYC flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYC::Clear)
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
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt clear
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<'_, CICRrs> {
        MSIRDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI ready interrupt clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 5 - PLL ready interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<'_, CICRrs> {
        PLLRDYC_W::new(self, 5)
    }
    ///Bit 6 - PLLSAI1 ready interrupt clear
    #[inline(always)]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W<'_, CICRrs> {
        PLLSAI1RDYC_W::new(self, 6)
    }
    ///Bit 7 - PLLSAI2 ready interrupt clear
    #[inline(always)]
    pub fn pllsai2rdyc(&mut self) -> PLLSAI2RDYC_W<'_, CICRrs> {
        PLLSAI2RDYC_W::new(self, 7)
    }
    ///Bit 8 - Clock security system interrupt clear
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<'_, CICRrs> {
        CSSC_W::new(self, 8)
    }
    ///Bit 9 - LSE Clock security system interrupt clear
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<'_, CICRrs> {
        LSECSSC_W::new(self, 9)
    }
    ///Bit 10 - HSI48 oscillator ready interrupt clear
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<'_, CICRrs> {
        HSI48RDYC_W::new(self, 10)
    }
}
/**Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:CICR)*/
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cicr::W`](W) writer structure
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {}
