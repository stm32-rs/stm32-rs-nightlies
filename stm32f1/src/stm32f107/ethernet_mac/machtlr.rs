#[doc = "Register `MACHTLR` reader"]
pub type R = crate::R<MACHTLRrs>;
#[doc = "Register `MACHTLR` writer"]
pub type W = crate::W<MACHTLRrs>;
#[doc = "Field `HTL` reader - Lower 32 bits of hash table"]
pub type HTL_R = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Lower 32 bits of hash table"]
pub type HTL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Lower 32 bits of hash table"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower 32 bits of hash table"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HTL_W<MACHTLRrs> {
        HTL_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTLRrs;
impl crate::RegisterSpec for MACHTLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machtlr::R`](R) reader structure"]
impl crate::Readable for MACHTLRrs {}
#[doc = "`write(|w| ..)` method takes [`machtlr::W`](W) writer structure"]
impl crate::Writable for MACHTLRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHTLR to value 0"]
impl crate::Resettable for MACHTLRrs {
    const RESET_VALUE: u32 = 0;
}
