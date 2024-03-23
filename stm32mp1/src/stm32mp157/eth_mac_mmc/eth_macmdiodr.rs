#[doc = "Register `ETH_MACMDIODR` reader"]
pub type R = crate::R<ETH_MACMDIODRrs>;
#[doc = "Register `ETH_MACMDIODR` writer"]
pub type W = crate::W<ETH_MACMDIODRrs>;
#[doc = "Field `GD` reader - GD"]
pub type GD_R = crate::FieldReader<u16>;
#[doc = "Field `GD` writer - GD"]
pub type GD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RA` reader - RA"]
pub type RA_R = crate::FieldReader<u16>;
#[doc = "Field `RA` writer - RA"]
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GD"]
    #[inline(always)]
    pub fn gd(&self) -> GD_R {
        GD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GD"]
    #[inline(always)]
    #[must_use]
    pub fn gd(&mut self) -> GD_W<ETH_MACMDIODRrs> {
        GD_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<ETH_MACMDIODRrs> {
        RA_W::new(self, 16)
    }
}
#[doc = "The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macmdiodr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macmdiodr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACMDIODRrs;
impl crate::RegisterSpec for ETH_MACMDIODRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macmdiodr::R`](R) reader structure"]
impl crate::Readable for ETH_MACMDIODRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macmdiodr::W`](W) writer structure"]
impl crate::Writable for ETH_MACMDIODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACMDIODR to value 0"]
impl crate::Resettable for ETH_MACMDIODRrs {
    const RESET_VALUE: u32 = 0;
}
