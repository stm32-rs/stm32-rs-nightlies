#[doc = "Register `DCACHE_CMDREADDRR` reader"]
pub type R = crate::R<DCACHE_CMDREADDRRrs>;
#[doc = "Register `DCACHE_CMDREADDRR` writer"]
pub type W = crate::W<DCACHE_CMDREADDRRrs>;
#[doc = "Field `CMDENDADDR` reader - CMDENDADDR"]
pub type CMDENDADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CMDENDADDR` writer - CMDENDADDR"]
pub type CMDENDADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - CMDENDADDR"]
    #[inline(always)]
    pub fn cmdendaddr(&self) -> CMDENDADDR_R {
        CMDENDADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - CMDENDADDR"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendaddr(&mut self) -> CMDENDADDR_W<DCACHE_CMDREADDRRrs> {
        CMDENDADDR_W::new(self, 4)
    }
}
#[doc = "command range start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_cmdreaddrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_cmdreaddrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_CMDREADDRRrs;
impl crate::RegisterSpec for DCACHE_CMDREADDRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_cmdreaddrr::R`](R) reader structure"]
impl crate::Readable for DCACHE_CMDREADDRRrs {}
#[doc = "`write(|w| ..)` method takes [`dcache_cmdreaddrr::W`](W) writer structure"]
impl crate::Writable for DCACHE_CMDREADDRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_CMDREADDRR to value 0"]
impl crate::Resettable for DCACHE_CMDREADDRRrs {
    const RESET_VALUE: u32 = 0;
}
