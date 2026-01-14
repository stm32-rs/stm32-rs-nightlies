///Register `CICR` writer
pub type W = crate::W<CICRrs>;
/**LSI ready Interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<LSIRDYCW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI ready Interrupt clear
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYCW>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYCW::Clear)
    }
}
///Field `LSERDYC` writer - LSE ready Interrupt clear
pub use LSIRDYC_W as LSERDYC_W;
///Field `HSI16RDYC` writer - HSI16 ready Interrupt clear
pub use LSIRDYC_W as HSI16RDYC_W;
///Field `HSERDYC` writer - HSE ready Interrupt clear
pub use LSIRDYC_W as HSERDYC_W;
///Field `PLLRDYC` writer - PLL ready Interrupt clear
pub use LSIRDYC_W as PLLRDYC_W;
///Field `MSIRDYC` writer - MSI ready Interrupt clear
pub use LSIRDYC_W as MSIRDYC_W;
///Field `HSI48RDYC` writer - HSI48 ready Interrupt clear
pub use LSIRDYC_W as HSI48RDYC_W;
///Field `CSSLSEC` writer - LSE Clock Security System Interrupt clear
pub use LSIRDYC_W as CSSLSEC_W;
///Field `CSSHSEC` writer - Clock Security System Interrupt clear
pub use LSIRDYC_W as CSSHSEC_W;
impl core::fmt::Debug for crate::generic::Reg<CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready Interrupt clear
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready Interrupt clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - HSI16 ready Interrupt clear
    #[inline(always)]
    pub fn hsi16rdyc(&mut self) -> HSI16RDYC_W<'_, CICRrs> {
        HSI16RDYC_W::new(self, 2)
    }
    ///Bit 3 - HSE ready Interrupt clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CICRrs> {
        HSERDYC_W::new(self, 3)
    }
    ///Bit 4 - PLL ready Interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<'_, CICRrs> {
        PLLRDYC_W::new(self, 4)
    }
    ///Bit 5 - MSI ready Interrupt clear
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<'_, CICRrs> {
        MSIRDYC_W::new(self, 5)
    }
    ///Bit 6 - HSI48 ready Interrupt clear
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<'_, CICRrs> {
        HSI48RDYC_W::new(self, 6)
    }
    ///Bit 7 - LSE Clock Security System Interrupt clear
    #[inline(always)]
    pub fn csslsec(&mut self) -> CSSLSEC_W<'_, CICRrs> {
        CSSLSEC_W::new(self, 7)
    }
    ///Bit 8 - Clock Security System Interrupt clear
    #[inline(always)]
    pub fn csshsec(&mut self) -> CSSHSEC_W<'_, CICRrs> {
        CSSHSEC_W::new(self, 8)
    }
}
/**Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#RCC:CICR)*/
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
