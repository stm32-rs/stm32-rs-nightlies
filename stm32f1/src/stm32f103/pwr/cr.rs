#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `LPDS` reader - Low Power Deep Sleep"]
pub type LPDS_R = crate::BitReader;
#[doc = "Field `LPDS` writer - Low Power Deep Sleep"]
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Power Down Deep Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS {
    #[doc = "0: Enter Stop mode when the CPU enters deepsleep"]
    StopMode = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    StandbyMode = 1,
}
impl From<PDDS> for bool {
    #[inline(always)]
    fn from(variant: PDDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDDS` reader - Power Down Deep Sleep"]
pub type PDDS_R = crate::BitReader<PDDS>;
impl PDDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDDS {
        match self.bits {
            false => PDDS::StopMode,
            true => PDDS::StandbyMode,
        }
    }
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS::StopMode
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS::StandbyMode
    }
}
#[doc = "Field `PDDS` writer - Power Down Deep Sleep"]
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG, PDDS>;
impl<'a, REG> PDDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StopMode)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StandbyMode)
    }
}
#[doc = "Field `CWUF` reader - Clear Wake-up Flag"]
pub type CWUF_R = crate::BitReader;
#[doc = "Field `CWUF` writer - Clear Wake-up Flag"]
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` reader - Clear STANDBY Flag"]
pub type CSBF_R = crate::BitReader;
#[doc = "Field `CSBF` writer - Clear STANDBY Flag"]
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDE` reader - Power Voltage Detector Enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - Power Voltage Detector Enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PVD Level Selection"]
pub type PLS_R = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD Level Selection"]
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBP` reader - Disable Backup Domain write protection"]
pub type DBP_R = crate::BitReader;
#[doc = "Field `DBP` writer - Disable Backup Domain write protection"]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<CRrs> {
        LPDS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<CRrs> {
        PDDS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CWUF_W<CRrs> {
        CWUF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<CRrs> {
        CSBF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CRrs> {
        PVDE_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CRrs> {
        PLS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CRrs> {
        DBP_W::new(self, 8)
    }
}
#[doc = "Power control register (PWR_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
