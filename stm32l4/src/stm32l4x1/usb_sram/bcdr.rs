///Register `BCDR` reader
pub type R = crate::R<BCDRrs>;
///Register `BCDR` writer
pub type W = crate::W<BCDRrs>;
/**Battery charging detector

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCDEN {
    ///0: disable the BCD support
    Disabled = 0,
    ///1: enable the BCD support within the USB device
    Enabled = 1,
}
impl From<BCDEN> for bool {
    #[inline(always)]
    fn from(variant: BCDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BCDEN` reader - Battery charging detector
pub type BCDEN_R = crate::BitReader<BCDEN>;
impl BCDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCDEN {
        match self.bits {
            false => BCDEN::Disabled,
            true => BCDEN::Enabled,
        }
    }
    ///disable the BCD support
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BCDEN::Disabled
    }
    ///enable the BCD support within the USB device
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BCDEN::Enabled
    }
}
///Field `BCDEN` writer - Battery charging detector
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG, BCDEN>;
impl<'a, REG> BCDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disable the BCD support
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BCDEN::Disabled)
    }
    ///enable the BCD support within the USB device
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BCDEN::Enabled)
    }
}
/**Data contact detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDEN {
    ///0: Data contact detection (DCD) mode disabled
    Disabled = 0,
    ///1: Data contact detection (DCD) mode enabled
    Enabled = 1,
}
impl From<DCDEN> for bool {
    #[inline(always)]
    fn from(variant: DCDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCDEN` reader - Data contact detection
pub type DCDEN_R = crate::BitReader<DCDEN>;
impl DCDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCDEN {
        match self.bits {
            false => DCDEN::Disabled,
            true => DCDEN::Enabled,
        }
    }
    ///Data contact detection (DCD) mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDEN::Disabled
    }
    ///Data contact detection (DCD) mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDEN::Enabled
    }
}
///Field `DCDEN` writer - Data contact detection
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG, DCDEN>;
impl<'a, REG> DCDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data contact detection (DCD) mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCDEN::Disabled)
    }
    ///Data contact detection (DCD) mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCDEN::Enabled)
    }
}
/**Primary detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN {
    ///0: Primary detection (PD) mode disabled
    Disabled = 0,
    ///1: Primary detection (PD) mode enabled
    Enabled = 1,
}
impl From<PDEN> for bool {
    #[inline(always)]
    fn from(variant: PDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PDEN` reader - Primary detection
pub type PDEN_R = crate::BitReader<PDEN>;
impl PDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDEN {
        match self.bits {
            false => PDEN::Disabled,
            true => PDEN::Enabled,
        }
    }
    ///Primary detection (PD) mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDEN::Disabled
    }
    ///Primary detection (PD) mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDEN::Enabled
    }
}
///Field `PDEN` writer - Primary detection
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG, PDEN>;
impl<'a, REG> PDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Primary detection (PD) mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDEN::Disabled)
    }
    ///Primary detection (PD) mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDEN::Enabled)
    }
}
/**Secondary detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDEN {
    ///0: Secondary detection (SD) mode disabled
    Disabled = 0,
    ///1: Secondary detection (SD) mode enabled
    Enabled = 1,
}
impl From<SDEN> for bool {
    #[inline(always)]
    fn from(variant: SDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SDEN` reader - Secondary detection
pub type SDEN_R = crate::BitReader<SDEN>;
impl SDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDEN {
        match self.bits {
            false => SDEN::Disabled,
            true => SDEN::Enabled,
        }
    }
    ///Secondary detection (SD) mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDEN::Disabled
    }
    ///Secondary detection (SD) mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDEN::Enabled
    }
}
///Field `SDEN` writer - Secondary detection
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG, SDEN>;
impl<'a, REG> SDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secondary detection (SD) mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDEN::Disabled)
    }
    ///Secondary detection (SD) mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDEN::Enabled)
    }
}
/**Data contact detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDET {
    ///0: data lines contact not detected
    NotDetected = 0,
    ///1: data lines contact detected
    Detected = 1,
}
impl From<DCDET> for bool {
    #[inline(always)]
    fn from(variant: DCDET) -> Self {
        variant as u8 != 0
    }
}
///Field `DCDET` reader - Data contact detection
pub type DCDET_R = crate::BitReader<DCDET>;
impl DCDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCDET {
        match self.bits {
            false => DCDET::NotDetected,
            true => DCDET::Detected,
        }
    }
    ///data lines contact not detected
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DCDET::NotDetected
    }
    ///data lines contact detected
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DCDET::Detected
    }
}
/**Primary detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDET {
    ///0: no BCD support detected
    NoBcd = 0,
    ///1: BCD support detected
    Bcd = 1,
}
impl From<PDET> for bool {
    #[inline(always)]
    fn from(variant: PDET) -> Self {
        variant as u8 != 0
    }
}
///Field `PDET` reader - Primary detection
pub type PDET_R = crate::BitReader<PDET>;
impl PDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDET {
        match self.bits {
            false => PDET::NoBcd,
            true => PDET::Bcd,
        }
    }
    ///no BCD support detected
    #[inline(always)]
    pub fn is_no_bcd(&self) -> bool {
        *self == PDET::NoBcd
    }
    ///BCD support detected
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == PDET::Bcd
    }
}
/**Secondary detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDET {
    ///0: CDP detected
    Cdp = 0,
    ///1: DCP detected
    Dcp = 1,
}
impl From<SDET> for bool {
    #[inline(always)]
    fn from(variant: SDET) -> Self {
        variant as u8 != 0
    }
}
///Field `SDET` reader - Secondary detection
pub type SDET_R = crate::BitReader<SDET>;
impl SDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDET {
        match self.bits {
            false => SDET::Cdp,
            true => SDET::Dcp,
        }
    }
    ///CDP detected
    #[inline(always)]
    pub fn is_cdp(&self) -> bool {
        *self == SDET::Cdp
    }
    ///DCP detected
    #[inline(always)]
    pub fn is_dcp(&self) -> bool {
        *self == SDET::Dcp
    }
}
/**DM pull-up detection status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS2DET {
    ///0: Normal port detected
    Normal = 0,
    ///1: PS2 port or proprietary charger detected
    Ps2 = 1,
}
impl From<PS2DET> for bool {
    #[inline(always)]
    fn from(variant: PS2DET) -> Self {
        variant as u8 != 0
    }
}
///Field `PS2DET` reader - DM pull-up detection status
pub type PS2DET_R = crate::BitReader<PS2DET>;
impl PS2DET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PS2DET {
        match self.bits {
            false => PS2DET::Normal,
            true => PS2DET::Ps2,
        }
    }
    ///Normal port detected
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PS2DET::Normal
    }
    ///PS2 port or proprietary charger detected
    #[inline(always)]
    pub fn is_ps2(&self) -> bool {
        *self == PS2DET::Ps2
    }
}
/**DP pull-up control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPPU {
    ///0: signalize disconnect to the host when needed by the user software
    Disabled = 0,
    ///1: enable the embedded pull-up on the DP line
    Enabled = 1,
}
impl From<DPPU> for bool {
    #[inline(always)]
    fn from(variant: DPPU) -> Self {
        variant as u8 != 0
    }
}
///Field `DPPU` reader - DP pull-up control
pub type DPPU_R = crate::BitReader<DPPU>;
impl DPPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPPU {
        match self.bits {
            false => DPPU::Disabled,
            true => DPPU::Enabled,
        }
    }
    ///signalize disconnect to the host when needed by the user software
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DPPU::Disabled
    }
    ///enable the embedded pull-up on the DP line
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DPPU::Enabled
    }
}
///Field `DPPU` writer - DP pull-up control
pub type DPPU_W<'a, REG> = crate::BitWriter<'a, REG, DPPU>;
impl<'a, REG> DPPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///signalize disconnect to the host when needed by the user software
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DPPU::Disabled)
    }
    ///enable the embedded pull-up on the DP line
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DPPU::Enabled)
    }
}
impl R {
    ///Bit 0 - Battery charging detector
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data contact detection
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Primary detection
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Secondary detection
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data contact detection
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Primary detection
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Secondary detection
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCDR")
            .field("bcden", &self.bcden())
            .field("dcden", &self.dcden())
            .field("pden", &self.pden())
            .field("sden", &self.sden())
            .field("dcdet", &self.dcdet())
            .field("pdet", &self.pdet())
            .field("sdet", &self.sdet())
            .field("ps2det", &self.ps2det())
            .field("dppu", &self.dppu())
            .finish()
    }
}
impl W {
    ///Bit 0 - Battery charging detector
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W<'_, BCDRrs> {
        BCDEN_W::new(self, 0)
    }
    ///Bit 1 - Data contact detection
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W<'_, BCDRrs> {
        DCDEN_W::new(self, 1)
    }
    ///Bit 2 - Primary detection
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<'_, BCDRrs> {
        PDEN_W::new(self, 2)
    }
    ///Bit 3 - Secondary detection
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<'_, BCDRrs> {
        SDEN_W::new(self, 3)
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&mut self) -> DPPU_W<'_, BCDRrs> {
        DPPU_W::new(self, 15)
    }
}
/**Battery charging detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#USB_SRAM:BCDR)*/
pub struct BCDRrs;
impl crate::RegisterSpec for BCDRrs {
    type Ux = u32;
}
///`read()` method returns [`bcdr::R`](R) reader structure
impl crate::Readable for BCDRrs {}
///`write(|w| ..)` method takes [`bcdr::W`](W) writer structure
impl crate::Writable for BCDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCDR to value 0
impl crate::Resettable for BCDRrs {}
