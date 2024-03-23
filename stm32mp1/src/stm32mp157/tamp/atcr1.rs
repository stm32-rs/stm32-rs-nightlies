#[doc = "Register `ATCR1` reader"]
pub type R = crate::R<ATCR1rs>;
#[doc = "Register `ATCR1` writer"]
pub type W = crate::W<ATCR1rs>;
#[doc = "Field `TAMP1AM` reader - TAMP1AM"]
pub type TAMP1AM_R = crate::BitReader;
#[doc = "Field `TAMP1AM` writer - TAMP1AM"]
pub type TAMP1AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2AM` reader - TAMP2AM"]
pub type TAMP2AM_R = crate::BitReader;
#[doc = "Field `TAMP2AM` writer - TAMP2AM"]
pub type TAMP2AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3AM` reader - TAMP3AM"]
pub type TAMP3AM_R = crate::BitReader;
#[doc = "Field `TAMP3AM` writer - TAMP3AM"]
pub type TAMP3AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATOSEL1` reader - ATOSEL1"]
pub type ATOSEL1_R = crate::FieldReader;
#[doc = "Field `ATOSEL1` writer - ATOSEL1"]
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATOSEL2` reader - ATOSEL2"]
pub type ATOSEL2_R = crate::FieldReader;
#[doc = "Field `ATOSEL2` writer - ATOSEL2"]
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATOSEL3` reader - ATOSEL3"]
pub type ATOSEL3_R = crate::FieldReader;
#[doc = "Field `ATOSEL3` writer - ATOSEL3"]
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATCKSEL` reader - ATCKSEL"]
pub type ATCKSEL_R = crate::FieldReader;
#[doc = "Field `ATCKSEL` writer - ATCKSEL"]
pub type ATCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATPER` reader - ATPER"]
pub type ATPER_R = crate::FieldReader;
#[doc = "Field `ATPER` writer - ATPER"]
pub type ATPER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSHARE` reader - ATOSHARE"]
pub type ATOSHARE_R = crate::BitReader;
#[doc = "Field `ATOSHARE` writer - ATOSHARE"]
pub type ATOSHARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTEN` reader - FLTEN"]
pub type FLTEN_R = crate::BitReader;
#[doc = "Field `FLTEN` writer - FLTEN"]
pub type FLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - ATCKSEL"]
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - ATPER"]
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<ATCR1rs> {
        TAMP1AM_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<ATCR1rs> {
        TAMP2AM_W::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3am(&mut self) -> TAMP3AM_W<ATCR1rs> {
        TAMP3AM_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<ATCR1rs> {
        ATOSEL1_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<ATCR1rs> {
        ATOSEL2_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<ATCR1rs> {
        ATOSEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - ATCKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn atcksel(&mut self) -> ATCKSEL_W<ATCR1rs> {
        ATCKSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - ATPER"]
    #[inline(always)]
    #[must_use]
    pub fn atper(&mut self) -> ATPER_W<ATCR1rs> {
        ATPER_W::new(self, 24)
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    #[must_use]
    pub fn atoshare(&mut self) -> ATOSHARE_W<ATCR1rs> {
        ATOSHARE_W::new(self, 30)
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<ATCR1rs> {
        FLTEN_W::new(self, 31)
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATCR1rs;
impl crate::RegisterSpec for ATCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atcr1::R`](R) reader structure"]
impl crate::Readable for ATCR1rs {}
#[doc = "`write(|w| ..)` method takes [`atcr1::W`](W) writer structure"]
impl crate::Writable for ATCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATCR1 to value 0x0007_0000"]
impl crate::Resettable for ATCR1rs {
    const RESET_VALUE: u32 = 0x0007_0000;
}
