#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_R = crate::BitReader;
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_R = crate::BitReader;
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader;
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1` reader - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
pub type OIS1_R = crate::BitReader;
#[doc = "Field `OIS1` writer - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
pub type OIS1N_R = crate::BitReader;
#[doc = "Field `OIS1N` writer - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<CR2rs> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<CR2rs> {
        CCUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<CR2rs> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bit 8 - Output Idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<CR2rs> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BKR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<CR2rs> {
        OIS1N_W::new(self, 9)
    }
}
#[doc = "TIM17 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u16 = 0;
}
