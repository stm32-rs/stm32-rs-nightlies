///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
/**USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    ///0: PCLK2 selected as clock source (rcc_pclk2)
    RccPclk2 = 0,
    ///1: PLL2 Q clock selected as clock source (pll2_q_ck)
    Pll2Q = 1,
    ///3: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 3,
    ///4: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 4,
    ///5: LSE clock selected as clock source (lse_ck)
    Lse = 5,
}
impl From<USART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART1SEL {}
///Field `USART1SEL` reader - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART1SEL> {
        match self.bits {
            0 => Some(USART1SEL::RccPclk2),
            1 => Some(USART1SEL::Pll2Q),
            3 => Some(USART1SEL::HsiKer),
            4 => Some(USART1SEL::CsiKer),
            5 => Some(USART1SEL::Lse),
            _ => None,
        }
    }
    ///PCLK2 selected as clock source (rcc_pclk2)
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == USART1SEL::RccPclk2
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART1SEL::Pll2Q
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART1SEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART1SEL::CsiKer
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
///Field `USART1SEL` writer - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART1SEL>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK2 selected as clock source (rcc_pclk2)
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::RccPclk2)
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pll2Q)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::CsiKer)
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
/**USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2SEL {
    ///0: PCLK1 selected as clock source (rcc_pclk1)
    RccPclk1 = 0,
    ///1: PLL2 Q clock selected as clock source (pll2_q_ck)
    Pll2Q = 1,
    ///3: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 3,
    ///4: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 4,
    ///5: LSE clock selected as clock source (lse_ck)
    Lse = 5,
}
impl From<USART2SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART2SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART2SEL {}
///Field `USART2SEL` reader - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART2SEL_R = crate::FieldReader<USART2SEL>;
impl USART2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART2SEL> {
        match self.bits {
            0 => Some(USART2SEL::RccPclk1),
            1 => Some(USART2SEL::Pll2Q),
            3 => Some(USART2SEL::HsiKer),
            4 => Some(USART2SEL::CsiKer),
            5 => Some(USART2SEL::Lse),
            _ => None,
        }
    }
    ///PCLK1 selected as clock source (rcc_pclk1)
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART2SEL::RccPclk1
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART2SEL::Pll2Q
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART2SEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART2SEL::CsiKer
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART2SEL::Lse
    }
}
///Field `USART2SEL` writer - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART2SEL>;
impl<'a, REG> USART2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK1 selected as clock source (rcc_pclk1)
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::RccPclk1)
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Pll2Q)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::CsiKer)
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL::Lse)
    }
}
/**USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART3SEL {
    ///0: PCLK1 selected as clock source (rcc_pclk1)
    RccPclk1 = 0,
    ///1: PLL2 Q clock selected as clock source (pll2_q_ck)
    Pll2Q = 1,
    ///3: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 3,
    ///4: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 4,
    ///5: LSE clock selected as clock source (lse_ck)
    Lse = 5,
}
impl From<USART3SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART3SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART3SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART3SEL {}
///Field `USART3SEL` reader - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART3SEL_R = crate::FieldReader<USART3SEL>;
impl USART3SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART3SEL> {
        match self.bits {
            0 => Some(USART3SEL::RccPclk1),
            1 => Some(USART3SEL::Pll2Q),
            3 => Some(USART3SEL::HsiKer),
            4 => Some(USART3SEL::CsiKer),
            5 => Some(USART3SEL::Lse),
            _ => None,
        }
    }
    ///PCLK1 selected as clock source (rcc_pclk1)
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART3SEL::RccPclk1
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART3SEL::Pll2Q
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART3SEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART3SEL::CsiKer
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART3SEL::Lse
    }
}
///Field `USART3SEL` writer - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART3SEL>;
impl<'a, REG> USART3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK1 selected as clock source (rcc_pclk1)
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::RccPclk1)
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::Pll2Q)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::CsiKer)
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL::Lse)
    }
}
/**TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMICSEL {
    ///0: No internal clock available for timers input capture
    Disabled = 0,
    ///1: hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture
    Enabled = 1,
}
impl From<TIMICSEL> for bool {
    #[inline(always)]
    fn from(variant: TIMICSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMICSEL` reader - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.
pub type TIMICSEL_R = crate::BitReader<TIMICSEL>;
impl TIMICSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMICSEL {
        match self.bits {
            false => TIMICSEL::Disabled,
            true => TIMICSEL::Enabled,
        }
    }
    ///No internal clock available for timers input capture
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMICSEL::Disabled
    }
    ///hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMICSEL::Enabled
    }
}
///Field `TIMICSEL` writer - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.
pub type TIMICSEL_W<'a, REG> = crate::BitWriter<'a, REG, TIMICSEL>;
impl<'a, REG> TIMICSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No internal clock available for timers input capture
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::Disabled)
    }
    ///hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL::Enabled)
    }
}
impl R {
    ///Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR1")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("usart3sel", &self.usart3sel())
            .field("timicsel", &self.timicsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPR1rs> {
        USART2SEL_W::new(self, 3)
    }
    ///Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<CCIPR1rs> {
        USART3SEL_W::new(self, 6)
    }
    ///Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn timicsel(&mut self) -> TIMICSEL_W<CCIPR1rs> {
        TIMICSEL_W::new(self, 31)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:CCIPR1)*/
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr1::R`](R) reader structure
impl crate::Readable for CCIPR1rs {}
///`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR1 to value 0
impl crate::Resettable for CCIPR1rs {
    const RESET_VALUE: u32 = 0;
}
