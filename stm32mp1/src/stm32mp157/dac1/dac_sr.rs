#[doc = "Register `DAC_SR` reader"]
pub type R = crate::R<DAC_SRrs>;
#[doc = "Register `DAC_SR` writer"]
pub type W = crate::W<DAC_SRrs>;
#[doc = "Field `DMAUDR1` reader - DMAUDR1"]
pub type DMAUDR1_R = crate::BitReader;
#[doc = "Field `DMAUDR1` writer - DMAUDR1"]
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_FLAG1` reader - CAL_FLAG1"]
pub type CAL_FLAG1_R = crate::BitReader;
#[doc = "Field `BWST1` reader - BWST1"]
pub type BWST1_R = crate::BitReader;
#[doc = "Field `DMAUDR2` reader - DMAUDR2"]
pub type DMAUDR2_R = crate::BitReader;
#[doc = "Field `DMAUDR2` writer - DMAUDR2"]
pub type DMAUDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_FLAG2` reader - CAL_FLAG2"]
pub type CAL_FLAG2_R = crate::BitReader;
#[doc = "Field `BWST2` reader - BWST2"]
pub type BWST2_R = crate::BitReader;
impl R {
    #[doc = "Bit 13 - DMAUDR1"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAL_FLAG1"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BWST1"]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - DMAUDR2"]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CAL_FLAG2"]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BWST2"]
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DMAUDR1"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<DAC_SRrs> {
        DMAUDR1_W::new(self, 13)
    }
    #[doc = "Bit 29 - DMAUDR2"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<DAC_SRrs> {
        DMAUDR2_W::new(self, 29)
    }
}
#[doc = "DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_SRrs;
impl crate::RegisterSpec for DAC_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_sr::R`](R) reader structure"]
impl crate::Readable for DAC_SRrs {}
#[doc = "`write(|w| ..)` method takes [`dac_sr::W`](W) writer structure"]
impl crate::Writable for DAC_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SR to value 0"]
impl crate::Resettable for DAC_SRrs {
    const RESET_VALUE: u32 = 0;
}
