///Register `HPTXSTS` reader
pub type R = crate::R<HPTXSTSrs>;
///Register `HPTXSTS` writer
pub type W = crate::W<HPTXSTSrs>;
///Field `PTXFSAVL` reader - Periodic transmit data FIFO space available
pub type PTXFSAVL_R = crate::FieldReader<u16>;
///Field `PTXFSAVL` writer - Periodic transmit data FIFO space available
pub type PTXFSAVL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PTXQSAV` reader - Periodic transmit request queue space available
pub type PTXQSAV_R = crate::FieldReader;
///Field `PTXQTOP` reader - Top of the periodic transmit request queue
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Periodic transmit data FIFO space available
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Periodic transmit request queue space available
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Top of the periodic transmit request queue
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field("ptxfsavl", &self.ptxfsavl())
            .field("ptxqsav", &self.ptxqsav())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Periodic transmit data FIFO space available
    #[inline(always)]
    pub fn ptxfsavl(&mut self) -> PTXFSAVL_W<'_, HPTXSTSrs> {
        PTXFSAVL_W::new(self, 0)
    }
}
/**OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)

You can [`read`](crate::Reg::read) this register and get [`hptxsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F405.html#OTG_FS_HOST:HPTXSTS)*/
pub struct HPTXSTSrs;
impl crate::RegisterSpec for HPTXSTSrs {
    type Ux = u32;
}
///`read()` method returns [`hptxsts::R`](R) reader structure
impl crate::Readable for HPTXSTSrs {}
///`write(|w| ..)` method takes [`hptxsts::W`](W) writer structure
impl crate::Writable for HPTXSTSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPTXSTS to value 0x0008_0100
impl crate::Resettable for HPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0100;
}
