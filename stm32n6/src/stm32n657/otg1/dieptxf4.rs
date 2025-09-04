///Register `DIEPTXF4` reader
pub type R = crate::R<DIEPTXF4rs>;
///Register `DIEPTXF4` writer
pub type W = crate::W<DIEPTXF4rs>;
///Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address
pub type INEPTXSA_R = crate::FieldReader<u16>;
///Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INEPTXFD` reader - IN endpoint Tx FIFO depth
pub type INEPTXFD_R = crate::FieldReader<u16>;
///Field `INEPTXFD` writer - IN endpoint Tx FIFO depth
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN endpoint FIFOx transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - IN endpoint Tx FIFO depth
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF4")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN endpoint FIFOx transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<DIEPTXF4rs> {
        INEPTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - IN endpoint Tx FIFO depth
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<DIEPTXF4rs> {
        INEPTXFD_W::new(self, 16)
    }
}
/**OTG device IN endpoint transmit FIFO 4 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:DIEPTXF4)*/
pub struct DIEPTXF4rs;
impl crate::RegisterSpec for DIEPTXF4rs {
    type Ux = u32;
}
///`read()` method returns [`dieptxf4::R`](R) reader structure
impl crate::Readable for DIEPTXF4rs {}
///`write(|w| ..)` method takes [`dieptxf4::W`](W) writer structure
impl crate::Writable for DIEPTXF4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTXF4 to value 0x0200_0a00
impl crate::Resettable for DIEPTXF4rs {
    const RESET_VALUE: u32 = 0x0200_0a00;
}
