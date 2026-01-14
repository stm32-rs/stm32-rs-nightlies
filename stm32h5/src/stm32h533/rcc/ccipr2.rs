///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
///Field `USART11SEL` reader - USART11 kernel clock source selection
pub use super::ccipr1::USART1SEL_R as USART11SEL_R;
///Field `UART12SEL` reader - UART12 kernel clock source selection
pub use super::ccipr1::USART1SEL_R as UART12SEL_R;
///Field `USART11SEL` writer - USART11 kernel clock source selection
pub use super::ccipr1::USART1SEL_W as USART11SEL_W;
///Field `UART12SEL` writer - UART12 kernel clock source selection
pub use super::ccipr1::USART1SEL_W as UART12SEL_W;
///USART11 kernel clock source selection
pub use super::ccipr1::USARTSEL;
/**LPTIM1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIMSEL {
    ///0: Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    Pclk = 0,
    ///1: PLL2 P clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///2: PLL3 R clock selected as clock source (pll3_r_ck)
    Pll3R = 2,
    ///3: LSE kernel selected as clock source (lse_ck)
    LseKer = 3,
    ///4: LSI kernel selected as clock source (lsi_ker_ck)
    LsiKer = 4,
    ///5: per_ck clock selected as clock source
    PerCk = 5,
}
impl From<LPTIMSEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIMSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIMSEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIMSEL {}
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection
pub type LPTIM1SEL_R = crate::FieldReader<LPTIMSEL>;
impl LPTIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIMSEL> {
        match self.bits {
            0 => Some(LPTIMSEL::Pclk),
            1 => Some(LPTIMSEL::Pll2P),
            2 => Some(LPTIMSEL::Pll3R),
            3 => Some(LPTIMSEL::LseKer),
            4 => Some(LPTIMSEL::LsiKer),
            5 => Some(LPTIMSEL::PerCk),
            _ => None,
        }
    }
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPTIMSEL::Pclk
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIMSEL::Pll2P
    }
    ///PLL3 R clock selected as clock source (pll3_r_ck)
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIMSEL::Pll3R
    }
    ///LSE kernel selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse_ker(&self) -> bool {
        *self == LPTIMSEL::LseKer
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn is_lsi_ker(&self) -> bool {
        *self == LPTIMSEL::LsiKer
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == LPTIMSEL::PerCk
    }
}
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIMSEL>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIMSEL::Pclk)
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIMSEL::Pll2P)
    }
    ///PLL3 R clock selected as clock source (pll3_r_ck)
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIMSEL::Pll3R)
    }
    ///LSE kernel selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIMSEL::LseKer)
    }
    ///LSI kernel selected as clock source (lsi_ker_ck)
    #[inline(always)]
    pub fn lsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIMSEL::LsiKer)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIMSEL::PerCk)
    }
}
///Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection
pub use LPTIM1SEL_R as LPTIM2SEL_R;
///Field `LPTIM3SEL` reader - LPTIM3 kernel clock source selection
pub use LPTIM1SEL_R as LPTIM3SEL_R;
///Field `LPTIM4SEL` reader - LPTIM4 kernel clock source selection
pub use LPTIM1SEL_R as LPTIM4SEL_R;
///Field `LPTIM5SEL` reader - LPTIM5 kernel clock source selection
pub use LPTIM1SEL_R as LPTIM5SEL_R;
///Field `LPTIM6SEL` reader - LPTIM6 kernel clock source selection
pub use LPTIM1SEL_R as LPTIM6SEL_R;
///Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection
pub use LPTIM1SEL_W as LPTIM2SEL_W;
///Field `LPTIM3SEL` writer - LPTIM3 kernel clock source selection
pub use LPTIM1SEL_W as LPTIM3SEL_W;
///Field `LPTIM4SEL` writer - LPTIM4 kernel clock source selection
pub use LPTIM1SEL_W as LPTIM4SEL_W;
///Field `LPTIM5SEL` writer - LPTIM5 kernel clock source selection
pub use LPTIM1SEL_W as LPTIM5SEL_W;
///Field `LPTIM6SEL` writer - LPTIM6 kernel clock source selection
pub use LPTIM1SEL_W as LPTIM6SEL_W;
impl R {
    ///Bits 0:2 - USART11 kernel clock source selection
    #[inline(always)]
    pub fn usart11sel(&self) -> USART11SEL_R {
        USART11SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - UART12 kernel clock source selection
    #[inline(always)]
    pub fn uart12sel(&self) -> UART12SEL_R {
        UART12SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - LPTIM2 kernel clock source selection
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - LPTIM3 kernel clock source selection
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - LPTIM4 kernel clock source selection
    #[inline(always)]
    pub fn lptim4sel(&self) -> LPTIM4SEL_R {
        LPTIM4SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - LPTIM5 kernel clock source selection
    #[inline(always)]
    pub fn lptim5sel(&self) -> LPTIM5SEL_R {
        LPTIM5SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - LPTIM6 kernel clock source selection
    #[inline(always)]
    pub fn lptim6sel(&self) -> LPTIM6SEL_R {
        LPTIM6SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("usart11sel", &self.usart11sel())
            .field("uart12sel", &self.uart12sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("lptim3sel", &self.lptim3sel())
            .field("lptim4sel", &self.lptim4sel())
            .field("lptim5sel", &self.lptim5sel())
            .field("lptim6sel", &self.lptim6sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USART11 kernel clock source selection
    #[inline(always)]
    pub fn usart11sel(&mut self) -> USART11SEL_W<'_, CCIPR2rs> {
        USART11SEL_W::new(self, 0)
    }
    ///Bits 4:6 - UART12 kernel clock source selection
    #[inline(always)]
    pub fn uart12sel(&mut self) -> UART12SEL_W<'_, CCIPR2rs> {
        UART12SEL_W::new(self, 4)
    }
    ///Bits 8:10 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, CCIPR2rs> {
        LPTIM1SEL_W::new(self, 8)
    }
    ///Bits 12:14 - LPTIM2 kernel clock source selection
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<'_, CCIPR2rs> {
        LPTIM2SEL_W::new(self, 12)
    }
    ///Bits 16:18 - LPTIM3 kernel clock source selection
    #[inline(always)]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<'_, CCIPR2rs> {
        LPTIM3SEL_W::new(self, 16)
    }
    ///Bits 20:22 - LPTIM4 kernel clock source selection
    #[inline(always)]
    pub fn lptim4sel(&mut self) -> LPTIM4SEL_W<'_, CCIPR2rs> {
        LPTIM4SEL_W::new(self, 20)
    }
    ///Bits 24:26 - LPTIM5 kernel clock source selection
    #[inline(always)]
    pub fn lptim5sel(&mut self) -> LPTIM5SEL_W<'_, CCIPR2rs> {
        LPTIM5SEL_W::new(self, 24)
    }
    ///Bits 28:30 - LPTIM6 kernel clock source selection
    #[inline(always)]
    pub fn lptim6sel(&mut self) -> LPTIM6SEL_W<'_, CCIPR2rs> {
        LPTIM6SEL_W::new(self, 28)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
