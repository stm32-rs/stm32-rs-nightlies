#[doc = "Register `MACRWKPFR` reader"]
pub type R = crate::R<MACRWKPFRrs>;
#[doc = "Register `MACRWKPFR` writer"]
pub type W = crate::W<MACRWKPFRrs>;
#[doc = "Field `MACRWKPFR` reader - Remote wakeup packet filter"]
pub type MACRWKPFR_R = crate::FieldReader<u32>;
#[doc = "Field `MACRWKPFR` writer - Remote wakeup packet filter"]
pub type MACRWKPFR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remote wakeup packet filter"]
    #[inline(always)]
    pub fn macrwkpfr(&self) -> MACRWKPFR_R {
        MACRWKPFR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote wakeup packet filter"]
    #[inline(always)]
    #[must_use]
    pub fn macrwkpfr(&mut self) -> MACRWKPFR_W<MACRWKPFRrs> {
        MACRWKPFR_W::new(self, 0)
    }
}
#[doc = "Remove wakeup packet filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwkpfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwkpfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRWKPFRrs;
impl crate::RegisterSpec for MACRWKPFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwkpfr::R`](R) reader structure"]
impl crate::Readable for MACRWKPFRrs {}
#[doc = "`write(|w| ..)` method takes [`macrwkpfr::W`](W) writer structure"]
impl crate::Writable for MACRWKPFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACRWKPFR to value 0"]
impl crate::Resettable for MACRWKPFRrs {
    const RESET_VALUE: u32 = 0;
}
