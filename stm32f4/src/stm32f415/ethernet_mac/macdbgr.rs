///Register `MACDBGR` reader
pub type R = crate::R<MACDBGRrs>;
///Field `CR` reader - CR
pub type CR_R = crate::BitReader;
///Field `CSR` reader - CSR
pub type CSR_R = crate::BitReader;
///Field `ROR` reader - ROR
pub type ROR_R = crate::BitReader;
///Field `MCF` reader - MCF
pub type MCF_R = crate::BitReader;
///Field `MCP` reader - MCP
pub type MCP_R = crate::BitReader;
///Field `MCFHP` reader - MCFHP
pub type MCFHP_R = crate::BitReader;
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
        f.debug_struct("MACDBGR")
            .field("cr", &self.cr())
            .field("csr", &self.csr())
            .field("ror", &self.ror())
            .field("mcf", &self.mcf())
            .field("mcp", &self.mcp())
            .field("mcfhp", &self.mcfhp())
            .finish()
    }
}
/**Ethernet MAC debug register

You can [`read`](crate::Reg::read) this register and get [`macdbgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#Ethernet_MAC:MACDBGR)*/
pub struct MACDBGRrs;
impl crate::RegisterSpec for MACDBGRrs {
    type Ux = u32;
}
///`read()` method returns [`macdbgr::R`](R) reader structure
impl crate::Readable for MACDBGRrs {}
///`reset()` method sets MACDBGR to value 0
impl crate::Resettable for MACDBGRrs {}
