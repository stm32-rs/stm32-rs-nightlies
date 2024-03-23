#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader;
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type ICEN_R = crate::BitReader;
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DCEN_R = crate::BitReader;
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type ICRST_R = crate::BitReader;
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DCRST_R = crate::BitReader;
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES` reader - CPU1 CortexM4 program erase suspend request"]
pub type PES_R = crate::BitReader;
#[doc = "Field `PES` writer - CPU1 CortexM4 program erase suspend request"]
pub type PES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - Flash User area empty"]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `EMPTY` writer - Flash User area empty"]
pub type EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU1 CortexM4 program erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<ACRrs> {
        ICEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DCEN_W<ACRrs> {
        DCEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<ACRrs> {
        ICRST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcrst(&mut self) -> DCRST_W<ACRrs> {
        DCRST_W::new(self, 12)
    }
    #[doc = "Bit 15 - CPU1 CortexM4 program erase suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<ACRrs> {
        PES_W::new(self, 15)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<ACRrs> {
        EMPTY_W::new(self, 16)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0x0600"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
