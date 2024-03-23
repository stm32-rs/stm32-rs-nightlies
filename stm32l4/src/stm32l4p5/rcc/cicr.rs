#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICRrs>;
#[doc = "LSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC {
    #[doc = "1: Clear the LSIRDYF flag"]
    Clear = 1,
}
impl From<LSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYC>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the LSIRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC::Clear)
    }
}
#[doc = "LSE ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYC {
    #[doc = "1: Clear the LSERDYF flag"]
    Clear = 1,
}
impl From<LSERDYC> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYC>;
impl<'a, REG> LSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the LSERDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYC::Clear)
    }
}
#[doc = "MSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYC {
    #[doc = "1: Clear the MSIRDYF flag"]
    Clear = 1,
}
impl From<MSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDYC` writer - MSI ready interrupt clear"]
pub type MSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, MSIRDYC>;
impl<'a, REG> MSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the MSIRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRDYC::Clear)
    }
}
#[doc = "HSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC {
    #[doc = "1: Clear HSIRDYF flag"]
    Clear = 1,
}
impl From<HSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYC>;
impl<'a, REG> HSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear HSIRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYC::Clear)
    }
}
#[doc = "HSE ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC {
    #[doc = "1: Clear HSERDYF flag"]
    Clear = 1,
}
impl From<HSERDYC> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYC>;
impl<'a, REG> HSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear HSERDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYC::Clear)
    }
}
#[doc = "PLL ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYC {
    #[doc = "1: Clear PLLRDYF flag"]
    Clear = 1,
}
impl From<PLLRDYC> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub type PLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLLRDYC>;
impl<'a, REG> PLLRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PLLRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDYC::Clear)
    }
}
#[doc = "PLLSAI1 ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYC {
    #[doc = "1: Clear PLLSAI1RDYF flag"]
    Clear = 1,
}
impl From<PLLSAI1RDYC> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear"]
pub type PLLSAI1RDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI1RDYC>;
impl<'a, REG> PLLSAI1RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PLLSAI1RDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI1RDYC::Clear)
    }
}
#[doc = "PLLSAI2 ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYC {
    #[doc = "1: Clear PLLSAI2RDYF flag"]
    Clear = 1,
}
impl From<PLLSAI2RDYC> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2RDYC` writer - PLLSAI2 ready interrupt clear"]
pub type PLLSAI2RDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSAI2RDYC>;
impl<'a, REG> PLLSAI2RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PLLSAI2RDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAI2RDYC::Clear)
    }
}
#[doc = "Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSC {
    #[doc = "1: Clear the CSSF flag"]
    Clear = 1,
}
impl From<CSSC> for bool {
    #[inline(always)]
    fn from(variant: CSSC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG, CSSC>;
impl<'a, REG> CSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the CSSF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSSC::Clear)
    }
}
#[doc = "LSE Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSC {
    #[doc = "1: Clear the LSECSSF flag"]
    Clear = 1,
}
impl From<LSECSSC> for bool {
    #[inline(always)]
    fn from(variant: LSECSSC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear"]
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSC>;
impl<'a, REG> LSECSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the LSECSSF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSC::Clear)
    }
}
#[doc = "HSI48 oscillator ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYC {
    #[doc = "1: Clear the HSI48RDYC flag"]
    Clear = 1,
}
impl From<HSI48RDYC> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear"]
pub type HSI48RDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSI48RDYC>;
impl<'a, REG> HSI48RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the HSI48RDYC flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYC::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<CICRrs> {
        MSIRDYC_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CICRrs> {
        PLLRDYC_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W<CICRrs> {
        PLLSAI1RDYC_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2rdyc(&mut self) -> PLLSAI2RDYC_W<CICRrs> {
        PLLSAI2RDYC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<CICRrs> {
        CSSC_W::new(self, 8)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<CICRrs> {
        LSECSSC_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 oscillator ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<CICRrs> {
        HSI48RDYC_W::new(self, 10)
    }
}
#[doc = "Clock interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICRrs {
    const RESET_VALUE: u32 = 0;
}
