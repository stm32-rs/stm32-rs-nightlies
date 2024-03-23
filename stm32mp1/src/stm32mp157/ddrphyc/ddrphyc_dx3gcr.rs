#[doc = "Register `DDRPHYC_DX3GCR` reader"]
pub type R = crate::R<DDRPHYC_DX3GCRrs>;
#[doc = "Register `DDRPHYC_DX3GCR` writer"]
pub type W = crate::W<DDRPHYC_DX3GCRrs>;
#[doc = "Field `DXEN` reader - DXEN"]
pub type DXEN_R = crate::BitReader;
#[doc = "Field `DXEN` writer - DXEN"]
pub type DXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSODT` reader - DQSODT"]
pub type DQSODT_R = crate::BitReader;
#[doc = "Field `DQSODT` writer - DQSODT"]
pub type DQSODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQODT` reader - DQODT"]
pub type DQODT_R = crate::BitReader;
#[doc = "Field `DQODT` writer - DQODT"]
pub type DQODT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `DQSRPD` reader - DQSRPD"]
pub type DQSRPD_R = crate::BitReader;
#[doc = "Field `DQSRPD` writer - DQSRPD"]
pub type DQSRPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSEN` reader - DSEN"]
pub type DSEN_R = crate::FieldReader;
#[doc = "Field `DSEN` writer - DSEN"]
pub type DSEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQSRTT` reader - DQSRTT"]
pub type DQSRTT_R = crate::BitReader;
#[doc = "Field `DQSRTT` writer - DQSRTT"]
pub type DQSRTT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQRTT` reader - DQRTT"]
pub type DQRTT_R = crate::BitReader;
#[doc = "Field `DQRTT` writer - DQRTT"]
pub type DQRTT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTTOH` reader - RTTOH"]
pub type RTTOH_R = crate::FieldReader;
#[doc = "Field `RTTOH` writer - RTTOH"]
pub type RTTOH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTTOAL` reader - RTTOAL"]
pub type RTTOAL_R = crate::BitReader;
#[doc = "Field `RTTOAL` writer - RTTOAL"]
pub type RTTOAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0RVSL` reader - R0RVSL"]
pub type R0RVSL_R = crate::FieldReader;
#[doc = "Field `R0RVSL` writer - R0RVSL"]
pub type R0RVSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - DXEN"]
    #[inline(always)]
    pub fn dxen(&self) -> DXEN_R {
        DXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DQSODT"]
    #[inline(always)]
    pub fn dqsodt(&self) -> DQSODT_R {
        DQSODT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DQODT"]
    #[inline(always)]
    pub fn dqodt(&self) -> DQODT_R {
        DQODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DQSRPD"]
    #[inline(always)]
    pub fn dqsrpd(&self) -> DQSRPD_R {
        DQSRPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - DSEN"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - DQSRTT"]
    #[inline(always)]
    pub fn dqsrtt(&self) -> DQSRTT_R {
        DQSRTT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DQRTT"]
    #[inline(always)]
    pub fn dqrtt(&self) -> DQRTT_R {
        DQRTT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - RTTOH"]
    #[inline(always)]
    pub fn rttoh(&self) -> RTTOH_R {
        RTTOH_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - RTTOAL"]
    #[inline(always)]
    pub fn rttoal(&self) -> RTTOAL_R {
        RTTOAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - R0RVSL"]
    #[inline(always)]
    pub fn r0rvsl(&self) -> R0RVSL_R {
        R0RVSL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DXEN"]
    #[inline(always)]
    #[must_use]
    pub fn dxen(&mut self) -> DXEN_W<DDRPHYC_DX3GCRrs> {
        DXEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DQSODT"]
    #[inline(always)]
    #[must_use]
    pub fn dqsodt(&mut self) -> DQSODT_W<DDRPHYC_DX3GCRrs> {
        DQSODT_W::new(self, 1)
    }
    #[doc = "Bit 2 - DQODT"]
    #[inline(always)]
    #[must_use]
    pub fn dqodt(&mut self) -> DQODT_W<DDRPHYC_DX3GCRrs> {
        DQODT_W::new(self, 2)
    }
    #[doc = "Bit 3 - DXIOM"]
    #[inline(always)]
    #[must_use]
    pub fn dxiom(&mut self) -> DXIOM_W<DDRPHYC_DX3GCRrs> {
        DXIOM_W::new(self, 3)
    }
    #[doc = "Bit 4 - DXPDD"]
    #[inline(always)]
    #[must_use]
    pub fn dxpdd(&mut self) -> DXPDD_W<DDRPHYC_DX3GCRrs> {
        DXPDD_W::new(self, 4)
    }
    #[doc = "Bit 5 - DXPDR"]
    #[inline(always)]
    #[must_use]
    pub fn dxpdr(&mut self) -> DXPDR_W<DDRPHYC_DX3GCRrs> {
        DXPDR_W::new(self, 5)
    }
    #[doc = "Bit 6 - DQSRPD"]
    #[inline(always)]
    #[must_use]
    pub fn dqsrpd(&mut self) -> DQSRPD_W<DDRPHYC_DX3GCRrs> {
        DQSRPD_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - DSEN"]
    #[inline(always)]
    #[must_use]
    pub fn dsen(&mut self) -> DSEN_W<DDRPHYC_DX3GCRrs> {
        DSEN_W::new(self, 7)
    }
    #[doc = "Bit 9 - DQSRTT"]
    #[inline(always)]
    #[must_use]
    pub fn dqsrtt(&mut self) -> DQSRTT_W<DDRPHYC_DX3GCRrs> {
        DQSRTT_W::new(self, 9)
    }
    #[doc = "Bit 10 - DQRTT"]
    #[inline(always)]
    #[must_use]
    pub fn dqrtt(&mut self) -> DQRTT_W<DDRPHYC_DX3GCRrs> {
        DQRTT_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - RTTOH"]
    #[inline(always)]
    #[must_use]
    pub fn rttoh(&mut self) -> RTTOH_W<DDRPHYC_DX3GCRrs> {
        RTTOH_W::new(self, 11)
    }
    #[doc = "Bit 13 - RTTOAL"]
    #[inline(always)]
    #[must_use]
    pub fn rttoal(&mut self) -> RTTOAL_W<DDRPHYC_DX3GCRrs> {
        RTTOAL_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - R0RVSL"]
    #[inline(always)]
    #[must_use]
    pub fn r0rvsl(&mut self) -> R0RVSL_W<DDRPHYC_DX3GCRrs> {
        R0RVSL_W::new(self, 14)
    }
}
#[doc = "DDRPHYC byte lane 3 GC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dx3gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dx3gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DX3GCRrs;
impl crate::RegisterSpec for DDRPHYC_DX3GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dx3gcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dx3gcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DX3GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DX3GCR to value 0xee81"]
impl crate::Resettable for DDRPHYC_DX3GCRrs {
    const RESET_VALUE: u32 = 0xee81;
}
