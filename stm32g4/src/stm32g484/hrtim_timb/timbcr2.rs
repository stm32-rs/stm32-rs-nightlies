#[doc = "Register `TIMBCR2` reader"]
pub type R = crate::R<TIMBCR2rs>;
#[doc = "Register `TIMBCR2` writer"]
pub type W = crate::W<TIMBCR2rs>;
#[doc = "Field `DCDE` reader - Dual Channel DAC trigger enable"]
pub type DCDE_R = crate::BitReader;
#[doc = "Field `DCDE` writer - Dual Channel DAC trigger enable"]
pub type DCDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDS` reader - Dual Channel DAC Step trigger"]
pub type DCDS_R = crate::BitReader;
#[doc = "Field `DCDS` writer - Dual Channel DAC Step trigger"]
pub type DCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDR` reader - Dual Channel DAC Reset trigger"]
pub type DCDR_R = crate::BitReader;
#[doc = "Field `DCDR` writer - Dual Channel DAC Reset trigger"]
pub type DCDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDM` reader - Up-Down Mode"]
pub type UDM_R = crate::BitReader;
#[doc = "Field `UDM` writer - Up-Down Mode"]
pub type UDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - Roll-Over Mode"]
pub type ROM_R = crate::FieldReader;
#[doc = "Field `ROM` writer - Roll-Over Mode"]
pub type ROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUTROM` reader - Output Roll-Over Mode"]
pub type OUTROM_R = crate::FieldReader;
#[doc = "Field `OUTROM` writer - Output Roll-Over Mode"]
pub type OUTROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADROM` reader - ADC Roll-Over Mode"]
pub type ADROM_R = crate::FieldReader;
#[doc = "Field `ADROM` writer - ADC Roll-Over Mode"]
pub type ADROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BMROM` reader - Burst Mode Roll-Over Mode"]
pub type BMROM_R = crate::FieldReader;
#[doc = "Field `BMROM` writer - Burst Mode Roll-Over Mode"]
pub type BMROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FEROM` reader - Fault and Event Roll-Over Mode"]
pub type FEROM_R = crate::FieldReader;
#[doc = "Field `FEROM` writer - Fault and Event Roll-Over Mode"]
pub type FEROM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTCMP1` reader - Greater than Compare 1 PWM mode"]
pub type GTCMP1_R = crate::BitReader;
#[doc = "Field `GTCMP1` writer - Greater than Compare 1 PWM mode"]
pub type GTCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTCMP3` reader - Greater than Compare 3 PWM mode"]
pub type GTCMP3_R = crate::BitReader;
#[doc = "Field `GTCMP3` writer - Greater than Compare 3 PWM mode"]
pub type GTCMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGHLF` reader - Triggered-half mode"]
pub type TRGHLF_R = crate::BitReader;
#[doc = "Field `TRGHLF` writer - Triggered-half mode"]
pub type TRGHLF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    pub fn dcde(&self) -> DCDE_R {
        DCDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    pub fn dcds(&self) -> DCDS_R {
        DCDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    pub fn dcdr(&self) -> DCDR_R {
        DCDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    pub fn udm(&self) -> UDM_R {
        UDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    pub fn outrom(&self) -> OUTROM_R {
        OUTROM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    pub fn adrom(&self) -> ADROM_R {
        ADROM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    pub fn bmrom(&self) -> BMROM_R {
        BMROM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    pub fn ferom(&self) -> FEROM_R {
        FEROM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    pub fn gtcmp1(&self) -> GTCMP1_R {
        GTCMP1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    pub fn gtcmp3(&self) -> GTCMP3_R {
        GTCMP3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    pub fn trghlf(&self) -> TRGHLF_R {
        TRGHLF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcde(&mut self) -> DCDE_W<TIMBCR2rs> {
        DCDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dcds(&mut self) -> DCDS_W<TIMBCR2rs> {
        DCDS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dcdr(&mut self) -> DCDR_W<TIMBCR2rs> {
        DCDR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    #[must_use]
    pub fn udm(&mut self) -> UDM_W<TIMBCR2rs> {
        UDM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<TIMBCR2rs> {
        ROM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn outrom(&mut self) -> OUTROM_W<TIMBCR2rs> {
        OUTROM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adrom(&mut self) -> ADROM_W<TIMBCR2rs> {
        ADROM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmrom(&mut self) -> BMROM_W<TIMBCR2rs> {
        BMROM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ferom(&mut self) -> FEROM_W<TIMBCR2rs> {
        FEROM_W::new(self, 14)
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gtcmp1(&mut self) -> GTCMP1_W<TIMBCR2rs> {
        GTCMP1_W::new(self, 16)
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gtcmp3(&mut self) -> GTCMP3_W<TIMBCR2rs> {
        GTCMP3_W::new(self, 17)
    }
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    #[must_use]
    pub fn trghlf(&mut self) -> TRGHLF_W<TIMBCR2rs> {
        TRGHLF_W::new(self, 20)
    }
}
#[doc = "HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMBCR2rs;
impl crate::RegisterSpec for TIMBCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timbcr2::R`](R) reader structure"]
impl crate::Readable for TIMBCR2rs {}
#[doc = "`write(|w| ..)` method takes [`timbcr2::W`](W) writer structure"]
impl crate::Writable for TIMBCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMBCR2 to value 0"]
impl crate::Resettable for TIMBCR2rs {
    const RESET_VALUE: u32 = 0;
}
