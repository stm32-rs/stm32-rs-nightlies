#[doc = "Register `CRR%s` reader"]
pub type R = crate::R<CRRrs>;
#[doc = "Register `CRR%s` writer"]
pub type W = crate::W<CRRrs>;
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
    pub fn baseaddr(&mut self) -> BASEADDR_W<CRRrs> {
        BASEADDR_W::new(self, 0)
    }
    #[doc = "Bits 9:11 - RSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn rsize(&mut self) -> RSIZE_W<CRRrs> {
        RSIZE_W::new(self, 9)
    }
    #[doc = "Bit 15 - REN"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CRRrs> {
        REN_W::new(self, 15)
    }
    #[doc = "Bits 16:26 - REMAPADDR"]
    #[inline(always)]
    #[must_use]
    pub fn remapaddr(&mut self) -> REMAPADDR_W<CRRrs> {
        REMAPADDR_W::new(self, 16)
    }
    #[doc = "Bit 28 - MSTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn mstsel(&mut self) -> MSTSEL_W<CRRrs> {
        MSTSEL_W::new(self, 28)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<CRRrs> {
        HBURST_W::new(self, 31)
    }
}
#[doc = "ICACHE region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRRrs;
impl crate::RegisterSpec for CRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crr::R`](R) reader structure"]
impl crate::Readable for CRRrs {}
#[doc = "`write(|w| ..)` method takes [`crr::W`](W) writer structure"]
impl crate::Writable for CRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRR%s to value 0x0200"]
impl crate::Resettable for CRRrs {
    const RESET_VALUE: u32 = 0x0200;
}
