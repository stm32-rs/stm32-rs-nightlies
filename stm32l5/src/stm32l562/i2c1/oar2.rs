#[doc = "Register `OAR2` reader"]
pub type R = crate::R<OAR2rs>;
#[doc = "Register `OAR2` writer"]
pub type W = crate::W<OAR2rs>;
#[doc = "Field `OA2` reader - Interface address"]
pub type OA2_R = crate::FieldReader;
#[doc = "Field `OA2` writer - Interface address"]
pub type OA2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA2MSK` reader - Own Address 2 masks"]
pub type OA2MSK_R = crate::FieldReader;
#[doc = "Field `OA2MSK` writer - Own Address 2 masks"]
pub type OA2MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub type OA2EN_R = crate::BitReader;
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub type OA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn oa2(&mut self) -> OA2_W<OAR2rs> {
        OA2_W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline(always)]
    #[must_use]
    pub fn oa2msk(&mut self) -> OA2MSK_W<OAR2rs> {
        OA2MSK_W::new(self, 8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn oa2en(&mut self) -> OA2EN_W<OAR2rs> {
        OA2EN_W::new(self, 15)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR2rs;
impl crate::RegisterSpec for OAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar2::R`](R) reader structure"]
impl crate::Readable for OAR2rs {}
#[doc = "`write(|w| ..)` method takes [`oar2::W`](W) writer structure"]
impl crate::Writable for OAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for OAR2rs {
    const RESET_VALUE: u32 = 0;
}
