#[doc = "Register `CONF1R` reader"]
pub type R = crate::R<CONF1Rrs>;
#[doc = "Register `CONF1R` writer"]
pub type W = crate::W<CONF1Rrs>;
#[doc = "Field `OFFSET1` reader - Twelve-bit calibration offset for configuration 1"]
pub type OFFSET1_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET1` writer - Twelve-bit calibration offset for configuration 1"]
pub type OFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `GAIN1` reader - Gain setting for configuration 1"]
pub type GAIN1_R = crate::FieldReader;
#[doc = "Field `GAIN1` writer - Gain setting for configuration 1"]
pub type GAIN1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SE1` reader - Single-ended mode for configuration 1"]
pub type SE1_R = crate::FieldReader;
#[doc = "Field `SE1` writer - Single-ended mode for configuration 1"]
pub type SE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMMON1` reader - Common mode for configuration 1"]
pub type COMMON1_R = crate::FieldReader;
#[doc = "Field `COMMON1` writer - Common mode for configuration 1"]
pub type COMMON1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 1"]
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 1"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 1"]
    #[inline(always)]
    pub fn common1(&self) -> COMMON1_R {
        COMMON1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<CONF1Rrs> {
        OFFSET1_W::new(self, 0)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn gain1(&mut self) -> GAIN1_W<CONF1Rrs> {
        GAIN1_W::new(self, 20)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn se1(&mut self) -> SE1_W<CONF1Rrs> {
        SE1_W::new(self, 26)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn common1(&mut self) -> COMMON1_W<CONF1Rrs> {
        COMMON1_W::new(self, 30)
    }
}
#[doc = "configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1Rrs;
impl crate::RegisterSpec for CONF1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1r::R`](R) reader structure"]
impl crate::Readable for CONF1Rrs {}
#[doc = "`write(|w| ..)` method takes [`conf1r::W`](W) writer structure"]
impl crate::Writable for CONF1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1R to value 0"]
impl crate::Resettable for CONF1Rrs {
    const RESET_VALUE: u32 = 0;
}
