///Register `HPTXFSIZ` reader
pub type R = crate::R<HPTXFSIZrs>;
///Register `HPTXFSIZ` writer
pub type W = crate::W<HPTXFSIZrs>;
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
        f.debug_struct("HPTXFSIZ")
            .field("ptxsa", &self.ptxsa())
            .field("ptxfd", &self.ptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Host periodic TxFIFO start address
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W<'_, HPTXFSIZrs> {
        PTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - Host periodic TxFIFO depth
    #[inline(always)]
    pub fn ptxfd(&mut self) -> PTXFD_W<'_, HPTXFSIZrs> {
        PTXFD_W::new(self, 16)
    }
}
/**OTG_HS Host periodic transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#OTG1_HS_GLOBAL:HPTXFSIZ)*/
pub struct HPTXFSIZrs;
impl crate::RegisterSpec for HPTXFSIZrs {
    type Ux = u32;
}
///`read()` method returns [`hptxfsiz::R`](R) reader structure
impl crate::Readable for HPTXFSIZrs {}
///`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure
impl crate::Writable for HPTXFSIZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPTXFSIZ to value 0x0200_0600
impl crate::Resettable for HPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200_0600;
}
