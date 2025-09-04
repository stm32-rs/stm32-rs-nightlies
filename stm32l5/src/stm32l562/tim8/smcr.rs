///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS` reader - Slave mode selection
pub type SMS_R = crate::FieldReader;
///Field `SMS` writer - Slave mode selection
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TS` reader - Trigger selection
pub type TS_R = crate::FieldReader;
///Field `TS` writer - Trigger selection
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///External clock enable
pub use crate::stm32l562::tim1::smcr::ECE;
///Field `ECE` reader - External clock enable
pub use crate::stm32l562::tim1::smcr::ECE_R;
///Field `ECE` writer - External clock enable
pub use crate::stm32l562::tim1::smcr::ECE_W;
///External trigger filter
pub use crate::stm32l562::tim1::smcr::ETF;
///Field `ETF` reader - External trigger filter
pub use crate::stm32l562::tim1::smcr::ETF_R;
///Field `ETF` writer - External trigger filter
pub use crate::stm32l562::tim1::smcr::ETF_W;
///External trigger polarity
pub use crate::stm32l562::tim1::smcr::ETP;
///External trigger prescaler
pub use crate::stm32l562::tim1::smcr::ETPS;
///Field `ETPS` reader - External trigger prescaler
pub use crate::stm32l562::tim1::smcr::ETPS_R;
///Field `ETPS` writer - External trigger prescaler
pub use crate::stm32l562::tim1::smcr::ETPS_W;
///Field `ETP` reader - External trigger polarity
pub use crate::stm32l562::tim1::smcr::ETP_R;
///Field `ETP` writer - External trigger polarity
pub use crate::stm32l562::tim1::smcr::ETP_W;
///Master/Slave mode
pub use crate::stm32l562::tim1::smcr::MSM;
///Field `MSM` reader - Master/Slave mode
pub use crate::stm32l562::tim1::smcr::MSM_R;
///Field `MSM` writer - Master/Slave mode
pub use crate::stm32l562::tim1::smcr::MSM_W;
///Field `SMS_3` reader - Slave mode selection - bit 3
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - Slave mode selection - bit 3
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("etp", &self.etp())
            .field("ece", &self.ece())
            .field("etps", &self.etps())
            .field("etf", &self.etf())
            .field("msm", &self.msm())
            .field("ts", &self.ts())
            .field("sms", &self.sms())
            .field("sms_3", &self.sms_3())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<SMCRrs> {
        SMS_3_W::new(self, 16)
    }
}
/**slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TIM8:SMCR)*/
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
