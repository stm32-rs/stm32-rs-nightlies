#[doc = "Register `STR2` reader"]
pub type R = crate::R<STR2rs>;
#[doc = "Register `STR2` writer"]
pub type W = crate::W<STR2rs>;
#[doc = "Field `STRSTDATA2` reader - DAC Channel 2 Sawtooth reset value"]
pub type STRSTDATA2_R = crate::FieldReader<u16>;
#[doc = "Field `STRSTDATA2` writer - DAC Channel 2 Sawtooth reset value"]
pub type STRSTDATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `STDIR2` reader - DAC Channel2 Sawtooth direction setting"]
pub type STDIR2_R = crate::BitReader;
#[doc = "Field `STDIR2` writer - DAC Channel2 Sawtooth direction setting"]
pub type STDIR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STINCDATA2` reader - DAC CH2 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA2_R = crate::FieldReader<u16>;
#[doc = "Field `STINCDATA2` writer - DAC CH2 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata2(&self) -> STRSTDATA2_R {
        STRSTDATA2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir2(&self) -> STDIR2_R {
        STDIR2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata2(&self) -> STINCDATA2_R {
        STINCDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    #[must_use]
    pub fn strstdata2(&mut self) -> STRSTDATA2_W<STR2rs> {
        STRSTDATA2_W::new(self, 0)
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    #[must_use]
    pub fn stdir2(&mut self) -> STDIR2_W<STR2rs> {
        STDIR2_W::new(self, 12)
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    #[must_use]
    pub fn stincdata2(&mut self) -> STINCDATA2_W<STR2rs> {
        STINCDATA2_W::new(self, 16)
    }
}
#[doc = "Sawtooth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`str2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`str2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STR2rs;
impl crate::RegisterSpec for STR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str2::R`](R) reader structure"]
impl crate::Readable for STR2rs {}
#[doc = "`write(|w| ..)` method takes [`str2::W`](W) writer structure"]
impl crate::Writable for STR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STR2 to value 0"]
impl crate::Resettable for STR2rs {
    const RESET_VALUE: u32 = 0;
}
