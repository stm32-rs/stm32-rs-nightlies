///Register `OTG_HS_DIEPTXF5` reader
pub type R = crate::R<OTG_HS_DIEPTXF5rs>;
///Register `OTG_HS_DIEPTXF5` writer
pub type W = crate::W<OTG_HS_DIEPTXF5rs>;
///Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address
pub type INEPTXSA_R = crate::FieldReader<u16>;
///Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INEPTXFD` reader - IN endpoint TxFIFO depth
pub type INEPTXFD_R = crate::FieldReader<u16>;
///Field `INEPTXFD` writer - IN endpoint TxFIFO depth
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN endpoint FIFOx transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DIEPTXF5")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN endpoint FIFOx transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<'_, OTG_HS_DIEPTXF5rs> {
        INEPTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<'_, OTG_HS_DIEPTXF5rs> {
        INEPTXFD_W::new(self, 16)
    }
}
/**OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF5)*/
pub struct OTG_HS_DIEPTXF5rs;
impl crate::RegisterSpec for OTG_HS_DIEPTXF5rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_dieptxf5::R`](R) reader structure
impl crate::Readable for OTG_HS_DIEPTXF5rs {}
///`write(|w| ..)` method takes [`otg_hs_dieptxf5::W`](W) writer structure
impl crate::Writable for OTG_HS_DIEPTXF5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DIEPTXF5 to value 0x0200_0400
impl crate::Resettable for OTG_HS_DIEPTXF5rs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
