#[doc = "Register `DDRPHYC_DTDR1` reader"]
pub type R = crate::R<DDRPHYC_DTDR1rs>;
#[doc = "Register `DDRPHYC_DTDR1` writer"]
pub type W = crate::W<DDRPHYC_DTDR1rs>;
#[doc = "Field `DTBYTE4` reader - DTBYTE4"]
pub type DTBYTE4_R = crate::FieldReader;
#[doc = "Field `DTBYTE4` writer - DTBYTE4"]
pub type DTBYTE4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTBYTE5` reader - DTBYTE5"]
pub type DTBYTE5_R = crate::FieldReader;
#[doc = "Field `DTBYTE5` writer - DTBYTE5"]
pub type DTBYTE5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTBYTE6` reader - DTBYTE6"]
pub type DTBYTE6_R = crate::FieldReader;
#[doc = "Field `DTBYTE6` writer - DTBYTE6"]
pub type DTBYTE6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTBYTE7` reader - DTBYTE7"]
pub type DTBYTE7_R = crate::FieldReader;
#[doc = "Field `DTBYTE7` writer - DTBYTE7"]
pub type DTBYTE7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    pub fn dtbyte4(&self) -> DTBYTE4_R {
        DTBYTE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    pub fn dtbyte5(&self) -> DTBYTE5_R {
        DTBYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    pub fn dtbyte6(&self) -> DTBYTE6_R {
        DTBYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    pub fn dtbyte7(&self) -> DTBYTE7_R {
        DTBYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte4(&mut self) -> DTBYTE4_W<DDRPHYC_DTDR1rs> {
        DTBYTE4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte5(&mut self) -> DTBYTE5_W<DDRPHYC_DTDR1rs> {
        DTBYTE5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte6(&mut self) -> DTBYTE6_W<DDRPHYC_DTDR1rs> {
        DTBYTE6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte7(&mut self) -> DTBYTE7_W<DDRPHYC_DTDR1rs> {
        DTBYTE7_W::new(self, 24)
    }
}
#[doc = "DDRPHYC DTD register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dtdr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dtdr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DTDR1rs;
impl crate::RegisterSpec for DDRPHYC_DTDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dtdr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dtdr1::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTDR1 to value 0x7788_bb44"]
impl crate::Resettable for DDRPHYC_DTDR1rs {
    const RESET_VALUE: u32 = 0x7788_bb44;
}
