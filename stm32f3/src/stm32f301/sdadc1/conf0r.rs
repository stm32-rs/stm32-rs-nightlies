#[doc = "Register `CONF0R` reader"]
pub type R = crate::R<CONF0Rrs>;
#[doc = "Register `CONF0R` writer"]
pub type W = crate::W<CONF0Rrs>;
#[doc = "Field `OFFSET0` reader - Twelve-bit calibration offset for configuration 0"]
pub type OFFSET0_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET0` writer - Twelve-bit calibration offset for configuration 0"]
pub type OFFSET0_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `GAIN0` reader - Gain setting for configuration 0"]
pub type GAIN0_R = crate::FieldReader;
#[doc = "Field `GAIN0` writer - Gain setting for configuration 0"]
pub type GAIN0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SE0` reader - Single-ended mode for configuration 0"]
pub type SE0_R = crate::FieldReader;
#[doc = "Field `SE0` writer - Single-ended mode for configuration 0"]
pub type SE0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMMON0` reader - Common mode for configuration 0"]
pub type COMMON0_R = crate::FieldReader;
#[doc = "Field `COMMON0` writer - Common mode for configuration 0"]
pub type COMMON0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    pub fn offset0(&self) -> OFFSET0_R {
        OFFSET0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    pub fn common0(&self) -> COMMON0_R {
        COMMON0_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn offset0(&mut self) -> OFFSET0_W<CONF0Rrs> {
        OFFSET0_W::new(self, 0)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn gain0(&mut self) -> GAIN0_W<CONF0Rrs> {
        GAIN0_W::new(self, 20)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn se0(&mut self) -> SE0_W<CONF0Rrs> {
        SE0_W::new(self, 26)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn common0(&mut self) -> COMMON0_W<CONF0Rrs> {
        COMMON0_W::new(self, 30)
    }
}
#[doc = "configuration 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0Rrs;
impl crate::RegisterSpec for CONF0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0r::R`](R) reader structure"]
impl crate::Readable for CONF0Rrs {}
#[doc = "`write(|w| ..)` method takes [`conf0r::W`](W) writer structure"]
impl crate::Writable for CONF0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF0R to value 0"]
impl crate::Resettable for CONF0Rrs {
    const RESET_VALUE: u32 = 0;
}
