///Register `IMSCR` reader
pub type R = crate::R<IMSCRrs>;
///Register `IMSCR` writer
pub type W = crate::W<IMSCRrs>;
///Field `INIM` reader - Input FIFO service interrupt mask
pub type INIM_R = crate::BitReader;
///Field `INIM` writer - Input FIFO service interrupt mask
pub type INIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTIM` reader - Output FIFO service interrupt mask
pub type OUTIM_R = crate::BitReader;
///Field `OUTIM` writer - Output FIFO service interrupt mask
pub type OUTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Input FIFO service interrupt mask
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output FIFO service interrupt mask
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMSCR")
            .field("outim", &self.outim())
            .field("inim", &self.inim())
            .finish()
    }
}
impl W {
    ///Bit 0 - Input FIFO service interrupt mask
    #[inline(always)]
    pub fn inim(&mut self) -> INIM_W<'_, IMSCRrs> {
        INIM_W::new(self, 0)
    }
    ///Bit 1 - Output FIFO service interrupt mask
    #[inline(always)]
    pub fn outim(&mut self) -> OUTIM_W<'_, IMSCRrs> {
        OUTIM_W::new(self, 1)
    }
}
/**interrupt mask set/clear register

You can [`read`](crate::Reg::read) this register and get [`imscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F405.html#CRYP:IMSCR)*/
pub struct IMSCRrs;
impl crate::RegisterSpec for IMSCRrs {
    type Ux = u32;
}
///`read()` method returns [`imscr::R`](R) reader structure
impl crate::Readable for IMSCRrs {}
///`write(|w| ..)` method takes [`imscr::W`](W) writer structure
impl crate::Writable for IMSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMSCR to value 0
impl crate::Resettable for IMSCRrs {}
