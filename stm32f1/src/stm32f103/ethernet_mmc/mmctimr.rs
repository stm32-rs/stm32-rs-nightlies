///Register `MMCTIMR` reader
pub type R = crate::R<MMCTIMRrs>;
///Register `MMCTIMR` writer
pub type W = crate::W<MMCTIMRrs>;
///Field `TGFSCM` reader - Transmitted good frames single collision mask
pub type TGFSCM_R = crate::BitReader;
///Field `TGFSCM` writer - Transmitted good frames single collision mask
pub type TGFSCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TGFMSCM` reader - Transmitted good frames more single collision mask
pub type TGFMSCM_R = crate::BitReader;
///Field `TGFMSCM` writer - Transmitted good frames more single collision mask
pub type TGFMSCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TGFM` reader - Transmitted good frames mask
pub type TGFM_R = crate::BitReader;
///Field `TGFM` writer - Transmitted good frames mask
pub type TGFM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 14 - Transmitted good frames single collision mask
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transmitted good frames more single collision mask
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - Transmitted good frames mask
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTIMR")
            .field("tgfscm", &self.tgfscm())
            .field("tgfmscm", &self.tgfmscm())
            .field("tgfm", &self.tgfm())
            .finish()
    }
}
impl W {
    ///Bit 14 - Transmitted good frames single collision mask
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TGFSCM_W<'_, MMCTIMRrs> {
        TGFSCM_W::new(self, 14)
    }
    ///Bit 15 - Transmitted good frames more single collision mask
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W<'_, MMCTIMRrs> {
        TGFMSCM_W::new(self, 15)
    }
    ///Bit 21 - Transmitted good frames mask
    #[inline(always)]
    pub fn tgfm(&mut self) -> TGFM_W<'_, MMCTIMRrs> {
        TGFM_W::new(self, 21)
    }
}
/**Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)

You can [`read`](crate::Reg::read) this register and get [`mmctimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MMC:MMCTIMR)*/
pub struct MMCTIMRrs;
impl crate::RegisterSpec for MMCTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`mmctimr::R`](R) reader structure
impl crate::Readable for MMCTIMRrs {}
///`write(|w| ..)` method takes [`mmctimr::W`](W) writer structure
impl crate::Writable for MMCTIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCTIMR to value 0
impl crate::Resettable for MMCTIMRrs {}
