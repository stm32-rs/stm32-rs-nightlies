///Register `ETH_MACMDIODR` reader
pub type R = crate::R<ETH_MACMDIODRrs>;
///Register `ETH_MACMDIODR` writer
pub type W = crate::W<ETH_MACMDIODRrs>;
///Field `GD` reader - GD
pub type GD_R = crate::FieldReader<u16>;
///Field `GD` writer - GD
pub type GD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RA` reader - RA
pub type RA_R = crate::FieldReader<u16>;
///Field `RA` writer - RA
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - GD
    #[inline(always)]
    pub fn gd(&self) -> GD_R {
        GD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - RA
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACMDIODR")
            .field("gd", &self.gd())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - GD
    #[inline(always)]
    #[must_use]
    pub fn gd(&mut self) -> GD_W<ETH_MACMDIODRrs> {
        GD_W::new(self, 0)
    }
    ///Bits 16:31 - RA
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<ETH_MACMDIODRrs> {
        RA_W::new(self, 16)
    }
}
/**The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.

You can [`read`](crate::Reg::read) this register and get [`eth_macmdiodr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macmdiodr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACMDIODR)*/
pub struct ETH_MACMDIODRrs;
impl crate::RegisterSpec for ETH_MACMDIODRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macmdiodr::R`](R) reader structure
impl crate::Readable for ETH_MACMDIODRrs {}
///`write(|w| ..)` method takes [`eth_macmdiodr::W`](W) writer structure
impl crate::Writable for ETH_MACMDIODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACMDIODR to value 0
impl crate::Resettable for ETH_MACMDIODRrs {
    const RESET_VALUE: u32 = 0;
}
