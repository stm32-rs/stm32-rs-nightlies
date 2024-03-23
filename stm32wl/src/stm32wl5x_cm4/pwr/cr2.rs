#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Power voltage detector enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    #[doc = "0: PVD Disabled"]
    Disabled = 0,
    #[doc = "1: PVD Enabled"]
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
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    #[doc = "PVD Enabled"]
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
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    #[doc = "PVD Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
#[doc = "Power voltage detector level selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS {
    #[doc = "0: 2.0V"]
    V2_0 = 0,
    #[doc = "1: 2.2V"]
    V2_2 = 1,
    #[doc = "2: 2.4V"]
    V2_4 = 2,
    #[doc = "3: 2.5V"]
    V2_5 = 3,
    #[doc = "4: 2.6V"]
    V2_6 = 4,
    #[doc = "5: 2.8V"]
    V2_8 = 5,
    #[doc = "6: 2.9V"]
    V2_9 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to VREFINT)"]
    External = 7,
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
#[doc = "Field `PLS` reader - Power voltage detector level selection."]
pub type PLS_R = crate::FieldReader<PLS>;
impl PLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLS {
        match self.bits {
            0 => PLS::V2_0,
            1 => PLS::V2_2,
            2 => PLS::V2_4,
            3 => PLS::V2_5,
            4 => PLS::V2_6,
            5 => PLS::V2_8,
            6 => PLS::V2_9,
            7 => PLS::External,
            _ => unreachable!(),
        }
    }
    #[doc = "2.0V"]
    #[inline(always)]
    pub fn is_v2_0(&self) -> bool {
        *self == PLS::V2_0
    }
    #[doc = "2.2V"]
    #[inline(always)]
    pub fn is_v2_2(&self) -> bool {
        *self == PLS::V2_2
    }
    #[doc = "2.4V"]
    #[inline(always)]
    pub fn is_v2_4(&self) -> bool {
        *self == PLS::V2_4
    }
    #[doc = "2.5V"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == PLS::V2_5
    }
    #[doc = "2.6V"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == PLS::V2_6
    }
    #[doc = "2.8V"]
    #[inline(always)]
    pub fn is_v2_8(&self) -> bool {
        *self == PLS::V2_8
    }
    #[doc = "2.9V"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == PLS::V2_9
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLS::External
    }
}
#[doc = "Field `PLS` writer - Power voltage detector level selection."]
pub type PLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PLS>;
impl<'a, REG> PLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.0V"]
    #[inline(always)]
    pub fn v2_0(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_0)
    }
    #[doc = "2.2V"]
    #[inline(always)]
    pub fn v2_2(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_2)
    }
    #[doc = "2.4V"]
    #[inline(always)]
    pub fn v2_4(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_4)
    }
    #[doc = "2.5V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_5)
    }
    #[doc = "2.6V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_6)
    }
    #[doc = "2.8V"]
    #[inline(always)]
    pub fn v2_8(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_8)
    }
    #[doc = "2.9V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_9)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::External)
    }
}
#[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME3 {
    #[doc = "0: PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
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
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME3::Disabled
    }
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
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
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME3::Disabled)
    }
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME3::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CR2rs> {
        PVDE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection."]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CR2rs> {
        PLS_W::new(self, 1)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme3(&mut self) -> PVME3_W<CR2rs> {
        PVME3_W::new(self, 6)
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
