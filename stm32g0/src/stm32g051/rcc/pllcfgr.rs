///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
/**PLL input clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC {
    ///0: No clock selected (saves power)
    NoClock = 0,
    ///2: HSI16 clock selected
    Hsi16 = 2,
    ///3: HSE clock selected
    Hse = 3,
}
impl From<PLLSRC> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSRC {
    type Ux = u8;
}
impl crate::IsEnum for PLLSRC {}
///Field `PLLSRC` reader - PLL input clock source
pub type PLLSRC_R = crate::FieldReader<PLLSRC>;
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLSRC> {
        match self.bits {
            0 => Some(PLLSRC::NoClock),
            2 => Some(PLLSRC::Hsi16),
            3 => Some(PLLSRC::Hse),
            _ => None,
        }
    }
    ///No clock selected (saves power)
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PLLSRC::NoClock
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC::Hsi16
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
    }
}
///Field `PLLSRC` writer - PLL input clock source
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock selected (saves power)
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::NoClock)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi16)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
}
///Field `PLLM` reader - Division factor M of the PLL input clock divider
pub type PLLM_R = crate::FieldReader;
///Field `PLLM` writer - Division factor M of the PLL input clock divider
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Field `PLLN` reader - PLL frequency multiplication factor N
pub type PLLN_R = crate::FieldReader;
///Field `PLLN` writer - PLL frequency multiplication factor N
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**PLLPCLK clock output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPEN {
    ///0: PLL output disabled (saves power)
    Disabled = 0,
    ///1: PLL output enabled
    Enabled = 1,
}
impl From<PLLPEN> for bool {
    #[inline(always)]
    fn from(variant: PLLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLPEN` reader - PLLPCLK clock output enable
pub type PLLPEN_R = crate::BitReader<PLLPEN>;
impl PLLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLPEN {
        match self.bits {
            false => PLLPEN::Disabled,
            true => PLLPEN::Enabled,
        }
    }
    ///PLL output disabled (saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLPEN::Disabled
    }
    ///PLL output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLPEN::Enabled
    }
}
///Field `PLLPEN` writer - PLLPCLK clock output enable
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLPEN>;
impl<'a, REG> PLLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL output disabled (saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Disabled)
    }
    ///PLL output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPEN::Enabled)
    }
}
///Field `PLLP` reader - PLL VCO division factor P for PLLPCLK clock output
pub type PLLP_R = crate::FieldReader;
///Field `PLLP` writer - PLL VCO division factor P for PLLPCLK clock output
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PLLQEN` reader - PLLQCLK clock output enable
pub use PLLPEN_R as PLLQEN_R;
///Field `PLLQEN` writer - PLLQCLK clock output enable
pub use PLLPEN_W as PLLQEN_W;
///Field `PLLQ` reader - PLL VCO division factor Q for PLLQCLK clock output
pub type PLLQ_R = crate::FieldReader;
///Field `PLLQ` writer - PLL VCO division factor Q for PLLQCLK clock output
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLLREN` reader - PLLRCLK clock output enable
pub use PLLPEN_R as PLLREN_R;
///Field `PLLREN` writer - PLLRCLK clock output enable
pub use PLLPEN_W as PLLREN_W;
///Field `PLLR` reader - PLL VCO division factor R for PLLRCLK clock output
pub type PLLR_R = crate::FieldReader;
///Field `PLLR` writer - PLL VCO division factor R for PLLRCLK clock output
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:1 - PLL input clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Division factor M of the PLL input clock divider
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:14 - PLL frequency multiplication factor N
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - PLLPCLK clock output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - PLLQCLK clock output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - PLLRCLK clock output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pllsrc", &self.pllsrc())
            .field("pllm", &self.pllm())
            .field("plln", &self.plln())
            .field("pllpen", &self.pllpen())
            .field("pllp", &self.pllp())
            .field("pllqen", &self.pllqen())
            .field("pllq", &self.pllq())
            .field("pllren", &self.pllren())
            .field("pllr", &self.pllr())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL input clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<'_, PLLCFGRrs> {
        PLLSRC_W::new(self, 0)
    }
    ///Bits 4:6 - Division factor M of the PLL input clock divider
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W<'_, PLLCFGRrs> {
        PLLM_W::new(self, 4)
    }
    ///Bits 8:14 - PLL frequency multiplication factor N
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W<'_, PLLCFGRrs> {
        PLLN_W::new(self, 8)
    }
    ///Bit 16 - PLLPCLK clock output enable
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W<'_, PLLCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    ///Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W<'_, PLLCFGRrs> {
        PLLP_W::new(self, 17)
    }
    ///Bit 24 - PLLQCLK clock output enable
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W<'_, PLLCFGRrs> {
        PLLQEN_W::new(self, 24)
    }
    ///Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<'_, PLLCFGRrs> {
        PLLQ_W::new(self, 25)
    }
    ///Bit 28 - PLLRCLK clock output enable
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W<'_, PLLCFGRrs> {
        PLLREN_W::new(self, 28)
    }
    ///Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W<'_, PLLCFGRrs> {
        PLLR_W::new(self, 29)
    }
}
/**PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#RCC:PLLCFGR)*/
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
///`reset()` method sets PLLCFGR to value 0x1000
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
