#[doc = "Register `MACHTHR` reader"]
pub type R = crate::R<MACHTHRrs>;
#[doc = "Register `MACHTHR` writer"]
pub type W = crate::W<MACHTHRrs>;
#[doc = "Field `HTH` reader - Upper 32 bits of hash table"]
pub type HTH_R = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Upper 32 bits of hash table"]
pub type HTH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Upper 32 bits of hash table"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Upper 32 bits of hash table"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HTH_W<MACHTHRrs> {
        HTH_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machthr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machthr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTHRrs;
impl crate::RegisterSpec for MACHTHRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machthr::R`](R) reader structure"]
impl crate::Readable for MACHTHRrs {}
#[doc = "`write(|w| ..)` method takes [`machthr::W`](W) writer structure"]
impl crate::Writable for MACHTHRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHTHR to value 0"]
impl crate::Resettable for MACHTHRrs {
    const RESET_VALUE: u32 = 0;
}
