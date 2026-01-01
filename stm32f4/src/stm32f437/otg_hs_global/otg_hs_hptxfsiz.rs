///Register `OTG_HS_HPTXFSIZ` reader
pub type R = crate::R<OTG_HS_HPTXFSIZrs>;
///Register `OTG_HS_HPTXFSIZ` writer
pub type W = crate::W<OTG_HS_HPTXFSIZrs>;
///Field `PTXSA` reader - Host periodic TxFIFO start address
pub type PTXSA_R = crate::FieldReader<u16>;
///Field `PTXSA` writer - Host periodic TxFIFO start address
pub type PTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PTXFD` reader - Host periodic TxFIFO depth
pub type PTXFD_R = crate::FieldReader<u16>;
///Field `PTXFD` writer - Host periodic TxFIFO depth
pub type PTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Host periodic TxFIFO start address
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Host periodic TxFIFO depth
    #[inline(always)]
    pub fn ptxfd(&self) -> PTXFD_R {
        PTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_HPTXFSIZ")
            .field("ptxsa", &self.ptxsa())
            .field("ptxfd", &self.ptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Host periodic TxFIFO start address
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W<'_, OTG_HS_HPTXFSIZrs> {
        PTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - Host periodic TxFIFO depth
    #[inline(always)]
    pub fn ptxfd(&mut self) -> PTXFD_W<'_, OTG_HS_HPTXFSIZrs> {
        PTXFD_W::new(self, 16)
    }
}
/**OTG_HS Host periodic transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_GLOBAL:OTG_HS_HPTXFSIZ)*/
pub struct OTG_HS_HPTXFSIZrs;
impl crate::RegisterSpec for OTG_HS_HPTXFSIZrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_hptxfsiz::R`](R) reader structure
impl crate::Readable for OTG_HS_HPTXFSIZrs {}
///`write(|w| ..)` method takes [`otg_hs_hptxfsiz::W`](W) writer structure
impl crate::Writable for OTG_HS_HPTXFSIZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_HPTXFSIZ to value 0x0200_0600
impl crate::Resettable for OTG_HS_HPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200_0600;
}
