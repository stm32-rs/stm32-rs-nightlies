#[doc = "Register `TIM15_CR2` reader"]
pub type R = crate::R<TIM15_CR2rs>;
#[doc = "Register `TIM15_CR2` writer"]
pub type W = crate::W<TIM15_CR2rs>;
#[doc = "Field `CCPC` reader - CCPC"]
pub type CCPC_R = crate::BitReader;
#[doc = "Field `CCPC` writer - CCPC"]
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - CCUS"]
pub type CCUS_R = crate::BitReader;
#[doc = "Field `CCUS` writer - CCUS"]
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - CCDS"]
pub type CCDS_R = crate::BitReader;
#[doc = "Field `CCDS` writer - CCDS"]
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS` reader - MMS"]
pub type MMS_R = crate::FieldReader;
#[doc = "Field `MMS` writer - MMS"]
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1S` reader - TI1S"]
pub type TI1S_R = crate::BitReader;
#[doc = "Field `TI1S` writer - TI1S"]
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1` reader - OIS1"]
pub type OIS1_R = crate::BitReader;
#[doc = "Field `OIS1` writer - OIS1"]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - OIS1N"]
pub type OIS1N_R = crate::BitReader;
#[doc = "Field `OIS1N` writer - OIS1N"]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2` reader - OIS2"]
pub type OIS2_R = crate::BitReader;
#[doc = "Field `OIS2` writer - OIS2"]
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<TIM15_CR2rs> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<TIM15_CR2rs> {
        CCUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<TIM15_CR2rs> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<TIM15_CR2rs> {
        MMS_W::new(self, 4)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<TIM15_CR2rs> {
        TI1S_W::new(self, 7)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<TIM15_CR2rs> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<TIM15_CR2rs> {
        OIS1N_W::new(self, 9)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<TIM15_CR2rs> {
        OIS2_W::new(self, 10)
    }
}
#[doc = "TIM15 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM15_CR2rs;
impl crate::RegisterSpec for TIM15_CR2rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim15_cr2::R`](R) reader structure"]
impl crate::Readable for TIM15_CR2rs {}
#[doc = "`write(|w| ..)` method takes [`tim15_cr2::W`](W) writer structure"]
impl crate::Writable for TIM15_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM15_CR2 to value 0"]
impl crate::Resettable for TIM15_CR2rs {
    const RESET_VALUE: u16 = 0;
}
