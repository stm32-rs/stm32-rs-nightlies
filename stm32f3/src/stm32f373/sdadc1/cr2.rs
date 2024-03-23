#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `ADON` reader - SDADC enable"]
pub type ADON_R = crate::BitReader;
#[doc = "Field `ADON` writer - SDADC enable"]
pub type ADON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALIBCNT` reader - Number of calibration sequences to be performed (number of valid configurations)"]
pub type CALIBCNT_R = crate::FieldReader;
#[doc = "Field `CALIBCNT` writer - Number of calibration sequences to be performed (number of valid configurations)"]
pub type CALIBCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STARTCALIB` reader - Start calibration"]
pub type STARTCALIB_R = crate::BitReader;
#[doc = "Field `STARTCALIB` writer - Start calibration"]
pub type STARTCALIB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JCONT` reader - Continuous mode selection for injected conversions"]
pub type JCONT_R = crate::BitReader;
#[doc = "Field `JCONT` writer - Continuous mode selection for injected conversions"]
pub type JCONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JDS` reader - Delay start of injected conversions."]
pub type JDS_R = crate::BitReader;
#[doc = "Field `JDS` writer - Delay start of injected conversions."]
pub type JDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_R = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels"]
pub type JSWSTART_R = crate::BitReader;
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels"]
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCH` reader - Regular channel selection"]
pub type RCH_R = crate::FieldReader;
#[doc = "Field `RCH` writer - Regular channel selection"]
pub type RCH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions"]
pub type RCONT_R = crate::BitReader;
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions"]
pub type RCONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel"]
pub type RSWSTART_R = crate::BitReader;
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel"]
pub type RSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST` reader - Fast conversion mode selection"]
pub type FAST_R = crate::BitReader;
#[doc = "Field `FAST` writer - Fast conversion mode selection"]
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SDADC enable"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Number of calibration sequences to be performed (number of valid configurations)"]
    #[inline(always)]
    pub fn calibcnt(&self) -> CALIBCNT_R {
        CALIBCNT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Start calibration"]
    #[inline(always)]
    pub fn startcalib(&self) -> STARTCALIB_R {
        STARTCALIB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Continuous mode selection for injected conversions"]
    #[inline(always)]
    pub fn jcont(&self) -> JCONT_R {
        JCONT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Delay start of injected conversions."]
    #[inline(always)]
    pub fn jds(&self) -> JDS_R {
        JDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast conversion mode selection"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> ADON_W<CR2rs> {
        ADON_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Number of calibration sequences to be performed (number of valid configurations)"]
    #[inline(always)]
    #[must_use]
    pub fn calibcnt(&mut self) -> CALIBCNT_W<CR2rs> {
        CALIBCNT_W::new(self, 1)
    }
    #[doc = "Bit 4 - Start calibration"]
    #[inline(always)]
    #[must_use]
    pub fn startcalib(&mut self) -> STARTCALIB_W<CR2rs> {
        STARTCALIB_W::new(self, 4)
    }
    #[doc = "Bit 5 - Continuous mode selection for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jcont(&mut self) -> JCONT_W<CR2rs> {
        JCONT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Delay start of injected conversions."]
    #[inline(always)]
    #[must_use]
    pub fn jds(&mut self) -> JDS_W<CR2rs> {
        JDS_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<CR2rs> {
        JEXTSEL_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<CR2rs> {
        JEXTEN_W::new(self, 13)
    }
    #[doc = "Bit 15 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<CR2rs> {
        JSWSTART_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Regular channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn rch(&mut self) -> RCH_W<CR2rs> {
        RCH_W::new(self, 16)
    }
    #[doc = "Bit 22 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<CR2rs> {
        RCONT_W::new(self, 22)
    }
    #[doc = "Bit 23 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn rswstart(&mut self) -> RSWSTART_W<CR2rs> {
        RSWSTART_W::new(self, 23)
    }
    #[doc = "Bit 24 - Fast conversion mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<CR2rs> {
        FAST_W::new(self, 24)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
