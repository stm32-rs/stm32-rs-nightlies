#[doc = "Register `ETH_MACA1LR` reader"]
pub type R = crate::R<ETH_MACA1LRrs>;
#[doc = "Register `ETH_MACA1LR` writer"]
pub type W = crate::W<ETH_MACA1LRrs>;
#[doc = "Field `ADDRLO` reader - ADDRLO"]
pub type ADDRLO_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO` writer - ADDRLO"]
pub type ADDRLO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDRLO"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDRLO"]
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<ETH_MACA1LRrs> {
        ADDRLO_W::new(self, 0)
    }
}
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca1lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca1lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACA1LRrs;
impl crate::RegisterSpec for ETH_MACA1LRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_maca1lr::R`](R) reader structure"]
impl crate::Readable for ETH_MACA1LRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_maca1lr::W`](W) writer structure"]
impl crate::Writable for ETH_MACA1LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACA1LR to value 0xffff_ffff"]
impl crate::Resettable for ETH_MACA1LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
