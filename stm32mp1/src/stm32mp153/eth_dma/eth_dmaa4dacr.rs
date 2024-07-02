///Register `ETH_DMAA4DACR` reader
pub type R = crate::R<ETH_DMAA4DACRrs>;
///Register `ETH_DMAA4DACR` writer
pub type W = crate::W<ETH_DMAA4DACRrs>;
///Field `TDWC` reader - TDWC
pub type TDWC_R = crate::FieldReader;
///Field `TDWC` writer - TDWC
pub type TDWC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TDWD` reader - TDWD
pub type TDWD_R = crate::FieldReader;
///Field `TDWD` writer - TDWD
pub type TDWD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RDRC` reader - RDRC
pub type RDRC_R = crate::FieldReader;
///Field `RDRC` writer - RDRC
pub type RDRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RDP` reader - RDP
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - RDP
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WRP` reader - WRP
pub type WRP_R = crate::FieldReader;
///Field `WRP` writer - WRP
pub type WRP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - TDWC
    #[inline(always)]
    pub fn tdwc(&self) -> TDWC_R {
        TDWC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - TDWD
    #[inline(always)]
    pub fn tdwd(&self) -> TDWD_R {
        TDWD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - RDRC
    #[inline(always)]
    pub fn rdrc(&self) -> RDRC_R {
        RDRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:18 - RDP
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - WRP
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAA4DACR")
            .field("tdwc", &self.tdwc())
            .field("tdwd", &self.tdwd())
            .field("rdrc", &self.rdrc())
            .field("rdp", &self.rdp())
            .field("wrp", &self.wrp())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TDWC
    #[inline(always)]
    #[must_use]
    pub fn tdwc(&mut self) -> TDWC_W<ETH_DMAA4DACRrs> {
        TDWC_W::new(self, 0)
    }
    ///Bits 4:5 - TDWD
    #[inline(always)]
    #[must_use]
    pub fn tdwd(&mut self) -> TDWD_W<ETH_DMAA4DACRrs> {
        TDWD_W::new(self, 4)
    }
    ///Bits 8:11 - RDRC
    #[inline(always)]
    #[must_use]
    pub fn rdrc(&mut self) -> RDRC_W<ETH_DMAA4DACRrs> {
        RDRC_W::new(self, 8)
    }
    ///Bits 16:18 - RDP
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<ETH_DMAA4DACRrs> {
        RDP_W::new(self, 16)
    }
    ///Bits 20:22 - WRP
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WRP_W<ETH_DMAA4DACRrs> {
        WRP_W::new(self, 20)
    }
}
/**AXI4 descriptor ACE control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmaa4dacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmaa4dacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAA4DACR)*/
pub struct ETH_DMAA4DACRrs;
impl crate::RegisterSpec for ETH_DMAA4DACRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmaa4dacr::R`](R) reader structure
impl crate::Readable for ETH_DMAA4DACRrs {}
///`write(|w| ..)` method takes [`eth_dmaa4dacr::W`](W) writer structure
impl crate::Writable for ETH_DMAA4DACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_DMAA4DACR to value 0
impl crate::Resettable for ETH_DMAA4DACRrs {
    const RESET_VALUE: u32 = 0;
}
