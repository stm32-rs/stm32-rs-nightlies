#[doc = "Register `BCDR` reader"]
pub type R = crate::R<BCDRrs>;
#[doc = "Register `BCDR` writer"]
pub type W = crate::W<BCDRrs>;
#[doc = "Battery charging detector (BCD) enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCDEN {
    #[doc = "0: disable the BCD support"]
    Disabled = 0,
    #[doc = "1: enable the BCD support within the USB device"]
    Enabled = 1,
}
impl From<BCDEN> for bool {
    #[inline(always)]
    fn from(variant: BCDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCDEN` reader - Battery charging detector (BCD) enable"]
pub type BCDEN_R = crate::BitReader<BCDEN>;
impl BCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCDEN {
        match self.bits {
            false => BCDEN::Disabled,
            true => BCDEN::Enabled,
        }
    }
    #[doc = "disable the BCD support"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BCDEN::Disabled
    }
    #[doc = "enable the BCD support within the USB device"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BCDEN::Enabled
    }
}
#[doc = "Field `BCDEN` writer - Battery charging detector (BCD) enable"]
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG, BCDEN>;
impl<'a, REG> BCDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable the BCD support"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BCDEN::Disabled)
    }
    #[doc = "enable the BCD support within the USB device"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BCDEN::Enabled)
    }
}
#[doc = "Data contact detection (DCD) mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDEN {
    #[doc = "0: Data contact detection (DCD) mode disabled"]
    Disabled = 0,
    #[doc = "1: Data contact detection (DCD) mode enabled"]
    Enabled = 1,
}
impl From<DCDEN> for bool {
    #[inline(always)]
    fn from(variant: DCDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDEN` reader - Data contact detection (DCD) mode enable"]
pub type DCDEN_R = crate::BitReader<DCDEN>;
impl DCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCDEN {
        match self.bits {
            false => DCDEN::Disabled,
            true => DCDEN::Enabled,
        }
    }
    #[doc = "Data contact detection (DCD) mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDEN::Disabled
    }
    #[doc = "Data contact detection (DCD) mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDEN::Enabled
    }
}
#[doc = "Field `DCDEN` writer - Data contact detection (DCD) mode enable"]
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG, DCDEN>;
impl<'a, REG> DCDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data contact detection (DCD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCDEN::Disabled)
    }
    #[doc = "Data contact detection (DCD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCDEN::Enabled)
    }
}
#[doc = "Primary detection (PD) mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN {
    #[doc = "0: Primary detection (PD) mode disabled"]
    Disabled = 0,
    #[doc = "1: Primary detection (PD) mode enabled"]
    Enabled = 1,
}
impl From<PDEN> for bool {
    #[inline(always)]
    fn from(variant: PDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN` reader - Primary detection (PD) mode enable"]
pub type PDEN_R = crate::BitReader<PDEN>;
impl PDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDEN {
        match self.bits {
            false => PDEN::Disabled,
            true => PDEN::Enabled,
        }
    }
    #[doc = "Primary detection (PD) mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDEN::Disabled
    }
    #[doc = "Primary detection (PD) mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDEN::Enabled
    }
}
#[doc = "Field `PDEN` writer - Primary detection (PD) mode enable"]
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG, PDEN>;
impl<'a, REG> PDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Primary detection (PD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDEN::Disabled)
    }
    #[doc = "Primary detection (PD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDEN::Enabled)
    }
}
#[doc = "Secondary detection (SD) mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDEN {
    #[doc = "0: Secondary detection (SD) mode disabled"]
    Disabled = 0,
    #[doc = "1: Secondary detection (SD) mode enabled"]
    Enabled = 1,
}
impl From<SDEN> for bool {
    #[inline(always)]
    fn from(variant: SDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDEN` reader - Secondary detection (SD) mode enable"]
pub type SDEN_R = crate::BitReader<SDEN>;
impl SDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDEN {
        match self.bits {
            false => SDEN::Disabled,
            true => SDEN::Enabled,
        }
    }
    #[doc = "Secondary detection (SD) mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDEN::Disabled
    }
    #[doc = "Secondary detection (SD) mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDEN::Enabled
    }
}
#[doc = "Field `SDEN` writer - Secondary detection (SD) mode enable"]
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG, SDEN>;
impl<'a, REG> SDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secondary detection (SD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDEN::Disabled)
    }
    #[doc = "Secondary detection (SD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDEN::Enabled)
    }
}
#[doc = "Data contact detection (DCD) status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDET {
    #[doc = "0: data lines contact not detected"]
    NotDetected = 0,
    #[doc = "1: data lines contact detected"]
    Detected = 1,
}
impl From<DCDET> for bool {
    #[inline(always)]
    fn from(variant: DCDET) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDET` reader - Data contact detection (DCD) status"]
pub type DCDET_R = crate::BitReader<DCDET>;
impl DCDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCDET {
        match self.bits {
            false => DCDET::NotDetected,
            true => DCDET::Detected,
        }
    }
    #[doc = "data lines contact not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DCDET::NotDetected
    }
    #[doc = "data lines contact detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DCDET::Detected
    }
}
#[doc = "Primary detection (PD) status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDET {
    #[doc = "0: no BCD support detected"]
    NoBcd = 0,
    #[doc = "1: BCD support detected"]
    Bcd = 1,
}
impl From<PDET> for bool {
    #[inline(always)]
    fn from(variant: PDET) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDET` reader - Primary detection (PD) status"]
pub type PDET_R = crate::BitReader<PDET>;
impl PDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDET {
        match self.bits {
            false => PDET::NoBcd,
            true => PDET::Bcd,
        }
    }
    #[doc = "no BCD support detected"]
    #[inline(always)]
    pub fn is_no_bcd(&self) -> bool {
        *self == PDET::NoBcd
    }
    #[doc = "BCD support detected"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == PDET::Bcd
    }
}
#[doc = "Secondary detection (SD) status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDET {
    #[doc = "0: CDP detected"]
    Cdp = 0,
    #[doc = "1: DCP detected"]
    Dcp = 1,
}
impl From<SDET> for bool {
    #[inline(always)]
    fn from(variant: SDET) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDET` reader - Secondary detection (SD) status"]
pub type SDET_R = crate::BitReader<SDET>;
impl SDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDET {
        match self.bits {
            false => SDET::Cdp,
            true => SDET::Dcp,
        }
    }
    #[doc = "CDP detected"]
    #[inline(always)]
    pub fn is_cdp(&self) -> bool {
        *self == SDET::Cdp
    }
    #[doc = "DCP detected"]
    #[inline(always)]
    pub fn is_dcp(&self) -> bool {
        *self == SDET::Dcp
    }
}
#[doc = "DM pull-up detection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS2DET {
    #[doc = "0: Normal port detected"]
    Normal = 0,
    #[doc = "1: PS2 port or proprietary charger detected"]
    Ps2 = 1,
}
impl From<PS2DET> for bool {
    #[inline(always)]
    fn from(variant: PS2DET) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS2DET` reader - DM pull-up detection status"]
pub type PS2DET_R = crate::BitReader<PS2DET>;
impl PS2DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PS2DET {
        match self.bits {
            false => PS2DET::Normal,
            true => PS2DET::Ps2,
        }
    }
    #[doc = "Normal port detected"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PS2DET::Normal
    }
    #[doc = "PS2 port or proprietary charger detected"]
    #[inline(always)]
    pub fn is_ps2(&self) -> bool {
        *self == PS2DET::Ps2
    }
}
#[doc = "DP pull-up control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPPU {
    #[doc = "0: signalize disconnect to the host when needed by the user software"]
    Disabled = 0,
    #[doc = "1: enable the embedded pull-up on the DP line"]
    Enabled = 1,
}
impl From<DPPU> for bool {
    #[inline(always)]
    fn from(variant: DPPU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPU` reader - DP pull-up control"]
pub type DPPU_R = crate::BitReader<DPPU>;
impl DPPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPPU {
        match self.bits {
            false => DPPU::Disabled,
            true => DPPU::Enabled,
        }
    }
    #[doc = "signalize disconnect to the host when needed by the user software"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DPPU::Disabled
    }
    #[doc = "enable the embedded pull-up on the DP line"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DPPU::Enabled
    }
}
#[doc = "Field `DPPU` writer - DP pull-up control"]
pub type DPPU_W<'a, REG> = crate::BitWriter<'a, REG, DPPU>;
impl<'a, REG> DPPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "signalize disconnect to the host when needed by the user software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DPPU::Disabled)
    }
    #[doc = "enable the embedded pull-up on the DP line"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DPPU::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data contact detection (DCD) status"]
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Primary detection (PD) status"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secondary detection (SD) status"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<BCDRrs> {
        BCDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<BCDRrs> {
        DCDEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<BCDRrs> {
        PDEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<BCDRrs> {
        SDEN_W::new(self, 3)
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    #[must_use]
    pub fn dppu(&mut self) -> DPPU_W<BCDRrs> {
        DPPU_W::new(self, 15)
    }
}
#[doc = "Battery charging detector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCDRrs;
impl crate::RegisterSpec for BCDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdr::R`](R) reader structure"]
impl crate::Readable for BCDRrs {}
#[doc = "`write(|w| ..)` method takes [`bcdr::W`](W) writer structure"]
impl crate::Writable for BCDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDR to value 0"]
impl crate::Resettable for BCDRrs {
    const RESET_VALUE: u32 = 0;
}
