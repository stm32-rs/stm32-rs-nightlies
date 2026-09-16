///Register `DIEPTXF%s` reader
pub type R = crate::R<DIEPTXFrs>;
///Register `DIEPTXF%s` writer
pub type W = crate::W<DIEPTXFrs>;
///Field `INEPTXSA` reader - IN endpoint FIFO2 transmit RAM start address
pub type INEPTXSA_R = crate::FieldReader<u16>;
///Field `INEPTXSA` writer - IN endpoint FIFO2 transmit RAM start address
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INEPTXFD` reader - IN endpoint TxFIFO depth
pub type INEPTXFD_R = crate::FieldReader<u16>;
///Field `INEPTXFD` writer - IN endpoint TxFIFO depth
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN endpoint FIFO2 transmit RAM start address
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
        f.debug_struct("DIEPTXF")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN endpoint FIFO2 transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<'_, DIEPTXFrs> {
        INEPTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<'_, DIEPTXFrs> {
        INEPTXFD_W::new(self, 16)
    }
}
/**OTG_FS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#OTG_FS_GLOBAL:DIEPTXF[1])*/
pub struct DIEPTXFrs;
impl crate::RegisterSpec for DIEPTXFrs {
    type Ux = u32;
}
///`read()` method returns [`dieptxf::R`](R) reader structure
impl crate::Readable for DIEPTXFrs {}
///`write(|w| ..)` method takes [`dieptxf::W`](W) writer structure
impl crate::Writable for DIEPTXFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTXF%s to value 0x0200_0400
impl crate::Resettable for DIEPTXFrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
