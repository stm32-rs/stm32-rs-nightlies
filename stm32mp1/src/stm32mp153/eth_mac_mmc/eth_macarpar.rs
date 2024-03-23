#[doc = "Register `ETH_MACARPAR` reader"]
pub type R = crate::R<ETH_MACARPARrs>;
#[doc = "Register `ETH_MACARPAR` writer"]
pub type W = crate::W<ETH_MACARPARrs>;
#[doc = "Field `ARPPA` reader - ARPPA"]
pub type ARPPA_R = crate::FieldReader<u32>;
#[doc = "Field `ARPPA` writer - ARPPA"]
pub type ARPPA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ARPPA"]
    #[inline(always)]
    pub fn arppa(&self) -> ARPPA_R {
        ARPPA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ARPPA"]
    #[inline(always)]
    #[must_use]
    pub fn arppa(&mut self) -> ARPPA_W<ETH_MACARPARrs> {
        ARPPA_W::new(self, 0)
    }
}
#[doc = "The ARP Address register contains the IPv4 Destination Address of the MAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macarpar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macarpar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACARPARrs;
impl crate::RegisterSpec for ETH_MACARPARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macarpar::R`](R) reader structure"]
impl crate::Readable for ETH_MACARPARrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macarpar::W`](W) writer structure"]
impl crate::Writable for ETH_MACARPARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACARPAR to value 0"]
impl crate::Resettable for ETH_MACARPARrs {
    const RESET_VALUE: u32 = 0;
}
