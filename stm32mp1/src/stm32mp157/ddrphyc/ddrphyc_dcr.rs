#[doc = "Register `DDRPHYC_DCR` reader"]
pub type R = crate::R<DDRPHYC_DCRrs>;
#[doc = "Register `DDRPHYC_DCR` writer"]
pub type W = crate::W<DDRPHYC_DCRrs>;
#[doc = "Field `DDRMD` reader - DDRMD"]
pub type DDRMD_R = crate::FieldReader;
#[doc = "Field `DDRMD` writer - DDRMD"]
pub type DDRMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DDR8BNK` reader - DDR8BNK"]
pub type DDR8BNK_R = crate::BitReader;
#[doc = "Field `DDR8BNK` writer - DDR8BNK"]
pub type DDR8BNK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDQ` reader - PDQ"]
pub type PDQ_R = crate::FieldReader;
#[doc = "Field `PDQ` writer - PDQ"]
pub type PDQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MPRDQ` reader - MPRDQ"]
pub type MPRDQ_R = crate::BitReader;
#[doc = "Field `MPRDQ` writer - MPRDQ"]
pub type MPRDQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRTYPE` reader - DDRTYPE"]
pub type DDRTYPE_R = crate::FieldReader;
#[doc = "Field `DDRTYPE` writer - DDRTYPE"]
pub type DDRTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOSRA` reader - NOSRA"]
pub type NOSRA_R = crate::BitReader;
#[doc = "Field `NOSRA` writer - NOSRA"]
pub type NOSRA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR2T` reader - DDR2T"]
pub type DDR2T_R = crate::BitReader;
#[doc = "Field `DDR2T` writer - DDR2T"]
pub type DDR2T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIMM` reader - UDIMM"]
pub type UDIMM_R = crate::BitReader;
#[doc = "Field `UDIMM` writer - UDIMM"]
pub type UDIMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIMM` reader - RDIMM"]
pub type RDIMM_R = crate::BitReader;
#[doc = "Field `RDIMM` writer - RDIMM"]
pub type RDIMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPD` reader - TPD"]
pub type TPD_R = crate::BitReader;
#[doc = "Field `TPD` writer - TPD"]
pub type TPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    pub fn ddrmd(&self) -> DDRMD_R {
        DDRMD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    pub fn ddr8bnk(&self) -> DDR8BNK_R {
        DDR8BNK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    pub fn pdq(&self) -> PDQ_R {
        PDQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    pub fn mprdq(&self) -> MPRDQ_R {
        MPRDQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    pub fn ddrtype(&self) -> DDRTYPE_R {
        DDRTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    pub fn nosra(&self) -> NOSRA_R {
        NOSRA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    pub fn ddr2t(&self) -> DDR2T_R {
        DDR2T_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    pub fn udimm(&self) -> UDIMM_R {
        UDIMM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    pub fn rdimm(&self) -> RDIMM_R {
        RDIMM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    #[must_use]
    pub fn ddrmd(&mut self) -> DDRMD_W<DDRPHYC_DCRrs> {
        DDRMD_W::new(self, 0)
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    #[must_use]
    pub fn ddr8bnk(&mut self) -> DDR8BNK_W<DDRPHYC_DCRrs> {
        DDR8BNK_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    #[must_use]
    pub fn pdq(&mut self) -> PDQ_W<DDRPHYC_DCRrs> {
        PDQ_W::new(self, 4)
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    #[must_use]
    pub fn mprdq(&mut self) -> MPRDQ_W<DDRPHYC_DCRrs> {
        MPRDQ_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn ddrtype(&mut self) -> DDRTYPE_W<DDRPHYC_DCRrs> {
        DDRTYPE_W::new(self, 8)
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    #[must_use]
    pub fn nosra(&mut self) -> NOSRA_W<DDRPHYC_DCRrs> {
        NOSRA_W::new(self, 27)
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2t(&mut self) -> DDR2T_W<DDRPHYC_DCRrs> {
        DDR2T_W::new(self, 28)
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    #[must_use]
    pub fn udimm(&mut self) -> UDIMM_W<DDRPHYC_DCRrs> {
        UDIMM_W::new(self, 29)
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    #[must_use]
    pub fn rdimm(&mut self) -> RDIMM_W<DDRPHYC_DCRrs> {
        RDIMM_W::new(self, 30)
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TPD_W<DDRPHYC_DCRrs> {
        TPD_W::new(self, 31)
    }
}
#[doc = "DDRPHYC DC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DCRrs;
impl crate::RegisterSpec for DDRPHYC_DCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DCR to value 0x0b"]
impl crate::Resettable for DDRPHYC_DCRrs {
    const RESET_VALUE: u32 = 0x0b;
}
