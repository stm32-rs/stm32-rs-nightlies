///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS` reader - Slave mode selection
pub type SMS_R = crate::FieldReader;
///Field `SMS` writer - Slave mode selection
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TS_2_0` reader - Trigger selection
pub type TS_2_0_R = crate::FieldReader;
///Field `TS_2_0` writer - Trigger selection
pub type TS_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSM` reader - Master/slave mode
pub type MSM_R = crate::BitReader;
///Field `MSM` writer - Master/slave mode
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - Capture/Compare 1 DMA request enable
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMS_3` reader - Slave mode selection
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - Slave mode selection
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_4_3` reader - Trigger selection
pub type TS_4_3_R = crate::FieldReader;
///Field `TS_4_3` writer - Trigger selection
pub type TS_4_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts_2_0(&self) -> TS_2_0_R {
        TS_2_0_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    pub fn ts_4_3(&self) -> TS_4_3_R {
        TS_4_3_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("ts_4_3", &self.ts_4_3())
            .field("sms_3", &self.sms_3())
            .field("cc1de", &self.cc1de())
            .field("msm", &self.msm())
            .field("ts_2_0", &self.ts_2_0())
            .field("sms", &self.sms())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    #[must_use]
    pub fn ts_2_0(&mut self) -> TS_2_0_W<SMCRrs> {
        TS_2_0_W::new(self, 4)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<SMCRrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 16 - Slave mode selection
    #[inline(always)]
    #[must_use]
    pub fn sms_3(&mut self) -> SMS_3_W<SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    #[must_use]
    pub fn ts_4_3(&mut self) -> TS_4_3_W<SMCRrs> {
        TS_4_3_W::new(self, 20)
    }
}
/**slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#TIM15:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}
