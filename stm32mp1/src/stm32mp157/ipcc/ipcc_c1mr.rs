#[doc = "Register `IPCC_C1MR` reader"]
pub type R = crate::R<IPCC_C1MRrs>;
#[doc = "Register `IPCC_C1MR` writer"]
pub type W = crate::W<IPCC_C1MRrs>;
#[doc = "Field `CHxOM` reader - CHxOM"]
pub type CHX_OM_R = crate::FieldReader;
#[doc = "Field `CHxOM` writer - CHxOM"]
pub type CHX_OM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CHxFM` reader - CHxFM"]
pub type CHX_FM_R = crate::FieldReader;
#[doc = "Field `CHxFM` writer - CHxFM"]
pub type CHX_FM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    pub fn chx_om(&self) -> CHX_OM_R {
        CHX_OM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    pub fn chx_fm(&self) -> CHX_FM_R {
        CHX_FM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    #[must_use]
    pub fn chx_om(&mut self) -> CHX_OM_W<IPCC_C1MRrs> {
        CHX_OM_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    #[must_use]
    pub fn chx_fm(&mut self) -> CHX_FM_W<IPCC_C1MRrs> {
        CHX_FM_W::new(self, 16)
    }
}
#[doc = "IPCC Processor 1 mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcc_c1mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcc_c1mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPCC_C1MRrs;
impl crate::RegisterSpec for IPCC_C1MRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcc_c1mr::R`](R) reader structure"]
impl crate::Readable for IPCC_C1MRrs {}
#[doc = "`write(|w| ..)` method takes [`ipcc_c1mr::W`](W) writer structure"]
impl crate::Writable for IPCC_C1MRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCC_C1MR to value 0xffff_ffff"]
impl crate::Resettable for IPCC_C1MRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
