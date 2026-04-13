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
///Field `LSERDYC` writer - LSE ready interrupt clear
pub use LSIRDYC_W as LSERDYC_W;
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub use LSIRDYC_W as MSIRDYC_W;
///Field `HSIRDYC` writer - HSI16 ready interrupt clear
pub use LSIRDYC_W as HSIRDYC_W;
///Field `HSERDYC` writer - HSE32 ready interrupt clear
pub use LSIRDYC_W as HSERDYC_W;
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub use LSIRDYC_W as PLLRDYC_W;
///Field `CSSC` writer - HSE32 Clock security system interrupt clear
pub use LSIRDYC_W as CSSC_W;
///Field `LSECSSC` writer - LSE Clock security system interrupt clear
pub use LSIRDYC_W as LSECSSC_W;
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
    ///Bit 3 - HSI16 ready interrupt clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE32 ready interrupt clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 5 - PLL ready interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<'_, CICRrs> {
        PLLRDYC_W::new(self, 5)
    }
    ///Bit 8 - HSE32 Clock security system interrupt clear
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<'_, CICRrs> {
        CSSC_W::new(self, 8)
    }
    ///Bit 9 - LSE Clock security system interrupt clear
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<'_, CICRrs> {
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
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {}
