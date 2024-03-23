#[doc = "Register `MACMIIDR` reader"]
pub type R = crate::R<MACMIIDRrs>;
#[doc = "Register `MACMIIDR` writer"]
pub type W = crate::W<MACMIIDRrs>;
#[doc = "Field `MD` reader - MII data read from/written to the PHY"]
pub type MD_R = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII data read from/written to the PHY"]
pub type MD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII data read from/written to the PHY"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data read from/written to the PHY"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<MACMIIDRrs> {
        MD_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC MII data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIDRrs;
impl crate::RegisterSpec for MACMIIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiidr::R`](R) reader structure"]
impl crate::Readable for MACMIIDRrs {}
#[doc = "`write(|w| ..)` method takes [`macmiidr::W`](W) writer structure"]
impl crate::Writable for MACMIIDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACMIIDR to value 0"]
impl crate::Resettable for MACMIIDRrs {
    const RESET_VALUE: u32 = 0;
}
