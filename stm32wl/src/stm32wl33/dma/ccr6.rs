///Register `CCR6` reader
pub type R = crate::R<CCR6rs>;
///Register `CCR6` writer
pub type W = crate::W<CCR6rs>;
///Field `EN` reader - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled
pub type HTIE_R = crate::BitReader;
///Field `HTIE` writer - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory
pub type DIR_R = crate::BitReader;
///Field `DIR` writer - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIRC` reader - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled
pub type CIRC_R = crate::BitReader;
///Field `CIRC` writer - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINC` reader - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled
pub type PINC_R = crate::BitReader;
///Field `PINC` writer - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINC` reader - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled
pub type MINC_R = crate::BitReader;
///Field `MINC` writer - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSIZE` reader - PSIZE\[1:0\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - PSIZE\[1:0\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSIZE` reader - MSIZE\[1:0\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
pub type MSIZE_R = crate::FieldReader;
///Field `MSIZE` writer - MSIZE\[1:0\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PL` reader - PL\[1:0\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high
pub type PL_R = crate::FieldReader;
///Field `PL` writer - PL\[1:0\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MEM2MEM` reader - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled
pub type MEM2MEM_R = crate::BitReader;
///Field `MEM2MEM` writer - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - PSIZE\[1:0\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - MSIZE\[1:0\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - PL\[1:0\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR6")
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
    ///Bit 0 - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CCR6rs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CCR6rs> {
        TCIE_W::new(self, 1)
    }
    ///Bit 2 - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, CCR6rs> {
        HTIE_W::new(self, 2)
    }
    ///Bit 3 - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CCR6rs> {
        TEIE_W::new(self, 3)
    }
    ///Bit 4 - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, CCR6rs> {
        DIR_W::new(self, 4)
    }
    ///Bit 5 - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<'_, CCR6rs> {
        CIRC_W::new(self, 5)
    }
    ///Bit 6 - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<'_, CCR6rs> {
        PINC_W::new(self, 6)
    }
    ///Bit 7 - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<'_, CCR6rs> {
        MINC_W::new(self, 7)
    }
    ///Bits 8:9 - PSIZE\[1:0\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, CCR6rs> {
        PSIZE_W::new(self, 8)
    }
    ///Bits 10:11 - MSIZE\[1:0\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<'_, CCR6rs> {
        MSIZE_W::new(self, 10)
    }
    ///Bits 12:13 - PL\[1:0\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, CCR6rs> {
        PL_W::new(self, 12)
    }
    ///Bit 14 - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<'_, CCR6rs> {
        MEM2MEM_W::new(self, 14)
    }
}
/**DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR6)*/
pub struct CCR6rs;
impl crate::RegisterSpec for CCR6rs {
    type Ux = u32;
}
///`read()` method returns [`ccr6::R`](R) reader structure
impl crate::Readable for CCR6rs {}
///`write(|w| ..)` method takes [`ccr6::W`](W) writer structure
impl crate::Writable for CCR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR6 to value 0
impl crate::Resettable for CCR6rs {}
