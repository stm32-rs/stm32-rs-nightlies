#[doc = "Register `DDRPHYC_DTPR0` reader"]
pub type R = crate::R<DDRPHYC_DTPR0rs>;
#[doc = "Register `DDRPHYC_DTPR0` writer"]
pub type W = crate::W<DDRPHYC_DTPR0rs>;
#[doc = "Field `TMRD` reader - TMRD"]
pub type TMRD_R = crate::FieldReader;
#[doc = "Field `TMRD` writer - TMRD"]
pub type TMRD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRTP` reader - TRTP"]
pub type TRTP_R = crate::FieldReader;
#[doc = "Field `TRTP` writer - TRTP"]
pub type TRTP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TWTR` reader - TWTR"]
pub type TWTR_R = crate::FieldReader;
#[doc = "Field `TWTR` writer - TWTR"]
pub type TWTR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRP` reader - TRP"]
pub type TRP_R = crate::FieldReader;
#[doc = "Field `TRP` writer - TRP"]
pub type TRP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCD` reader - TRCD"]
pub type TRCD_R = crate::FieldReader;
#[doc = "Field `TRCD` writer - TRCD"]
pub type TRCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRAS` reader - TRAS"]
pub type TRAS_R = crate::FieldReader;
#[doc = "Field `TRAS` writer - TRAS"]
pub type TRAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRRD` reader - TRRD"]
pub type TRRD_R = crate::FieldReader;
#[doc = "Field `TRRD` writer - TRRD"]
pub type TRRD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRC` reader - TRC"]
pub type TRC_R = crate::FieldReader;
#[doc = "Field `TRC` writer - TRC"]
pub type TRC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TCCD` reader - TCCD"]
pub type TCCD_R = crate::BitReader;
#[doc = "Field `TCCD` writer - TCCD"]
pub type TCCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - TMRD"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - TRTP"]
    #[inline(always)]
    pub fn trtp(&self) -> TRTP_R {
        TRTP_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - TWTR"]
    #[inline(always)]
    pub fn twtr(&self) -> TWTR_R {
        TWTR_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - TRP"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TRCD"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - TRAS"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:24 - TRRD"]
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:30 - TRC"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - TCCD"]
    #[inline(always)]
    pub fn tccd(&self) -> TCCD_R {
        TCCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TMRD"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd(&mut self) -> TMRD_W<DDRPHYC_DTPR0rs> {
        TMRD_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - TRTP"]
    #[inline(always)]
    #[must_use]
    pub fn trtp(&mut self) -> TRTP_W<DDRPHYC_DTPR0rs> {
        TRTP_W::new(self, 2)
    }
    #[doc = "Bits 5:7 - TWTR"]
    #[inline(always)]
    #[must_use]
    pub fn twtr(&mut self) -> TWTR_W<DDRPHYC_DTPR0rs> {
        TWTR_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - TRP"]
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TRP_W<DDRPHYC_DTPR0rs> {
        TRP_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - TRCD"]
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TRCD_W<DDRPHYC_DTPR0rs> {
        TRCD_W::new(self, 12)
    }
    #[doc = "Bits 16:20 - TRAS"]
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TRAS_W<DDRPHYC_DTPR0rs> {
        TRAS_W::new(self, 16)
    }
    #[doc = "Bits 21:24 - TRRD"]
    #[inline(always)]
    #[must_use]
    pub fn trrd(&mut self) -> TRRD_W<DDRPHYC_DTPR0rs> {
        TRRD_W::new(self, 21)
    }
    #[doc = "Bits 25:30 - TRC"]
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TRC_W<DDRPHYC_DTPR0rs> {
        TRC_W::new(self, 25)
    }
    #[doc = "Bit 31 - TCCD"]
    #[inline(always)]
    #[must_use]
    pub fn tccd(&mut self) -> TCCD_W<DDRPHYC_DTPR0rs> {
        TCCD_W::new(self, 31)
    }
}
#[doc = "DDRPHYC DTP register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dtpr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dtpr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DTPR0rs;
impl crate::RegisterSpec for DDRPHYC_DTPR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dtpr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dtpr0::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR0 to value 0x3012_666e"]
impl crate::Resettable for DDRPHYC_DTPR0rs {
    const RESET_VALUE: u32 = 0x3012_666e;
}
