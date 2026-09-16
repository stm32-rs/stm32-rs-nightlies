///Register `BCR4` reader
pub type R = crate::R<BCR4rs>;
///Register `BCR4` writer
pub type W = crate::W<BCR4rs>;
///Field `MBKEN` reader - Memory region enable bit
pub type MBKEN_R = crate::BitReader;
///Field `MBKEN` writer - Memory region enable bit
pub type MBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUXEN` reader - Address/data multiplexing enable bit
pub type MUXEN_R = crate::BitReader;
///Field `MUXEN` writer - Address/data multiplexing enable bit
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MTYP` reader - Memory type
pub type MTYP_R = crate::FieldReader;
///Field `MTYP` writer - Memory type
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MWID` reader - Memory data bus width
pub type MWID_R = crate::FieldReader;
///Field `MWID` writer - Memory data bus width
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FACCEN` reader - Flash memory access enable
pub type FACCEN_R = crate::BitReader;
///Field `FACCEN` writer - Flash memory access enable
pub type FACCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BURSTEN` reader - Burst enable bit
pub type BURSTEN_R = crate::BitReader;
///Field `BURSTEN` writer - Burst enable bit
pub type BURSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITPOL` reader - Wait signal polarity bit
pub type WAITPOL_R = crate::BitReader;
///Field `WAITPOL` writer - Wait signal polarity bit
pub type WAITPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITCFG` reader - Wait timing configuration
pub type WAITCFG_R = crate::BitReader;
///Field `WAITCFG` writer - Wait timing configuration
pub type WAITCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WREN` reader - Write enable bit
pub type WREN_R = crate::BitReader;
///Field `WREN` writer - Write enable bit
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITEN` reader - Wait enable bit
pub type WAITEN_R = crate::BitReader;
///Field `WAITEN` writer - Wait enable bit
pub type WAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTMOD` reader - Extended mode enable
pub type EXTMOD_R = crate::BitReader;
///Field `EXTMOD` writer - Extended mode enable
pub type EXTMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers
pub type ASYNCWAIT_R = crate::BitReader;
///Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers
pub type ASYNCWAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPSIZE` reader - CRAM page size
pub type CPSIZE_R = crate::FieldReader;
///Field `CPSIZE` writer - CRAM page size
pub type CPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CBURSTRW` reader - Write burst enable
pub type CBURSTRW_R = crate::BitReader;
///Field `CBURSTRW` writer - Write burst enable
pub type CBURSTRW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSCOUNT0` reader - Chip Select (CS) counter
pub type CSCOUNT0_R = crate::FieldReader;
///Field `CSCOUNT0` writer - Chip Select (CS) counter
pub type CSCOUNT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NBLSET` reader - Byte lane (NBL) setup
pub type NBLSET_R = crate::FieldReader;
///Field `NBLSET` writer - Byte lane (NBL) setup
pub type NBLSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Memory region enable bit
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Flash memory access enable
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - CRAM page size
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Chip Select (CS) counter
    #[inline(always)]
    pub fn cscount0(&self) -> CSCOUNT0_R {
        CSCOUNT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&self) -> NBLSET_R {
        NBLSET_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR4")
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
            .field("cpsize", &self.cpsize())
            .field("cburstrw", &self.cburstrw())
            .field("cscount0", &self.cscount0())
            .field("nblset", &self.nblset())
            .finish()
    }
}
impl W {
    ///Bit 0 - Memory region enable bit
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<'_, BCR4rs> {
        MBKEN_W::new(self, 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<'_, BCR4rs> {
        MUXEN_W::new(self, 1)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<'_, BCR4rs> {
        MTYP_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<'_, BCR4rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Flash memory access enable
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<'_, BCR4rs> {
        FACCEN_W::new(self, 6)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<'_, BCR4rs> {
        BURSTEN_W::new(self, 8)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<'_, BCR4rs> {
        WAITPOL_W::new(self, 9)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<'_, BCR4rs> {
        WAITCFG_W::new(self, 11)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<'_, BCR4rs> {
        WREN_W::new(self, 12)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<'_, BCR4rs> {
        WAITEN_W::new(self, 13)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<'_, BCR4rs> {
        EXTMOD_W::new(self, 14)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<'_, BCR4rs> {
        ASYNCWAIT_W::new(self, 15)
    }
    ///Bits 16:18 - CRAM page size
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<'_, BCR4rs> {
        CPSIZE_W::new(self, 16)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<'_, BCR4rs> {
        CBURSTRW_W::new(self, 19)
    }
    ///Bits 20:21 - Chip Select (CS) counter
    #[inline(always)]
    pub fn cscount0(&mut self) -> CSCOUNT0_W<'_, BCR4rs> {
        CSCOUNT0_W::new(self, 20)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&mut self) -> NBLSET_W<'_, BCR4rs> {
        NBLSET_W::new(self, 22)
    }
}
/**SRAM/NOR Flash chip-select control register for memory region 4

You can [`read`](crate::Reg::read) this register and get [`bcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FMC1:BCR4)*/
pub struct BCR4rs;
impl crate::RegisterSpec for BCR4rs {
    type Ux = u32;
}
///`read()` method returns [`bcr4::R`](R) reader structure
impl crate::Readable for BCR4rs {}
///`write(|w| ..)` method takes [`bcr4::W`](W) writer structure
impl crate::Writable for BCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR4 to value 0x30d2
impl crate::Resettable for BCR4rs {
    const RESET_VALUE: u32 = 0x30d2;
}
