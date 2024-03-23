#[doc = "Register `MACA0LR` reader"]
pub type R = crate::R<MACA0LRrs>;
#[doc = "Register `MACA0LR` writer"]
pub type W = crate::W<MACA0LRrs>;
#[doc = "Field `ADDRLO` reader - MAC Address 0 \\[31:0\\]"]
pub type ADDRLO_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO` writer - MAC Address 0 \\[31:0\\]"]
pub type ADDRLO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Address 0 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address 0 \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<MACA0LRrs> {
        ADDRLO_W::new(self, 0)
    }
}
#[doc = "Address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0LRrs;
impl crate::RegisterSpec for MACA0LRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0lr::R`](R) reader structure"]
impl crate::Readable for MACA0LRrs {}
#[doc = "`write(|w| ..)` method takes [`maca0lr::W`](W) writer structure"]
impl crate::Writable for MACA0LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA0LR to value 0xffff_ffff"]
impl crate::Resettable for MACA0LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
