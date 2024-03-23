#[doc = "Register `STMODR` reader"]
pub type R = crate::R<STMODRrs>;
#[doc = "Register `STMODR` writer"]
pub type W = crate::W<STMODRrs>;
#[doc = "Field `STRSTTRIGSEL1` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL1_R = crate::FieldReader;
#[doc = "Field `STRSTTRIGSEL1` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STINCTRIGSEL1` reader - DAC Channel 1 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL1_R = crate::FieldReader;
#[doc = "Field `STINCTRIGSEL1` writer - DAC Channel 1 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STRSTTRIGSEL2` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL2_R = crate::FieldReader;
#[doc = "Field `STRSTTRIGSEL2` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STINCTRIGSEL2` reader - DAC Channel 2 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL2_R = crate::FieldReader;
#[doc = "Field `STINCTRIGSEL2` writer - DAC Channel 2 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&self) -> STRSTTRIGSEL1_R {
        STRSTTRIGSEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&self) -> STINCTRIGSEL1_R {
        STINCTRIGSEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&self) -> STRSTTRIGSEL2_R {
        STRSTTRIGSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&self) -> STINCTRIGSEL2_R {
        STINCTRIGSEL2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn strsttrigsel1(&mut self) -> STRSTTRIGSEL1_W<STMODRrs> {
        STRSTTRIGSEL1_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn stinctrigsel1(&mut self) -> STINCTRIGSEL1_W<STMODRrs> {
        STINCTRIGSEL1_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn strsttrigsel2(&mut self) -> STRSTTRIGSEL2_W<STMODRrs> {
        STRSTTRIGSEL2_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn stinctrigsel2(&mut self) -> STINCTRIGSEL2_W<STMODRrs> {
        STINCTRIGSEL2_W::new(self, 24)
    }
}
#[doc = "Sawtooth Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmodr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmodr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STMODRrs;
impl crate::RegisterSpec for STMODRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmodr::R`](R) reader structure"]
impl crate::Readable for STMODRrs {}
#[doc = "`write(|w| ..)` method takes [`stmodr::W`](W) writer structure"]
impl crate::Writable for STMODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMODR to value 0"]
impl crate::Resettable for STMODRrs {
    const RESET_VALUE: u32 = 0;
}
