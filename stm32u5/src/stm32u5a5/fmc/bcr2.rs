#[doc = "Register `BCR2` reader"]
pub type R = crate::R<BCR2rs>;
#[doc = "Register `BCR2` writer"]
pub type W = crate::W<BCR2rs>;
#[doc = "Field `MBKEN` reader - Memory bank enable bit"]
pub type MBKEN_R = crate::BitReader;
#[doc = "Field `MBKEN` writer - Memory bank enable bit"]
pub type MBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUXEN` reader - Address/data multiplexing enable bit"]
pub type MUXEN_R = crate::BitReader;
#[doc = "Field `MUXEN` writer - Address/data multiplexing enable bit"]
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTYP` reader - Memory type"]
pub type MTYP_R = crate::FieldReader;
#[doc = "Field `MTYP` writer - Memory type"]
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWID` reader - Memory data bus width"]
pub type MWID_R = crate::FieldReader;
#[doc = "Field `MWID` writer - Memory data bus width"]
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FACCEN` reader - Flash access enable"]
pub type FACCEN_R = crate::BitReader;
#[doc = "Field `FACCEN` writer - Flash access enable"]
pub type FACCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURSTEN` reader - Burst enable bit"]
pub type BURSTEN_R = crate::BitReader;
#[doc = "Field `BURSTEN` writer - Burst enable bit"]
pub type BURSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPOL` reader - Wait signal polarity bit"]
pub type WAITPOL_R = crate::BitReader;
#[doc = "Field `WAITPOL` writer - Wait signal polarity bit"]
pub type WAITPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITCFG` reader - Wait timing configuration"]
pub type WAITCFG_R = crate::BitReader;
#[doc = "Field `WAITCFG` writer - Wait timing configuration"]
pub type WAITCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WREN` reader - Write enable bit"]
pub type WREN_R = crate::BitReader;
#[doc = "Field `WREN` writer - Write enable bit"]
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITEN` reader - Wait enable bit"]
pub type WAITEN_R = crate::BitReader;
#[doc = "Field `WAITEN` writer - Wait enable bit"]
pub type WAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTMOD` reader - Extended mode enable"]
pub type EXTMOD_R = crate::BitReader;
#[doc = "Field `EXTMOD` writer - Extended mode enable"]
pub type EXTMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers"]
pub type ASYNCWAIT_R = crate::BitReader;
#[doc = "Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers"]
pub type ASYNCWAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSIZE` reader - CRAM Page Size"]
pub type CPSIZE_R = crate::FieldReader;
#[doc = "Field `CPSIZE` writer - CRAM Page Size"]
pub type CPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CBURSTRW` reader - Write burst enable"]
pub type CBURSTRW_R = crate::BitReader;
#[doc = "Field `CBURSTRW` writer - Write burst enable"]
pub type CBURSTRW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCLKEN` reader - Continuous clock enable"]
pub type CCLKEN_R = crate::BitReader;
#[doc = "Field `CCLKEN` writer - Continuous clock enable"]
pub type CCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFDIS` reader - Write FIFO disable"]
pub type WFDIS_R = crate::BitReader;
#[doc = "Field `WFDIS` writer - Write FIFO disable"]
pub type WFDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBLSET` reader - Byte lane (NBL) setup"]
pub type NBLSET_R = crate::FieldReader;
#[doc = "Field `NBLSET` writer - Byte lane (NBL) setup"]
pub type NBLSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FMCEN` reader - FMC controller enable"]
pub type FMCEN_R = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMC controller enable"]
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Memory bank enable bit"]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address/data multiplexing enable bit"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Memory type"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Flash access enable"]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Burst enable bit"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wait signal polarity bit"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable bit"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wait enable bit"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wait signal during asynchronous transfers"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - CRAM Page Size"]
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Write burst enable"]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Continuous clock enable"]
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write FIFO disable"]
    #[inline(always)]
    pub fn wfdis(&self) -> WFDIS_R {
        WFDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Byte lane (NBL) setup"]
    #[inline(always)]
    pub fn nblset(&self) -> NBLSET_R {
        NBLSET_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 31 - FMC controller enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory bank enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbken(&mut self) -> MBKEN_W<BCR2rs> {
        MBKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Address/data multiplexing enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<BCR2rs> {
        MUXEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Memory type"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<BCR2rs> {
        MTYP_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<BCR2rs> {
        MWID_W::new(self, 4)
    }
    #[doc = "Bit 6 - Flash access enable"]
    #[inline(always)]
    #[must_use]
    pub fn faccen(&mut self) -> FACCEN_W<BCR2rs> {
        FACCEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Burst enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bursten(&mut self) -> BURSTEN_W<BCR2rs> {
        BURSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Wait signal polarity bit"]
    #[inline(always)]
    #[must_use]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCR2rs> {
        WAITPOL_W::new(self, 9)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    #[must_use]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCR2rs> {
        WAITCFG_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<BCR2rs> {
        WREN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Wait enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<BCR2rs> {
        WAITEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn extmod(&mut self) -> EXTMOD_W<BCR2rs> {
        EXTMOD_W::new(self, 14)
    }
    #[doc = "Bit 15 - Wait signal during asynchronous transfers"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCR2rs> {
        ASYNCWAIT_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - CRAM Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn cpsize(&mut self) -> CPSIZE_W<BCR2rs> {
        CPSIZE_W::new(self, 16)
    }
    #[doc = "Bit 19 - Write burst enable"]
    #[inline(always)]
    #[must_use]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCR2rs> {
        CBURSTRW_W::new(self, 19)
    }
    #[doc = "Bit 20 - Continuous clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cclken(&mut self) -> CCLKEN_W<BCR2rs> {
        CCLKEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Write FIFO disable"]
    #[inline(always)]
    #[must_use]
    pub fn wfdis(&mut self) -> WFDIS_W<BCR2rs> {
        WFDIS_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - Byte lane (NBL) setup"]
    #[inline(always)]
    #[must_use]
    pub fn nblset(&mut self) -> NBLSET_W<BCR2rs> {
        NBLSET_W::new(self, 22)
    }
    #[doc = "Bit 31 - FMC controller enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<BCR2rs> {
        FMCEN_W::new(self, 31)
    }
}
#[doc = "SRAM/NOR-Flash chip-select control register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR2rs;
impl crate::RegisterSpec for BCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr2::R`](R) reader structure"]
impl crate::Readable for BCR2rs {}
#[doc = "`write(|w| ..)` method takes [`bcr2::W`](W) writer structure"]
impl crate::Writable for BCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCR2 to value 0x30d2"]
impl crate::Resettable for BCR2rs {
    const RESET_VALUE: u32 = 0x30d2;
}
