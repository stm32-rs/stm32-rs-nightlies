#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Firewall disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWDIS {
    #[doc = "0: Firewall access enabled"]
    Enabled = 0,
    #[doc = "1: Firewall access disabled"]
    Disabled = 1,
}
impl From<FWDIS> for bool {
    #[inline(always)]
    fn from(variant: FWDIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWDIS` reader - Firewall disable bit"]
pub type FWDIS_R = crate::BitReader<FWDIS>;
impl FWDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FWDIS {
        match self.bits {
            false => FWDIS::Enabled,
            true => FWDIS::Disabled,
        }
    }
    #[doc = "Firewall access enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWDIS::Enabled
    }
    #[doc = "Firewall access disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWDIS::Disabled
    }
}
#[doc = "Field `FWDIS` writer - Firewall disable bit"]
pub type FWDIS_W<'a, REG> = crate::BitWriter<'a, REG, FWDIS>;
impl<'a, REG> FWDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Firewall access enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWDIS::Enabled)
    }
    #[doc = "Firewall access disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWDIS::Disabled)
    }
}
#[doc = "Fm+ drive capability on PB6 enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `I2C_PB6_FMP` reader - Fm+ drive capability on PB6 enable bit"]
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
#[doc = "Field `I2C_PB6_FMP` writer - Fm+ drive capability on PB6 enable bit"]
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
#[doc = "Fm+ drive capability on PB7 enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `I2C_PB7_FMP` reader - Fm+ drive capability on PB7 enable bit"]
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
#[doc = "Field `I2C_PB7_FMP` writer - Fm+ drive capability on PB7 enable bit"]
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
#[doc = "Fm+ drive capability on PB8 enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `I2C_PB8_FMP` reader - Fm+ drive capability on PB8 enable bit"]
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
#[doc = "Field `I2C_PB8_FMP` writer - Fm+ drive capability on PB8 enable bit"]
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
#[doc = "Fm+ drive capability on PB9 enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `I2C_PB9_FMP` reader - Fm+ drive capability on PB9 enable bit"]
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
#[doc = "Field `I2C_PB9_FMP` writer - Fm+ drive capability on PB9 enable bit"]
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
#[doc = "I2C1 Fm+ drive capability enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `I2C1_FMP` reader - I2C1 Fm+ drive capability enable bit"]
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
#[doc = "Field `I2C1_FMP` writer - I2C1 Fm+ drive capability enable bit"]
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
#[doc = "I2C2 Fm+ drive capability enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `I2C2_FMP` reader - I2C2 Fm+ drive capability enable bit"]
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
#[doc = "Field `I2C2_FMP` writer - I2C2 Fm+ drive capability enable bit"]
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
#[doc = "I2C3 Fm+ drive capability enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `I2C3_FMP` reader - I2C3 Fm+ drive capability enable bit"]
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
#[doc = "Field `I2C3_FMP` writer - I2C3 Fm+ drive capability enable bit"]
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
    #[doc = "Bit 0 - Firewall disable bit"]
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Fm+ drive capability on PB6 enable bit"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fm+ drive capability on PB7 enable bit"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fm+ drive capability on PB8 enable bit"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fm+ drive capability on PB9 enable bit"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C1 Fm+ drive capability enable bit"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C2 Fm+ drive capability enable bit"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C3 Fm+ drive capability enable bit"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fwdis(&mut self) -> FWDIS_W<CFGR2rs> {
        FWDIS_W::new(self, 0)
    }
    #[doc = "Bit 8 - Fm+ drive capability on PB6 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR2rs> {
        I2C_PB6_FMP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Fm+ drive capability on PB7 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR2rs> {
        I2C_PB7_FMP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Fm+ drive capability on PB8 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR2rs> {
        I2C_PB8_FMP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Fm+ drive capability on PB9 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR2rs> {
        I2C_PB9_FMP_W::new(self, 11)
    }
    #[doc = "Bit 12 - I2C1 Fm+ drive capability enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR2rs> {
        I2C1_FMP_W::new(self, 12)
    }
    #[doc = "Bit 13 - I2C2 Fm+ drive capability enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR2rs> {
        I2C2_FMP_W::new(self, 13)
    }
    #[doc = "Bit 14 - I2C3 Fm+ drive capability enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<CFGR2rs> {
        I2C3_FMP_W::new(self, 14)
    }
}
#[doc = "SYSCFG configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
