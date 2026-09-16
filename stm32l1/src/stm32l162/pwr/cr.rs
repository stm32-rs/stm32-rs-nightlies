///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LPSDSR` reader - Low-power deep sleep
pub type LPSDSR_R = crate::BitReader;
///Field `LPSDSR` writer - Low-power deep sleep
pub type LPSDSR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `CWUF` reader - Clear wakeup flag
pub type CWUF_R = crate::BitReader;
///Field `CWUF` writer - Clear wakeup flag
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `ULP` reader - Ultralow power mode
pub type ULP_R = crate::BitReader;
///Field `ULP` writer - Ultralow power mode
pub type ULP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FWU` reader - Fast wakeup
pub type FWU_R = crate::BitReader;
///Field `FWU` writer - Fast wakeup
pub type FWU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VOS` reader - Voltage scaling range selection
pub type VOS_R = crate::FieldReader;
///Field `VOS` writer - Voltage scaling range selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPRUN` reader - Low power run mode
pub type LPRUN_R = crate::BitReader;
///Field `LPRUN` writer - Low power run mode
pub type LPRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpsdsr(&self) -> LPSDSR_R {
        LPSDSR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
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
    ///Bit 9 - Ultralow power mode
    #[inline(always)]
    pub fn ulp(&self) -> ULP_R {
        ULP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Fast wakeup
    #[inline(always)]
    pub fn fwu(&self) -> FWU_R {
        FWU_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 14 - Low power run mode
    #[inline(always)]
    pub fn lprun(&self) -> LPRUN_R {
        LPRUN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lprun", &self.lprun())
            .field("vos", &self.vos())
            .field("fwu", &self.fwu())
            .field("ulp", &self.ulp())
            .field("dbp", &self.dbp())
            .field("pls", &self.pls())
            .field("pvde", &self.pvde())
            .field("csbf", &self.csbf())
            .field("cwuf", &self.cwuf())
            .field("pdds", &self.pdds())
            .field("lpsdsr", &self.lpsdsr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpsdsr(&mut self) -> LPSDSR_W<'_, CRrs> {
        LPSDSR_W::new(self, 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CRrs> {
        PDDS_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W<'_, CRrs> {
        CWUF_W::new(self, 2)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, CRrs> {
        CSBF_W::new(self, 3)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CRrs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CRrs> {
        DBP_W::new(self, 8)
    }
    ///Bit 9 - Ultralow power mode
    #[inline(always)]
    pub fn ulp(&mut self) -> ULP_W<'_, CRrs> {
        ULP_W::new(self, 9)
    }
    ///Bit 10 - Fast wakeup
    #[inline(always)]
    pub fn fwu(&mut self) -> FWU_W<'_, CRrs> {
        FWU_W::new(self, 10)
    }
    ///Bits 11:12 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CRrs> {
        VOS_W::new(self, 11)
    }
    ///Bit 14 - Low power run mode
    #[inline(always)]
    pub fn lprun(&mut self) -> LPRUN_W<'_, CRrs> {
        LPRUN_W::new(self, 14)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#PWR:CR)*/
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
///`reset()` method sets CR to value 0x1000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x1000;
}
