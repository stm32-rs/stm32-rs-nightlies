#[doc = "Register `TZC_REGION_ATTRIBUTE4` reader"]
pub type R = crate::R<TZC_REGION_ATTRIBUTE4rs>;
#[doc = "Register `TZC_REGION_ATTRIBUTE4` writer"]
pub type W = crate::W<TZC_REGION_ATTRIBUTE4rs>;
#[doc = "Field `FILTER_EN` reader - FILTER_EN"]
pub type FILTER_EN_R = crate::FieldReader;
#[doc = "Field `FILTER_EN` writer - FILTER_EN"]
pub type FILTER_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `S_RD_EN` reader - S_RD_EN"]
pub type S_RD_EN_R = crate::BitReader;
#[doc = "Field `S_RD_EN` writer - S_RD_EN"]
pub type S_RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_WR_EN` reader - S_WR_EN"]
pub type S_WR_EN_R = crate::BitReader;
#[doc = "Field `S_WR_EN` writer - S_WR_EN"]
pub type S_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    pub fn s_rd_en(&self) -> S_RD_EN_R {
        S_RD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    pub fn s_wr_en(&self) -> S_WR_EN_R {
        S_WR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<TZC_REGION_ATTRIBUTE4rs> {
        FILTER_EN_W::new(self, 0)
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    #[must_use]
    pub fn s_rd_en(&mut self) -> S_RD_EN_W<TZC_REGION_ATTRIBUTE4rs> {
        S_RD_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn s_wr_en(&mut self) -> S_WR_EN_W<TZC_REGION_ATTRIBUTE4rs> {
        S_WR_EN_W::new(self, 31)
    }
}
#[doc = "Region x attributes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_attribute4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_attribute4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_REGION_ATTRIBUTE4rs;
impl crate::RegisterSpec for TZC_REGION_ATTRIBUTE4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_region_attribute4::R`](R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE4rs {}
#[doc = "`write(|w| ..)` method takes [`tzc_region_attribute4::W`](W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZC_REGION_ATTRIBUTE4 to value 0"]
impl crate::Resettable for TZC_REGION_ATTRIBUTE4rs {
    const RESET_VALUE: u32 = 0;
}
