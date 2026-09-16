///Register `CCIPR3` reader
pub type R = crate::R<CCIPR3rs>;
///Register `CCIPR3` writer
pub type W = crate::W<CCIPR3rs>;
/**SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI123SEL {
    ///0: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 0,
    ///1: PLL2 P clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///2: PLL3 P clock selected as clock source (pll3_p_ck)
    Pll3P = 2,
    ///3: AUDIOCLK clock selected as clock source
    Audioclk = 3,
    ///4: per_ck clock selected as clock source
    PerCk = 4,
}
impl From<SPI123SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI123SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI123SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI123SEL {}
///Field `SPI1SEL` reader - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI1SEL_R = crate::FieldReader<SPI123SEL>;
impl SPI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI123SEL> {
        match self.bits {
            0 => Some(SPI123SEL::Pll1Q),
            1 => Some(SPI123SEL::Pll2P),
            2 => Some(SPI123SEL::Pll3P),
            3 => Some(SPI123SEL::Audioclk),
            4 => Some(SPI123SEL::PerCk),
            _ => None,
        }
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPI123SEL::Pll1Q
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SPI123SEL::Pll2P
    }
    ///PLL3 P clock selected as clock source (pll3_p_ck)
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SPI123SEL::Pll3P
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == SPI123SEL::Audioclk
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == SPI123SEL::PerCk
    }
}
///Field `SPI1SEL` writer - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI123SEL>;
impl<'a, REG> SPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI123SEL::Pll1Q)
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI123SEL::Pll2P)
    }
    ///PLL3 P clock selected as clock source (pll3_p_ck)
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI123SEL::Pll3P)
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(SPI123SEL::Audioclk)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(SPI123SEL::PerCk)
    }
}
///Field `SPI2SEL` reader - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use SPI1SEL_R as SPI2SEL_R;
///Field `SPI3SEL` reader - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use SPI1SEL_R as SPI3SEL_R;
///Field `SPI2SEL` writer - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use SPI1SEL_W as SPI2SEL_W;
///Field `SPI3SEL` writer - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use SPI1SEL_W as SPI3SEL_W;
/**SPI4 kernel clock source selection others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI456SEL {
    ///0: Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    Pclk = 0,
    ///1: PLL2 Q clock selected as clock source (pll2_p_ck)
    Pll2Q = 1,
    ///2: PLL3 Q clock selected as clock source (pll3_p_ck)
    Pll3Q = 2,
    ///3: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 3,
    ///4: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 4,
    ///5: HSE clock selected as clock source (hse_ck)
    Hse = 5,
}
impl From<SPI456SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI456SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI456SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI456SEL {}
///Field `SPI4SEL` reader - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SPI4SEL_R = crate::FieldReader<SPI456SEL>;
impl SPI4SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI456SEL> {
        match self.bits {
            0 => Some(SPI456SEL::Pclk),
            1 => Some(SPI456SEL::Pll2Q),
            2 => Some(SPI456SEL::Pll3Q),
            3 => Some(SPI456SEL::HsiKer),
            4 => Some(SPI456SEL::CsiKer),
            5 => Some(SPI456SEL::Hse),
            _ => None,
        }
    }
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == SPI456SEL::Pclk
    }
    ///PLL2 Q clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI456SEL::Pll2Q
    }
    ///PLL3 Q clock selected as clock source (pll3_p_ck)
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI456SEL::Pll3Q
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI456SEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI456SEL::CsiKer
    }
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI456SEL::Hse
    }
}
///Field `SPI4SEL` writer - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled
pub type SPI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI456SEL>;
impl<'a, REG> SPI4SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(SPI456SEL::Pclk)
    }
    ///PLL2 Q clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI456SEL::Pll2Q)
    }
    ///PLL3 Q clock selected as clock source (pll3_p_ck)
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI456SEL::Pll3Q)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI456SEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI456SEL::CsiKer)
    }
    ///HSE clock selected as clock source (hse_ck)
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SPI456SEL::Hse)
    }
}
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
pub use super::ccipr1::USART1SEL_R as LPUART1SEL_R;
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
pub use super::ccipr1::USART1SEL_W as LPUART1SEL_W;
///LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
pub use super::ccipr1::USARTSEL;
///Field `SPI5SEL` reader - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled
pub use SPI4SEL_R as SPI5SEL_R;
///Field `SPI6SEL` reader - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled
pub use SPI4SEL_R as SPI6SEL_R;
///Field `SPI5SEL` writer - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled
pub use SPI4SEL_W as SPI5SEL_W;
///Field `SPI6SEL` writer - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled
pub use SPI4SEL_W as SPI6SEL_W;
impl R {
    ///Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi4sel(&self) -> SPI4SEL_R {
        SPI4SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi5sel(&self) -> SPI5SEL_R {
        SPI5SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR3")
            .field("spi1sel", &self.spi1sel())
            .field("spi2sel", &self.spi2sel())
            .field("spi3sel", &self.spi3sel())
            .field("spi4sel", &self.spi4sel())
            .field("spi5sel", &self.spi5sel())
            .field("spi6sel", &self.spi6sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<'_, CCIPR3rs> {
        SPI1SEL_W::new(self, 0)
    }
    ///Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<'_, CCIPR3rs> {
        SPI2SEL_W::new(self, 3)
    }
    ///Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<'_, CCIPR3rs> {
        SPI3SEL_W::new(self, 6)
    }
    ///Bits 9:11 - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi4sel(&mut self) -> SPI4SEL_W<'_, CCIPR3rs> {
        SPI4SEL_W::new(self, 9)
    }
    ///Bits 12:14 - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi5sel(&mut self) -> SPI5SEL_W<'_, CCIPR3rs> {
        SPI5SEL_W::new(self, 12)
    }
    ///Bits 15:17 - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<'_, CCIPR3rs> {
        SPI6SEL_W::new(self, 15)
    }
    ///Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<'_, CCIPR3rs> {
        LPUART1SEL_W::new(self, 24)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RCC:CCIPR3)*/
pub struct CCIPR3rs;
impl crate::RegisterSpec for CCIPR3rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr3::R`](R) reader structure
impl crate::Readable for CCIPR3rs {}
///`write(|w| ..)` method takes [`ccipr3::W`](W) writer structure
impl crate::Writable for CCIPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR3 to value 0
impl crate::Resettable for CCIPR3rs {}
