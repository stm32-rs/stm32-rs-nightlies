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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
