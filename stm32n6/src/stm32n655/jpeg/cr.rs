///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `JCEN` reader - JPEG core enable
pub type JCEN_R = crate::BitReader;
///Field `JCEN` writer - JPEG core enable
pub type JCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFTIE` reader - Input FIFO threshold interrupt enable
pub type IFTIE_R = crate::BitReader;
///Field `IFTIE` writer - Input FIFO threshold interrupt enable
pub type IFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFNFIE` reader - Input FIFO not full interrupt enable
pub type IFNFIE_R = crate::BitReader;
///Field `IFNFIE` writer - Input FIFO not full interrupt enable
pub type IFNFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFTIE` reader - Output FIFO threshold interrupt enable
pub type OFTIE_R = crate::BitReader;
///Field `OFTIE` writer - Output FIFO threshold interrupt enable
pub type OFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFNEIE` reader - Output FIFO not empty interrupt enable
pub type OFNEIE_R = crate::BitReader;
///Field `OFNEIE` writer - Output FIFO not empty interrupt enable
pub type OFNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCIE` reader - End of conversion interrupt enable
pub type EOCIE_R = crate::BitReader;
///Field `EOCIE` writer - End of conversion interrupt enable
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPDIE` reader - Header parsing done interrupt enable
pub type HPDIE_R = crate::BitReader;
///Field `HPDIE` writer - Header parsing done interrupt enable
pub type HPDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDMAEN` reader - Input DMA enable
pub type IDMAEN_R = crate::BitReader;
///Field `IDMAEN` writer - Input DMA enable
pub type IDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODMAEN` reader - Output DMA enable
pub type ODMAEN_R = crate::BitReader;
///Field `ODMAEN` writer - Output DMA enable
pub type ODMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFF` writer - Input FIFO flush
pub type IFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFF` writer - Output FIFO flush
pub type OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - JPEG core enable
    #[inline(always)]
    pub fn jcen(&self) -> JCEN_R {
        JCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Input FIFO threshold interrupt enable
    #[inline(always)]
    pub fn iftie(&self) -> IFTIE_R {
        IFTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Input FIFO not full interrupt enable
    #[inline(always)]
    pub fn ifnfie(&self) -> IFNFIE_R {
        IFNFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output FIFO threshold interrupt enable
    #[inline(always)]
    pub fn oftie(&self) -> OFTIE_R {
        OFTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Output FIFO not empty interrupt enable
    #[inline(always)]
    pub fn ofneie(&self) -> OFNEIE_R {
        OFNEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of conversion interrupt enable
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Header parsing done interrupt enable
    #[inline(always)]
    pub fn hpdie(&self) -> HPDIE_R {
        HPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - Input DMA enable
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output DMA enable
    #[inline(always)]
    pub fn odmaen(&self) -> ODMAEN_R {
        ODMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("jcen", &self.jcen())
            .field("iftie", &self.iftie())
            .field("ifnfie", &self.ifnfie())
            .field("oftie", &self.oftie())
            .field("ofneie", &self.ofneie())
            .field("eocie", &self.eocie())
            .field("hpdie", &self.hpdie())
            .field("idmaen", &self.idmaen())
            .field("odmaen", &self.odmaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - JPEG core enable
    #[inline(always)]
    pub fn jcen(&mut self) -> JCEN_W<'_, CRrs> {
        JCEN_W::new(self, 0)
    }
    ///Bit 1 - Input FIFO threshold interrupt enable
    #[inline(always)]
    pub fn iftie(&mut self) -> IFTIE_W<'_, CRrs> {
        IFTIE_W::new(self, 1)
    }
    ///Bit 2 - Input FIFO not full interrupt enable
    #[inline(always)]
    pub fn ifnfie(&mut self) -> IFNFIE_W<'_, CRrs> {
        IFNFIE_W::new(self, 2)
    }
    ///Bit 3 - Output FIFO threshold interrupt enable
    #[inline(always)]
    pub fn oftie(&mut self) -> OFTIE_W<'_, CRrs> {
        OFTIE_W::new(self, 3)
    }
    ///Bit 4 - Output FIFO not empty interrupt enable
    #[inline(always)]
    pub fn ofneie(&mut self) -> OFNEIE_W<'_, CRrs> {
        OFNEIE_W::new(self, 4)
    }
    ///Bit 5 - End of conversion interrupt enable
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<'_, CRrs> {
        EOCIE_W::new(self, 5)
    }
    ///Bit 6 - Header parsing done interrupt enable
    #[inline(always)]
    pub fn hpdie(&mut self) -> HPDIE_W<'_, CRrs> {
        HPDIE_W::new(self, 6)
    }
    ///Bit 11 - Input DMA enable
    #[inline(always)]
    pub fn idmaen(&mut self) -> IDMAEN_W<'_, CRrs> {
        IDMAEN_W::new(self, 11)
    }
    ///Bit 12 - Output DMA enable
    #[inline(always)]
    pub fn odmaen(&mut self) -> ODMAEN_W<'_, CRrs> {
        ODMAEN_W::new(self, 12)
    }
    ///Bit 13 - Input FIFO flush
    #[inline(always)]
    pub fn iff(&mut self) -> IFF_W<'_, CRrs> {
        IFF_W::new(self, 13)
    }
    ///Bit 14 - Output FIFO flush
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W<'_, CRrs> {
        OFF_W::new(self, 14)
    }
}
/**JPEG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
