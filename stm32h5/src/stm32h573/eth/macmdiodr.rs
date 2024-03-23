#[doc = "Register `MACMDIODR` reader"]
pub type R = crate::R<MACMDIODRrs>;
#[doc = "Register `MACMDIODR` writer"]
pub type W = crate::W<MACMDIODRrs>;
#[doc = "Field `MD` reader - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MD_R = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RA` reader - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
pub type RA_R = crate::FieldReader<u16>;
#[doc = "Field `RA` writer - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<MACMDIODRrs> {
        MD_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<MACMDIODRrs> {
        RA_W::new(self, 16)
    }
}
#[doc = "MDIO data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmdiodr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmdiodr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMDIODRrs;
impl crate::RegisterSpec for MACMDIODRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmdiodr::R`](R) reader structure"]
impl crate::Readable for MACMDIODRrs {}
#[doc = "`write(|w| ..)` method takes [`macmdiodr::W`](W) writer structure"]
impl crate::Writable for MACMDIODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACMDIODR to value 0"]
impl crate::Resettable for MACMDIODRrs {
    const RESET_VALUE: u32 = 0;
}
