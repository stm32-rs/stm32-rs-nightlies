///Register `_SMCR` reader
pub type R = crate::R<_SMCRrs>;
///Register `_SMCR` writer
pub type W = crate::W<_SMCRrs>;
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
///Field `SMS_3` reader - SMS_3
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - SMS_3
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_3` reader - TS_3
pub type TS_3_R = crate::BitReader;
///Field `TS_3` writer - TS_3
pub type TS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_4` reader - TS_4
pub type TS_4_R = crate::BitReader;
///Field `TS_4` writer - TS_4
pub type TS_4_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 16 - SMS_3
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - TS_3
    #[inline(always)]
    pub fn ts_3(&self) -> TS_3_R {
        TS_3_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TS_4
    #[inline(always)]
    pub fn ts_4(&self) -> TS_4_R {
        TS_4_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_SMCR")
            .field("sms", &self.sms())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("sms_3", &self.sms_3())
            .field("ts_3", &self.ts_3())
            .field("ts_4", &self.ts_4())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, _SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - TS
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, _SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - MSM
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, _SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bit 16 - SMS_3
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<'_, _SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bit 20 - TS_3
    #[inline(always)]
    pub fn ts_3(&mut self) -> TS_3_W<'_, _SMCRrs> {
        TS_3_W::new(self, 20)
    }
    ///Bit 21 - TS_4
    #[inline(always)]
    pub fn ts_4(&mut self) -> TS_4_W<'_, _SMCRrs> {
        TS_4_W::new(self, 21)
    }
}
/**TIM12 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`_smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM12:_SMCR)*/
pub struct _SMCRrs;
impl crate::RegisterSpec for _SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`_smcr::R`](R) reader structure
impl crate::Readable for _SMCRrs {}
///`write(|w| ..)` method takes [`_smcr::W`](W) writer structure
impl crate::Writable for _SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _SMCR to value 0
impl crate::Resettable for _SMCRrs {}
