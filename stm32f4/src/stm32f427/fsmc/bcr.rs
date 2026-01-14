///Register `BCR%s` reader
pub type R = crate::R<BCRrs>;
///Register `BCR%s` writer
pub type W = crate::W<BCRrs>;
///ASYNCWAIT
pub use super::bcr1::ASYNCWAIT;
///Field `ASYNCWAIT` reader - ASYNCWAIT
pub use super::bcr1::ASYNCWAIT_R;
///Field `ASYNCWAIT` writer - ASYNCWAIT
pub use super::bcr1::ASYNCWAIT_W;
///BURSTEN
pub use super::bcr1::BURSTEN;
///Field `BURSTEN` reader - BURSTEN
pub use super::bcr1::BURSTEN_R;
///Field `BURSTEN` writer - BURSTEN
pub use super::bcr1::BURSTEN_W;
///CBURSTRW
pub use super::bcr1::CBURSTRW;
///Field `CBURSTRW` reader - CBURSTRW
pub use super::bcr1::CBURSTRW_R;
///Field `CBURSTRW` writer - CBURSTRW
pub use super::bcr1::CBURSTRW_W;
///CPSIZE
pub use super::bcr1::CPSIZE;
///Field `CPSIZE` reader - CPSIZE
pub use super::bcr1::CPSIZE_R;
///Field `CPSIZE` writer - CPSIZE
pub use super::bcr1::CPSIZE_W;
///EXTMOD
pub use super::bcr1::EXTMOD;
///Field `EXTMOD` reader - EXTMOD
pub use super::bcr1::EXTMOD_R;
///Field `EXTMOD` writer - EXTMOD
pub use super::bcr1::EXTMOD_W;
///FACCEN
pub use super::bcr1::FACCEN;
///Field `FACCEN` reader - FACCEN
pub use super::bcr1::FACCEN_R;
///Field `FACCEN` writer - FACCEN
pub use super::bcr1::FACCEN_W;
///MBKEN
pub use super::bcr1::MBKEN;
///Field `MBKEN` reader - MBKEN
pub use super::bcr1::MBKEN_R;
///Field `MBKEN` writer - MBKEN
pub use super::bcr1::MBKEN_W;
///MTYP
pub use super::bcr1::MTYP;
///Field `MTYP` reader - MTYP
pub use super::bcr1::MTYP_R;
///Field `MTYP` writer - MTYP
pub use super::bcr1::MTYP_W;
///MUXEN
pub use super::bcr1::MUXEN;
///Field `MUXEN` reader - MUXEN
pub use super::bcr1::MUXEN_R;
///Field `MUXEN` writer - MUXEN
pub use super::bcr1::MUXEN_W;
///MWID
pub use super::bcr1::MWID;
///Field `MWID` reader - MWID
pub use super::bcr1::MWID_R;
///Field `MWID` writer - MWID
pub use super::bcr1::MWID_W;
///WAITCFG
pub use super::bcr1::WAITCFG;
///Field `WAITCFG` reader - WAITCFG
pub use super::bcr1::WAITCFG_R;
///Field `WAITCFG` writer - WAITCFG
pub use super::bcr1::WAITCFG_W;
///WAITEN
pub use super::bcr1::WAITEN;
///Field `WAITEN` reader - WAITEN
pub use super::bcr1::WAITEN_R;
///Field `WAITEN` writer - WAITEN
pub use super::bcr1::WAITEN_W;
///WAITPOL
pub use super::bcr1::WAITPOL;
///Field `WAITPOL` reader - WAITPOL
pub use super::bcr1::WAITPOL_R;
///Field `WAITPOL` writer - WAITPOL
pub use super::bcr1::WAITPOL_W;
///WRAPMOD
pub use super::bcr1::WRAPMOD;
///Field `WRAPMOD` reader - WRAPMOD
pub use super::bcr1::WRAPMOD_R;
///Field `WRAPMOD` writer - WRAPMOD
pub use super::bcr1::WRAPMOD_W;
///WREN
pub use super::bcr1::WREN;
///Field `WREN` reader - WREN
pub use super::bcr1::WREN_R;
///Field `WREN` writer - WREN
pub use super::bcr1::WREN_W;
///Field `CCLKEN` reader - CCLKEN
pub type CCLKEN_R = crate::BitReader;
///Field `CCLKEN` writer - CCLKEN
pub type CCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 10 - WRAPMOD
    #[inline(always)]
    pub fn wrapmod(&self) -> WRAPMOD_R {
        WRAPMOD_R::new(((self.bits >> 10) & 1) != 0)
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
    ///Bits 16:18 - CPSIZE
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR")
            .field("cburstrw", &self.cburstrw())
            .field("cpsize", &self.cpsize())
            .field("asyncwait", &self.asyncwait())
            .field("extmod", &self.extmod())
            .field("waiten", &self.waiten())
            .field("wren", &self.wren())
            .field("waitcfg", &self.waitcfg())
            .field("wrapmod", &self.wrapmod())
            .field("waitpol", &self.waitpol())
            .field("bursten", &self.bursten())
            .field("faccen", &self.faccen())
            .field("mwid", &self.mwid())
            .field("mtyp", &self.mtyp())
            .field("muxen", &self.muxen())
            .field("mbken", &self.mbken())
            .field("cclken", &self.cclken())
            .finish()
    }
}
impl W {
    ///Bit 0 - MBKEN
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<'_, BCRrs> {
        MBKEN_W::new(self, 0)
    }
    ///Bit 1 - MUXEN
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<'_, BCRrs> {
        MUXEN_W::new(self, 1)
    }
    ///Bits 2:3 - MTYP
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<'_, BCRrs> {
        MTYP_W::new(self, 2)
    }
    ///Bits 4:5 - MWID
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<'_, BCRrs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - FACCEN
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<'_, BCRrs> {
        FACCEN_W::new(self, 6)
    }
    ///Bit 8 - BURSTEN
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<'_, BCRrs> {
        BURSTEN_W::new(self, 8)
    }
    ///Bit 9 - WAITPOL
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<'_, BCRrs> {
        WAITPOL_W::new(self, 9)
    }
    ///Bit 10 - WRAPMOD
    #[inline(always)]
    pub fn wrapmod(&mut self) -> WRAPMOD_W<'_, BCRrs> {
        WRAPMOD_W::new(self, 10)
    }
    ///Bit 11 - WAITCFG
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<'_, BCRrs> {
        WAITCFG_W::new(self, 11)
    }
    ///Bit 12 - WREN
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<'_, BCRrs> {
        WREN_W::new(self, 12)
    }
    ///Bit 13 - WAITEN
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<'_, BCRrs> {
        WAITEN_W::new(self, 13)
    }
    ///Bit 14 - EXTMOD
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<'_, BCRrs> {
        EXTMOD_W::new(self, 14)
    }
    ///Bit 15 - ASYNCWAIT
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<'_, BCRrs> {
        ASYNCWAIT_W::new(self, 15)
    }
    ///Bits 16:18 - CPSIZE
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<'_, BCRrs> {
        CPSIZE_W::new(self, 16)
    }
    ///Bit 19 - CBURSTRW
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<'_, BCRrs> {
        CBURSTRW_W::new(self, 19)
    }
    ///Bit 20 - CCLKEN
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W<'_, BCRrs> {
        CCLKEN_W::new(self, 20)
    }
}
/**SRAM/NOR-Flash chip-select control register %s

You can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#FSMC:BCR[2])*/
pub struct BCRrs;
impl crate::RegisterSpec for BCRrs {
    type Ux = u32;
}
///`read()` method returns [`bcr::R`](R) reader structure
impl crate::Readable for BCRrs {}
///`write(|w| ..)` method takes [`bcr::W`](W) writer structure
impl crate::Writable for BCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR%s to value 0x30d2
impl crate::Resettable for BCRrs {
    const RESET_VALUE: u32 = 0x30d2;
}
