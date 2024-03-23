#[doc = "Register `MDIOS_CR` reader"]
pub type R = crate::R<MDIOS_CRrs>;
#[doc = "Register `MDIOS_CR` writer"]
pub type W = crate::W<MDIOS_CRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRIE` reader - WRIE"]
pub type WRIE_R = crate::BitReader;
#[doc = "Field `WRIE` writer - WRIE"]
pub type WRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIE` reader - RDIE"]
pub type RDIE_R = crate::BitReader;
#[doc = "Field `RDIE` writer - RDIE"]
pub type RDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - EIE"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - EIE"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC` reader - DPC"]
pub type DPC_R = crate::BitReader;
#[doc = "Field `DPC` writer - DPC"]
pub type DPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_ADDRESS` reader - PORT_ADDRESS"]
pub type PORT_ADDRESS_R = crate::FieldReader;
#[doc = "Field `PORT_ADDRESS` writer - PORT_ADDRESS"]
pub type PORT_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRIE"]
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RDIE"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EIE"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - DPC"]
    #[inline(always)]
    pub fn dpc(&self) -> DPC_R {
        DPC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - PORT_ADDRESS"]
    #[inline(always)]
    pub fn port_address(&self) -> PORT_ADDRESS_R {
        PORT_ADDRESS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<MDIOS_CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - WRIE"]
    #[inline(always)]
    #[must_use]
    pub fn wrie(&mut self) -> WRIE_W<MDIOS_CRrs> {
        WRIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - RDIE"]
    #[inline(always)]
    #[must_use]
    pub fn rdie(&mut self) -> RDIE_W<MDIOS_CRrs> {
        RDIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EIE"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<MDIOS_CRrs> {
        EIE_W::new(self, 3)
    }
    #[doc = "Bit 7 - DPC"]
    #[inline(always)]
    #[must_use]
    pub fn dpc(&mut self) -> DPC_W<MDIOS_CRrs> {
        DPC_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - PORT_ADDRESS"]
    #[inline(always)]
    #[must_use]
    pub fn port_address(&mut self) -> PORT_ADDRESS_W<MDIOS_CRrs> {
        PORT_ADDRESS_W::new(self, 8)
    }
}
#[doc = "MDIOS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_CRrs;
impl crate::RegisterSpec for MDIOS_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_cr::R`](R) reader structure"]
impl crate::Readable for MDIOS_CRrs {}
#[doc = "`write(|w| ..)` method takes [`mdios_cr::W`](W) writer structure"]
impl crate::Writable for MDIOS_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIOS_CR to value 0"]
impl crate::Resettable for MDIOS_CRrs {
    const RESET_VALUE: u32 = 0;
}
