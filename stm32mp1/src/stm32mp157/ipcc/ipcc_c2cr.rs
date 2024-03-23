#[doc = "Register `IPCC_C2CR` reader"]
pub type R = crate::R<IPCC_C2CRrs>;
#[doc = "Register `IPCC_C2CR` writer"]
pub type W = crate::W<IPCC_C2CRrs>;
#[doc = "Field `RXOIE` reader - RXOIE"]
pub type RXOIE_R = crate::BitReader;
#[doc = "Field `RXOIE` writer - RXOIE"]
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIE` reader - TXFIE"]
pub type TXFIE_R = crate::BitReader;
#[doc = "Field `TXFIE` writer - TXFIE"]
pub type TXFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<IPCC_C2CRrs> {
        RXOIE_W::new(self, 0)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfie(&mut self) -> TXFIE_W<IPCC_C2CRrs> {
        TXFIE_W::new(self, 16)
    }
}
#[doc = "IPCC Processor 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcc_c2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcc_c2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPCC_C2CRrs;
impl crate::RegisterSpec for IPCC_C2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcc_c2cr::R`](R) reader structure"]
impl crate::Readable for IPCC_C2CRrs {}
#[doc = "`write(|w| ..)` method takes [`ipcc_c2cr::W`](W) writer structure"]
impl crate::Writable for IPCC_C2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCC_C2CR to value 0"]
impl crate::Resettable for IPCC_C2CRrs {
    const RESET_VALUE: u32 = 0;
}
