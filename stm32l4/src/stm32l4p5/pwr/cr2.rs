#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Power voltage detector enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    #[doc = "0: Power voltage detector disabled"]
    Disabled = 0,
    #[doc = "1: Power voltage detector enabled"]
    Enabled = 1,
}
impl From<PVDE> for bool {
    #[inline(always)]
    fn from(variant: PVDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader<PVDE>;
impl PVDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDE {
        match self.bits {
            false => PVDE::Disabled,
            true => PVDE::Enabled,
        }
    }
    #[doc = "Power voltage detector disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    #[doc = "Power voltage detector enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE::Enabled
    }
}
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power voltage detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    #[doc = "Power voltage detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
#[doc = "Power voltage detector level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS {
    #[doc = "0: VPVD0 around 2.0 V"]
    Vpvd0 = 0,
    #[doc = "1: VPVD1 around 2.2 V"]
    Vpvd1 = 1,
    #[doc = "2: VPVD2 around 2.4 V"]
    Vpvd2 = 2,
    #[doc = "3: VPVD3 around 2.5 V"]
    Vpvd3 = 3,
    #[doc = "4: VPVD4 around 2.6 V"]
    Vpvd4 = 4,
    #[doc = "5: VPVD5 around 2.8 V"]
    Vpvd5 = 5,
    #[doc = "6: VPVD6 around 2.9 V"]
    Vpvd6 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to VREFINT)"]
    Pvdin = 7,
}
impl From<PLS> for u8 {
    #[inline(always)]
    fn from(variant: PLS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLS {
    type Ux = u8;
}
#[doc = "Field `PLS` reader - Power voltage detector level selection"]
pub type PLS_R = crate::FieldReader<PLS>;
impl PLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLS {
        match self.bits {
            0 => PLS::Vpvd0,
            1 => PLS::Vpvd1,
            2 => PLS::Vpvd2,
            3 => PLS::Vpvd3,
            4 => PLS::Vpvd4,
            5 => PLS::Vpvd5,
            6 => PLS::Vpvd6,
            7 => PLS::Pvdin,
            _ => unreachable!(),
        }
    }
    #[doc = "VPVD0 around 2.0 V"]
    #[inline(always)]
    pub fn is_vpvd0(&self) -> bool {
        *self == PLS::Vpvd0
    }
    #[doc = "VPVD1 around 2.2 V"]
    #[inline(always)]
    pub fn is_vpvd1(&self) -> bool {
        *self == PLS::Vpvd1
    }
    #[doc = "VPVD2 around 2.4 V"]
    #[inline(always)]
    pub fn is_vpvd2(&self) -> bool {
        *self == PLS::Vpvd2
    }
    #[doc = "VPVD3 around 2.5 V"]
    #[inline(always)]
    pub fn is_vpvd3(&self) -> bool {
        *self == PLS::Vpvd3
    }
    #[doc = "VPVD4 around 2.6 V"]
    #[inline(always)]
    pub fn is_vpvd4(&self) -> bool {
        *self == PLS::Vpvd4
    }
    #[doc = "VPVD5 around 2.8 V"]
    #[inline(always)]
    pub fn is_vpvd5(&self) -> bool {
        *self == PLS::Vpvd5
    }
    #[doc = "VPVD6 around 2.9 V"]
    #[inline(always)]
    pub fn is_vpvd6(&self) -> bool {
        *self == PLS::Vpvd6
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn is_pvdin(&self) -> bool {
        *self == PLS::Pvdin
    }
}
#[doc = "Field `PLS` writer - Power voltage detector level selection"]
pub type PLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PLS>;
impl<'a, REG> PLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VPVD0 around 2.0 V"]
    #[inline(always)]
    pub fn vpvd0(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd0)
    }
    #[doc = "VPVD1 around 2.2 V"]
    #[inline(always)]
    pub fn vpvd1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd1)
    }
    #[doc = "VPVD2 around 2.4 V"]
    #[inline(always)]
    pub fn vpvd2(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd2)
    }
    #[doc = "VPVD3 around 2.5 V"]
    #[inline(always)]
    pub fn vpvd3(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd3)
    }
    #[doc = "VPVD4 around 2.6 V"]
    #[inline(always)]
    pub fn vpvd4(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd4)
    }
    #[doc = "VPVD5 around 2.8 V"]
    #[inline(always)]
    pub fn vpvd5(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd5)
    }
    #[doc = "VPVD6 around 2.9 V"]
    #[inline(always)]
    pub fn vpvd6(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd6)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn pvdin(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Pvdin)
    }
}
#[doc = "Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME1 {
    #[doc = "0: PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable"]
    Enabled = 1,
}
impl From<PVME1> for bool {
    #[inline(always)]
    fn from(variant: PVME1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME1` reader - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_R = crate::BitReader<PVME1>;
impl PVME1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVME1 {
        match self.bits {
            false => PVME1::Disabled,
            true => PVME1::Enabled,
        }
    }
    #[doc = "PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME1::Disabled
    }
    #[doc = "PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME1::Enabled
    }
}
#[doc = "Field `PVME1` writer - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_W<'a, REG> = crate::BitWriter<'a, REG, PVME1>;
impl<'a, REG> PVME1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME1::Disabled)
    }
    #[doc = "PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME1::Enabled)
    }
}
#[doc = "Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME2 {
    #[doc = "0: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable"]
    Enabled = 1,
}
impl From<PVME2> for bool {
    #[inline(always)]
    fn from(variant: PVME2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME2` reader - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type PVME2_R = crate::BitReader<PVME2>;
impl PVME2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVME2 {
        match self.bits {
            false => PVME2::Disabled,
            true => PVME2::Enabled,
        }
    }
    #[doc = "PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME2::Disabled
    }
    #[doc = "PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME2::Enabled
    }
}
#[doc = "Field `PVME2` writer - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type PVME2_W<'a, REG> = crate::BitWriter<'a, REG, PVME2>;
impl<'a, REG> PVME2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME2::Disabled)
    }
    #[doc = "PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME2::Enabled)
    }
}
#[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME3 {
    #[doc = "0: PVM3 (VDDA monitoring vs. 1.62V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM3 (VDDA monitoring vs. 1.62V threshold) enable"]
    Enabled = 1,
}
impl From<PVME3> for bool {
    #[inline(always)]
    fn from(variant: PVME3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_R = crate::BitReader<PVME3>;
impl PVME3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVME3 {
        match self.bits {
            false => PVME3::Disabled,
            true => PVME3::Enabled,
        }
    }
    #[doc = "PVM3 (VDDA monitoring vs. 1.62V threshold) disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME3::Disabled
    }
    #[doc = "PVM3 (VDDA monitoring vs. 1.62V threshold) enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME3::Enabled
    }
}
#[doc = "Field `PVME3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_W<'a, REG> = crate::BitWriter<'a, REG, PVME3>;
impl<'a, REG> PVME3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM3 (VDDA monitoring vs. 1.62V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME3::Disabled)
    }
    #[doc = "PVM3 (VDDA monitoring vs. 1.62V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME3::Enabled)
    }
}
#[doc = "Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME4 {
    #[doc = "0: PVM4 (VDDA monitoring vs. 2.2V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM4 (VDDA monitoring vs. 2.2V threshold) enable"]
    Enabled = 1,
}
impl From<PVME4> for bool {
    #[inline(always)]
    fn from(variant: PVME4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type PVME4_R = crate::BitReader<PVME4>;
impl PVME4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVME4 {
        match self.bits {
            false => PVME4::Disabled,
            true => PVME4::Enabled,
        }
    }
    #[doc = "PVM4 (VDDA monitoring vs. 2.2V threshold) disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME4::Disabled
    }
    #[doc = "PVM4 (VDDA monitoring vs. 2.2V threshold) enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME4::Enabled
    }
}
#[doc = "Field `PVME4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type PVME4_W<'a, REG> = crate::BitWriter<'a, REG, PVME4>;
impl<'a, REG> PVME4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM4 (VDDA monitoring vs. 2.2V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME4::Disabled)
    }
    #[doc = "PVM4 (VDDA monitoring vs. 2.2V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME4::Enabled)
    }
}
#[doc = "VDDIO2 Independent I/Os supply valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSV {
    #[doc = "0: VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply"]
    NotPresent = 0,
    #[doc = "1: VDDIO2 is valid"]
    Valid = 1,
}
impl From<IOSV> for bool {
    #[inline(always)]
    fn from(variant: IOSV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOSV` reader - VDDIO2 Independent I/Os supply valid"]
pub type IOSV_R = crate::BitReader<IOSV>;
impl IOSV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOSV {
        match self.bits {
            false => IOSV::NotPresent,
            true => IOSV::Valid,
        }
    }
    #[doc = "VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == IOSV::NotPresent
    }
    #[doc = "VDDIO2 is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == IOSV::Valid
    }
}
#[doc = "Field `IOSV` writer - VDDIO2 Independent I/Os supply valid"]
pub type IOSV_W<'a, REG> = crate::BitWriter<'a, REG, IOSV>;
impl<'a, REG> IOSV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(IOSV::NotPresent)
    }
    #[doc = "VDDIO2 is valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(IOSV::Valid)
    }
}
#[doc = "VDDUSB USB supply valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USV {
    #[doc = "0: VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply"]
    NotPresent = 0,
    #[doc = "1: VDDUSB is valid"]
    Valid = 1,
}
impl From<USV> for bool {
    #[inline(always)]
    fn from(variant: USV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USV` reader - VDDUSB USB supply valid"]
pub type USV_R = crate::BitReader<USV>;
impl USV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USV {
        match self.bits {
            false => USV::NotPresent,
            true => USV::Valid,
        }
    }
    #[doc = "VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == USV::NotPresent
    }
    #[doc = "VDDUSB is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == USV::Valid
    }
}
#[doc = "Field `USV` writer - VDDUSB USB supply valid"]
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG, USV>;
impl<'a, REG> USV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(USV::NotPresent)
    }
    #[doc = "VDDUSB is valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(USV::Valid)
    }
}
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    pub fn pvme1(&self) -> PVME1_R {
        PVME1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    pub fn pvme2(&self) -> PVME2_R {
        PVME2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    pub fn pvme4(&self) -> PVME4_R {
        PVME4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CR2rs> {
        PVDE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CR2rs> {
        PLS_W::new(self, 1)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme1(&mut self) -> PVME1_W<CR2rs> {
        PVME1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme2(&mut self) -> PVME2_W<CR2rs> {
        PVME2_W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme3(&mut self) -> PVME3_W<CR2rs> {
        PVME3_W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme4(&mut self) -> PVME4_W<CR2rs> {
        PVME4_W::new(self, 7)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    #[must_use]
    pub fn iosv(&mut self) -> IOSV_W<CR2rs> {
        IOSV_W::new(self, 9)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> USV_W<CR2rs> {
        USV_W::new(self, 10)
    }
}
#[doc = "Power control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
