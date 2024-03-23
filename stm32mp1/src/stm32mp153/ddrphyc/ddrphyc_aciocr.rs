#[doc = "Register `DDRPHYC_ACIOCR` reader"]
pub type R = crate::R<DDRPHYC_ACIOCRrs>;
#[doc = "Register `DDRPHYC_ACIOCR` writer"]
pub type W = crate::W<DDRPHYC_ACIOCRrs>;
#[doc = "Field `ACIOM` reader - ACIOM"]
pub type ACIOM_R = crate::BitReader;
#[doc = "Field `ACIOM` writer - ACIOM"]
pub type ACIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACOE` reader - ACOE"]
pub type ACOE_R = crate::BitReader;
#[doc = "Field `ACOE` writer - ACOE"]
pub type ACOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACODT` reader - ACODT"]
pub type ACODT_R = crate::BitReader;
#[doc = "Field `ACODT` writer - ACODT"]
pub type ACODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACPDD` reader - ACPDD"]
pub type ACPDD_R = crate::BitReader;
#[doc = "Field `ACPDD` writer - ACPDD"]
pub type ACPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACPDR` reader - ACPDR"]
pub type ACPDR_R = crate::BitReader;
#[doc = "Field `ACPDR` writer - ACPDR"]
pub type ACPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKODT` reader - CKODT"]
pub type CKODT_R = crate::FieldReader;
#[doc = "Field `CKODT` writer - CKODT"]
pub type CKODT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKPDD` reader - CKPDD"]
pub type CKPDD_R = crate::FieldReader;
#[doc = "Field `CKPDD` writer - CKPDD"]
pub type CKPDD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKPDR` reader - CKPDR"]
pub type CKPDR_R = crate::FieldReader;
#[doc = "Field `CKPDR` writer - CKPDR"]
pub type CKPDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANKODT` reader - RANKODT"]
pub type RANKODT_R = crate::BitReader;
#[doc = "Field `RANKODT` writer - RANKODT"]
pub type RANKODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPDD` reader - CSPDD"]
pub type CSPDD_R = crate::BitReader;
#[doc = "Field `CSPDD` writer - CSPDD"]
pub type CSPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANKPDR` reader - RANKPDR"]
pub type RANKPDR_R = crate::BitReader;
#[doc = "Field `RANKPDR` writer - RANKPDR"]
pub type RANKPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTODT` reader - RSTODT"]
pub type RSTODT_R = crate::BitReader;
#[doc = "Field `RSTODT` writer - RSTODT"]
pub type RSTODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPDD` reader - RSTPDD"]
pub type RSTPDD_R = crate::BitReader;
#[doc = "Field `RSTPDD` writer - RSTPDD"]
pub type RSTPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPDR` reader - RSTPDR"]
pub type RSTPDR_R = crate::BitReader;
#[doc = "Field `RSTPDR` writer - RSTPDR"]
pub type RSTPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIOM` reader - RSTIOM"]
pub type RSTIOM_R = crate::BitReader;
#[doc = "Field `RSTIOM` writer - RSTIOM"]
pub type RSTIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACSR` reader - ACSR"]
pub type ACSR_R = crate::FieldReader;
#[doc = "Field `ACSR` writer - ACSR"]
pub type ACSR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - ACIOM"]
    #[inline(always)]
    pub fn aciom(&self) -> ACIOM_R {
        ACIOM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACOE"]
    #[inline(always)]
    pub fn acoe(&self) -> ACOE_R {
        ACOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACODT"]
    #[inline(always)]
    pub fn acodt(&self) -> ACODT_R {
        ACODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ACPDD"]
    #[inline(always)]
    pub fn acpdd(&self) -> ACPDD_R {
        ACPDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACPDR"]
    #[inline(always)]
    pub fn acpdr(&self) -> ACPDR_R {
        ACPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - CKODT"]
    #[inline(always)]
    pub fn ckodt(&self) -> CKODT_R {
        CKODT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - CKPDD"]
    #[inline(always)]
    pub fn ckpdd(&self) -> CKPDD_R {
        CKPDD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - CKPDR"]
    #[inline(always)]
    pub fn ckpdr(&self) -> CKPDR_R {
        CKPDR_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - RANKODT"]
    #[inline(always)]
    pub fn rankodt(&self) -> RANKODT_R {
        RANKODT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - CSPDD"]
    #[inline(always)]
    pub fn cspdd(&self) -> CSPDD_R {
        CSPDD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - RANKPDR"]
    #[inline(always)]
    pub fn rankpdr(&self) -> RANKPDR_R {
        RANKPDR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - RSTODT"]
    #[inline(always)]
    pub fn rstodt(&self) -> RSTODT_R {
        RSTODT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RSTPDD"]
    #[inline(always)]
    pub fn rstpdd(&self) -> RSTPDD_R {
        RSTPDD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RSTPDR"]
    #[inline(always)]
    pub fn rstpdr(&self) -> RSTPDR_R {
        RSTPDR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RSTIOM"]
    #[inline(always)]
    pub fn rstiom(&self) -> RSTIOM_R {
        RSTIOM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ACSR"]
    #[inline(always)]
    pub fn acsr(&self) -> ACSR_R {
        ACSR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ACIOM"]
    #[inline(always)]
    #[must_use]
    pub fn aciom(&mut self) -> ACIOM_W<DDRPHYC_ACIOCRrs> {
        ACIOM_W::new(self, 0)
    }
    #[doc = "Bit 1 - ACOE"]
    #[inline(always)]
    #[must_use]
    pub fn acoe(&mut self) -> ACOE_W<DDRPHYC_ACIOCRrs> {
        ACOE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ACODT"]
    #[inline(always)]
    #[must_use]
    pub fn acodt(&mut self) -> ACODT_W<DDRPHYC_ACIOCRrs> {
        ACODT_W::new(self, 2)
    }
    #[doc = "Bit 3 - ACPDD"]
    #[inline(always)]
    #[must_use]
    pub fn acpdd(&mut self) -> ACPDD_W<DDRPHYC_ACIOCRrs> {
        ACPDD_W::new(self, 3)
    }
    #[doc = "Bit 4 - ACPDR"]
    #[inline(always)]
    #[must_use]
    pub fn acpdr(&mut self) -> ACPDR_W<DDRPHYC_ACIOCRrs> {
        ACPDR_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - CKODT"]
    #[inline(always)]
    #[must_use]
    pub fn ckodt(&mut self) -> CKODT_W<DDRPHYC_ACIOCRrs> {
        CKODT_W::new(self, 5)
    }
    #[doc = "Bits 8:10 - CKPDD"]
    #[inline(always)]
    #[must_use]
    pub fn ckpdd(&mut self) -> CKPDD_W<DDRPHYC_ACIOCRrs> {
        CKPDD_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - CKPDR"]
    #[inline(always)]
    #[must_use]
    pub fn ckpdr(&mut self) -> CKPDR_W<DDRPHYC_ACIOCRrs> {
        CKPDR_W::new(self, 11)
    }
    #[doc = "Bit 14 - RANKODT"]
    #[inline(always)]
    #[must_use]
    pub fn rankodt(&mut self) -> RANKODT_W<DDRPHYC_ACIOCRrs> {
        RANKODT_W::new(self, 14)
    }
    #[doc = "Bit 18 - CSPDD"]
    #[inline(always)]
    #[must_use]
    pub fn cspdd(&mut self) -> CSPDD_W<DDRPHYC_ACIOCRrs> {
        CSPDD_W::new(self, 18)
    }
    #[doc = "Bit 22 - RANKPDR"]
    #[inline(always)]
    #[must_use]
    pub fn rankpdr(&mut self) -> RANKPDR_W<DDRPHYC_ACIOCRrs> {
        RANKPDR_W::new(self, 22)
    }
    #[doc = "Bit 26 - RSTODT"]
    #[inline(always)]
    #[must_use]
    pub fn rstodt(&mut self) -> RSTODT_W<DDRPHYC_ACIOCRrs> {
        RSTODT_W::new(self, 26)
    }
    #[doc = "Bit 27 - RSTPDD"]
    #[inline(always)]
    #[must_use]
    pub fn rstpdd(&mut self) -> RSTPDD_W<DDRPHYC_ACIOCRrs> {
        RSTPDD_W::new(self, 27)
    }
    #[doc = "Bit 28 - RSTPDR"]
    #[inline(always)]
    #[must_use]
    pub fn rstpdr(&mut self) -> RSTPDR_W<DDRPHYC_ACIOCRrs> {
        RSTPDR_W::new(self, 28)
    }
    #[doc = "Bit 29 - RSTIOM"]
    #[inline(always)]
    #[must_use]
    pub fn rstiom(&mut self) -> RSTIOM_W<DDRPHYC_ACIOCRrs> {
        RSTIOM_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - ACSR"]
    #[inline(always)]
    #[must_use]
    pub fn acsr(&mut self) -> ACSR_W<DDRPHYC_ACIOCRrs> {
        ACSR_W::new(self, 30)
    }
}
#[doc = "DDRPHYC ACIOC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_aciocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_aciocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_ACIOCRrs;
impl crate::RegisterSpec for DDRPHYC_ACIOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_aciocr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_ACIOCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_aciocr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_ACIOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_ACIOCR to value 0x33c0_3812"]
impl crate::Resettable for DDRPHYC_ACIOCRrs {
    const RESET_VALUE: u32 = 0x33c0_3812;
}
