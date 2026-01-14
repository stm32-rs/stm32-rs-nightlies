///Register `TIM1_AF2` reader
pub type R = crate::R<TIM1_AF2rs>;
///Register `TIM1_AF2` writer
pub type W = crate::W<TIM1_AF2rs>;
/**BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer s BRK2 input. BKIN2 input is ORed with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INE {
    ///0: BKIN2 input disabled
    B0x0 = 0,
    ///1: BKIN2 input enabled
    B0x1 = 1,
}
impl From<BK2INE> for bool {
    #[inline(always)]
    fn from(variant: BK2INE) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2INE` reader - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer s BRK2 input. BKIN2 input is ORed with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INE_R = crate::BitReader<BK2INE>;
impl BK2INE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2INE {
        match self.bits {
            false => BK2INE::B0x0,
            true => BK2INE::B0x1,
        }
    }
    ///BKIN2 input disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2INE::B0x0
    }
    ///BKIN2 input enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2INE::B0x1
    }
}
///Field `BK2INE` writer - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer s BRK2 input. BKIN2 input is ORed with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG, BK2INE>;
impl<'a, REG> BK2INE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BKIN2 input disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::B0x0)
    }
    ///BKIN2 input enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::B0x1)
    }
}
/**BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INP {
    ///0: BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)
    B0x0 = 0,
    ///1: BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)
    B0x1 = 1,
}
impl From<BK2INP> for bool {
    #[inline(always)]
    fn from(variant: BK2INP) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2INP` reader - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INP_R = crate::BitReader<BK2INP>;
impl BK2INP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2INP {
        match self.bits {
            false => BK2INP::B0x0,
            true => BK2INP::B0x1,
        }
    }
    ///BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2INP::B0x0
    }
    ///BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2INP::B0x1
    }
}
///Field `BK2INP` writer - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG, BK2INP>;
impl<'a, REG> BK2INP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::B0x0)
    }
    ///BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::B0x1)
    }
}
impl R {
    ///Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer s BRK2 input. BKIN2 input is ORed with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_AF2")
            .field("bk2ine", &self.bk2ine())
            .field("bk2inp", &self.bk2inp())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer s BRK2 input. BKIN2 input is ORed with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W<'_, TIM1_AF2rs> {
        BK2INE_W::new(self, 0)
    }
    ///Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W<'_, TIM1_AF2rs> {
        BK2INP_W::new(self, 9)
    }
}
/**TIM1 Alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM1:TIM1_AF2)*/
pub struct TIM1_AF2rs;
impl crate::RegisterSpec for TIM1_AF2rs {
    type Ux = u32;
}
///`read()` method returns [`tim1_af2::R`](R) reader structure
impl crate::Readable for TIM1_AF2rs {}
///`write(|w| ..)` method takes [`tim1_af2::W`](W) writer structure
impl crate::Writable for TIM1_AF2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_AF2 to value 0x01
impl crate::Resettable for TIM1_AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
