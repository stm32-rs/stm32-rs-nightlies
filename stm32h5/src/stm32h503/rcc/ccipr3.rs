///Register `CCIPR3` reader
pub type R = crate::R<CCIPR3rs>;
///Register `CCIPR3` writer
pub type W = crate::W<CCIPR3rs>;
/**SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1SEL {
    ///0: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 0,
    ///1: PLL2 P clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///3: AUDIOCLK clock selected as clock source
    Audioclk = 3,
    ///4: per_ck clock selected as clock source
    PerCk = 4,
}
impl From<SPI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI1SEL {}
///Field `SPI1SEL` reader - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI1SEL_R = crate::FieldReader<SPI1SEL>;
impl SPI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI1SEL> {
        match self.bits {
            0 => Some(SPI1SEL::Pll1Q),
            1 => Some(SPI1SEL::Pll2P),
            3 => Some(SPI1SEL::Audioclk),
            4 => Some(SPI1SEL::PerCk),
            _ => None,
        }
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPI1SEL::Pll1Q
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SPI1SEL::Pll2P
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == SPI1SEL::Audioclk
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == SPI1SEL::PerCk
    }
}
///Field `SPI1SEL` writer - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI1SEL>;
impl<'a, REG> SPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::Pll1Q)
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::Pll2P)
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::Audioclk)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::PerCk)
    }
}
/**SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI2SEL {
    ///0: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 0,
    ///1: PLL2 QP clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///3: AUDIOCLK clock selected as clock source
    Audioclk = 3,
    ///4: per_ck clock selected as clock source
    PerCk = 4,
}
impl From<SPI2SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI2SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI2SEL {}
///Field `SPI2SEL` reader - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI2SEL_R = crate::FieldReader<SPI2SEL>;
impl SPI2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI2SEL> {
        match self.bits {
            0 => Some(SPI2SEL::Pll1Q),
            1 => Some(SPI2SEL::Pll2P),
            3 => Some(SPI2SEL::Audioclk),
            4 => Some(SPI2SEL::PerCk),
            _ => None,
        }
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPI2SEL::Pll1Q
    }
    ///PLL2 QP clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SPI2SEL::Pll2P
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == SPI2SEL::Audioclk
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == SPI2SEL::PerCk
    }
}
///Field `SPI2SEL` writer - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI2SEL>;
impl<'a, REG> SPI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL::Pll1Q)
    }
    ///PLL2 QP clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL::Pll2P)
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL::Audioclk)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL::PerCk)
    }
}
/**SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI3SEL {
    ///0: PLL1 Q clock selected as clock source (pll1_q_ck)
    Pll1Q = 0,
    ///1: PLL2 P clock selected as clock source (pll2_p_ck)
    Pll2P = 1,
    ///3: AUDIOCLK clock selected as clock source
    Audioclk = 3,
    ///4: per_ck clock selected as clock source
    PerCk = 4,
}
impl From<SPI3SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI3SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI3SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI3SEL {}
///Field `SPI3SEL` reader - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI3SEL_R = crate::FieldReader<SPI3SEL>;
impl SPI3SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI3SEL> {
        match self.bits {
            0 => Some(SPI3SEL::Pll1Q),
            1 => Some(SPI3SEL::Pll2P),
            3 => Some(SPI3SEL::Audioclk),
            4 => Some(SPI3SEL::PerCk),
            _ => None,
        }
    }
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPI3SEL::Pll1Q
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SPI3SEL::Pll2P
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn is_audioclk(&self) -> bool {
        *self == SPI3SEL::Audioclk
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn is_per_ck(&self) -> bool {
        *self == SPI3SEL::PerCk
    }
}
///Field `SPI3SEL` writer - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI3SEL>;
impl<'a, REG> SPI3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL1 Q clock selected as clock source (pll1_q_ck)
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::Pll1Q)
    }
    ///PLL2 P clock selected as clock source (pll2_p_ck)
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::Pll2P)
    }
    ///AUDIOCLK clock selected as clock source
    #[inline(always)]
    pub fn audioclk(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::Audioclk)
    }
    ///per_ck clock selected as clock source
    #[inline(always)]
    pub fn per_ck(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL::PerCk)
    }
}
/**LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL {
    ///0: PCLK3 selected as clock source (rcc_pclk3)
    RccPclk3 = 0,
    ///1: PLL2 Q clock selected as clock source (pll2_q_ck)
    Pll2Q = 1,
    ///3: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 3,
    ///4: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 4,
    ///5: LSE selected as clock source (lse_ck)
    Lse = 5,
}
impl From<LPUART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPUART1SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPUART1SEL {}
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL>;
impl LPUART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPUART1SEL> {
        match self.bits {
            0 => Some(LPUART1SEL::RccPclk3),
            1 => Some(LPUART1SEL::Pll2Q),
            3 => Some(LPUART1SEL::HsiKer),
            4 => Some(LPUART1SEL::CsiKer),
            5 => Some(LPUART1SEL::Lse),
            _ => None,
        }
    }
    ///PCLK3 selected as clock source (rcc_pclk3)
    #[inline(always)]
    pub fn is_rcc_pclk3(&self) -> bool {
        *self == LPUART1SEL::RccPclk3
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == LPUART1SEL::Pll2Q
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == LPUART1SEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == LPUART1SEL::CsiKer
    }
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL::Lse
    }
}
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPUART1SEL>;
impl<'a, REG> LPUART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK3 selected as clock source (rcc_pclk3)
    #[inline(always)]
    pub fn rcc_pclk3(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::RccPclk3)
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Pll2Q)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::CsiKer)
    }
    ///LSE selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL::Lse)
    }
}
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
            .field("lpuart1sel", &self.lpuart1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<CCIPR3rs> {
        SPI1SEL_W::new(self, 0)
    }
    ///Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<CCIPR3rs> {
        SPI2SEL_W::new(self, 3)
    }
    ///Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<CCIPR3rs> {
        SPI3SEL_W::new(self, 6)
    }
    ///Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPR3rs> {
        LPUART1SEL_W::new(self, 24)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:CCIPR3)*/
pub struct CCIPR3rs;
impl crate::RegisterSpec for CCIPR3rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr3::R`](R) reader structure
impl crate::Readable for CCIPR3rs {}
///`write(|w| ..)` method takes [`ccipr3::W`](W) writer structure
impl crate::Writable for CCIPR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR3 to value 0
impl crate::Resettable for CCIPR3rs {
    const RESET_VALUE: u32 = 0;
}
