///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPDS` reader - Low-power deep sleep
pub type LPDS_R = crate::BitReader;
///Field `LPDS` writer - Low-power deep sleep
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Power down deepsleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS {
    ///0: Enter Stop mode when the CPU enters deepsleep
    StopMode = 0,
    ///1: Enter Standby mode when the CPU enters deepsleep
    StandbyMode = 1,
}
impl From<PDDS> for bool {
    #[inline(always)]
    fn from(variant: PDDS) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDS` reader - Power down deepsleep
pub type PDDS_R = crate::BitReader<PDDS>;
impl PDDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDDS {
        match self.bits {
            false => PDDS::StopMode,
            true => PDDS::StandbyMode,
        }
    }
    ///Enter Stop mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS::StopMode
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS::StandbyMode
    }
}
///Field `PDDS` writer - Power down deepsleep
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG, PDDS>;
impl<'a, REG> PDDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enter Stop mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StopMode)
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StandbyMode)
    }
}
///Field `CSBF` reader - Clear standby flag
pub type CSBF_R = crate::BitReader;
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - PVD level selection
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - PVD level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPDS` reader - Flash power down in Stop mode
pub type FPDS_R = crate::BitReader;
///Field `FPDS` writer - Flash power down in Stop mode
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUDS` reader - Low-power regulator in deepsleep under-drive mode
pub type LPUDS_R = crate::BitReader;
///Field `LPUDS` writer - Low-power regulator in deepsleep under-drive mode
pub type LPUDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRUDS` reader - Main regulator in deepsleep under-drive mode
pub type MRUDS_R = crate::BitReader;
///Field `MRUDS` writer - Main regulator in deepsleep under-drive mode
pub type MRUDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCDC1` reader - ADCDC1
pub type ADCDC1_R = crate::BitReader;
///Field `ADCDC1` writer - ADCDC1
pub type ADCDC1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Regulator voltage scaling output selection

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    ///1: Scale 3 mode
    Scale3 = 1,
    ///2: Scale 2 mode
    Scale2 = 2,
    ///3: Scale 1 mode (reset value)
    Scale1 = 3,
}
impl From<VOS> for u8 {
    #[inline(always)]
    fn from(variant: VOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS {
    type Ux = u8;
}
impl crate::IsEnum for VOS {}
///Field `VOS` reader - Regulator voltage scaling output selection
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::Scale3),
            2 => Some(VOS::Scale2),
            3 => Some(VOS::Scale1),
            _ => None,
        }
    }
    ///Scale 3 mode
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == VOS::Scale3
    }
    ///Scale 2 mode
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == VOS::Scale2
    }
    ///Scale 1 mode (reset value)
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == VOS::Scale1
    }
}
///Field `VOS` writer - Regulator voltage scaling output selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Scale 3 mode
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale3)
    }
    ///Scale 2 mode
    #[inline(always)]
    pub fn scale2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale2)
    }
    ///Scale 1 mode (reset value)
    #[inline(always)]
    pub fn scale1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale1)
    }
}
///Field `ODEN` reader - Over-drive enable
pub type ODEN_R = crate::BitReader;
///Field `ODEN` writer - Over-drive enable
pub type ODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODSWEN` reader - Over-drive switching enabled
pub type ODSWEN_R = crate::BitReader;
///Field `ODSWEN` writer - Over-drive switching enabled
pub type ODSWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDEN` reader - Under-drive enable in stop mode
pub type UDEN_R = crate::FieldReader;
///Field `UDEN` writer - Under-drive enable in stop mode
pub type UDEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Low-power regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn lpuds(&self) -> LPUDS_R {
        LPUDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Main regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn mruds(&self) -> MRUDS_R {
        MRUDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - ADCDC1
    #[inline(always)]
    pub fn adcdc1(&self) -> ADCDC1_R {
        ADCDC1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    pub fn odswen(&self) -> ODSWEN_R {
        ODSWEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpds", &self.lpds())
            .field("pdds", &self.pdds())
            .field("csbf", &self.csbf())
            .field("pvde", &self.pvde())
            .field("pls", &self.pls())
            .field("dbp", &self.dbp())
            .field("fpds", &self.fpds())
            .field("lpuds", &self.lpuds())
            .field("mruds", &self.mruds())
            .field("adcdc1", &self.adcdc1())
            .field("vos", &self.vos())
            .field("oden", &self.oden())
            .field("odswen", &self.odswen())
            .field("uden", &self.uden())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<'_, CR1rs> {
        LPDS_W::new(self, 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CR1rs> {
        PDDS_W::new(self, 1)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, CR1rs> {
        CSBF_W::new(self, 3)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR1rs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CR1rs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CR1rs> {
        DBP_W::new(self, 8)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W<'_, CR1rs> {
        FPDS_W::new(self, 9)
    }
    ///Bit 10 - Low-power regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn lpuds(&mut self) -> LPUDS_W<'_, CR1rs> {
        LPUDS_W::new(self, 10)
    }
    ///Bit 11 - Main regulator in deepsleep under-drive mode
    #[inline(always)]
    pub fn mruds(&mut self) -> MRUDS_W<'_, CR1rs> {
        MRUDS_W::new(self, 11)
    }
    ///Bit 13 - ADCDC1
    #[inline(always)]
    pub fn adcdc1(&mut self) -> ADCDC1_W<'_, CR1rs> {
        ADCDC1_W::new(self, 13)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CR1rs> {
        VOS_W::new(self, 14)
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W<'_, CR1rs> {
        ODEN_W::new(self, 16)
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    pub fn odswen(&mut self) -> ODSWEN_W<'_, CR1rs> {
        ODSWEN_W::new(self, 17)
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    pub fn uden(&mut self) -> UDEN_W<'_, CR1rs> {
        UDEN_W::new(self, 18)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0xc000
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0xc000;
}
