#[doc = "Register `BRR` reader"]
pub type R = crate::R<BRRrs>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRRrs>;
#[doc = "Field `BRR_0_3` reader - BRR_0_3"]
pub type BRR_0_3_R = crate::FieldReader;
#[doc = "Field `BRR_0_3` writer - BRR_0_3"]
pub type BRR_0_3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BRR_4_15` reader - BRR_4_15"]
pub type BRR_4_15_R = crate::FieldReader<u16>;
#[doc = "Field `BRR_4_15` writer - BRR_4_15"]
pub type BRR_4_15_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - BRR_0_3"]
    #[inline(always)]
    pub fn brr_0_3(&self) -> BRR_0_3_R {
        BRR_0_3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - BRR_4_15"]
    #[inline(always)]
    pub fn brr_4_15(&self) -> BRR_4_15_R {
        BRR_4_15_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - BRR_0_3"]
    #[inline(always)]
    #[must_use]
    pub fn brr_0_3(&mut self) -> BRR_0_3_W<BRRrs> {
        BRR_0_3_W::new(self, 0)
    }
    #[doc = "Bits 4:15 - BRR_4_15"]
    #[inline(always)]
    #[must_use]
    pub fn brr_4_15(&mut self) -> BRR_4_15_W<BRRrs> {
        BRR_4_15_W::new(self, 4)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BRRrs {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
