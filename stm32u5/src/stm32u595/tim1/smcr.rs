#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCRrs>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCRrs>;
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SMS_R = crate::FieldReader;
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCCS` reader - OCREF clear selection"]
pub type OCCS_R = crate::BitReader;
#[doc = "Field `OCCS` writer - OCREF clear selection"]
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - Trigger selection"]
pub type TS_R = crate::FieldReader;
#[doc = "Field `TS` writer - Trigger selection"]
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - External trigger filter"]
pub type ETF_R = crate::FieldReader;
#[doc = "Field `ETF` writer - External trigger filter"]
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - External trigger prescaler"]
pub type ETPS_R = crate::FieldReader;
#[doc = "Field `ETPS` writer - External trigger prescaler"]
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - External clock enable"]
pub type ECE_R = crate::BitReader;
#[doc = "Field `ECE` writer - External clock enable"]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS3_0` reader - Slave mode selection"]
pub type SMS3_0_R = crate::BitReader;
#[doc = "Field `SMS3_0` writer - Slave mode selection"]
pub type SMS3_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS4_3` reader - Trigger selection"]
pub type TS4_3_R = crate::FieldReader;
#[doc = "Field `TS4_3` writer - Trigger selection"]
pub type TS4_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMSPE` reader - SMS preload enable"]
pub type SMSPE_R = crate::BitReader;
#[doc = "Field `SMSPE` writer - SMS preload enable"]
pub type SMSPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSPS` reader - SMS preload source"]
pub type SMSPS_R = crate::BitReader;
#[doc = "Field `SMSPS` writer - SMS preload source"]
pub type SMSPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection"]
    #[inline(always)]
    pub fn sms3_0(&self) -> SMS3_0_R {
        SMS3_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection"]
    #[inline(always)]
    pub fn ts4_3(&self) -> TS4_3_R {
        TS4_3_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - SMS preload enable"]
    #[inline(always)]
    pub fn smspe(&self) -> SMSPE_R {
        SMSPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SMS preload source"]
    #[inline(always)]
    pub fn smsps(&self) -> SMSPS_R {
        SMSPS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<SMCRrs> {
        SMS_W::new(self, 0)
    }
    #[doc = "Bit 3 - OCREF clear selection"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<SMCRrs> {
        OCCS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<SMCRrs> {
        TS_W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<SMCRrs> {
        ETF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<SMCRrs> {
        ETPS_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<SMCRrs> {
        ECE_W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<SMCRrs> {
        ETP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sms3_0(&mut self) -> SMS3_0_W<SMCRrs> {
        SMS3_0_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn ts4_3(&mut self) -> TS4_3_W<SMCRrs> {
        TS4_3_W::new(self, 20)
    }
    #[doc = "Bit 24 - SMS preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn smspe(&mut self) -> SMSPE_W<SMCRrs> {
        SMSPE_W::new(self, 24)
    }
    #[doc = "Bit 25 - SMS preload source"]
    #[inline(always)]
    #[must_use]
    pub fn smsps(&mut self) -> SMSPS_W<SMCRrs> {
        SMSPS_W::new(self, 25)
    }
}
#[doc = "slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCRrs {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}
