#[doc = "Register `DDRPHYC_DXCCR` reader"]
pub type R = crate::R<DDRPHYC_DXCCRrs>;
#[doc = "Register `DDRPHYC_DXCCR` writer"]
pub type W = crate::W<DDRPHYC_DXCCRrs>;
#[doc = "Field `DXODT` reader - DXODT"]
pub type DXODT_R = crate::BitReader;
#[doc = "Field `DXODT` writer - DXODT"]
pub type DXODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DXIOM` reader - DXIOM"]
pub type DXIOM_R = crate::BitReader;
#[doc = "Field `DXIOM` writer - DXIOM"]
pub type DXIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DXPDD` reader - DXPDD"]
pub type DXPDD_R = crate::BitReader;
#[doc = "Field `DXPDD` writer - DXPDD"]
pub type DXPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DXPDR` reader - DXPDR"]
pub type DXPDR_R = crate::BitReader;
#[doc = "Field `DXPDR` writer - DXPDR"]
pub type DXPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSRES` reader - DQSRES"]
pub type DQSRES_R = crate::FieldReader;
#[doc = "Field `DQSRES` writer - DQSRES"]
pub type DQSRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQSNRES` reader - DQSNRES"]
pub type DQSNRES_R = crate::FieldReader;
#[doc = "Field `DQSNRES` writer - DQSNRES"]
pub type DQSNRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQSNRST` reader - DQSNRST"]
pub type DQSNRST_R = crate::BitReader;
#[doc = "Field `DQSNRST` writer - DQSNRST"]
pub type DQSNRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RVSEL` reader - RVSEL"]
pub type RVSEL_R = crate::BitReader;
#[doc = "Field `RVSEL` writer - RVSEL"]
pub type RVSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDT` reader - AWDT"]
pub type AWDT_R = crate::BitReader;
#[doc = "Field `AWDT` writer - AWDT"]
pub type AWDT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    pub fn dxodt(&self) -> DXODT_R {
        DXODT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    pub fn dqsres(&self) -> DQSRES_R {
        DQSRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    pub fn dqsnres(&self) -> DQSNRES_R {
        DQSNRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    pub fn dqsnrst(&self) -> DQSNRST_R {
        DQSNRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    pub fn rvsel(&self) -> RVSEL_R {
        RVSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    pub fn awdt(&self) -> AWDT_R {
        AWDT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    #[must_use]
    pub fn dxodt(&mut self) -> DXODT_W<DDRPHYC_DXCCRrs> {
        DXODT_W::new(self, 0)
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    #[must_use]
    pub fn dxiom(&mut self) -> DXIOM_W<DDRPHYC_DXCCRrs> {
        DXIOM_W::new(self, 1)
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    #[must_use]
    pub fn dxpdd(&mut self) -> DXPDD_W<DDRPHYC_DXCCRrs> {
        DXPDD_W::new(self, 2)
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    #[must_use]
    pub fn dxpdr(&mut self) -> DXPDR_W<DDRPHYC_DXCCRrs> {
        DXPDR_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    #[must_use]
    pub fn dqsres(&mut self) -> DQSRES_W<DDRPHYC_DXCCRrs> {
        DQSRES_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    #[must_use]
    pub fn dqsnres(&mut self) -> DQSNRES_W<DDRPHYC_DXCCRrs> {
        DQSNRES_W::new(self, 8)
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    #[must_use]
    pub fn dqsnrst(&mut self) -> DQSNRST_W<DDRPHYC_DXCCRrs> {
        DQSNRST_W::new(self, 14)
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    #[must_use]
    pub fn rvsel(&mut self) -> RVSEL_W<DDRPHYC_DXCCRrs> {
        RVSEL_W::new(self, 15)
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    #[must_use]
    pub fn awdt(&mut self) -> AWDT_W<DDRPHYC_DXCCRrs> {
        AWDT_W::new(self, 16)
    }
}
#[doc = "DDRPHYC DXCC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dxccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dxccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DXCCRrs;
impl crate::RegisterSpec for DDRPHYC_DXCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dxccr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DXCCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dxccr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DXCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DXCCR to value 0x0800"]
impl crate::Resettable for DDRPHYC_DXCCRrs {
    const RESET_VALUE: u32 = 0x0800;
}
