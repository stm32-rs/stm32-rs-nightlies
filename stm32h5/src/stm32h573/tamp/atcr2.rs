#[doc = "Register `ATCR2` reader"]
pub type R = crate::R<ATCR2rs>;
#[doc = "Register `ATCR2` writer"]
pub type W = crate::W<ATCR2rs>;
#[doc = "Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL1_R = crate::FieldReader;
#[doc = "Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL2_R = crate::FieldReader;
#[doc = "Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL3_R = crate::FieldReader;
#[doc = "Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSEL4` reader - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL4_R = crate::FieldReader;
#[doc = "Field `ATOSEL4` writer - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSEL5` reader - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
pub type ATOSEL5_R = crate::FieldReader;
#[doc = "Field `ATOSEL5` writer - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
pub type ATOSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSEL6` reader - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
pub type ATOSEL6_R = crate::FieldReader;
#[doc = "Field `ATOSEL6` writer - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
pub type ATOSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSEL7` reader - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
pub type ATOSEL7_R = crate::FieldReader;
#[doc = "Field `ATOSEL7` writer - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
pub type ATOSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSEL8` reader - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
pub type ATOSEL8_R = crate::FieldReader;
#[doc = "Field `ATOSEL8` writer - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
pub type ATOSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:10 - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn atosel5(&self) -> ATOSEL5_R {
        ATOSEL5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn atosel6(&self) -> ATOSEL6_R {
        ATOSEL6_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn atosel7(&self) -> ATOSEL7_R {
        ATOSEL7_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn atosel8(&self) -> ATOSEL8_R {
        ATOSEL8_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<ATCR2rs> {
        ATOSEL1_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<ATCR2rs> {
        ATOSEL2_W::new(self, 11)
    }
    #[doc = "Bits 14:16 - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<ATCR2rs> {
        ATOSEL3_W::new(self, 14)
    }
    #[doc = "Bits 17:19 - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    #[must_use]
    pub fn atosel4(&mut self) -> ATOSEL4_W<ATCR2rs> {
        ATOSEL4_W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    #[must_use]
    pub fn atosel5(&mut self) -> ATOSEL5_W<ATCR2rs> {
        ATOSEL5_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    #[must_use]
    pub fn atosel6(&mut self) -> ATOSEL6_W<ATCR2rs> {
        ATOSEL6_W::new(self, 23)
    }
    #[doc = "Bits 26:28 - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    #[must_use]
    pub fn atosel7(&mut self) -> ATOSEL7_W<ATCR2rs> {
        ATOSEL7_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    #[must_use]
    pub fn atosel8(&mut self) -> ATOSEL8_W<ATCR2rs> {
        ATOSEL8_W::new(self, 29)
    }
}
#[doc = "TAMP active tamper control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATCR2rs;
impl crate::RegisterSpec for ATCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atcr2::R`](R) reader structure"]
impl crate::Readable for ATCR2rs {}
#[doc = "`write(|w| ..)` method takes [`atcr2::W`](W) writer structure"]
impl crate::Writable for ATCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATCR2 to value 0"]
impl crate::Resettable for ATCR2rs {
    const RESET_VALUE: u32 = 0;
}
