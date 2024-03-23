#[doc = "Register `STR1` reader"]
pub type R = crate::R<STR1rs>;
#[doc = "Register `STR1` writer"]
pub type W = crate::W<STR1rs>;
#[doc = "Field `STRSTDATA1` reader - DAC Channel 1 Sawtooth reset value"]
pub type STRSTDATA1_R = crate::FieldReader<u16>;
#[doc = "Field `STRSTDATA1` writer - DAC Channel 1 Sawtooth reset value"]
pub type STRSTDATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `STDIR1` reader - DAC Channel1 Sawtooth direction setting"]
pub type STDIR1_R = crate::BitReader;
#[doc = "Field `STDIR1` writer - DAC Channel1 Sawtooth direction setting"]
pub type STDIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STINCDATA1` reader - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA1_R = crate::FieldReader<u16>;
#[doc = "Field `STINCDATA1` writer - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata1(&self) -> STRSTDATA1_R {
        STRSTDATA1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir1(&self) -> STDIR1_R {
        STDIR1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata1(&self) -> STINCDATA1_R {
        STINCDATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    #[must_use]
    pub fn strstdata1(&mut self) -> STRSTDATA1_W<STR1rs> {
        STRSTDATA1_W::new(self, 0)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    #[must_use]
    pub fn stdir1(&mut self) -> STDIR1_W<STR1rs> {
        STDIR1_W::new(self, 12)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    #[must_use]
    pub fn stincdata1(&mut self) -> STINCDATA1_W<STR1rs> {
        STINCDATA1_W::new(self, 16)
    }
}
#[doc = "Sawtooth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`str1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`str1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STR1rs;
impl crate::RegisterSpec for STR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str1::R`](R) reader structure"]
impl crate::Readable for STR1rs {}
#[doc = "`write(|w| ..)` method takes [`str1::W`](W) writer structure"]
impl crate::Writable for STR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STR1 to value 0"]
impl crate::Resettable for STR1rs {
    const RESET_VALUE: u32 = 0;
}
