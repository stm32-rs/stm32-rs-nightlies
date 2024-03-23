#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HCFGrs>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HCFGrs>;
#[doc = "Field `FSLSPCS` reader - FS/LS PHY clock select"]
pub type FSLSPCS_R = crate::FieldReader;
#[doc = "Field `FSLSPCS` writer - FS/LS PHY clock select"]
pub type FSLSPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSS` reader - FS- and LS-only support"]
pub type FSLSS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<HCFGrs> {
        FSLSPCS_W::new(self, 0)
    }
}
#[doc = "OTG_HS host configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCFGrs;
impl crate::RegisterSpec for HCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HCFGrs {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0"]
impl crate::Resettable for HCFGrs {
    const RESET_VALUE: u32 = 0;
}
