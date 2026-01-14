///Register `HSECR` reader
pub type R = crate::R<HSECRrs>;
///Register `HSECR` writer
pub type W = crate::W<HSECRrs>;
///Field `UNLOCKED` reader - Register lock system
pub type UNLOCKED_R = crate::BitReader;
///Field `UNLOCKED` writer - Register lock system
pub type UNLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
/**HSE Sense amplifier threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSES {
    ///0: HSE bias current factor 1/2
    OneHalf = 0,
    ///1: HSE bias current factor 3/4
    ThreeQuarter = 1,
}
impl From<HSES> for bool {
    #[inline(always)]
    fn from(variant: HSES) -> Self {
        variant as u8 != 0
    }
}
///Field `HSES` reader - HSE Sense amplifier threshold
pub type HSES_R = crate::BitReader<HSES>;
impl HSES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSES {
        match self.bits {
            false => HSES::OneHalf,
            true => HSES::ThreeQuarter,
        }
    }
    ///HSE bias current factor 1/2
    #[inline(always)]
    pub fn is_one_half(&self) -> bool {
        *self == HSES::OneHalf
    }
    ///HSE bias current factor 3/4
    #[inline(always)]
    pub fn is_three_quarter(&self) -> bool {
        *self == HSES::ThreeQuarter
    }
}
///Field `HSES` writer - HSE Sense amplifier threshold
pub type HSES_W<'a, REG> = crate::BitWriter<'a, REG, HSES>;
impl<'a, REG> HSES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE bias current factor 1/2
    #[inline(always)]
    pub fn one_half(self) -> &'a mut crate::W<REG> {
        self.variant(HSES::OneHalf)
    }
    ///HSE bias current factor 3/4
    #[inline(always)]
    pub fn three_quarter(self) -> &'a mut crate::W<REG> {
        self.variant(HSES::ThreeQuarter)
    }
}
/**HSE current control

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSEGMC {
    ///0: Current max limit 0.18 mA/V
    Max0_18 = 0,
    ///1: Current max limit 0.57 mA/V
    Max0_57 = 1,
    ///2: Current max limit 0.78 mA/V
    Max0_78 = 2,
    ///3: Current max limit 1.13 mA/V
    Max1_13 = 3,
    ///4: Current max limit 0.61 mA/V
    Max0_61 = 4,
    ///5: Current max limit 1.65 mA/V
    Max1_65 = 5,
    ///6: Current max limit 2.12 mA/V
    Max2_12 = 6,
    ///7: Current max limit 2.84 mA/V
    Max2_84 = 7,
}
impl From<HSEGMC> for u8 {
    #[inline(always)]
    fn from(variant: HSEGMC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSEGMC {
    type Ux = u8;
}
impl crate::IsEnum for HSEGMC {}
///Field `HSEGMC` reader - HSE current control
pub type HSEGMC_R = crate::FieldReader<HSEGMC>;
impl HSEGMC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEGMC {
        match self.bits {
            0 => HSEGMC::Max0_18,
            1 => HSEGMC::Max0_57,
            2 => HSEGMC::Max0_78,
            3 => HSEGMC::Max1_13,
            4 => HSEGMC::Max0_61,
            5 => HSEGMC::Max1_65,
            6 => HSEGMC::Max2_12,
            7 => HSEGMC::Max2_84,
            _ => unreachable!(),
        }
    }
    ///Current max limit 0.18 mA/V
    #[inline(always)]
    pub fn is_max0_18(&self) -> bool {
        *self == HSEGMC::Max0_18
    }
    ///Current max limit 0.57 mA/V
    #[inline(always)]
    pub fn is_max0_57(&self) -> bool {
        *self == HSEGMC::Max0_57
    }
    ///Current max limit 0.78 mA/V
    #[inline(always)]
    pub fn is_max0_78(&self) -> bool {
        *self == HSEGMC::Max0_78
    }
    ///Current max limit 1.13 mA/V
    #[inline(always)]
    pub fn is_max1_13(&self) -> bool {
        *self == HSEGMC::Max1_13
    }
    ///Current max limit 0.61 mA/V
    #[inline(always)]
    pub fn is_max0_61(&self) -> bool {
        *self == HSEGMC::Max0_61
    }
    ///Current max limit 1.65 mA/V
    #[inline(always)]
    pub fn is_max1_65(&self) -> bool {
        *self == HSEGMC::Max1_65
    }
    ///Current max limit 2.12 mA/V
    #[inline(always)]
    pub fn is_max2_12(&self) -> bool {
        *self == HSEGMC::Max2_12
    }
    ///Current max limit 2.84 mA/V
    #[inline(always)]
    pub fn is_max2_84(&self) -> bool {
        *self == HSEGMC::Max2_84
    }
}
///Field `HSEGMC` writer - HSE current control
pub type HSEGMC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, HSEGMC, crate::Safe>;
impl<'a, REG> HSEGMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Current max limit 0.18 mA/V
    #[inline(always)]
    pub fn max0_18(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max0_18)
    }
    ///Current max limit 0.57 mA/V
    #[inline(always)]
    pub fn max0_57(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max0_57)
    }
    ///Current max limit 0.78 mA/V
    #[inline(always)]
    pub fn max0_78(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max0_78)
    }
    ///Current max limit 1.13 mA/V
    #[inline(always)]
    pub fn max1_13(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max1_13)
    }
    ///Current max limit 0.61 mA/V
    #[inline(always)]
    pub fn max0_61(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max0_61)
    }
    ///Current max limit 1.65 mA/V
    #[inline(always)]
    pub fn max1_65(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max1_65)
    }
    ///Current max limit 2.12 mA/V
    #[inline(always)]
    pub fn max2_12(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max2_12)
    }
    ///Current max limit 2.84 mA/V
    #[inline(always)]
    pub fn max2_84(self) -> &'a mut crate::W<REG> {
        self.variant(HSEGMC::Max2_84)
    }
}
///Field `HSETUNE` reader - HSE capacitor tuning
pub type HSETUNE_R = crate::FieldReader;
///Field `HSETUNE` writer - HSE capacitor tuning
pub type HSETUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Register lock system
    #[inline(always)]
    pub fn unlocked(&self) -> UNLOCKED_R {
        UNLOCKED_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - HSE Sense amplifier threshold
    #[inline(always)]
    pub fn hses(&self) -> HSES_R {
        HSES_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - HSE current control
    #[inline(always)]
    pub fn hsegmc(&self) -> HSEGMC_R {
        HSEGMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:13 - HSE capacitor tuning
    #[inline(always)]
    pub fn hsetune(&self) -> HSETUNE_R {
        HSETUNE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSECR")
            .field("hsetune", &self.hsetune())
            .field("hsegmc", &self.hsegmc())
            .field("hses", &self.hses())
            .field("unlocked", &self.unlocked())
            .finish()
    }
}
impl W {
    ///Bit 0 - Register lock system
    #[inline(always)]
    pub fn unlocked(&mut self) -> UNLOCKED_W<'_, HSECRrs> {
        UNLOCKED_W::new(self, 0)
    }
    ///Bit 3 - HSE Sense amplifier threshold
    #[inline(always)]
    pub fn hses(&mut self) -> HSES_W<'_, HSECRrs> {
        HSES_W::new(self, 3)
    }
    ///Bits 4:6 - HSE current control
    #[inline(always)]
    pub fn hsegmc(&mut self) -> HSEGMC_W<'_, HSECRrs> {
        HSEGMC_W::new(self, 4)
    }
    ///Bits 8:13 - HSE capacitor tuning
    #[inline(always)]
    pub fn hsetune(&mut self) -> HSETUNE_W<'_, HSECRrs> {
        HSETUNE_W::new(self, 8)
    }
}
/**Clock HSE register

You can [`read`](crate::Reg::read) this register and get [`hsecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:HSECR)*/
pub struct HSECRrs;
impl crate::RegisterSpec for HSECRrs {
    type Ux = u32;
}
///`read()` method returns [`hsecr::R`](R) reader structure
impl crate::Readable for HSECRrs {}
///`write(|w| ..)` method takes [`hsecr::W`](W) writer structure
impl crate::Writable for HSECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HSECR to value 0x30
impl crate::Resettable for HSECRrs {
    const RESET_VALUE: u32 = 0x30;
}
