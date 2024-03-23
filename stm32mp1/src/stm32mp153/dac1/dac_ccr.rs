#[doc = "Register `DAC_CCR` reader"]
pub type R = crate::R<DAC_CCRrs>;
#[doc = "Register `DAC_CCR` writer"]
pub type W = crate::W<DAC_CCRrs>;
#[doc = "Field `OTRIM1` reader - OTRIM1"]
pub type OTRIM1_R = crate::FieldReader;
#[doc = "Field `OTRIM1` writer - OTRIM1"]
pub type OTRIM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OTRIM2` reader - OTRIM2"]
pub type OTRIM2_R = crate::FieldReader;
#[doc = "Field `OTRIM2` writer - OTRIM2"]
pub type OTRIM2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - OTRIM1"]
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - OTRIM2"]
    #[inline(always)]
    pub fn otrim2(&self) -> OTRIM2_R {
        OTRIM2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - OTRIM1"]
    #[inline(always)]
    #[must_use]
    pub fn otrim1(&mut self) -> OTRIM1_W<DAC_CCRrs> {
        OTRIM1_W::new(self, 0)
    }
    #[doc = "Bits 16:20 - OTRIM2"]
    #[inline(always)]
    #[must_use]
    pub fn otrim2(&mut self) -> OTRIM2_W<DAC_CCRrs> {
        OTRIM2_W::new(self, 16)
    }
}
#[doc = "DAC calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_CCRrs;
impl crate::RegisterSpec for DAC_CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_ccr::R`](R) reader structure"]
impl crate::Readable for DAC_CCRrs {}
#[doc = "`write(|w| ..)` method takes [`dac_ccr::W`](W) writer structure"]
impl crate::Writable for DAC_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_CCR to value 0"]
impl crate::Resettable for DAC_CCRrs {
    const RESET_VALUE: u32 = 0;
}
