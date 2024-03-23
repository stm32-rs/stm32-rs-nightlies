#[doc = "Register `DAC_MCR` reader"]
pub type R = crate::R<DAC_MCRrs>;
#[doc = "Register `DAC_MCR` writer"]
pub type W = crate::W<DAC_MCRrs>;
#[doc = "Field `MODE1` reader - MODE1"]
pub type MODE1_R = crate::FieldReader;
#[doc = "Field `MODE1` writer - MODE1"]
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE2` reader - MODE2"]
pub type MODE2_R = crate::FieldReader;
#[doc = "Field `MODE2` writer - MODE2"]
pub type MODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<DAC_MCRrs> {
        MODE1_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<DAC_MCRrs> {
        MODE2_W::new(self, 16)
    }
}
#[doc = "DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_MCRrs;
impl crate::RegisterSpec for DAC_MCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_mcr::R`](R) reader structure"]
impl crate::Readable for DAC_MCRrs {}
#[doc = "`write(|w| ..)` method takes [`dac_mcr::W`](W) writer structure"]
impl crate::Writable for DAC_MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_MCR to value 0"]
impl crate::Resettable for DAC_MCRrs {
    const RESET_VALUE: u32 = 0;
}
