///Register `MMCTIR` reader
pub type R = crate::R<MMCTIRrs>;
///Register `MMCTIR` writer
pub type W = crate::W<MMCTIRrs>;
///Field `TGFSCS` reader - Transmitted good frames single collision status
pub type TGFSCS_R = crate::BitReader;
///Field `TGFSCS` writer - Transmitted good frames single collision status
pub type TGFSCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TGFMSCS` reader - Transmitted good frames more single collision status
pub type TGFMSCS_R = crate::BitReader;
///Field `TGFMSCS` writer - Transmitted good frames more single collision status
pub type TGFMSCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TGFS` reader - Transmitted good frames status
pub type TGFS_R = crate::BitReader;
///Field `TGFS` writer - Transmitted good frames status
pub type TGFS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 14 - Transmitted good frames single collision status
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transmitted good frames more single collision status
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - Transmitted good frames status
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTIR")
            .field("tgfscs", &self.tgfscs())
            .field("tgfmscs", &self.tgfmscs())
            .field("tgfs", &self.tgfs())
            .finish()
    }
}
impl W {
    ///Bit 14 - Transmitted good frames single collision status
    #[inline(always)]
    pub fn tgfscs(&mut self) -> TGFSCS_W<'_, MMCTIRrs> {
        TGFSCS_W::new(self, 14)
    }
    ///Bit 15 - Transmitted good frames more single collision status
    #[inline(always)]
    pub fn tgfmscs(&mut self) -> TGFMSCS_W<'_, MMCTIRrs> {
        TGFMSCS_W::new(self, 15)
    }
    ///Bit 21 - Transmitted good frames status
    #[inline(always)]
    pub fn tgfs(&mut self) -> TGFS_W<'_, MMCTIRrs> {
        TGFS_W::new(self, 21)
    }
}
/**Ethernet MMC transmit interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmctir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#Ethernet_MMC:MMCTIR)*/
pub struct MMCTIRrs;
impl crate::RegisterSpec for MMCTIRrs {
    type Ux = u32;
}
///`read()` method returns [`mmctir::R`](R) reader structure
impl crate::Readable for MMCTIRrs {}
///`write(|w| ..)` method takes [`mmctir::W`](W) writer structure
impl crate::Writable for MMCTIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCTIR to value 0
impl crate::Resettable for MMCTIRrs {}
