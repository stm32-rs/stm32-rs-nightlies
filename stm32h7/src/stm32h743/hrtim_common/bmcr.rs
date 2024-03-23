#[doc = "Register `BMCR` reader"]
pub type R = crate::R<BMCRrs>;
#[doc = "Register `BMCR` writer"]
pub type W = crate::W<BMCRrs>;
#[doc = "Field `BME` reader - Burst Mode enable"]
pub type BME_R = crate::BitReader;
#[doc = "Field `BME` writer - Burst Mode enable"]
pub type BME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMOM` reader - Burst Mode operating mode"]
pub type BMOM_R = crate::BitReader;
#[doc = "Field `BMOM` writer - Burst Mode operating mode"]
pub type BMOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMCLK` reader - Burst Mode Clock source"]
pub type BMCLK_R = crate::FieldReader;
#[doc = "Field `BMCLK` writer - Burst Mode Clock source"]
pub type BMCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BMPRSC` reader - Burst Mode Prescaler"]
pub type BMPRSC_R = crate::FieldReader;
#[doc = "Field `BMPRSC` writer - Burst Mode Prescaler"]
pub type BMPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BMPREN` reader - Burst Mode Preload Enable"]
pub type BMPREN_R = crate::BitReader;
#[doc = "Field `BMPREN` writer - Burst Mode Preload Enable"]
pub type BMPREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTBM` reader - Master Timer Burst Mode"]
pub type MTBM_R = crate::BitReader;
#[doc = "Field `MTBM` writer - Master Timer Burst Mode"]
pub type MTBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TABM` reader - Timer A Burst Mode"]
pub type TABM_R = crate::BitReader;
#[doc = "Field `TABM` writer - Timer A Burst Mode"]
pub type TABM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBBM` reader - Timer B Burst Mode"]
pub type TBBM_R = crate::BitReader;
#[doc = "Field `TBBM` writer - Timer B Burst Mode"]
pub type TBBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBM` reader - Timer C Burst Mode"]
pub type TCBM_R = crate::BitReader;
#[doc = "Field `TCBM` writer - Timer C Burst Mode"]
pub type TCBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBM` reader - Timer D Burst Mode"]
pub type TDBM_R = crate::BitReader;
#[doc = "Field `TDBM` writer - Timer D Burst Mode"]
pub type TDBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEBM` reader - Timer E Burst Mode"]
pub type TEBM_R = crate::BitReader;
#[doc = "Field `TEBM` writer - Timer E Burst Mode"]
pub type TEBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMSTAT` reader - Burst Mode Status"]
pub type BMSTAT_R = crate::BitReader;
#[doc = "Field `BMSTAT` writer - Burst Mode Status"]
pub type BMSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&self) -> BME_R {
        BME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&self) -> BMOM_R {
        BMOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&self) -> BMCLK_R {
        BMCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&self) -> BMPRSC_R {
        BMPRSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&self) -> BMPREN_R {
        BMPREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&self) -> MTBM_R {
        MTBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&self) -> TABM_R {
        TABM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&self) -> TBBM_R {
        TBBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&self) -> TCBM_R {
        TCBM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&self) -> TDBM_R {
        TDBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&self) -> TEBM_R {
        TEBM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&self) -> BMSTAT_R {
        BMSTAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bme(&mut self) -> BME_W<BMCRrs> {
        BME_W::new(self, 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmom(&mut self) -> BMOM_W<BMCRrs> {
        BMOM_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn bmclk(&mut self) -> BMCLK_W<BMCRrs> {
        BMCLK_W::new(self, 2)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn bmprsc(&mut self) -> BMPRSC_W<BMCRrs> {
        BMPRSC_W::new(self, 6)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmpren(&mut self) -> BMPREN_W<BMCRrs> {
        BMPREN_W::new(self, 10)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtbm(&mut self) -> MTBM_W<BMCRrs> {
        MTBM_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tabm(&mut self) -> TABM_W<BMCRrs> {
        TABM_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbbm(&mut self) -> TBBM_W<BMCRrs> {
        TBBM_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcbm(&mut self) -> TCBM_W<BMCRrs> {
        TCBM_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tdbm(&mut self) -> TDBM_W<BMCRrs> {
        TDBM_W::new(self, 20)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tebm(&mut self) -> TEBM_W<BMCRrs> {
        TEBM_W::new(self, 21)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    #[must_use]
    pub fn bmstat(&mut self) -> BMSTAT_W<BMCRrs> {
        BMSTAT_W::new(self, 31)
    }
}
#[doc = "Burst Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMCRrs;
impl crate::RegisterSpec for BMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcr::R`](R) reader structure"]
impl crate::Readable for BMCRrs {}
#[doc = "`write(|w| ..)` method takes [`bmcr::W`](W) writer structure"]
impl crate::Writable for BMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMCR to value 0"]
impl crate::Resettable for BMCRrs {
    const RESET_VALUE: u32 = 0;
}
