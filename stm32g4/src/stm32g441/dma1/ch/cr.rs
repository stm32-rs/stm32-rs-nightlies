///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - channel enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - channel enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - HTIE
pub type HTIE_R = crate::BitReader;
///Field `HTIE` writer - HTIE
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - TEIE
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - TEIE
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - DIR
pub type DIR_R = crate::BitReader;
///Field `DIR` writer - DIR
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIRC` reader - CIRC
pub type CIRC_R = crate::BitReader;
///Field `CIRC` writer - CIRC
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINC` reader - PINC
pub type PINC_R = crate::BitReader;
///Field `PINC` writer - PINC
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINC` reader - MINC
pub type MINC_R = crate::BitReader;
///Field `MINC` writer - MINC
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSIZE` reader - PSIZE
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - PSIZE
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSIZE` reader - MSIZE
pub type MSIZE_R = crate::FieldReader;
///Field `MSIZE` writer - MSIZE
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PL` reader - PL
pub type PL_R = crate::FieldReader;
///Field `PL` writer - PL
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MEM2MEM` reader - MEM2MEM
pub type MEM2MEM_R = crate::BitReader;
///Field `MEM2MEM` writer - MEM2MEM
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - channel enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HTIE
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TEIE
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DIR
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CIRC
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PINC
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MINC
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - PSIZE
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - MSIZE
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - PL
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - MEM2MEM
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .finish()
    }
}
impl W {
    ///Bit 0 - channel enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - TCIE
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 1)
    }
    ///Bit 2 - HTIE
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<CRrs> {
        HTIE_W::new(self, 2)
    }
    ///Bit 3 - TEIE
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 3)
    }
    ///Bit 4 - DIR
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<CRrs> {
        DIR_W::new(self, 4)
    }
    ///Bit 5 - CIRC
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<CRrs> {
        CIRC_W::new(self, 5)
    }
    ///Bit 6 - PINC
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<CRrs> {
        PINC_W::new(self, 6)
    }
    ///Bit 7 - MINC
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<CRrs> {
        MINC_W::new(self, 7)
    }
    ///Bits 8:9 - PSIZE
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 8)
    }
    ///Bits 10:11 - MSIZE
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<CRrs> {
        MSIZE_W::new(self, 10)
    }
    ///Bits 12:13 - PL
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<CRrs> {
        PL_W::new(self, 12)
    }
    ///Bit 14 - MEM2MEM
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<CRrs> {
        MEM2MEM_W::new(self, 14)
    }
}
/**DMA channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
