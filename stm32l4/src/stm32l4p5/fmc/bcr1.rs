///Register `BCR1` reader
pub type R = crate::R<BCR1rs>;
///Register `BCR1` writer
pub type W = crate::W<BCR1rs>;
///Field `MBKEN` reader - MBKEN
pub type MBKEN_R = crate::BitReader;
///Field `MBKEN` writer - MBKEN
pub type MBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUXEN` reader - MUXEN
pub type MUXEN_R = crate::BitReader;
///Field `MUXEN` writer - MUXEN
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MTYP` reader - MTYP
pub type MTYP_R = crate::FieldReader;
///Field `MTYP` writer - MTYP
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MWID` reader - MWID
pub type MWID_R = crate::FieldReader;
///Field `MWID` writer - MWID
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FACCEN` reader - FACCEN
pub type FACCEN_R = crate::BitReader;
///Field `FACCEN` writer - FACCEN
pub type FACCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BURSTEN` reader - BURSTEN
pub type BURSTEN_R = crate::BitReader;
///Field `BURSTEN` writer - BURSTEN
pub type BURSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITPOL` reader - WAITPOL
pub type WAITPOL_R = crate::BitReader;
///Field `WAITPOL` writer - WAITPOL
pub type WAITPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITCFG` reader - WAITCFG
pub type WAITCFG_R = crate::BitReader;
///Field `WAITCFG` writer - WAITCFG
pub type WAITCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WREN` reader - WREN
pub type WREN_R = crate::BitReader;
///Field `WREN` writer - WREN
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITEN` reader - WAITEN
pub type WAITEN_R = crate::BitReader;
///Field `WAITEN` writer - WAITEN
pub type WAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTMOD` reader - EXTMOD
pub type EXTMOD_R = crate::BitReader;
///Field `EXTMOD` writer - EXTMOD
pub type EXTMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNCWAIT` reader - ASYNCWAIT
pub type ASYNCWAIT_R = crate::BitReader;
///Field `ASYNCWAIT` writer - ASYNCWAIT
pub type ASYNCWAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPSIZE` reader - CRAM page size
pub type CPSIZE_R = crate::FieldReader;
///Field `CPSIZE` writer - CRAM page size
pub type CPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CBURSTRW` reader - CBURSTRW
pub type CBURSTRW_R = crate::BitReader;
///Field `CBURSTRW` writer - CBURSTRW
pub type CBURSTRW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCLKEN` reader - CCLKEN
pub type CCLKEN_R = crate::BitReader;
///Field `CCLKEN` writer - CCLKEN
pub type CCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WFDIS` reader - Write FIFO Disable
pub type WFDIS_R = crate::BitReader;
///Field `WFDIS` writer - Write FIFO Disable
pub type WFDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBLSET` reader - Byte lane (NBL) setup
pub type NBLSET_R = crate::FieldReader;
///Field `NBLSET` writer - Byte lane (NBL) setup
pub type NBLSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - MBKEN
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MUXEN
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - MTYP
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - MWID
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - FACCEN
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - BURSTEN
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WAITPOL
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - WAITCFG
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - WREN
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - WAITEN
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - EXTMOD
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ASYNCWAIT
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - CRAM page size
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - CBURSTRW
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CCLKEN
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Write FIFO Disable
    #[inline(always)]
    pub fn wfdis(&self) -> WFDIS_R {
        WFDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&self) -> NBLSET_R {
        NBLSET_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR1")
            .field("mbken", &self.mbken())
            .field("muxen", &self.muxen())
            .field("mtyp", &self.mtyp())
            .field("mwid", &self.mwid())
            .field("faccen", &self.faccen())
            .field("bursten", &self.bursten())
            .field("waitpol", &self.waitpol())
            .field("waitcfg", &self.waitcfg())
            .field("wren", &self.wren())
            .field("waiten", &self.waiten())
            .field("extmod", &self.extmod())
            .field("asyncwait", &self.asyncwait())
            .field("cburstrw", &self.cburstrw())
            .field("cclken", &self.cclken())
            .field("wfdis", &self.wfdis())
            .field("cpsize", &self.cpsize())
            .field("nblset", &self.nblset())
            .finish()
    }
}
impl W {
    ///Bit 0 - MBKEN
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<BCR1rs> {
        MBKEN_W::new(self, 0)
    }
    ///Bit 1 - MUXEN
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<BCR1rs> {
        MUXEN_W::new(self, 1)
    }
    ///Bits 2:3 - MTYP
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<BCR1rs> {
        MTYP_W::new(self, 2)
    }
    ///Bits 4:5 - MWID
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<BCR1rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - FACCEN
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<BCR1rs> {
        FACCEN_W::new(self, 6)
    }
    ///Bit 8 - BURSTEN
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<BCR1rs> {
        BURSTEN_W::new(self, 8)
    }
    ///Bit 9 - WAITPOL
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCR1rs> {
        WAITPOL_W::new(self, 9)
    }
    ///Bit 11 - WAITCFG
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCR1rs> {
        WAITCFG_W::new(self, 11)
    }
    ///Bit 12 - WREN
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<BCR1rs> {
        WREN_W::new(self, 12)
    }
    ///Bit 13 - WAITEN
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<BCR1rs> {
        WAITEN_W::new(self, 13)
    }
    ///Bit 14 - EXTMOD
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<BCR1rs> {
        EXTMOD_W::new(self, 14)
    }
    ///Bit 15 - ASYNCWAIT
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCR1rs> {
        ASYNCWAIT_W::new(self, 15)
    }
    ///Bits 16:18 - CRAM page size
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<BCR1rs> {
        CPSIZE_W::new(self, 16)
    }
    ///Bit 19 - CBURSTRW
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCR1rs> {
        CBURSTRW_W::new(self, 19)
    }
    ///Bit 20 - CCLKEN
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W<BCR1rs> {
        CCLKEN_W::new(self, 20)
    }
    ///Bit 21 - Write FIFO Disable
    #[inline(always)]
    pub fn wfdis(&mut self) -> WFDIS_W<BCR1rs> {
        WFDIS_W::new(self, 21)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&mut self) -> NBLSET_W<BCR1rs> {
        NBLSET_W::new(self, 22)
    }
}
/**SRAM/NOR-Flash chip-select control register 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#FMC:BCR1)*/
pub struct BCR1rs;
impl crate::RegisterSpec for BCR1rs {
    type Ux = u32;
}
///`read()` method returns [`bcr1::R`](R) reader structure
impl crate::Readable for BCR1rs {}
///`write(|w| ..)` method takes [`bcr1::W`](W) writer structure
impl crate::Writable for BCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR1 to value 0x30d0
impl crate::Resettable for BCR1rs {
    const RESET_VALUE: u32 = 0x30d0;
}
