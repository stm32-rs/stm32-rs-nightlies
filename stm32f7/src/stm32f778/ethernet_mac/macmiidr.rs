///Register `MACMIIDR` reader
pub type R = crate::R<MACMIIDRrs>;
///Register `MACMIIDR` writer
pub type W = crate::W<MACMIIDRrs>;
///Field `MD` reader - MII data read from/written to the PHY
pub type MD_R = crate::FieldReader<u16>;
///Field `MD` writer - MII data read from/written to the PHY
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - MII data read from/written to the PHY
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMIIDR").field("md", &self.md()).finish()
    }
}
impl W {
    ///Bits 0:15 - MII data read from/written to the PHY
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<'_, MACMIIDRrs> {
        MD_W::new(self, 0)
    }
}
/**Ethernet MAC MII data register

You can [`read`](crate::Reg::read) this register and get [`macmiidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#Ethernet_MAC:MACMIIDR)*/
pub struct MACMIIDRrs;
impl crate::RegisterSpec for MACMIIDRrs {
    type Ux = u32;
}
///`read()` method returns [`macmiidr::R`](R) reader structure
impl crate::Readable for MACMIIDRrs {}
///`write(|w| ..)` method takes [`macmiidr::W`](W) writer structure
impl crate::Writable for MACMIIDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACMIIDR to value 0
impl crate::Resettable for MACMIIDRrs {}
