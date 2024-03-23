#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `OUT2_RMP` reader - OUT2_RMP"]
pub type OUT2_RMP_R = crate::BitReader;
#[doc = "Field `OUT2_RMP` writer - OUT2_RMP"]
pub type OUT2_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSCOEN` reader - LSCOEN"]
pub type LSCOEN_R = crate::FieldReader;
#[doc = "Field `LSCOEN` writer - LSCOEN"]
pub type LSCOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - OUT2_RMP"]
    #[inline(always)]
    pub fn out2_rmp(&self) -> OUT2_RMP_R {
        OUT2_RMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - LSCOEN"]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OUT2_RMP"]
    #[inline(always)]
    #[must_use]
    pub fn out2_rmp(&mut self) -> OUT2_RMP_W<CFGRrs> {
        OUT2_RMP_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - LSCOEN"]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<CFGRrs> {
        LSCOEN_W::new(self, 1)
    }
}
#[doc = "RTC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
