///Register `MMCCR` reader
pub type R = crate::R<MMCCRrs>;
///Register `MMCCR` writer
pub type W = crate::W<MMCCRrs>;
///Field `CR` reader - CR
pub type CR_R = crate::BitReader;
///Field `CR` writer - CR
pub type CR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSR` reader - CSR
pub type CSR_R = crate::BitReader;
///Field `CSR` writer - CSR
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROR` reader - ROR
pub type ROR_R = crate::BitReader;
///Field `ROR` writer - ROR
pub type ROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCF` reader - MCF
pub type MCF_R = crate::BitReader;
///Field `MCF` writer - MCF
pub type MCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCP` reader - MCP
pub type MCP_R = crate::BitReader;
///Field `MCP` writer - MCP
pub type MCP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCFHP` reader - MCFHP
pub type MCFHP_R = crate::BitReader;
///Field `MCFHP` writer - MCFHP
pub type MCFHP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CR
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CSR
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ROR
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCF
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MCP
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MCFHP
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCCR")
            .field("cr", &self.cr())
            .field("csr", &self.csr())
            .field("ror", &self.ror())
            .field("mcf", &self.mcf())
            .field("mcp", &self.mcp())
            .field("mcfhp", &self.mcfhp())
            .finish()
    }
}
impl W {
    ///Bit 0 - CR
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, MMCCRrs> {
        CR_W::new(self, 0)
    }
    ///Bit 1 - CSR
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<'_, MMCCRrs> {
        CSR_W::new(self, 1)
    }
    ///Bit 2 - ROR
    #[inline(always)]
    pub fn ror(&mut self) -> ROR_W<'_, MMCCRrs> {
        ROR_W::new(self, 2)
    }
    ///Bit 3 - MCF
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W<'_, MMCCRrs> {
        MCF_W::new(self, 3)
    }
    ///Bit 4 - MCP
    #[inline(always)]
    pub fn mcp(&mut self) -> MCP_W<'_, MMCCRrs> {
        MCP_W::new(self, 4)
    }
    ///Bit 5 - MCFHP
    #[inline(always)]
    pub fn mcfhp(&mut self) -> MCFHP_W<'_, MMCCRrs> {
        MCFHP_W::new(self, 5)
    }
}
/**Ethernet MMC control register

You can [`read`](crate::Reg::read) this register and get [`mmccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_MMC:MMCCR)*/
pub struct MMCCRrs;
impl crate::RegisterSpec for MMCCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmccr::R`](R) reader structure
impl crate::Readable for MMCCRrs {}
///`write(|w| ..)` method takes [`mmccr::W`](W) writer structure
impl crate::Writable for MMCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCCR to value 0
impl crate::Resettable for MMCCRrs {}
