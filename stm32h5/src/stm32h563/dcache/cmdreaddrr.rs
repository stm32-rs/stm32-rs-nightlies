#[doc = "Register `CMDREADDRR` reader"]
pub type R = crate::R<CMDREADDRRrs>;
#[doc = "Register `CMDREADDRR` writer"]
pub type W = crate::W<CMDREADDRRrs>;
#[doc = "Field `CMDENDADDR` reader - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
pub type CMDENDADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CMDENDADDR` writer - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
pub type CMDENDADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
    #[inline(always)]
    pub fn cmdendaddr(&self) -> CMDENDADDR_R {
        CMDENDADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
    #[inline(always)]
    #[must_use]
    pub fn cmdendaddr(&mut self) -> CMDENDADDR_W<CMDREADDRRrs> {
        CMDENDADDR_W::new(self, 4)
    }
}
#[doc = "DCACHE command range end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdreaddrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdreaddrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDREADDRRrs;
impl crate::RegisterSpec for CMDREADDRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdreaddrr::R`](R) reader structure"]
impl crate::Readable for CMDREADDRRrs {}
#[doc = "`write(|w| ..)` method takes [`cmdreaddrr::W`](W) writer structure"]
impl crate::Writable for CMDREADDRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDREADDRR to value 0"]
impl crate::Resettable for CMDREADDRRrs {
    const RESET_VALUE: u32 = 0;
}
