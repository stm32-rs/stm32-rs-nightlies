///Register `MMCRIMR` reader
pub type R = crate::R<MMCRIMRrs>;
///Register `MMCRIMR` writer
pub type W = crate::W<MMCRIMRrs>;
///Field `RFCEM` reader - Received frame CRC error mask
pub type RFCEM_R = crate::BitReader;
///Field `RFCEM` writer - Received frame CRC error mask
pub type RFCEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFAEM` reader - Received frames alignment error mask
pub type RFAEM_R = crate::BitReader;
///Field `RFAEM` writer - Received frames alignment error mask
pub type RFAEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RGUFM` reader - Received good unicast frames mask
pub type RGUFM_R = crate::BitReader;
///Field `RGUFM` writer - Received good unicast frames mask
pub type RGUFM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - Received frame CRC error mask
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Received frames alignment error mask
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - Received good unicast frames mask
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRIMR")
            .field("rfcem", &self.rfcem())
            .field("rfaem", &self.rfaem())
            .field("rgufm", &self.rgufm())
            .finish()
    }
}
impl W {
    ///Bit 5 - Received frame CRC error mask
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W<'_, MMCRIMRrs> {
        RFCEM_W::new(self, 5)
    }
    ///Bit 6 - Received frames alignment error mask
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W<'_, MMCRIMRrs> {
        RFAEM_W::new(self, 6)
    }
    ///Bit 17 - Received good unicast frames mask
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W<'_, MMCRIMRrs> {
        RGUFM_W::new(self, 17)
    }
}
/**Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)

You can [`read`](crate::Reg::read) this register and get [`mmcrimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MMC:MMCRIMR)*/
pub struct MMCRIMRrs;
impl crate::RegisterSpec for MMCRIMRrs {
    type Ux = u32;
}
///`read()` method returns [`mmcrimr::R`](R) reader structure
impl crate::Readable for MMCRIMRrs {}
///`write(|w| ..)` method takes [`mmcrimr::W`](W) writer structure
impl crate::Writable for MMCRIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCRIMR to value 0
impl crate::Resettable for MMCRIMRrs {}
