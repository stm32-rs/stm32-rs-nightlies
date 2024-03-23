#[doc = "Register `FMC_CSQAR1` reader"]
pub type R = crate::R<FMC_CSQAR1rs>;
#[doc = "Register `FMC_CSQAR1` writer"]
pub type W = crate::W<FMC_CSQAR1rs>;
#[doc = "Field `ADDC1` reader - ADDC1"]
pub type ADDC1_R = crate::FieldReader;
#[doc = "Field `ADDC1` writer - ADDC1"]
pub type ADDC1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDC2` reader - ADDC2"]
pub type ADDC2_R = crate::FieldReader;
#[doc = "Field `ADDC2` writer - ADDC2"]
pub type ADDC2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDC3` reader - ADDC3"]
pub type ADDC3_R = crate::FieldReader;
#[doc = "Field `ADDC3` writer - ADDC3"]
pub type ADDC3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDC4` reader - ADDC4"]
pub type ADDC4_R = crate::FieldReader;
#[doc = "Field `ADDC4` writer - ADDC4"]
pub type ADDC4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ADDC1"]
    #[inline(always)]
    pub fn addc1(&self) -> ADDC1_R {
        ADDC1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ADDC2"]
    #[inline(always)]
    pub fn addc2(&self) -> ADDC2_R {
        ADDC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ADDC3"]
    #[inline(always)]
    pub fn addc3(&self) -> ADDC3_R {
        ADDC3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ADDC4"]
    #[inline(always)]
    pub fn addc4(&self) -> ADDC4_R {
        ADDC4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDC1"]
    #[inline(always)]
    #[must_use]
    pub fn addc1(&mut self) -> ADDC1_W<FMC_CSQAR1rs> {
        ADDC1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ADDC2"]
    #[inline(always)]
    #[must_use]
    pub fn addc2(&mut self) -> ADDC2_W<FMC_CSQAR1rs> {
        ADDC2_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - ADDC3"]
    #[inline(always)]
    #[must_use]
    pub fn addc3(&mut self) -> ADDC3_W<FMC_CSQAR1rs> {
        ADDC3_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - ADDC4"]
    #[inline(always)]
    #[must_use]
    pub fn addc4(&mut self) -> ADDC4_W<FMC_CSQAR1rs> {
        ADDC4_W::new(self, 24)
    }
}
#[doc = "This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQAR1rs;
impl crate::RegisterSpec for FMC_CSQAR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqar1::R`](R) reader structure"]
impl crate::Readable for FMC_CSQAR1rs {}
#[doc = "`write(|w| ..)` method takes [`fmc_csqar1::W`](W) writer structure"]
impl crate::Writable for FMC_CSQAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQAR1 to value 0"]
impl crate::Resettable for FMC_CSQAR1rs {
    const RESET_VALUE: u32 = 0;
}
