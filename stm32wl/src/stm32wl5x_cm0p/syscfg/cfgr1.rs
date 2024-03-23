#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "I/O analog switch voltage booster enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTEN {
    #[doc = "0: I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation"]
    Disabled = 0,
    #[doc = "1: I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation"]
    Enabled = 1,
}
impl From<BOOSTEN> for bool {
    #[inline(always)]
    fn from(variant: BOOSTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable"]
pub type BOOSTEN_R = crate::BitReader<BOOSTEN>;
impl BOOSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTEN {
        match self.bits {
            false => BOOSTEN::Disabled,
            true => BOOSTEN::Enabled,
        }
    }
    #[doc = "I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOSTEN::Disabled
    }
    #[doc = "I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOSTEN::Enabled
    }
}
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable"]
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTEN>;
impl<'a, REG> BOOSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Disabled)
    }
    #[doc = "I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTEN::Enabled)
    }
}
#[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP {
    #[doc = "0: PB6 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB6_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP>;
impl I2C_PB6_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB6_FMP {
        match self.bits {
            false => I2C_PB6_FMP::Standard,
            true => I2C_PB6_FMP::Fmp,
        }
    }
    #[doc = "PB6 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB6_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB6_FMP>;
impl<'a, REG> I2C_PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB6 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Fmp)
    }
}
#[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP {
    #[doc = "0: PB7 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB7_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB7_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP>;
impl I2C_PB7_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB7_FMP {
        match self.bits {
            false => I2C_PB7_FMP::Standard,
            true => I2C_PB7_FMP::Fmp,
        }
    }
    #[doc = "PB7 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB7_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB7_FMP>;
impl<'a, REG> I2C_PB7_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB7 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Fmp)
    }
}
#[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP {
    #[doc = "0: PB8 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB8_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB8_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP>;
impl I2C_PB8_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB8_FMP {
        match self.bits {
            false => I2C_PB8_FMP::Standard,
            true => I2C_PB8_FMP::Fmp,
        }
    }
    #[doc = "PB8 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB8_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB8_FMP>;
impl<'a, REG> I2C_PB8_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB8 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Fmp)
    }
}
#[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP {
    #[doc = "0: PB9 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB9_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB9_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP>;
impl I2C_PB9_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB9_FMP {
        match self.bits {
            false => I2C_PB9_FMP::Standard,
            true => I2C_PB9_FMP::Fmp,
        }
    }
    #[doc = "PB9 pin operate in standard mode"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP::Standard
    }
    #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP::Fmp
    }
}
#[doc = "Field `I2C_PB9_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB9_FMP>;
impl<'a, REG> I2C_PB9_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB9 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Fmp)
    }
}
#[doc = "I2C1 Fast-mode Plus driving capability activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
    Fmp = 1,
}
impl From<I2C1_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation"]
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP>;
impl I2C1_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1_FMP {
        match self.bits {
            false => I2C1_FMP::Standard,
            true => I2C1_FMP::Fmp,
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP::Standard
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP::Fmp
    }
}
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation"]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C1_FMP>;
impl<'a, REG> I2C1_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Fmp)
    }
}
#[doc = "I2C2 Fast-mode Plus driving capability activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_FMP {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
    Fmp = 1,
}
impl From<I2C2_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast-mode Plus driving capability activation"]
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP>;
impl I2C2_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2_FMP {
        match self.bits {
            false => I2C2_FMP::Standard,
            true => I2C2_FMP::Fmp,
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C2_FMP::Standard
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP::Fmp
    }
}
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast-mode Plus driving capability activation"]
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C2_FMP>;
impl<'a, REG> I2C2_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::Fmp)
    }
}
#[doc = "I2C3 Fast-mode Plus driving capability activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3_FMP {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers"]
    Fmp = 1,
}
impl From<I2C3_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C3_FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation"]
pub type I2C3_FMP_R = crate::BitReader<I2C3_FMP>;
impl I2C3_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C3_FMP {
        match self.bits {
            false => I2C3_FMP::Standard,
            true => I2C3_FMP::Fmp,
        }
    }
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C3_FMP::Standard
    }
    #[doc = "FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C3_FMP::Fmp
    }
}
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation"]
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C3_FMP>;
impl<'a, REG> I2C3_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3_FMP::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3_FMP::Fmp)
    }
}
impl R {
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<CFGR1rs> {
        I2C3_FMP_W::new(self, 22)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0x7c00_0001"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0x7c00_0001;
}
