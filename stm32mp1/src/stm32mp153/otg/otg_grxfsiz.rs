///Register `OTG_GRXFSIZ` reader
pub type R = crate::R<OTG_GRXFSIZrs>;
///Register `OTG_GRXFSIZ` writer
pub type W = crate::W<OTG_GRXFSIZrs>;
///Field `RXFD` reader - RXFD
pub type RXFD_R = crate::FieldReader<u16>;
///Field `RXFD` writer - RXFD
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - RXFD
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_GRXFSIZ")
            .field("rxfd", &self.rxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - RXFD
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<OTG_GRXFSIZrs> {
        RXFD_W::new(self, 0)
    }
}
/**The application can program the RAM size that must be allocated to the Rx FIFO.

You can [`read`](crate::Reg::read) this register and get [`otg_grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:OTG_GRXFSIZ)*/
pub struct OTG_GRXFSIZrs;
impl crate::RegisterSpec for OTG_GRXFSIZrs {
    type Ux = u32;
}
///`read()` method returns [`otg_grxfsiz::R`](R) reader structure
impl crate::Readable for OTG_GRXFSIZrs {}
///`write(|w| ..)` method takes [`otg_grxfsiz::W`](W) writer structure
impl crate::Writable for OTG_GRXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_GRXFSIZ to value 0x0400
impl crate::Resettable for OTG_GRXFSIZrs {
    const RESET_VALUE: u32 = 0x0400;
}
