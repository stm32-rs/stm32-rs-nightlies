#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `LPDS` reader - Low-power deep sleep"]
pub type LPDS_R = crate::BitReader;
#[doc = "Field `LPDS` writer - Low-power deep sleep"]
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS` reader - Power down deepsleep"]
pub type PDDS_R = crate::BitReader;
#[doc = "Field `PDDS` writer - Power down deepsleep"]
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF` reader - Clear wakeup flag"]
pub type CWUF_R = crate::BitReader;
#[doc = "Field `CWUF` writer - Clear wakeup flag"]
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` reader - Clear standby flag"]
pub type CSBF_R = crate::BitReader;
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PVD level selection"]
pub type PLS_R = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD level selection"]
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDS` reader - Flash power down in Stop mode"]
pub type FPDS_R = crate::BitReader;
#[doc = "Field `FPDS` writer - Flash power down in Stop mode"]
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPLVDS` reader - Low-Power Regulator Low Voltage in deepsleep"]
pub type LPLVDS_R = crate::BitReader;
#[doc = "Field `LPLVDS` writer - Low-Power Regulator Low Voltage in deepsleep"]
pub type LPLVDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRLVDS` reader - Main regulator low voltage in deepsleep mode"]
pub type MRLVDS_R = crate::BitReader;
#[doc = "Field `MRLVDS` writer - Main regulator low voltage in deepsleep mode"]
pub type MRLVDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOS` reader - Regulator voltage scaling output selection"]
pub type VOS_R = crate::FieldReader;
#[doc = "Field `VOS` writer - Regulator voltage scaling output selection"]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODEN` reader - Over-drive enable"]
pub type ODEN_R = crate::BitReader;
#[doc = "Field `ODEN` writer - Over-drive enable"]
pub type ODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODSWEN` reader - Over-drive switching enabled"]
pub type ODSWEN_R = crate::BitReader;
#[doc = "Field `ODSWEN` writer - Over-drive switching enabled"]
pub type ODSWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDEN` reader - Under-drive enable in stop mode"]
pub type UDEN_R = crate::FieldReader;
#[doc = "Field `UDEN` writer - Under-drive enable in stop mode"]
pub type UDEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-Power Regulator Low Voltage in deepsleep"]
    #[inline(always)]
    pub fn lplvds(&self) -> LPLVDS_R {
        LPLVDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Main regulator low voltage in deepsleep mode"]
    #[inline(always)]
    pub fn mrlvds(&self) -> MRLVDS_R {
        MRLVDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Over-drive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Over-drive switching enabled"]
    #[inline(always)]
    pub fn odswen(&self) -> ODSWEN_R {
        ODSWEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Under-drive enable in stop mode"]
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<CRrs> {
        LPDS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<CRrs> {
        PDDS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CWUF_W<CRrs> {
        CWUF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<CRrs> {
        CSBF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CRrs> {
        PVDE_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CRrs> {
        PLS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CRrs> {
        DBP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<CRrs> {
        FPDS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Low-Power Regulator Low Voltage in deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn lplvds(&mut self) -> LPLVDS_W<CRrs> {
        LPLVDS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Main regulator low voltage in deepsleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn mrlvds(&mut self) -> MRLVDS_W<CRrs> {
        MRLVDS_W::new(self, 11)
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<CRrs> {
        VOS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Over-drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn oden(&mut self) -> ODEN_W<CRrs> {
        ODEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Over-drive switching enabled"]
    #[inline(always)]
    #[must_use]
    pub fn odswen(&mut self) -> ODSWEN_W<CRrs> {
        ODSWEN_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Under-drive enable in stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn uden(&mut self) -> UDEN_W<CRrs> {
        UDEN_W::new(self, 18)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0xc000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xc000;
}
