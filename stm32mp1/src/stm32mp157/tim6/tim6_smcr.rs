#[doc = "Register `TIM6_SMCR` reader"]
pub type R = crate::R<TIM6_SMCRrs>;
#[doc = "Register `TIM6_SMCR` writer"]
pub type W = crate::W<TIM6_SMCRrs>;
#[doc = "Field `SMS` reader - SMS"]
pub type SMS_R = crate::FieldReader;
#[doc = "Field `SMS` writer - SMS"]
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS` reader - TS"]
pub type TS_R = crate::FieldReader;
#[doc = "Field `TS` writer - TS"]
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - MSM"]
pub type MSM_R = crate::BitReader;
#[doc = "Field `MSM` writer - MSM"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - ETF"]
pub type ETF_R = crate::FieldReader;
#[doc = "Field `ETF` writer - ETF"]
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - ETPS"]
pub type ETPS_R = crate::FieldReader;
#[doc = "Field `ETPS` writer - ETPS"]
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - ECE"]
pub type ECE_R = crate::BitReader;
#[doc = "Field `ECE` writer - ECE"]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - ETP"]
pub type ETP_R = crate::BitReader;
#[doc = "Field `ETP` writer - ETP"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS3` reader - SMS3"]
pub type SMS3_R = crate::BitReader;
#[doc = "Field `SMS3` writer - SMS3"]
pub type SMS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS3` reader - TS3"]
pub type TS3_R = crate::BitReader;
#[doc = "Field `TS3` writer - TS3"]
pub type TS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS4` reader - TS4"]
pub type TS4_R = crate::BitReader;
#[doc = "Field `TS4` writer - TS4"]
pub type TS4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - ETPS"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - ECE"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ETP"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SMS3"]
    #[inline(always)]
    pub fn sms3(&self) -> SMS3_R {
        SMS3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - TS3"]
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TS4"]
    #[inline(always)]
    pub fn ts4(&self) -> TS4_R {
        TS4_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<TIM6_SMCRrs> {
        SMS_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<TIM6_SMCRrs> {
        TS_W::new(self, 4)
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<TIM6_SMCRrs> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<TIM6_SMCRrs> {
        ETF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - ETPS"]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<TIM6_SMCRrs> {
        ETPS_W::new(self, 12)
    }
    #[doc = "Bit 14 - ECE"]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<TIM6_SMCRrs> {
        ECE_W::new(self, 14)
    }
    #[doc = "Bit 15 - ETP"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<TIM6_SMCRrs> {
        ETP_W::new(self, 15)
    }
    #[doc = "Bit 16 - SMS3"]
    #[inline(always)]
    #[must_use]
    pub fn sms3(&mut self) -> SMS3_W<TIM6_SMCRrs> {
        SMS3_W::new(self, 16)
    }
    #[doc = "Bit 20 - TS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts3(&mut self) -> TS3_W<TIM6_SMCRrs> {
        TS3_W::new(self, 20)
    }
    #[doc = "Bit 21 - TS4"]
    #[inline(always)]
    #[must_use]
    pub fn ts4(&mut self) -> TS4_W<TIM6_SMCRrs> {
        TS4_W::new(self, 21)
    }
}
#[doc = "TIM6 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM6_SMCRrs;
impl crate::RegisterSpec for TIM6_SMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim6_smcr::R`](R) reader structure"]
impl crate::Readable for TIM6_SMCRrs {}
#[doc = "`write(|w| ..)` method takes [`tim6_smcr::W`](W) writer structure"]
impl crate::Writable for TIM6_SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM6_SMCR to value 0"]
impl crate::Resettable for TIM6_SMCRrs {
    const RESET_VALUE: u32 = 0;
}
