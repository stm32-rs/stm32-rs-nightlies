///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
/**USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USARTSEL {
    ///0: Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    Pclk = 0,
    ///1: PLL2 Q clock selected as clock source (pll2_q_ck)
    Pll2Q = 1,
    ///3: HSI kernel clock selected as clock source (hsi_ker_ck)
    HsiKer = 3,
    ///4: CSI kernel clock selected as clock source (csi_ker_ck)
    CsiKer = 4,
    ///5: LSE clock selected as clock source (lse_ck)
    Lse = 5,
}
impl From<USARTSEL> for u8 {
    #[inline(always)]
    fn from(variant: USARTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USARTSEL {
    type Ux = u8;
}
impl crate::IsEnum for USARTSEL {}
///Field `USART1SEL` reader - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART1SEL_R = crate::FieldReader<USARTSEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USARTSEL> {
        match self.bits {
            0 => Some(USARTSEL::Pclk),
            1 => Some(USARTSEL::Pll2Q),
            3 => Some(USARTSEL::HsiKer),
            4 => Some(USARTSEL::CsiKer),
            5 => Some(USARTSEL::Lse),
            _ => None,
        }
    }
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USARTSEL::Pclk
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USARTSEL::Pll2Q
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USARTSEL::HsiKer
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USARTSEL::CsiKer
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USARTSEL::Lse
    }
}
///Field `USART1SEL` writer - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USARTSEL>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Peripheral bus clock used as selected as clock source (rcc_pclk_x)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(USARTSEL::Pclk)
    }
    ///PLL2 Q clock selected as clock source (pll2_q_ck)
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USARTSEL::Pll2Q)
    }
    ///HSI kernel clock selected as clock source (hsi_ker_ck)
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USARTSEL::HsiKer)
    }
    ///CSI kernel clock selected as clock source (csi_ker_ck)
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USARTSEL::CsiKer)
    }
    ///LSE clock selected as clock source (lse_ck)
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USARTSEL::Lse)
    }
}
///Field `USART2SEL` reader - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use USART1SEL_R as USART2SEL_R;
///Field `USART3SEL` reader - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use USART1SEL_R as USART3SEL_R;
///Field `USART2SEL` writer - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use USART1SEL_W as USART2SEL_W;
///Field `USART3SEL` writer - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub use USART1SEL_W as USART3SEL_W;
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
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<'_, CCIPR1rs> {
        USART2SEL_W::new(self, 3)
    }
    ///Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W<'_, CCIPR1rs> {
        USART3SEL_W::new(self, 6)
    }
    ///Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.
    #[inline(always)]
    pub fn timicsel(&mut self) -> TIMICSEL_W<'_, CCIPR1rs> {
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
}
///`reset()` method sets CCIPR1 to value 0
impl crate::Resettable for CCIPR1rs {}
