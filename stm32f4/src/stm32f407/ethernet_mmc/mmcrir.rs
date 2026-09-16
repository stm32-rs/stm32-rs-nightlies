///Register `MMCRIR` reader
pub type R = crate::R<MMCRIRrs>;
///Register `MMCRIR` writer
pub type W = crate::W<MMCRIRrs>;
///Field `RFCES` reader - Received frames CRC error status
pub type RFCES_R = crate::BitReader;
///Field `RFCES` writer - Received frames CRC error status
pub type RFCES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFAES` reader - Received frames alignment error status
pub type RFAES_R = crate::BitReader;
///Field `RFAES` writer - Received frames alignment error status
pub type RFAES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RGUFS` reader - Received good Unicast frames status
pub type RGUFS_R = crate::BitReader;
///Field `RGUFS` writer - Received good Unicast frames status
pub type RGUFS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - Received frames CRC error status
    #[inline(always)]
    pub fn rfces(&self) -> RFCES_R {
        RFCES_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Received frames alignment error status
    #[inline(always)]
    pub fn rfaes(&self) -> RFAES_R {
        RFAES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - Received good Unicast frames status
    #[inline(always)]
    pub fn rgufs(&self) -> RGUFS_R {
        RGUFS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRIR")
            .field("rfces", &self.rfces())
            .field("rfaes", &self.rfaes())
            .field("rgufs", &self.rgufs())
            .finish()
    }
}
impl W {
    ///Bit 5 - Received frames CRC error status
    #[inline(always)]
    pub fn rfces(&mut self) -> RFCES_W<'_, MMCRIRrs> {
        RFCES_W::new(self, 5)
    }
    ///Bit 6 - Received frames alignment error status
    #[inline(always)]
    pub fn rfaes(&mut self) -> RFAES_W<'_, MMCRIRrs> {
        RFAES_W::new(self, 6)
    }
    ///Bit 17 - Received good Unicast frames status
    #[inline(always)]
    pub fn rgufs(&mut self) -> RGUFS_W<'_, MMCRIRrs> {
        RGUFS_W::new(self, 17)
    }
}
/**Ethernet MMC receive interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmcrir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_MMC:MMCRIR)*/
pub struct MMCRIRrs;
impl crate::RegisterSpec for MMCRIRrs {
    type Ux = u32;
}
///`read()` method returns [`mmcrir::R`](R) reader structure
impl crate::Readable for MMCRIRrs {}
///`write(|w| ..)` method takes [`mmcrir::W`](W) writer structure
impl crate::Writable for MMCRIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCRIR to value 0
impl crate::Resettable for MMCRIRrs {}
