///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Internal High Speed clock enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: Clock Off
    Off = 0,
    ///1: Clock On
    On = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
///Field `HSION` reader - Internal High Speed clock enable
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Off,
            true => HSION::On,
        }
    }
    ///Clock Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION::Off
    }
    ///Clock On
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION::On
    }
}
///Field `HSION` writer - Internal High Speed clock enable
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Off)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::On)
    }
}
/**Internal High Speed clock ready flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    ///0: Clock not ready
    NotReady = 0,
    ///1: Clock ready
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDY` reader - Internal High Speed clock ready flag
pub type HSIRDY_R = crate::BitReader<HSIRDYR>;
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYR {
        match self.bits {
            false => HSIRDYR::NotReady,
            true => HSIRDYR::Ready,
        }
    }
    ///Clock not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    ///Clock ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
///Field `HSITRIM` reader - Internal High Speed clock trimming
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - Internal High Speed clock trimming
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `HSICAL` reader - Internal High Speed clock Calibration
pub type HSICAL_R = crate::FieldReader;
///Field `HSEON` reader - External High Speed clock enable
pub use HSION_R as HSEON_R;
///Field `HSEON` writer - External High Speed clock enable
pub use HSION_W as HSEON_W;
///Field `HSERDY` reader - External High Speed clock ready flag
pub use HSIRDY_R as HSERDY_R;
/**External High Speed clock Bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    ///0: HSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEBYP` reader - External High Speed clock Bypass
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::NotBypassed,
            true => HSEBYP::Bypassed,
        }
    }
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
///Field `HSEBYP` writer - External High Speed clock Bypass
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
/**Clock Security System enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    ///0: Clock security system disabled (clock detector OFF)
    Off = 0,
    ///1: Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    On = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` reader - Clock Security System enable
pub type CSSON_R = crate::BitReader<CSSON>;
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::Off,
            true => CSSON::On,
        }
    }
    ///Clock security system disabled (clock detector OFF)
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CSSON::Off
    }
    ///Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CSSON::On
    }
}
///Field `CSSON` writer - Clock Security System enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock security system disabled (clock detector OFF)
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Off)
    }
    ///Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::On)
    }
}
///Field `PLLON` reader - PLL enable
pub use HSION_R as PLLON_R;
///Field `PLLON` writer - PLL enable
pub use HSION_W as PLLON_W;
///Field `PLLRDY` reader - PLL clock ready flag
pub use HSIRDY_R as PLLRDY_R;
impl R {
    ///Bit 0 - Internal High Speed clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal High Speed clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 3:7 - Internal High Speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:15 - Internal High Speed clock Calibration
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - External High Speed clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - External High Speed clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - External High Speed clock Bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock Security System enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("hsion", &self.hsion())
            .field("hsirdy", &self.hsirdy())
            .field("hsitrim", &self.hsitrim())
            .field("hsical", &self.hsical())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsebyp", &self.hsebyp())
            .field("csson", &self.csson())
            .field("pllon", &self.pllon())
            .field("pllrdy", &self.pllrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal High Speed clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 0)
    }
    ///Bits 3:7 - Internal High Speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<CRrs> {
        HSITRIM_W::new(self, 3)
    }
    ///Bit 16 - External High Speed clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - External High Speed clock Bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock Security System enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<CRrs> {
        PLLON_W::new(self, 24)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x83
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x83;
}
