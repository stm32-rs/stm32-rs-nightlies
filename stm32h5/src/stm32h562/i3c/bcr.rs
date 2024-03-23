#[doc = "Register `BCR` reader"]
pub type R = crate::R<BCRrs>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BCRrs>;
#[doc = "Field `BCR0` reader - max data speed limitation"]
pub type BCR0_R = crate::BitReader;
#[doc = "Field `BCR0` writer - max data speed limitation"]
pub type BCR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCR2` reader - in-band interrupt (IBI) payload"]
pub type BCR2_R = crate::BitReader;
#[doc = "Field `BCR2` writer - in-band interrupt (IBI) payload"]
pub type BCR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCR6` reader - controller capable"]
pub type BCR6_R = crate::BitReader;
#[doc = "Field `BCR6` writer - controller capable"]
pub type BCR6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - max data speed limitation"]
    #[inline(always)]
    pub fn bcr0(&self) -> BCR0_R {
        BCR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - in-band interrupt (IBI) payload"]
    #[inline(always)]
    pub fn bcr2(&self) -> BCR2_R {
        BCR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - controller capable"]
    #[inline(always)]
    pub fn bcr6(&self) -> BCR6_R {
        BCR6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - max data speed limitation"]
    #[inline(always)]
    #[must_use]
    pub fn bcr0(&mut self) -> BCR0_W<BCRrs> {
        BCR0_W::new(self, 0)
    }
    #[doc = "Bit 2 - in-band interrupt (IBI) payload"]
    #[inline(always)]
    #[must_use]
    pub fn bcr2(&mut self) -> BCR2_W<BCRrs> {
        BCR2_W::new(self, 2)
    }
    #[doc = "Bit 6 - controller capable"]
    #[inline(always)]
    #[must_use]
    pub fn bcr6(&mut self) -> BCR6_W<BCRrs> {
        BCR6_W::new(self, 6)
    }
}
#[doc = "I3C bus characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCRrs;
impl crate::RegisterSpec for BCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BCRrs {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCRrs {
    const RESET_VALUE: u32 = 0;
}
