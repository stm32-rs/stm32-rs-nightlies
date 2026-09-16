///Register `MACCSRSWCR` reader
pub type R = crate::R<MACCSRSWCRrs>;
///Register `MACCSRSWCR` writer
pub type W = crate::W<MACCSRSWCRrs>;
///Field `RCWE` reader - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read).
pub type RCWE_R = crate::BitReader;
///Field `RCWE` writer - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read).
pub type RCWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEEN` reader - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space.
pub type SEEN_R = crate::BitReader;
///Field `SEEN` writer - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space.
pub type SEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read).
    #[inline(always)]
    pub fn rcwe(&self) -> RCWE_R {
        RCWE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space.
    #[inline(always)]
    pub fn seen(&self) -> SEEN_R {
        SEEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACCSRSWCR")
            .field("rcwe", &self.rcwe())
            .field("seen", &self.seen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read).
    #[inline(always)]
    pub fn rcwe(&mut self) -> RCWE_W<'_, MACCSRSWCRrs> {
        RCWE_W::new(self, 0)
    }
    ///Bit 8 - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space.
    #[inline(always)]
    pub fn seen(&mut self) -> SEEN_W<'_, MACCSRSWCRrs> {
        SEEN_W::new(self, 8)
    }
}
/**CSR software control register

You can [`read`](crate::Reg::read) this register and get [`maccsrswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccsrswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACCSRSWCR)*/
pub struct MACCSRSWCRrs;
impl crate::RegisterSpec for MACCSRSWCRrs {
    type Ux = u32;
}
///`read()` method returns [`maccsrswcr::R`](R) reader structure
impl crate::Readable for MACCSRSWCRrs {}
///`write(|w| ..)` method takes [`maccsrswcr::W`](W) writer structure
impl crate::Writable for MACCSRSWCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACCSRSWCR to value 0
impl crate::Resettable for MACCSRSWCRrs {}
