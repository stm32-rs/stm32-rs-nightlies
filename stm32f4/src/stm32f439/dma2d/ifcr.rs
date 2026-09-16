///Register `IFCR` reader
pub type R = crate::R<IFCRrs>;
///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `CTEIF` reader - Clear Transfer error interrupt flag
pub type CTEIF_R = crate::BitReader;
///Field `CTEIF` writer - Clear Transfer error interrupt flag
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF` reader - Clear transfer complete interrupt flag
pub type CTCIF_R = crate::BitReader;
///Field `CTCIF` writer - Clear transfer complete interrupt flag
pub type CTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTWIF` reader - Clear transfer watermark interrupt flag
pub type CTWIF_R = crate::BitReader;
///Field `CTWIF` writer - Clear transfer watermark interrupt flag
pub type CTWIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAECIF` reader - Clear CLUT access error interrupt flag
pub type CAECIF_R = crate::BitReader;
///Field `CAECIF` writer - Clear CLUT access error interrupt flag
pub type CAECIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag
pub type CCTCIF_R = crate::BitReader;
///Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCEIF` reader - Clear configuration error interrupt flag
pub type CCEIF_R = crate::BitReader;
///Field `CCEIF` writer - Clear configuration error interrupt flag
pub type CCEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear Transfer error interrupt flag
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear transfer complete interrupt flag
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear configuration error interrupt flag
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFCR")
            .field("cceif", &self.cceif())
            .field("cctcif", &self.cctcif())
            .field("caecif", &self.caecif())
            .field("ctwif", &self.ctwif())
            .field("ctcif", &self.ctcif())
            .field("cteif", &self.cteif())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Transfer error interrupt flag
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    ///Bit 1 - Clear transfer complete interrupt flag
    #[inline(always)]
    pub fn ctcif(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 1)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag
    #[inline(always)]
    pub fn ctwif(&mut self) -> CTWIF_W<'_, IFCRrs> {
        CTWIF_W::new(self, 2)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag
    #[inline(always)]
    pub fn caecif(&mut self) -> CAECIF_W<'_, IFCRrs> {
        CAECIF_W::new(self, 3)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W<'_, IFCRrs> {
        CCTCIF_W::new(self, 4)
    }
    ///Bit 5 - Clear configuration error interrupt flag
    #[inline(always)]
    pub fn cceif(&mut self) -> CCEIF_W<'_, IFCRrs> {
        CCEIF_W::new(self, 5)
    }
}
/**interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DMA2D:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ifcr::R`](R) reader structure
impl crate::Readable for IFCRrs {}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
