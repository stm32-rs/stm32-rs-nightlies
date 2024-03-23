#[doc = "Register `DTS_DR` reader"]
pub type R = crate::R<DTS_DRrs>;
#[doc = "Register `DTS_DR` writer"]
pub type W = crate::W<DTS_DRrs>;
#[doc = "Field `TS1_MFREQ` reader - TS1_MFREQ"]
pub type TS1_MFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `TS1_MFREQ` writer - TS1_MFREQ"]
pub type TS1_MFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TS1_MFREQ"]
    #[inline(always)]
    pub fn ts1_mfreq(&self) -> TS1_MFREQ_R {
        TS1_MFREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TS1_MFREQ"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_mfreq(&mut self) -> TS1_MFREQ_W<DTS_DRrs> {
        TS1_MFREQ_W::new(self, 0)
    }
}
#[doc = "The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTS_DRrs;
impl crate::RegisterSpec for DTS_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts_dr::R`](R) reader structure"]
impl crate::Readable for DTS_DRrs {}
#[doc = "`write(|w| ..)` method takes [`dts_dr::W`](W) writer structure"]
impl crate::Writable for DTS_DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTS_DR to value 0"]
impl crate::Resettable for DTS_DRrs {
    const RESET_VALUE: u32 = 0;
}
