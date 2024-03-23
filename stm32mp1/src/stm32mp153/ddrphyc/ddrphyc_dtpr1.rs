#[doc = "Register `DDRPHYC_DTPR1` reader"]
pub type R = crate::R<DDRPHYC_DTPR1rs>;
#[doc = "Register `DDRPHYC_DTPR1` writer"]
pub type W = crate::W<DDRPHYC_DTPR1rs>;
#[doc = "Field `TAOND` reader - TAOND"]
pub type TAOND_R = crate::FieldReader;
#[doc = "Field `TAOND` writer - TAOND"]
pub type TAOND_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRTW` reader - TRTW"]
pub type TRTW_R = crate::BitReader;
#[doc = "Field `TRTW` writer - TRTW"]
pub type TRTW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFAW` reader - TFAW"]
pub type TFAW_R = crate::FieldReader;
#[doc = "Field `TFAW` writer - TFAW"]
pub type TFAW_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TMOD` reader - TMOD"]
pub type TMOD_R = crate::FieldReader;
#[doc = "Field `TMOD` writer - TMOD"]
pub type TMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRTODT` reader - TRTODT"]
pub type TRTODT_R = crate::BitReader;
#[doc = "Field `TRTODT` writer - TRTODT"]
pub type TRTODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFC` reader - TRFC"]
pub type TRFC_R = crate::FieldReader;
#[doc = "Field `TRFC` writer - TRFC"]
pub type TRFC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDQSCKMIN` reader - TDQSCKMIN"]
pub type TDQSCKMIN_R = crate::FieldReader;
#[doc = "Field `TDQSCKMIN` writer - TDQSCKMIN"]
pub type TDQSCKMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TDQSCKMAX` reader - TDQSCKMAX"]
pub type TDQSCKMAX_R = crate::FieldReader;
#[doc = "Field `TDQSCKMAX` writer - TDQSCKMAX"]
pub type TDQSCKMAX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    pub fn taond(&self) -> TAOND_R {
        TAOND_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    pub fn trtw(&self) -> TRTW_R {
        TRTW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    pub fn tfaw(&self) -> TFAW_R {
        TFAW_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    pub fn trtodt(&self) -> TRTODT_R {
        TRTODT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    pub fn tdqsckmin(&self) -> TDQSCKMIN_R {
        TDQSCKMIN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    pub fn tdqsckmax(&self) -> TDQSCKMAX_R {
        TDQSCKMAX_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    #[must_use]
    pub fn taond(&mut self) -> TAOND_W<DDRPHYC_DTPR1rs> {
        TAOND_W::new(self, 0)
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    #[must_use]
    pub fn trtw(&mut self) -> TRTW_W<DDRPHYC_DTPR1rs> {
        TRTW_W::new(self, 2)
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    #[must_use]
    pub fn tfaw(&mut self) -> TFAW_W<DDRPHYC_DTPR1rs> {
        TFAW_W::new(self, 3)
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<DDRPHYC_DTPR1rs> {
        TMOD_W::new(self, 9)
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    #[must_use]
    pub fn trtodt(&mut self) -> TRTODT_W<DDRPHYC_DTPR1rs> {
        TRTODT_W::new(self, 11)
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TRFC_W<DDRPHYC_DTPR1rs> {
        TRFC_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsckmin(&mut self) -> TDQSCKMIN_W<DDRPHYC_DTPR1rs> {
        TDQSCKMIN_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsckmax(&mut self) -> TDQSCKMAX_W<DDRPHYC_DTPR1rs> {
        TDQSCKMAX_W::new(self, 27)
    }
}
#[doc = "DDRPHYC DTP register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dtpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dtpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DTPR1rs;
impl crate::RegisterSpec for DDRPHYC_DTPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dtpr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dtpr1::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR1 to value 0x0a03_0090"]
impl crate::Resettable for DDRPHYC_DTPR1rs {
    const RESET_VALUE: u32 = 0x0a03_0090;
}
