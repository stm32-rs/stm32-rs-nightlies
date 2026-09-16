///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `TSOM` reader - Tx start of message
pub type TSOM_R = crate::BitReader;
///Field `TSOM` writer - Tx start of message
pub type TSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEOM` reader - Tx end of message
pub type TEOM_R = crate::BitReader;
///Field `TEOM` writer - Tx end of message
pub type TEOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERR` reader - Tx error
pub type TERR_R = crate::BitReader;
///Field `TERR` writer - Tx error
pub type TERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBTRF` reader - Tx byte transfer request or block transfer finished
pub type TBTRF_R = crate::BitReader;
///Field `TBTRF` writer - Tx byte transfer request or block transfer finished
pub type TBTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSOM` reader - Rx start of message
pub type RSOM_R = crate::BitReader;
///Field `RSOM` writer - Rx start of message
pub type RSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REOM` reader - Rx end of message
pub type REOM_R = crate::BitReader;
///Field `REOM` writer - Rx end of message
pub type REOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RERR` reader - Rx error
pub type RERR_R = crate::BitReader;
///Field `RERR` writer - Rx error
pub type RERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBTF` reader - Rx byte/block transfer finished
pub type RBTF_R = crate::BitReader;
///Field `RBTF` writer - Rx byte/block transfer finished
pub type RBTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tx start of message
    #[inline(always)]
    pub fn tsom(&self) -> TSOM_R {
        TSOM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx end of message
    #[inline(always)]
    pub fn teom(&self) -> TEOM_R {
        TEOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tx error
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tx byte transfer request or block transfer finished
    #[inline(always)]
    pub fn tbtrf(&self) -> TBTRF_R {
        TBTRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx start of message
    #[inline(always)]
    pub fn rsom(&self) -> RSOM_R {
        RSOM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx end of message
    #[inline(always)]
    pub fn reom(&self) -> REOM_R {
        REOM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx error
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx byte/block transfer finished
    #[inline(always)]
    pub fn rbtf(&self) -> RBTF_R {
        RBTF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("tsom", &self.tsom())
            .field("teom", &self.teom())
            .field("terr", &self.terr())
            .field("tbtrf", &self.tbtrf())
            .field("rsom", &self.rsom())
            .field("reom", &self.reom())
            .field("rerr", &self.rerr())
            .field("rbtf", &self.rbtf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tx start of message
    #[inline(always)]
    pub fn tsom(&mut self) -> TSOM_W<'_, CSRrs> {
        TSOM_W::new(self, 0)
    }
    ///Bit 1 - Tx end of message
    #[inline(always)]
    pub fn teom(&mut self) -> TEOM_W<'_, CSRrs> {
        TEOM_W::new(self, 1)
    }
    ///Bit 2 - Tx error
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W<'_, CSRrs> {
        TERR_W::new(self, 2)
    }
    ///Bit 3 - Tx byte transfer request or block transfer finished
    #[inline(always)]
    pub fn tbtrf(&mut self) -> TBTRF_W<'_, CSRrs> {
        TBTRF_W::new(self, 3)
    }
    ///Bit 4 - Rx start of message
    #[inline(always)]
    pub fn rsom(&mut self) -> RSOM_W<'_, CSRrs> {
        RSOM_W::new(self, 4)
    }
    ///Bit 5 - Rx end of message
    #[inline(always)]
    pub fn reom(&mut self) -> REOM_W<'_, CSRrs> {
        REOM_W::new(self, 5)
    }
    ///Bit 6 - Rx error
    #[inline(always)]
    pub fn rerr(&mut self) -> RERR_W<'_, CSRrs> {
        RERR_W::new(self, 6)
    }
    ///Bit 7 - Rx byte/block transfer finished
    #[inline(always)]
    pub fn rbtf(&mut self) -> RBTF_W<'_, CSRrs> {
        RBTF_W::new(self, 7)
    }
}
/**CEC control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
