#[doc = "Register `ABFSR` reader"]
pub type R = crate::R<ABFSRrs>;
#[doc = "Register `ABFSR` writer"]
pub type W = crate::W<ABFSRrs>;
#[doc = "Field `ITCM` reader - ITCM"]
pub type ITCM_R = crate::BitReader;
#[doc = "Field `ITCM` writer - ITCM"]
pub type ITCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCM` reader - DTCM"]
pub type DTCM_R = crate::BitReader;
#[doc = "Field `DTCM` writer - DTCM"]
pub type DTCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBP` reader - AHBP"]
pub type AHBP_R = crate::BitReader;
#[doc = "Field `AHBP` writer - AHBP"]
pub type AHBP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIM` reader - AXIM"]
pub type AXIM_R = crate::BitReader;
#[doc = "Field `AXIM` writer - AXIM"]
pub type AXIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPB` reader - EPPB"]
pub type EPPB_R = crate::BitReader;
#[doc = "Field `EPPB` writer - EPPB"]
pub type EPPB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIMTYPE` reader - AXIMTYPE"]
pub type AXIMTYPE_R = crate::FieldReader;
#[doc = "Field `AXIMTYPE` writer - AXIMTYPE"]
pub type AXIMTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    pub fn itcm(&self) -> ITCM_R {
        ITCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    pub fn axim(&self) -> AXIM_R {
        AXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    pub fn eppb(&self) -> EPPB_R {
        EPPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    pub fn aximtype(&self) -> AXIMTYPE_R {
        AXIMTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    #[must_use]
    pub fn itcm(&mut self) -> ITCM_W<ABFSRrs> {
        ITCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    #[must_use]
    pub fn dtcm(&mut self) -> DTCM_W<ABFSRrs> {
        DTCM_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<ABFSRrs> {
        AHBP_W::new(self, 2)
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    #[must_use]
    pub fn axim(&mut self) -> AXIM_W<ABFSRrs> {
        AXIM_W::new(self, 3)
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    #[must_use]
    pub fn eppb(&mut self) -> EPPB_W<ABFSRrs> {
        EPPB_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn aximtype(&mut self) -> AXIMTYPE_W<ABFSRrs> {
        AXIMTYPE_W::new(self, 8)
    }
}
#[doc = "Auxiliary Bus Fault Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ABFSRrs;
impl crate::RegisterSpec for ABFSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abfsr::R`](R) reader structure"]
impl crate::Readable for ABFSRrs {}
#[doc = "`write(|w| ..)` method takes [`abfsr::W`](W) writer structure"]
impl crate::Writable for ABFSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABFSR to value 0"]
impl crate::Resettable for ABFSRrs {
    const RESET_VALUE: u32 = 0;
}
