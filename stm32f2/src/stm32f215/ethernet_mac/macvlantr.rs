#[doc = "Register `MACVLANTR` reader"]
pub type R = crate::R<MACVLANTRrs>;
#[doc = "Register `MACVLANTR` writer"]
pub type W = crate::W<MACVLANTRrs>;
#[doc = "Field `VLANTI` reader - VLAN tag identifier"]
pub type VLANTI_R = crate::FieldReader<u16>;
#[doc = "Field `VLANTI` writer - VLAN tag identifier"]
pub type VLANTI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLANTC` reader - 12-bit VLAN tag comparison"]
pub type VLANTC_R = crate::BitReader;
#[doc = "Field `VLANTC` writer - 12-bit VLAN tag comparison"]
pub type VLANTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier"]
    #[inline(always)]
    #[must_use]
    pub fn vlanti(&mut self) -> VLANTI_W<MACVLANTRrs> {
        VLANTI_W::new(self, 0)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    #[must_use]
    pub fn vlantc(&mut self) -> VLANTC_W<MACVLANTRrs> {
        VLANTC_W::new(self, 16)
    }
}
#[doc = "Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvlantr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvlantr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVLANTRrs;
impl crate::RegisterSpec for MACVLANTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvlantr::R`](R) reader structure"]
impl crate::Readable for MACVLANTRrs {}
#[doc = "`write(|w| ..)` method takes [`macvlantr::W`](W) writer structure"]
impl crate::Writable for MACVLANTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACVLANTR to value 0"]
impl crate::Resettable for MACVLANTRrs {
    const RESET_VALUE: u32 = 0;
}
