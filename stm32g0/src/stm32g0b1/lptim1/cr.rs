///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**LPTIM enable The ENABLE bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE {
    ///0: LPTIM is disabled
    Disabled = 0,
    ///1: LPTIM is enabled
    Enabled = 1,
}
impl From<ENABLE> for bool {
    #[inline(always)]
    fn from(variant: ENABLE) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - LPTIM enable The ENABLE bit is set and cleared by software.
pub type ENABLE_R = crate::BitReader<ENABLE>;
impl ENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE {
        match self.bits {
            false => ENABLE::Disabled,
            true => ENABLE::Enabled,
        }
    }
    ///LPTIM is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE::Disabled
    }
    ///LPTIM is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE::Enabled
    }
}
///Field `ENABLE` writer - LPTIM enable The ENABLE bit is set and cleared by software.
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Disabled)
    }
    ///LPTIM is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Enabled)
    }
}
/**LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNGSTRTW {
    ///1: LPTIM start in Single mode
    Start = 1,
}
impl From<SNGSTRTW> for bool {
    #[inline(always)]
    fn from(variant: SNGSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `SNGSTRT` reader - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware.
pub type SNGSTRT_R = crate::BitReader<SNGSTRTW>;
impl SNGSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SNGSTRTW> {
        match self.bits {
            true => Some(SNGSTRTW::Start),
            _ => None,
        }
    }
    ///LPTIM start in Single mode
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SNGSTRTW::Start
    }
}
///Field `SNGSTRT` writer - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware.
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG, SNGSTRTW>;
impl<'a, REG> SNGSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM start in Single mode
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SNGSTRTW::Start)
    }
}
/**Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTSTRTW {
    ///1: Timer start in Continuous mode
    Start = 1,
}
impl From<CNTSTRTW> for bool {
    #[inline(always)]
    fn from(variant: CNTSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `CNTSTRT` reader - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware.
pub type CNTSTRT_R = crate::BitReader<CNTSTRTW>;
impl CNTSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CNTSTRTW> {
        match self.bits {
            true => Some(CNTSTRTW::Start),
            _ => None,
        }
    }
    ///Timer start in Continuous mode
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CNTSTRTW::Start
    }
}
///Field `CNTSTRT` writer - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware.
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG, CNTSTRTW>;
impl<'a, REG> CNTSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer start in Continuous mode
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CNTSTRTW::Start)
    }
}
/**Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTRSTR {
    ///0: Triggering of reset is possible
    Idle = 0,
    ///1: Reset in progress, do not write 1 to this field
    Busy = 1,
}
impl From<COUNTRSTR> for bool {
    #[inline(always)]
    fn from(variant: COUNTRSTR) -> Self {
        variant as u8 != 0
    }
}
///Field `COUNTRST` reader - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
pub type COUNTRST_R = crate::BitReader<COUNTRSTR>;
impl COUNTRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COUNTRSTR {
        match self.bits {
            false => COUNTRSTR::Idle,
            true => COUNTRSTR::Busy,
        }
    }
    ///Triggering of reset is possible
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == COUNTRSTR::Idle
    }
    ///Reset in progress, do not write 1 to this field
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == COUNTRSTR::Busy
    }
}
/**Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTRSTW {
    ///1: Trigger synchronous reset of CNT (3 LPTimer core clock cycles)
    Reset = 1,
}
impl From<COUNTRSTW> for bool {
    #[inline(always)]
    fn from(variant: COUNTRSTW) -> Self {
        variant as u8 != 0
    }
}
///Field `COUNTRST` writer - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG, COUNTRSTW>;
impl<'a, REG> COUNTRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger synchronous reset of CNT (3 LPTimer core clock cycles)
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTRSTW::Reset)
    }
}
/**Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTARE {
    ///0: CNT Register reads do not trigger reset
    Disabled = 0,
    ///1: CNT Register reads trigger reset of LPTIM
    Enabled = 1,
}
impl From<RSTARE> for bool {
    #[inline(always)]
    fn from(variant: RSTARE) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTARE` reader - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content.
pub type RSTARE_R = crate::BitReader<RSTARE>;
impl RSTARE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSTARE {
        match self.bits {
            false => RSTARE::Disabled,
            true => RSTARE::Enabled,
        }
    }
    ///CNT Register reads do not trigger reset
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTARE::Disabled
    }
    ///CNT Register reads trigger reset of LPTIM
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTARE::Enabled
    }
}
///Field `RSTARE` writer - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content.
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG, RSTARE>;
impl<'a, REG> RSTARE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CNT Register reads do not trigger reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTARE::Disabled)
    }
    ///CNT Register reads trigger reset of LPTIM
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTARE::Enabled)
    }
}
impl R {
    ///Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware.
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware.
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content.
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("enable", &self.enable())
            .field("sngstrt", &self.sngstrt())
            .field("cntstrt", &self.cntstrt())
            .field("countrst", &self.countrst())
            .field("rstare", &self.rstare())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, CRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware.
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<'_, CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    ///Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware.
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<'_, CRrs> {
        CNTSTRT_W::new(self, 2)
    }
    ///Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
    #[inline(always)]
    pub fn countrst(&mut self) -> COUNTRST_W<'_, CRrs> {
        COUNTRST_W::new(self, 3)
    }
    ///Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content.
    #[inline(always)]
    pub fn rstare(&mut self) -> RSTARE_W<'_, CRrs> {
        RSTARE_W::new(self, 4)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#LPTIM1:CR)*/
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
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
