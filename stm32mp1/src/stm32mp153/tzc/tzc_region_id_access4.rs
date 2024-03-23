#[doc = "Register `TZC_REGION_ID_ACCESS4` reader"]
pub type R = crate::R<TZC_REGION_ID_ACCESS4rs>;
#[doc = "Register `TZC_REGION_ID_ACCESS4` writer"]
pub type W = crate::W<TZC_REGION_ID_ACCESS4rs>;
#[doc = "Field `NSAID_RD_EN` reader - NSAID_RD_EN"]
pub type NSAID_RD_EN_R = crate::FieldReader<u16>;
#[doc = "Field `NSAID_RD_EN` writer - NSAID_RD_EN"]
pub type NSAID_RD_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NSAID_WR_EN` reader - NSAID_WR_EN"]
pub type NSAID_WR_EN_R = crate::FieldReader<u16>;
#[doc = "Field `NSAID_WR_EN` writer - NSAID_WR_EN"]
pub type NSAID_WR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NSAID_RD_EN"]
    #[inline(always)]
    pub fn nsaid_rd_en(&self) -> NSAID_RD_EN_R {
        NSAID_RD_EN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - NSAID_WR_EN"]
    #[inline(always)]
    pub fn nsaid_wr_en(&self) -> NSAID_WR_EN_R {
        NSAID_WR_EN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NSAID_RD_EN"]
    #[inline(always)]
    #[must_use]
    pub fn nsaid_rd_en(&mut self) -> NSAID_RD_EN_W<TZC_REGION_ID_ACCESS4rs> {
        NSAID_RD_EN_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - NSAID_WR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn nsaid_wr_en(&mut self) -> NSAID_WR_EN_W<TZC_REGION_ID_ACCESS4rs> {
        NSAID_WR_EN_W::new(self, 16)
    }
}
#[doc = "Region non-secure access based on NSAID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_id_access4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_id_access4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_REGION_ID_ACCESS4rs;
impl crate::RegisterSpec for TZC_REGION_ID_ACCESS4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_region_id_access4::R`](R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS4rs {}
#[doc = "`write(|w| ..)` method takes [`tzc_region_id_access4::W`](W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZC_REGION_ID_ACCESS4 to value 0"]
impl crate::Resettable for TZC_REGION_ID_ACCESS4rs {
    const RESET_VALUE: u32 = 0;
}
