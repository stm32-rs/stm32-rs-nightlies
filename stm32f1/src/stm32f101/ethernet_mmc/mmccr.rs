///Register `MMCCR` reader
pub type R = crate::R<MMCCRrs>;
///Register `MMCCR` writer
pub type W = crate::W<MMCCRrs>;
///Field `CR` reader - Counter reset
pub type CR_R = crate::BitReader;
///Field `CR` writer - Counter reset
pub type CR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSR` reader - Counter stop rollover
pub type CSR_R = crate::BitReader;
///Field `CSR` writer - Counter stop rollover
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROR` reader - Reset on read
pub type ROR_R = crate::BitReader;
///Field `ROR` writer - Reset on read
pub type ROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCF` reader - MMC counter freeze
pub type MCF_R = crate::BitReader;
///Field `MCF` writer - MMC counter freeze
pub type MCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Counter reset
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 31 - MMC counter freeze
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCCR")
            .field("cr", &self.cr())
            .field("csr", &self.csr())
            .field("ror", &self.ror())
            .field("mcf", &self.mcf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Counter reset
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<MMCCRrs> {
        CR_W::new(self, 0)
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<MMCCRrs> {
        CSR_W::new(self, 1)
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    pub fn ror(&mut self) -> ROR_W<MMCCRrs> {
        ROR_W::new(self, 2)
    }
    ///Bit 31 - MMC counter freeze
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W<MMCCRrs> {
        MCF_W::new(self, 31)
    }
}
/**Ethernet MMC control register (ETH_MMCCR)

You can [`read`](crate::Reg::read) this register and get [`mmccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#Ethernet_MMC:MMCCR)*/
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
