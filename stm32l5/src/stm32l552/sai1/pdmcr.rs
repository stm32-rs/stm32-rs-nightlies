#[doc = "Register `PDMCR` reader"]
pub type R = crate::R<PDMCRrs>;
#[doc = "Register `PDMCR` writer"]
pub type W = crate::W<PDMCRrs>;
#[doc = "Field `PDMEN` reader - PDM enable"]
pub type PDMEN_R = crate::BitReader;
#[doc = "Field `PDMEN` writer - PDM enable"]
pub type PDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICNBR` reader - MICNBR"]
pub type MICNBR_R = crate::FieldReader;
#[doc = "Field `MICNBR` writer - MICNBR"]
pub type MICNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKEN1` reader - Clock enable of bitstream clock number 1"]
pub type CKEN1_R = crate::BitReader;
#[doc = "Field `CKEN1` writer - Clock enable of bitstream clock number 1"]
pub type CKEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN2` reader - CKEN2"]
pub type CKEN2_R = crate::BitReader;
#[doc = "Field `CKEN2` writer - CKEN2"]
pub type CKEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    pub fn micnbr(&self) -> MICNBR_R {
        MICNBR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    pub fn cken2(&self) -> CKEN2_R {
        CKEN2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdmen(&mut self) -> PDMEN_W<PDMCRrs> {
        PDMEN_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    #[must_use]
    pub fn micnbr(&mut self) -> MICNBR_W<PDMCRrs> {
        MICNBR_W::new(self, 4)
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    #[must_use]
    pub fn cken1(&mut self) -> CKEN1_W<PDMCRrs> {
        CKEN1_W::new(self, 8)
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    #[must_use]
    pub fn cken2(&mut self) -> CKEN2_W<PDMCRrs> {
        CKEN2_W::new(self, 9)
    }
}
#[doc = "PDM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDMCRrs;
impl crate::RegisterSpec for PDMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdmcr::R`](R) reader structure"]
impl crate::Readable for PDMCRrs {}
#[doc = "`write(|w| ..)` method takes [`pdmcr::W`](W) writer structure"]
impl crate::Writable for PDMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDMCR to value 0"]
impl crate::Resettable for PDMCRrs {
    const RESET_VALUE: u32 = 0;
}
