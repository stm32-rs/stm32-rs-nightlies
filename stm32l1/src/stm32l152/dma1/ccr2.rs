///Register `CCR2` reader
pub type R = crate::R<CCR2rs>;
///Register `CCR2` writer
pub type W = crate::W<CCR2rs>;
///Field `EN` reader - Channel enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - Channel enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - Half transfer interrupt enable
pub type HTIE_R = crate::BitReader;
///Field `HTIE` writer - Half transfer interrupt enable
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - Transfer error interrupt enable
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transfer error interrupt enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - Data transfer direction
pub type DIR_R = crate::BitReader;
///Field `DIR` writer - Data transfer direction
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIRC` reader - Circular mode
pub type CIRC_R = crate::BitReader;
///Field `CIRC` writer - Circular mode
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINC` reader - Peripheral increment mode
pub type PINC_R = crate::BitReader;
///Field `PINC` writer - Peripheral increment mode
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINC` reader - Memory increment mode
pub type MINC_R = crate::BitReader;
///Field `MINC` writer - Memory increment mode
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSIZE` reader - Peripheral size
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - Peripheral size
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSIZE` reader - Memory size
pub type MSIZE_R = crate::FieldReader;
///Field `MSIZE` writer - Memory size
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PL` reader - Channel priority level
pub type PL_R = crate::FieldReader;
///Field `PL` writer - Channel priority level
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MEM2MEM` reader - Memory to memory mode
pub type MEM2MEM_R = crate::BitReader;
///Field `MEM2MEM` writer - Memory to memory mode
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Channel enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data transfer direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Circular mode
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Memory size
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Channel priority level
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR2")
            .field("mem2mem", &self.mem2mem())
            .field("pl", &self.pl())
            .field("msize", &self.msize())
            .field("psize", &self.psize())
            .field("minc", &self.minc())
            .field("pinc", &self.pinc())
            .field("circ", &self.circ())
            .field("dir", &self.dir())
            .field("teie", &self.teie())
            .field("htie", &self.htie())
            .field("tcie", &self.tcie())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CCR2rs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CCR2rs> {
        TCIE_W::new(self, 1)
    }
    ///Bit 2 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<CCR2rs> {
        HTIE_W::new(self, 2)
    }
    ///Bit 3 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<CCR2rs> {
        TEIE_W::new(self, 3)
    }
    ///Bit 4 - Data transfer direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<CCR2rs> {
        DIR_W::new(self, 4)
    }
    ///Bit 5 - Circular mode
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<CCR2rs> {
        CIRC_W::new(self, 5)
    }
    ///Bit 6 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<CCR2rs> {
        PINC_W::new(self, 6)
    }
    ///Bit 7 - Memory increment mode
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<CCR2rs> {
        MINC_W::new(self, 7)
    }
    ///Bits 8:9 - Peripheral size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<CCR2rs> {
        PSIZE_W::new(self, 8)
    }
    ///Bits 10:11 - Memory size
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<CCR2rs> {
        MSIZE_W::new(self, 10)
    }
    ///Bits 12:13 - Channel priority level
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<CCR2rs> {
        PL_W::new(self, 12)
    }
    ///Bit 14 - Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<CCR2rs> {
        MEM2MEM_W::new(self, 14)
    }
}
/**channel x configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#DMA1:CCR2)*/
pub struct CCR2rs;
impl crate::RegisterSpec for CCR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccr2::R`](R) reader structure
impl crate::Readable for CCR2rs {}
///`write(|w| ..)` method takes [`ccr2::W`](W) writer structure
impl crate::Writable for CCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR2 to value 0
impl crate::Resettable for CCR2rs {
    const RESET_VALUE: u32 = 0;
}
