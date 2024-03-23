#[doc = "Register `MACVHTR` reader"]
pub type R = crate::R<MACVHTRrs>;
#[doc = "Register `MACVHTR` writer"]
pub type W = crate::W<MACVHTRrs>;
#[doc = "Field `VLHT` reader - VLAN Hash Table"]
pub type VLHT_R = crate::FieldReader<u16>;
#[doc = "Field `VLHT` writer - VLAN Hash Table"]
pub type VLHT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    #[must_use]
    pub fn vlht(&mut self) -> VLHT_W<MACVHTRrs> {
        VLHT_W::new(self, 0)
    }
}
#[doc = "VLAN Hash table register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvhtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvhtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVHTRrs;
impl crate::RegisterSpec for MACVHTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvhtr::R`](R) reader structure"]
impl crate::Readable for MACVHTRrs {}
#[doc = "`write(|w| ..)` method takes [`macvhtr::W`](W) writer structure"]
impl crate::Writable for MACVHTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACVHTR to value 0"]
impl crate::Resettable for MACVHTRrs {
    const RESET_VALUE: u32 = 0;
}
