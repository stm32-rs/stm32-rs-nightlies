#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2rs>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2rs>;
#[doc = "Field `SWI33` reader - Software interrupt on event"]
pub type SWI33_R = crate::BitReader;
#[doc = "Field `SWI33` writer - Software interrupt on event"]
pub type SWI33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI40_41` reader - Software interrupt on event"]
pub type SWI40_41_R = crate::FieldReader;
#[doc = "Field `SWI40_41` writer - Software interrupt on event"]
pub type SWI40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi33(&self) -> SWI33_R {
        SWI33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi40_41(&self) -> SWI40_41_R {
        SWI40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Software interrupt on event"]
    #[inline(always)]
    #[must_use]
    pub fn swi33(&mut self) -> SWI33_W<SWIER2rs> {
        SWI33_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Software interrupt on event"]
    #[inline(always)]
    #[must_use]
    pub fn swi40_41(&mut self) -> SWI40_41_W<SWIER2rs> {
        SWI40_41_W::new(self, 8)
    }
}
#[doc = "software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for SWIER2rs {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2rs {
    const RESET_VALUE: u32 = 0;
}
