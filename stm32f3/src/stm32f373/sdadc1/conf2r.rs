#[doc = "Register `CONF2R` reader"]
pub type R = crate::R<CONF2Rrs>;
#[doc = "Register `CONF2R` writer"]
pub type W = crate::W<CONF2Rrs>;
#[doc = "Field `OFFSET2` reader - Twelve-bit calibration offset for configuration 2"]
pub type OFFSET2_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET2` writer - Twelve-bit calibration offset for configuration 2"]
pub type OFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `GAIN2` reader - Gain setting for configuration 2"]
pub type GAIN2_R = crate::FieldReader;
#[doc = "Field `GAIN2` writer - Gain setting for configuration 2"]
pub type GAIN2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SE2` reader - Single-ended mode for configuration 2"]
pub type SE2_R = crate::FieldReader;
#[doc = "Field `SE2` writer - Single-ended mode for configuration 2"]
pub type SE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMMON2` reader - Common mode for configuration 2"]
pub type COMMON2_R = crate::FieldReader;
#[doc = "Field `COMMON2` writer - Common mode for configuration 2"]
pub type COMMON2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    pub fn common2(&self) -> COMMON2_R {
        COMMON2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<CONF2Rrs> {
        OFFSET2_W::new(self, 0)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn gain2(&mut self) -> GAIN2_W<CONF2Rrs> {
        GAIN2_W::new(self, 20)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn se2(&mut self) -> SE2_W<CONF2Rrs> {
        SE2_W::new(self, 26)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn common2(&mut self) -> COMMON2_W<CONF2Rrs> {
        COMMON2_W::new(self, 30)
    }
}
#[doc = "configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF2Rrs;
impl crate::RegisterSpec for CONF2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf2r::R`](R) reader structure"]
impl crate::Readable for CONF2Rrs {}
#[doc = "`write(|w| ..)` method takes [`conf2r::W`](W) writer structure"]
impl crate::Writable for CONF2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF2R to value 0"]
impl crate::Resettable for CONF2Rrs {
    const RESET_VALUE: u32 = 0;
}
