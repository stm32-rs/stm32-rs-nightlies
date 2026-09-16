///Register `TIM16_AF1` reader
pub type R = crate::R<TIM16_AF1rs>;
///Register `TIM16_AF1` writer
pub type W = crate::W<TIM16_AF1rs>;
/**BRK BKIN input enable This bit enables the BKIN alternate function input for the timer s BRK input. BKIN input is ORed with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINE {
    ///0: BKIN input disabled
    B0x0 = 0,
    ///1: BKIN input enabled
    B0x1 = 1,
}
impl From<BKINE> for bool {
    #[inline(always)]
    fn from(variant: BKINE) -> Self {
        variant as u8 != 0
    }
}
///Field `BKINE` reader - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer s BRK input. BKIN input is ORed with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BKINE_R = crate::BitReader<BKINE>;
impl BKINE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKINE {
        match self.bits {
            false => BKINE::B0x0,
            true => BKINE::B0x1,
        }
    }
    ///BKIN input disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKINE::B0x0
    }
    ///BKIN input enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKINE::B0x1
    }
}
///Field `BKINE` writer - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer s BRK input. BKIN input is ORed with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG, BKINE>;
impl<'a, REG> BKINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BKIN input disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE::B0x0)
    }
    ///BKIN input enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE::B0x1)
    }
}
/**BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINP {
    ///0: BKIN input is active low
    B0x0 = 0,
    ///1: BKIN input is active high
    B0x1 = 1,
}
impl From<BKINP> for bool {
    #[inline(always)]
    fn from(variant: BKINP) -> Self {
        variant as u8 != 0
    }
}
///Field `BKINP` reader - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BKINP_R = crate::BitReader<BKINP>;
impl BKINP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKINP {
        match self.bits {
            false => BKINP::B0x0,
            true => BKINP::B0x1,
        }
    }
    ///BKIN input is active low
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKINP::B0x0
    }
    ///BKIN input is active high
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKINP::B0x1
    }
}
///Field `BKINP` writer - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG, BKINP>;
impl<'a, REG> BKINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BKIN input is active low
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::B0x0)
    }
    ///BKIN input is active high
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::B0x1)
    }
}
impl R {
    ///Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer s BRK input. BKIN input is ORed with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16_AF1")
            .field("bkine", &self.bkine())
            .field("bkinp", &self.bkinp())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer s BRK input. BKIN input is ORed with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W<'_, TIM16_AF1rs> {
        BKINE_W::new(self, 0)
    }
    ///Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W<'_, TIM16_AF1rs> {
        BKINP_W::new(self, 9)
    }
}
/**TIM16 alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`tim16_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM16:TIM16_AF1)*/
pub struct TIM16_AF1rs;
impl crate::RegisterSpec for TIM16_AF1rs {
    type Ux = u32;
}
///`read()` method returns [`tim16_af1::R`](R) reader structure
impl crate::Readable for TIM16_AF1rs {}
///`write(|w| ..)` method takes [`tim16_af1::W`](W) writer structure
impl crate::Writable for TIM16_AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM16_AF1 to value 0x01
impl crate::Resettable for TIM16_AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
