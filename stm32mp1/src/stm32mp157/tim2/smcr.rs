///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS` reader - SMS
pub type SMS_R = crate::FieldReader;
///Field `SMS` writer - SMS
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TS` reader - TS
pub type TS_R = crate::FieldReader;
///Field `TS` writer - TS
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSM` reader - MSM
pub type MSM_R = crate::BitReader;
///Field `MSM` writer - MSM
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETF` reader - ETF
pub type ETF_R = crate::FieldReader;
///Field `ETF` writer - ETF
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ETPS` reader - ETPS
pub type ETPS_R = crate::FieldReader;
///Field `ETPS` writer - ETPS
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECE` reader - ECE
pub type ECE_R = crate::BitReader;
///Field `ECE` writer - ECE
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETP` reader - ETP
pub type ETP_R = crate::BitReader;
///Field `ETP` writer - ETP
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMS3` reader - SMS3
pub type SMS3_R = crate::BitReader;
///Field `SMS3` writer - SMS3
pub type SMS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS3` reader - TS3
pub type TS3_R = crate::BitReader;
///Field `TS3` writer - TS3
pub type TS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS4` reader - TS4
pub type TS4_R = crate::BitReader;
///Field `TS4` writer - TS4
pub type TS4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - SMS
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TS
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - MSM
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - ETF
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - ETPS
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - ECE
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ETP
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SMS3
    #[inline(always)]
    pub fn sms3(&self) -> SMS3_R {
        SMS3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - TS3
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TS4
    #[inline(always)]
    pub fn ts4(&self) -> TS4_R {
        TS4_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("sms", &self.sms())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .field("sms3", &self.sms3())
            .field("ts3", &self.ts3())
            .field("ts4", &self.ts4())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - TS
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - MSM
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - ETF
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - ETPS
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<'_, SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - ECE
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<'_, SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - ETP
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<'_, SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - SMS3
    #[inline(always)]
    pub fn sms3(&mut self) -> SMS3_W<'_, SMCRrs> {
        SMS3_W::new(self, 16)
    }
    ///Bit 20 - TS3
    #[inline(always)]
    pub fn ts3(&mut self) -> TS3_W<'_, SMCRrs> {
        TS3_W::new(self, 20)
    }
    ///Bit 21 - TS4
    #[inline(always)]
    pub fn ts4(&mut self) -> TS4_W<'_, SMCRrs> {
        TS4_W::new(self, 21)
    }
}
/**TIM2 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM2:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {}
