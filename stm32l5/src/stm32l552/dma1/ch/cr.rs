///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
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
///Field `DBM` reader - double-buffer mode
pub type DBM_R = crate::BitReader;
///Field `DBM` writer - double-buffer mode
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CT` reader - current target memory of DMA transfer in double-buffer mode
pub type CT_R = crate::BitReader;
///Field `CT` writer - current target memory of DMA transfer in double-buffer mode
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECM` reader - secure mode
pub type SECM_R = crate::BitReader;
///Field `SECM` writer - secure mode
pub type SECM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSEC` reader - security of the DMA transfer from the source
pub type SSEC_R = crate::BitReader;
///Field `SSEC` writer - security of the DMA transfer from the source
pub type SSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSEC` reader - security of the DMA transfer to the destination
pub type DSEC_R = crate::BitReader;
///Field `DSEC` writer - security of the DMA transfer to the destination
pub type DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - privileged mode
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - privileged mode
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 15 - double-buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - current target memory of DMA transfer in double-buffer mode
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - secure mode
    #[inline(always)]
    pub fn secm(&self) -> SECM_R {
        SECM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - security of the DMA transfer from the source
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - security of the DMA transfer to the destination
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - privileged mode
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("priv_", &self.priv_())
            .field("dsec", &self.dsec())
            .field("ssec", &self.ssec())
            .field("secm", &self.secm())
            .field("ct", &self.ct())
            .field("dbm", &self.dbm())
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
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 1)
    }
    ///Bit 2 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<CRrs> {
        HTIE_W::new(self, 2)
    }
    ///Bit 3 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 3)
    }
    ///Bit 4 - Data transfer direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<CRrs> {
        DIR_W::new(self, 4)
    }
    ///Bit 5 - Circular mode
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<CRrs> {
        CIRC_W::new(self, 5)
    }
    ///Bit 6 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<CRrs> {
        PINC_W::new(self, 6)
    }
    ///Bit 7 - Memory increment mode
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<CRrs> {
        MINC_W::new(self, 7)
    }
    ///Bits 8:9 - Peripheral size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 8)
    }
    ///Bits 10:11 - Memory size
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<CRrs> {
        MSIZE_W::new(self, 10)
    }
    ///Bits 12:13 - Channel priority level
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<CRrs> {
        PL_W::new(self, 12)
    }
    ///Bit 14 - Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<CRrs> {
        MEM2MEM_W::new(self, 14)
    }
    ///Bit 15 - double-buffer mode
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<CRrs> {
        DBM_W::new(self, 15)
    }
    ///Bit 16 - current target memory of DMA transfer in double-buffer mode
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<CRrs> {
        CT_W::new(self, 16)
    }
    ///Bit 17 - secure mode
    #[inline(always)]
    pub fn secm(&mut self) -> SECM_W<CRrs> {
        SECM_W::new(self, 17)
    }
    ///Bit 18 - security of the DMA transfer from the source
    #[inline(always)]
    pub fn ssec(&mut self) -> SSEC_W<CRrs> {
        SSEC_W::new(self, 18)
    }
    ///Bit 19 - security of the DMA transfer to the destination
    #[inline(always)]
    pub fn dsec(&mut self) -> DSEC_W<CRrs> {
        DSEC_W::new(self, 19)
    }
    ///Bit 20 - privileged mode
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<CRrs> {
        PRIV_W::new(self, 20)
    }
}
/**channel x configuration register

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
