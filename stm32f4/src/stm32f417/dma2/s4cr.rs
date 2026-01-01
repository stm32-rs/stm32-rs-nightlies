///Register `S4CR` reader
pub type R = crate::R<S4CRrs>;
///Register `S4CR` writer
pub type W = crate::W<S4CRrs>;
///Field `EN` reader - Stream enable / flag stream ready when read low
pub type EN_R = crate::BitReader;
///Field `EN` writer - Stream enable / flag stream ready when read low
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMEIE` reader - Direct mode error interrupt enable
pub type DMEIE_R = crate::BitReader;
///Field `DMEIE` writer - Direct mode error interrupt enable
pub type DMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - Transfer error interrupt enable
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transfer error interrupt enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - Half transfer interrupt enable
pub type HTIE_R = crate::BitReader;
///Field `HTIE` writer - Half transfer interrupt enable
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PFCTRL` reader - Peripheral flow controller
pub type PFCTRL_R = crate::BitReader;
///Field `PFCTRL` writer - Peripheral flow controller
pub type PFCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - Data transfer direction
pub type DIR_R = crate::FieldReader;
///Field `DIR` writer - Data transfer direction
pub type DIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
///Field `PSIZE` reader - Peripheral data size
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - Peripheral data size
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSIZE` reader - Memory data size
pub type MSIZE_R = crate::FieldReader;
///Field `MSIZE` writer - Memory data size
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PINCOS` reader - Peripheral increment offset size
pub type PINCOS_R = crate::BitReader;
///Field `PINCOS` writer - Peripheral increment offset size
pub type PINCOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PL` reader - Priority level
pub type PL_R = crate::FieldReader;
///Field `PL` writer - Priority level
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DBM` reader - Double buffer mode
pub type DBM_R = crate::BitReader;
///Field `DBM` writer - Double buffer mode
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CT` reader - Current target (only in double buffer mode)
pub type CT_R = crate::BitReader;
///Field `CT` writer - Current target (only in double buffer mode)
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` reader - ACK
pub type ACK_R = crate::BitReader;
///Field `ACK` writer - ACK
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBURST` reader - Peripheral burst transfer configuration
pub type PBURST_R = crate::FieldReader;
///Field `PBURST` writer - Peripheral burst transfer configuration
pub type PBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MBURST` reader - Memory burst transfer configuration
pub type MBURST_R = crate::FieldReader;
///Field `MBURST` writer - Memory burst transfer configuration
pub type MBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHSEL` reader - Channel selection
pub type CHSEL_R = crate::FieldReader;
///Field `CHSEL` writer - Channel selection
pub type CHSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Stream enable / flag stream ready when read low
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Direct mode error interrupt enable
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Peripheral flow controller
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Data transfer direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Circular mode
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Peripheral data size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - Memory data size
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - Peripheral increment offset size
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Priority level
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Current target (only in double buffer mode)
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ACK
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Peripheral burst transfer configuration
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 23:24 - Memory burst transfer configuration
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bits 25:27 - Channel selection
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S4CR")
            .field("chsel", &self.chsel())
            .field("mburst", &self.mburst())
            .field("pburst", &self.pburst())
            .field("ack", &self.ack())
            .field("ct", &self.ct())
            .field("dbm", &self.dbm())
            .field("pl", &self.pl())
            .field("pincos", &self.pincos())
            .field("msize", &self.msize())
            .field("psize", &self.psize())
            .field("minc", &self.minc())
            .field("pinc", &self.pinc())
            .field("circ", &self.circ())
            .field("dir", &self.dir())
            .field("pfctrl", &self.pfctrl())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dmeie", &self.dmeie())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Stream enable / flag stream ready when read low
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, S4CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Direct mode error interrupt enable
    #[inline(always)]
    pub fn dmeie(&mut self) -> DMEIE_W<'_, S4CRrs> {
        DMEIE_W::new(self, 1)
    }
    ///Bit 2 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, S4CRrs> {
        TEIE_W::new(self, 2)
    }
    ///Bit 3 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, S4CRrs> {
        HTIE_W::new(self, 3)
    }
    ///Bit 4 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, S4CRrs> {
        TCIE_W::new(self, 4)
    }
    ///Bit 5 - Peripheral flow controller
    #[inline(always)]
    pub fn pfctrl(&mut self) -> PFCTRL_W<'_, S4CRrs> {
        PFCTRL_W::new(self, 5)
    }
    ///Bits 6:7 - Data transfer direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, S4CRrs> {
        DIR_W::new(self, 6)
    }
    ///Bit 8 - Circular mode
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<'_, S4CRrs> {
        CIRC_W::new(self, 8)
    }
    ///Bit 9 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<'_, S4CRrs> {
        PINC_W::new(self, 9)
    }
    ///Bit 10 - Memory increment mode
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<'_, S4CRrs> {
        MINC_W::new(self, 10)
    }
    ///Bits 11:12 - Peripheral data size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, S4CRrs> {
        PSIZE_W::new(self, 11)
    }
    ///Bits 13:14 - Memory data size
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<'_, S4CRrs> {
        MSIZE_W::new(self, 13)
    }
    ///Bit 15 - Peripheral increment offset size
    #[inline(always)]
    pub fn pincos(&mut self) -> PINCOS_W<'_, S4CRrs> {
        PINCOS_W::new(self, 15)
    }
    ///Bits 16:17 - Priority level
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, S4CRrs> {
        PL_W::new(self, 16)
    }
    ///Bit 18 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<'_, S4CRrs> {
        DBM_W::new(self, 18)
    }
    ///Bit 19 - Current target (only in double buffer mode)
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<'_, S4CRrs> {
        CT_W::new(self, 19)
    }
    ///Bit 20 - ACK
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<'_, S4CRrs> {
        ACK_W::new(self, 20)
    }
    ///Bits 21:22 - Peripheral burst transfer configuration
    #[inline(always)]
    pub fn pburst(&mut self) -> PBURST_W<'_, S4CRrs> {
        PBURST_W::new(self, 21)
    }
    ///Bits 23:24 - Memory burst transfer configuration
    #[inline(always)]
    pub fn mburst(&mut self) -> MBURST_W<'_, S4CRrs> {
        MBURST_W::new(self, 23)
    }
    ///Bits 25:27 - Channel selection
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W<'_, S4CRrs> {
        CHSEL_W::new(self, 25)
    }
}
/**stream x configuration register

You can [`read`](crate::Reg::read) this register and get [`s4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#DMA2:S4CR)*/
pub struct S4CRrs;
impl crate::RegisterSpec for S4CRrs {
    type Ux = u32;
}
///`read()` method returns [`s4cr::R`](R) reader structure
impl crate::Readable for S4CRrs {}
///`write(|w| ..)` method takes [`s4cr::W`](W) writer structure
impl crate::Writable for S4CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S4CR to value 0
impl crate::Resettable for S4CRrs {}
