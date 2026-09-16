///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
///Field `PLLM` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM_R = crate::FieldReader;
///Field `PLLM` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PLLN` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN_R = crate::FieldReader<u16>;
///Field `PLLN` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
/**Main PLL (PLL) division factor for main system clock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP {
    ///0: PLLP=2
    Div2 = 0,
    ///1: PLLP=4
    Div4 = 1,
    ///2: PLLP=6
    Div6 = 2,
    ///3: PLLP=8
    Div8 = 3,
}
impl From<PLLP> for u8 {
    #[inline(always)]
    fn from(variant: PLLP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLP {
    type Ux = u8;
}
impl crate::IsEnum for PLLP {}
///Field `PLLP` reader - Main PLL (PLL) division factor for main system clock
pub type PLLP_R = crate::FieldReader<PLLP>;
impl PLLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLP {
        match self.bits {
            0 => PLLP::Div2,
            1 => PLLP::Div4,
            2 => PLLP::Div6,
            3 => PLLP::Div8,
            _ => unreachable!(),
        }
    }
    ///PLLP=2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLP::Div2
    }
    ///PLLP=4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLP::Div4
    }
    ///PLLP=6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLP::Div6
    }
    ///PLLP=8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLP::Div8
    }
}
///Field `PLLP` writer - Main PLL (PLL) division factor for main system clock
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLP, crate::Safe>;
impl<'a, REG> PLLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLP=2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div2)
    }
    ///PLLP=4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div4)
    }
    ///PLLP=6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div6)
    }
    ///PLLP=8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div8)
    }
}
/**Main PLL(PLL) and audio PLL (PLLI2S) entry clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC {
    ///0: HSI clock selected as PLL and PLLI2S clock entry
    Hsi = 0,
    ///1: HSE oscillator clock selected as PLL and PLLI2S clock entry
    Hse = 1,
}
impl From<PLLSRC> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSRC` reader - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
pub type PLLSRC_R = crate::BitReader<PLLSRC>;
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSRC {
        match self.bits {
            false => PLLSRC::Hsi,
            true => PLLSRC::Hse,
        }
    }
    ///HSI clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC::Hsi
    }
    ///HSE oscillator clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
    }
}
///Field `PLLSRC` writer - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi)
    }
    ///HSE oscillator clock selected as PLL and PLLI2S clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
}
///Field `PLLQ` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ_R = crate::FieldReader;
///Field `PLLQ` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLLR` reader - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
pub type PLLR_R = crate::FieldReader;
///Field `PLLR` writer - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    ///Bits 16:17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pllq", &self.pllq())
            .field("pllsrc", &self.pllsrc())
            .field("pllp", &self.pllp())
            .field("plln", &self.plln())
            .field("pllm", &self.pllm())
            .field("pllr", &self.pllr())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W<'_, PLLCFGRrs> {
        PLLM_W::new(self, 0)
    }
    ///Bits 6:14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W<'_, PLLCFGRrs> {
        PLLN_W::new(self, 6)
    }
    ///Bits 16:17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W<'_, PLLCFGRrs> {
        PLLP_W::new(self, 16)
    }
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<'_, PLLCFGRrs> {
        PLLSRC_W::new(self, 22)
    }
    ///Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<'_, PLLCFGRrs> {
        PLLQ_W::new(self, 24)
    }
    ///Bits 28:30 - Main PLL division factor for I2Ss, SAIs, SYSTEM and SPDIF-Rx clocks
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W<'_, PLLCFGRrs> {
        PLLR_W::new(self, 28)
    }
}
/**PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#RCC:PLLCFGR)*/
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllcfgr::R`](R) reader structure
impl crate::Readable for PLLCFGRrs {}
///`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCFGR to value 0x2400_3010
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x2400_3010;
}
