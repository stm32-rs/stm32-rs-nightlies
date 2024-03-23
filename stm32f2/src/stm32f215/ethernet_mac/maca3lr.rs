#[doc = "Register `MACA3LR` reader"]
pub type R = crate::R<MACA3LRrs>;
#[doc = "Register `MACA3LR` writer"]
pub type W = crate::W<MACA3LRrs>;
#[doc = "Field `MACA3L` reader - MAC address3 low"]
pub type MACA3L_R = crate::FieldReader<u32>;
#[doc = "Field `MACA3L` writer - MAC address3 low"]
pub type MACA3L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    pub fn maca3l(&self) -> MACA3L_R {
        MACA3L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    #[must_use]
    pub fn maca3l(&mut self) -> MACA3L_W<MACA3LRrs> {
        MACA3L_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA3LRrs;
impl crate::RegisterSpec for MACA3LRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3lr::R`](R) reader structure"]
impl crate::Readable for MACA3LRrs {}
#[doc = "`write(|w| ..)` method takes [`maca3lr::W`](W) writer structure"]
impl crate::Writable for MACA3LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA3LR to value 0xffff_ffff"]
impl crate::Resettable for MACA3LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
