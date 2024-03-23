#[doc = "Register `TIM6_CR2` reader"]
pub type R = crate::R<TIM6_CR2rs>;
#[doc = "Register `TIM6_CR2` writer"]
pub type W = crate::W<TIM6_CR2rs>;
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
#[doc = "Field `OIS2N` reader - OIS2N"]
pub type OIS2N_R = crate::BitReader;
#[doc = "Field `OIS2N` writer - OIS2N"]
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - OIS3"]
pub type OIS3_R = crate::BitReader;
#[doc = "Field `OIS3` writer - OIS3"]
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - OIS3N"]
pub type OIS3N_R = crate::BitReader;
#[doc = "Field `OIS3N` writer - OIS3N"]
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - OIS4"]
pub type OIS4_R = crate::BitReader;
#[doc = "Field `OIS4` writer - OIS4"]
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - OIS5"]
pub type OIS5_R = crate::BitReader;
#[doc = "Field `OIS5` writer - OIS5"]
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - OIS6"]
pub type OIS6_R = crate::BitReader;
#[doc = "Field `OIS6` writer - OIS6"]
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS2` reader - MMS2"]
pub type MMS2_R = crate::FieldReader;
#[doc = "Field `MMS2` writer - MMS2"]
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bit 11 - OIS2N"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OIS3"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OIS3N"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OIS4"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - OIS5"]
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - OIS6"]
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - MMS2"]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<TIM6_CR2rs> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<TIM6_CR2rs> {
        CCUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<TIM6_CR2rs> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<TIM6_CR2rs> {
        MMS_W::new(self, 4)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<TIM6_CR2rs> {
        TI1S_W::new(self, 7)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<TIM6_CR2rs> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<TIM6_CR2rs> {
        OIS1N_W::new(self, 9)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<TIM6_CR2rs> {
        OIS2_W::new(self, 10)
    }
    #[doc = "Bit 11 - OIS2N"]
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<TIM6_CR2rs> {
        OIS2N_W::new(self, 11)
    }
    #[doc = "Bit 12 - OIS3"]
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<TIM6_CR2rs> {
        OIS3_W::new(self, 12)
    }
    #[doc = "Bit 13 - OIS3N"]
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<TIM6_CR2rs> {
        OIS3N_W::new(self, 13)
    }
    #[doc = "Bit 14 - OIS4"]
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<TIM6_CR2rs> {
        OIS4_W::new(self, 14)
    }
    #[doc = "Bit 16 - OIS5"]
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<TIM6_CR2rs> {
        OIS5_W::new(self, 16)
    }
    #[doc = "Bit 18 - OIS6"]
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<TIM6_CR2rs> {
        OIS6_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - MMS2"]
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<TIM6_CR2rs> {
        MMS2_W::new(self, 20)
    }
}
#[doc = "TIM6 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM6_CR2rs;
impl crate::RegisterSpec for TIM6_CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim6_cr2::R`](R) reader structure"]
impl crate::Readable for TIM6_CR2rs {}
#[doc = "`write(|w| ..)` method takes [`tim6_cr2::W`](W) writer structure"]
impl crate::Writable for TIM6_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM6_CR2 to value 0"]
impl crate::Resettable for TIM6_CR2rs {
    const RESET_VALUE: u32 = 0;
}
