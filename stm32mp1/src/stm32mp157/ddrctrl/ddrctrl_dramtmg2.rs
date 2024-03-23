#[doc = "Register `DDRCTRL_DRAMTMG2` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG2rs>;
#[doc = "Register `DDRCTRL_DRAMTMG2` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG2rs>;
#[doc = "Field `WR2RD` reader - WR2RD"]
pub type WR2RD_R = crate::FieldReader;
#[doc = "Field `WR2RD` writer - WR2RD"]
pub type WR2RD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD2WR` reader - RD2WR"]
pub type RD2WR_R = crate::FieldReader;
#[doc = "Field `RD2WR` writer - RD2WR"]
pub type RD2WR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `READ_LATENCY` reader - READ_LATENCY"]
pub type READ_LATENCY_R = crate::FieldReader;
#[doc = "Field `READ_LATENCY` writer - READ_LATENCY"]
pub type READ_LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRITE_LATENCY` reader - WRITE_LATENCY"]
pub type WRITE_LATENCY_R = crate::FieldReader;
#[doc = "Field `WRITE_LATENCY` writer - WRITE_LATENCY"]
pub type WRITE_LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - WR2RD"]
    #[inline(always)]
    pub fn wr2rd(&self) -> WR2RD_R {
        WR2RD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - RD2WR"]
    #[inline(always)]
    pub fn rd2wr(&self) -> RD2WR_R {
        RD2WR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - READ_LATENCY"]
    #[inline(always)]
    pub fn read_latency(&self) -> READ_LATENCY_R {
        READ_LATENCY_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - WRITE_LATENCY"]
    #[inline(always)]
    pub fn write_latency(&self) -> WRITE_LATENCY_R {
        WRITE_LATENCY_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WR2RD"]
    #[inline(always)]
    #[must_use]
    pub fn wr2rd(&mut self) -> WR2RD_W<DDRCTRL_DRAMTMG2rs> {
        WR2RD_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - RD2WR"]
    #[inline(always)]
    #[must_use]
    pub fn rd2wr(&mut self) -> RD2WR_W<DDRCTRL_DRAMTMG2rs> {
        RD2WR_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - READ_LATENCY"]
    #[inline(always)]
    #[must_use]
    pub fn read_latency(&mut self) -> READ_LATENCY_W<DDRCTRL_DRAMTMG2rs> {
        READ_LATENCY_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - WRITE_LATENCY"]
    #[inline(always)]
    #[must_use]
    pub fn write_latency(&mut self) -> WRITE_LATENCY_W<DDRCTRL_DRAMTMG2rs> {
        WRITE_LATENCY_W::new(self, 24)
    }
}
#[doc = "DDRCTRL SDRAM timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG2rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg2::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg2::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG2 to value 0x0305_060d"]
impl crate::Resettable for DDRCTRL_DRAMTMG2rs {
    const RESET_VALUE: u32 = 0x0305_060d;
}
