#[doc = "Register `DAC_SHHR` reader"]
pub type R = crate::R<DAC_SHHRrs>;
#[doc = "Register `DAC_SHHR` writer"]
pub type W = crate::W<DAC_SHHRrs>;
#[doc = "Field `THOLD1` reader - THOLD1"]
pub type THOLD1_R = crate::FieldReader<u16>;
#[doc = "Field `THOLD1` writer - THOLD1"]
pub type THOLD1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `THOLD2` reader - THOLD2"]
pub type THOLD2_R = crate::FieldReader<u16>;
#[doc = "Field `THOLD2` writer - THOLD2"]
pub type THOLD2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - THOLD1"]
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - THOLD2"]
    #[inline(always)]
    pub fn thold2(&self) -> THOLD2_R {
        THOLD2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - THOLD1"]
    #[inline(always)]
    #[must_use]
    pub fn thold1(&mut self) -> THOLD1_W<DAC_SHHRrs> {
        THOLD1_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - THOLD2"]
    #[inline(always)]
    #[must_use]
    pub fn thold2(&mut self) -> THOLD2_W<DAC_SHHRrs> {
        THOLD2_W::new(self, 16)
    }
}
#[doc = "DAC sample and hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_SHHRrs;
impl crate::RegisterSpec for DAC_SHHRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_shhr::R`](R) reader structure"]
impl crate::Readable for DAC_SHHRrs {}
#[doc = "`write(|w| ..)` method takes [`dac_shhr::W`](W) writer structure"]
impl crate::Writable for DAC_SHHRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SHHR to value 0x0001_0001"]
impl crate::Resettable for DAC_SHHRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
