///Register `TIM1_AF1` reader
pub type R = crate::R<TIM1_AF1rs>;
///Register `TIM1_AF1` writer
pub type W = crate::W<TIM1_AF1rs>;
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
    ///0: BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    B0x0 = 0,
    ///1: BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)
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
    ///BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKINP::B0x0
    }
    ///BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)
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
    ///BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::B0x0)
    }
    ///BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::B0x1)
    }
}
/**ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL {
    ///0: ETR legacy mode
    B0x0 = 0,
    ///3: ADC1 AWD1
    B0x3 = 3,
    ///4: ADC1 AWD2
    B0x4 = 4,
    ///5: ADC1 AWD3
    B0x5 = 5,
}
impl From<ETRSEL> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRSEL {
    type Ux = u8;
}
impl crate::IsEnum for ETRSEL {}
///Field `ETRSEL` reader - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type ETRSEL_R = crate::FieldReader<ETRSEL>;
impl ETRSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL> {
        match self.bits {
            0 => Some(ETRSEL::B0x0),
            3 => Some(ETRSEL::B0x3),
            4 => Some(ETRSEL::B0x4),
            5 => Some(ETRSEL::B0x5),
            _ => None,
        }
    }
    ///ETR legacy mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETRSEL::B0x0
    }
    ///ADC1 AWD1
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETRSEL::B0x3
    }
    ///ADC1 AWD2
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == ETRSEL::B0x4
    }
    ///ADC1 AWD3
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == ETRSEL::B0x5
    }
}
///Field `ETRSEL` writer - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ETR legacy mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x0)
    }
    ///ADC1 AWD1
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x3)
    }
    ///ADC1 AWD2
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x4)
    }
    ///ADC1 AWD3
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::B0x5)
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
    ///Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_AF1")
            .field("bkine", &self.bkine())
            .field("bkinp", &self.bkinp())
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timer s BRK input. BKIN input is ORed with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W<'_, TIM1_AF1rs> {
        BKINE_W::new(self, 0)
    }
    ///Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W<'_, TIM1_AF1rs> {
        BKINP_W::new(self, 9)
    }
    ///Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<'_, TIM1_AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM1 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM1:TIM1_AF1)*/
pub struct TIM1_AF1rs;
impl crate::RegisterSpec for TIM1_AF1rs {
    type Ux = u32;
}
///`read()` method returns [`tim1_af1::R`](R) reader structure
impl crate::Readable for TIM1_AF1rs {}
///`write(|w| ..)` method takes [`tim1_af1::W`](W) writer structure
impl crate::Writable for TIM1_AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_AF1 to value 0x01
impl crate::Resettable for TIM1_AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
