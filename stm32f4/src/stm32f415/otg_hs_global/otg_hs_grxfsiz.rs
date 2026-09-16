///Register `OTG_HS_GRXFSIZ` reader
pub type R = crate::R<OTG_HS_GRXFSIZrs>;
///Register `OTG_HS_GRXFSIZ` writer
pub type W = crate::W<OTG_HS_GRXFSIZrs>;
///Field `RXFD` reader - RxFIFO depth
pub type RXFD_R = crate::FieldReader<u16>;
///Field `RXFD` writer - RxFIFO depth
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - RxFIFO depth
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_GRXFSIZ")
            .field("rxfd", &self.rxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - RxFIFO depth
    #[inline(always)]
    pub fn rxfd(&mut self) -> RXFD_W<'_, OTG_HS_GRXFSIZrs> {
        RXFD_W::new(self, 0)
    }
}
/**OTG_HS Receive FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_HS_GLOBAL:OTG_HS_GRXFSIZ)*/
pub struct OTG_HS_GRXFSIZrs;
impl crate::RegisterSpec for OTG_HS_GRXFSIZrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_grxfsiz::R`](R) reader structure
impl crate::Readable for OTG_HS_GRXFSIZrs {}
///`write(|w| ..)` method takes [`otg_hs_grxfsiz::W`](W) writer structure
impl crate::Writable for OTG_HS_GRXFSIZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_GRXFSIZ to value 0x0200
impl crate::Resettable for OTG_HS_GRXFSIZrs {
    const RESET_VALUE: u32 = 0x0200;
}
