///Register `MACMDIODR` reader
pub type R = crate::R<MACMDIODRrs>;
///Register `MACMDIODR` writer
pub type W = crate::W<MACMDIODRrs>;
///Field `MD` reader - MII Data
pub type MD_R = crate::FieldReader<u16>;
///Field `MD` writer - MII Data
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RA` reader - Register Address
pub type RA_R = crate::FieldReader<u16>;
///Field `RA` writer - Register Address
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - MII Data
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Register Address
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMDIODR")
            .field("md", &self.md())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - MII Data
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<'_, MACMDIODRrs> {
        MD_W::new(self, 0)
    }
    ///Bits 16:31 - Register Address
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, MACMDIODRrs> {
        RA_W::new(self, 16)
    }
}
/**MDIO data register

You can [`read`](crate::Reg::read) this register and get [`macmdiodr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdiodr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#Ethernet_MAC:MACMDIODR)*/
pub struct MACMDIODRrs;
impl crate::RegisterSpec for MACMDIODRrs {
    type Ux = u32;
}
///`read()` method returns [`macmdiodr::R`](R) reader structure
impl crate::Readable for MACMDIODRrs {}
///`write(|w| ..)` method takes [`macmdiodr::W`](W) writer structure
impl crate::Writable for MACMDIODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACMDIODR to value 0
impl crate::Resettable for MACMDIODRrs {}
