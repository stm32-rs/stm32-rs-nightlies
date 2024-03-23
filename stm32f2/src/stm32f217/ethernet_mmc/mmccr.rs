#[doc = "Register `MMCCR` reader"]
pub type R = crate::R<MMCCRrs>;
#[doc = "Register `MMCCR` writer"]
pub type W = crate::W<MMCCRrs>;
#[doc = "Field `CR` reader - Counter reset"]
pub type CR_R = crate::BitReader;
#[doc = "Field `CR` writer - Counter reset"]
pub type CR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` reader - Counter stop rollover"]
pub type CSR_R = crate::BitReader;
#[doc = "Field `CSR` writer - Counter stop rollover"]
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROR` reader - Reset on read"]
pub type ROR_R = crate::BitReader;
#[doc = "Field `ROR` writer - Reset on read"]
pub type ROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCF` reader - MMC counter freeze"]
pub type MCF_R = crate::BitReader;
#[doc = "Field `MCF` writer - MMC counter freeze"]
pub type MCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCP` reader - MMC counter preset"]
pub type MCP_R = crate::BitReader;
#[doc = "Field `MCP` writer - MMC counter preset"]
pub type MCP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCFHP` reader - MMC counter Full-Half preset"]
pub type MCFHP_R = crate::BitReader;
#[doc = "Field `MCFHP` writer - MMC counter Full-Half preset"]
pub type MCFHP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC counter preset"]
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC counter Full-Half preset"]
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MMCCRrs> {
        CR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<MMCCRrs> {
        CSR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> ROR_W<MMCCRrs> {
        ROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - MMC counter freeze"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<MMCCRrs> {
        MCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - MMC counter preset"]
    #[inline(always)]
    #[must_use]
    pub fn mcp(&mut self) -> MCP_W<MMCCRrs> {
        MCP_W::new(self, 4)
    }
    #[doc = "Bit 5 - MMC counter Full-Half preset"]
    #[inline(always)]
    #[must_use]
    pub fn mcfhp(&mut self) -> MCFHP_W<MMCCRrs> {
        MCFHP_W::new(self, 5)
    }
}
#[doc = "Ethernet MMC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCCRrs;
impl crate::RegisterSpec for MMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmccr::R`](R) reader structure"]
impl crate::Readable for MMCCRrs {}
#[doc = "`write(|w| ..)` method takes [`mmccr::W`](W) writer structure"]
impl crate::Writable for MMCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCCR to value 0"]
impl crate::Resettable for MMCCRrs {
    const RESET_VALUE: u32 = 0;
}
