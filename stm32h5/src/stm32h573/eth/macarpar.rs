#[doc = "Register `MACARPAR` reader"]
pub type R = crate::R<MACARPARrs>;
#[doc = "Register `MACARPAR` writer"]
pub type W = crate::W<MACARPARrs>;
#[doc = "Field `ARPPA` reader - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet."]
pub type ARPPA_R = crate::FieldReader<u32>;
#[doc = "Field `ARPPA` writer - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet."]
pub type ARPPA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet."]
    #[inline(always)]
    pub fn arppa(&self) -> ARPPA_R {
        ARPPA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ARP Protocol Address This field contains the IPv4 Destination Address of the MAC. This address is used for perfect match with the Protocol Address of Target field in the received ARP packet."]
    #[inline(always)]
    #[must_use]
    pub fn arppa(&mut self) -> ARPPA_W<MACARPARrs> {
        ARPPA_W::new(self, 0)
    }
}
#[doc = "ARP address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macarpar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macarpar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACARPARrs;
impl crate::RegisterSpec for MACARPARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macarpar::R`](R) reader structure"]
impl crate::Readable for MACARPARrs {}
#[doc = "`write(|w| ..)` method takes [`macarpar::W`](W) writer structure"]
impl crate::Writable for MACARPARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACARPAR to value 0"]
impl crate::Resettable for MACARPARrs {
    const RESET_VALUE: u32 = 0;
}
