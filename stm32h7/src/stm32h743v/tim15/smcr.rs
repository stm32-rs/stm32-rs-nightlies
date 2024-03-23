#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCRrs>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCRrs>;
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SMS_R = crate::FieldReader;
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS_2_0` reader - Trigger selection"]
pub type TS_2_0_R = crate::FieldReader;
#[doc = "Field `TS_2_0` writer - Trigger selection"]
pub type TS_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_3` reader - Slave mode selection bit 3"]
pub type SMS_3_R = crate::BitReader;
#[doc = "Field `SMS_3` writer - Slave mode selection bit 3"]
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_4_3` reader - Trigger selection - bit 4:3"]
pub type TS_4_3_R = crate::FieldReader;
#[doc = "Field `TS_4_3` writer - Trigger selection - bit 4:3"]
pub type TS_4_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts_2_0(&self) -> TS_2_0_R {
        TS_2_0_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection bit 3"]
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection - bit 4:3"]
    #[inline(always)]
    pub fn ts_4_3(&self) -> TS_4_3_R {
        TS_4_3_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<SMCRrs> {
        SMS_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn ts_2_0(&mut self) -> TS_2_0_W<SMCRrs> {
        TS_2_0_W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bit 16 - Slave mode selection bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sms_3(&mut self) -> SMS_3_W<SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection - bit 4:3"]
    #[inline(always)]
    #[must_use]
    pub fn ts_4_3(&mut self) -> TS_4_3_W<SMCRrs> {
        TS_4_3_W::new(self, 20)
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
