#[doc = "Register `ICACHE_CRR3` reader"]
pub type R = crate::R<ICACHE_CRR3rs>;
#[doc = "Register `ICACHE_CRR3` writer"]
pub type W = crate::W<ICACHE_CRR3rs>;
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub type BASEADDR_R = crate::FieldReader;
#[doc = "Field `BASEADDR` writer - BASEADDR"]
pub type BASEADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSIZE` reader - RSIZE"]
pub type RSIZE_R = crate::FieldReader;
#[doc = "Field `RSIZE` writer - RSIZE"]
pub type RSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REN` reader - REN"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - REN"]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMAPADDR` reader - REMAPADDR"]
pub type REMAPADDR_R = crate::FieldReader<u16>;
#[doc = "Field `REMAPADDR` writer - REMAPADDR"]
pub type REMAPADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MSTSEL` reader - MSTSEL"]
pub type MSTSEL_R = crate::BitReader;
#[doc = "Field `MSTSEL` writer - MSTSEL"]
pub type MSTSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBURST` reader - HBURST"]
pub type HBURST_R = crate::BitReader;
#[doc = "Field `HBURST` writer - HBURST"]
pub type HBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:11 - RSIZE"]
    #[inline(always)]
    pub fn rsize(&self) -> RSIZE_R {
        RSIZE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 15 - REN"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - REMAPADDR"]
    #[inline(always)]
    pub fn remapaddr(&self) -> REMAPADDR_R {
        REMAPADDR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - MSTSEL"]
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BASEADDR"]
    #[inline(always)]
    #[must_use]
    pub fn baseaddr(&mut self) -> BASEADDR_W<ICACHE_CRR3rs> {
        BASEADDR_W::new(self, 0)
    }
    #[doc = "Bits 9:11 - RSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn rsize(&mut self) -> RSIZE_W<ICACHE_CRR3rs> {
        RSIZE_W::new(self, 9)
    }
    #[doc = "Bit 15 - REN"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<ICACHE_CRR3rs> {
        REN_W::new(self, 15)
    }
    #[doc = "Bits 16:26 - REMAPADDR"]
    #[inline(always)]
    #[must_use]
    pub fn remapaddr(&mut self) -> REMAPADDR_W<ICACHE_CRR3rs> {
        REMAPADDR_W::new(self, 16)
    }
    #[doc = "Bit 28 - MSTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn mstsel(&mut self) -> MSTSEL_W<ICACHE_CRR3rs> {
        MSTSEL_W::new(self, 28)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<ICACHE_CRR3rs> {
        HBURST_W::new(self, 31)
    }
}
#[doc = "ICACHE region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_crr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_crr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_CRR3rs;
impl crate::RegisterSpec for ICACHE_CRR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_crr3::R`](R) reader structure"]
impl crate::Readable for ICACHE_CRR3rs {}
#[doc = "`write(|w| ..)` method takes [`icache_crr3::W`](W) writer structure"]
impl crate::Writable for ICACHE_CRR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICACHE_CRR3 to value 0x0200"]
impl crate::Resettable for ICACHE_CRR3rs {
    const RESET_VALUE: u32 = 0x0200;
}
