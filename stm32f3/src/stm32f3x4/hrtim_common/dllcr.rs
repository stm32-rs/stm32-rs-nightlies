///Register `DLLCR` reader
pub type R = crate::R<DLLCRrs>;
///Register `DLLCR` writer
pub type W = crate::W<DLLCRrs>;
/**DLL Calibration Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL {
    ///1: Calibration start
    Start = 1,
}
impl From<CAL> for bool {
    #[inline(always)]
    fn from(variant: CAL) -> Self {
        variant as u8 != 0
    }
}
///Field `CAL` reader - DLL Calibration Start
pub type CAL_R = crate::BitReader<CAL>;
impl CAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAL> {
        match self.bits {
            true => Some(CAL::Start),
            _ => None,
        }
    }
    ///Calibration start
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CAL::Start
    }
}
///Field `CAL` writer - DLL Calibration Start
pub type CAL_W<'a, REG> = crate::BitWriter<'a, REG, CAL>;
impl<'a, REG> CAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration start
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CAL::Start)
    }
}
/**DLL Calibration Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALEN {
    ///0: Periodic calibration disabled
    Disabled = 0,
    ///1: Calibration is performed periodically, as per CALRTE setting
    Enabled = 1,
}
impl From<CALEN> for bool {
    #[inline(always)]
    fn from(variant: CALEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CALEN` reader - DLL Calibration Enable
pub type CALEN_R = crate::BitReader<CALEN>;
impl CALEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CALEN {
        match self.bits {
            false => CALEN::Disabled,
            true => CALEN::Enabled,
        }
    }
    ///Periodic calibration disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALEN::Disabled
    }
    ///Calibration is performed periodically, as per CALRTE setting
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALEN::Enabled
    }
}
///Field `CALEN` writer - DLL Calibration Enable
pub type CALEN_W<'a, REG> = crate::BitWriter<'a, REG, CALEN>;
impl<'a, REG> CALEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Periodic calibration disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALEN::Disabled)
    }
    ///Calibration is performed periodically, as per CALRTE setting
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALEN::Enabled)
    }
}
/**DLL Calibration rate

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALRTE {
    ///0: 1048576*t_HRTIM (6.168 ms for fHRTIM = 170 MHz)
    Clk1048576 = 0,
    ///1: 131072*t_HRTIM (771 µs for f_HRTIM = 170 MHz)
    Clk131072 = 1,
    ///2: 16384*t_HRTIM (96 µs for f_HRTIM = 170 MHz)
    Clk16384 = 2,
    ///3: 2048*t_HRTIM (12 µs for f_HRTIM = 170 MHz)
    Clk2048 = 3,
}
impl From<CALRTE> for u8 {
    #[inline(always)]
    fn from(variant: CALRTE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CALRTE {
    type Ux = u8;
}
impl crate::IsEnum for CALRTE {}
///Field `CALRTE` reader - DLL Calibration rate
pub type CALRTE_R = crate::FieldReader<CALRTE>;
impl CALRTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CALRTE {
        match self.bits {
            0 => CALRTE::Clk1048576,
            1 => CALRTE::Clk131072,
            2 => CALRTE::Clk16384,
            3 => CALRTE::Clk2048,
            _ => unreachable!(),
        }
    }
    ///1048576*t_HRTIM (6.168 ms for fHRTIM = 170 MHz)
    #[inline(always)]
    pub fn is_clk1048576(&self) -> bool {
        *self == CALRTE::Clk1048576
    }
    ///131072*t_HRTIM (771 µs for f_HRTIM = 170 MHz)
    #[inline(always)]
    pub fn is_clk131072(&self) -> bool {
        *self == CALRTE::Clk131072
    }
    ///16384*t_HRTIM (96 µs for f_HRTIM = 170 MHz)
    #[inline(always)]
    pub fn is_clk16384(&self) -> bool {
        *self == CALRTE::Clk16384
    }
    ///2048*t_HRTIM (12 µs for f_HRTIM = 170 MHz)
    #[inline(always)]
    pub fn is_clk2048(&self) -> bool {
        *self == CALRTE::Clk2048
    }
}
///Field `CALRTE` writer - DLL Calibration rate
pub type CALRTE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CALRTE, crate::Safe>;
impl<'a, REG> CALRTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1048576*t_HRTIM (6.168 ms for fHRTIM = 170 MHz)
    #[inline(always)]
    pub fn clk1048576(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Clk1048576)
    }
    ///131072*t_HRTIM (771 µs for f_HRTIM = 170 MHz)
    #[inline(always)]
    pub fn clk131072(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Clk131072)
    }
    ///16384*t_HRTIM (96 µs for f_HRTIM = 170 MHz)
    #[inline(always)]
    pub fn clk16384(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Clk16384)
    }
    ///2048*t_HRTIM (12 µs for f_HRTIM = 170 MHz)
    #[inline(always)]
    pub fn clk2048(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Clk2048)
    }
}
impl R {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLLCR")
            .field("calrte", &self.calrte())
            .field("calen", &self.calen())
            .field("cal", &self.cal())
            .finish()
    }
}
impl W {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<'_, DLLCRrs> {
        CAL_W::new(self, 0)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W<'_, DLLCRrs> {
        CALEN_W::new(self, 1)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    pub fn calrte(&mut self) -> CALRTE_W<'_, DLLCRrs> {
        CALRTE_W::new(self, 2)
    }
}
/**DLL Control Register

You can [`read`](crate::Reg::read) this register and get [`dllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Common:DLLCR)*/
pub struct DLLCRrs;
impl crate::RegisterSpec for DLLCRrs {
    type Ux = u32;
}
///`read()` method returns [`dllcr::R`](R) reader structure
impl crate::Readable for DLLCRrs {}
///`write(|w| ..)` method takes [`dllcr::W`](W) writer structure
impl crate::Writable for DLLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLLCR to value 0
impl crate::Resettable for DLLCRrs {}
