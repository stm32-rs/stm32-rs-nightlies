///Register `BDMUPR` reader
pub type R = crate::R<BDMUPRrs>;
///Register `BDMUPR` writer
pub type W = crate::W<BDMUPRrs>;
/**MCR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCR {
    ///0: Register not updated by burst DMA access
    NotUpdated = 0,
    ///1: Register updated by burst DMA access
    Updated = 1,
}
impl From<MCR> for bool {
    #[inline(always)]
    fn from(variant: MCR) -> Self {
        variant as u8 != 0
    }
}
///Field `MCR` reader - MCR
pub type MCR_R = crate::BitReader<MCR>;
impl MCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCR {
        match self.bits {
            false => MCR::NotUpdated,
            true => MCR::Updated,
        }
    }
    ///Register not updated by burst DMA access
    #[inline(always)]
    pub fn is_not_updated(&self) -> bool {
        *self == MCR::NotUpdated
    }
    ///Register updated by burst DMA access
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == MCR::Updated
    }
}
///Field `MCR` writer - MCR
pub type MCR_W<'a, REG> = crate::BitWriter<'a, REG, MCR>;
impl<'a, REG> MCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Register not updated by burst DMA access
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut crate::W<REG> {
        self.variant(MCR::NotUpdated)
    }
    ///Register updated by burst DMA access
    #[inline(always)]
    pub fn updated(self) -> &'a mut crate::W<REG> {
        self.variant(MCR::Updated)
    }
}
///Field `MICR` reader - MICR
pub use MCR_R as MICR_R;
///Field `MDIER` reader - MDIER
pub use MCR_R as MDIER_R;
///Field `MCNT` reader - MCNT
pub use MCR_R as MCNT_R;
///Field `MPER` reader - MPER
pub use MCR_R as MPER_R;
///Field `MREP` reader - MREP
pub use MCR_R as MREP_R;
///Field `MCMP1` reader - MCMP1
pub use MCR_R as MCMP1_R;
///Field `MCMP2` reader - MCMP2
pub use MCR_R as MCMP2_R;
///Field `MCMP3` reader - MCMP3
pub use MCR_R as MCMP3_R;
///Field `MCMP4` reader - MCMP4
pub use MCR_R as MCMP4_R;
///Field `MICR` writer - MICR
pub use MCR_W as MICR_W;
///Field `MDIER` writer - MDIER
pub use MCR_W as MDIER_W;
///Field `MCNT` writer - MCNT
pub use MCR_W as MCNT_W;
///Field `MPER` writer - MPER
pub use MCR_W as MPER_W;
///Field `MREP` writer - MREP
pub use MCR_W as MREP_W;
///Field `MCMP1` writer - MCMP1
pub use MCR_W as MCMP1_W;
///Field `MCMP2` writer - MCMP2
pub use MCR_W as MCMP2_W;
///Field `MCMP3` writer - MCMP3
pub use MCR_W as MCMP3_W;
///Field `MCMP4` writer - MCMP4
pub use MCR_W as MCMP4_W;
impl R {
    ///Bit 0 - MCR
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MICR
    #[inline(always)]
    pub fn micr(&self) -> MICR_R {
        MICR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MDIER
    #[inline(always)]
    pub fn mdier(&self) -> MDIER_R {
        MDIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCNT
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MPER
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MREP
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MCMP1
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MCMP2
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MCMP3
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MCMP4
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDMUPR")
            .field("mcr", &self.mcr())
            .field("mcmp4", &self.mcmp4())
            .field("mcmp3", &self.mcmp3())
            .field("mcmp2", &self.mcmp2())
            .field("mcmp1", &self.mcmp1())
            .field("mrep", &self.mrep())
            .field("mper", &self.mper())
            .field("mcnt", &self.mcnt())
            .field("mdier", &self.mdier())
            .field("micr", &self.micr())
            .finish()
    }
}
impl W {
    ///Bit 0 - MCR
    #[inline(always)]
    pub fn mcr(&mut self) -> MCR_W<'_, BDMUPRrs> {
        MCR_W::new(self, 0)
    }
    ///Bit 1 - MICR
    #[inline(always)]
    pub fn micr(&mut self) -> MICR_W<'_, BDMUPRrs> {
        MICR_W::new(self, 1)
    }
    ///Bit 2 - MDIER
    #[inline(always)]
    pub fn mdier(&mut self) -> MDIER_W<'_, BDMUPRrs> {
        MDIER_W::new(self, 2)
    }
    ///Bit 3 - MCNT
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<'_, BDMUPRrs> {
        MCNT_W::new(self, 3)
    }
    ///Bit 4 - MPER
    #[inline(always)]
    pub fn mper(&mut self) -> MPER_W<'_, BDMUPRrs> {
        MPER_W::new(self, 4)
    }
    ///Bit 5 - MREP
    #[inline(always)]
    pub fn mrep(&mut self) -> MREP_W<'_, BDMUPRrs> {
        MREP_W::new(self, 5)
    }
    ///Bit 6 - MCMP1
    #[inline(always)]
    pub fn mcmp1(&mut self) -> MCMP1_W<'_, BDMUPRrs> {
        MCMP1_W::new(self, 6)
    }
    ///Bit 7 - MCMP2
    #[inline(always)]
    pub fn mcmp2(&mut self) -> MCMP2_W<'_, BDMUPRrs> {
        MCMP2_W::new(self, 7)
    }
    ///Bit 8 - MCMP3
    #[inline(always)]
    pub fn mcmp3(&mut self) -> MCMP3_W<'_, BDMUPRrs> {
        MCMP3_W::new(self, 8)
    }
    ///Bit 9 - MCMP4
    #[inline(always)]
    pub fn mcmp4(&mut self) -> MCMP4_W<'_, BDMUPRrs> {
        MCMP4_W::new(self, 9)
    }
}
/**BDMUPDR

You can [`read`](crate::Reg::read) this register and get [`bdmupr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmupr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HRTIM_Common:BDMUPR)*/
pub struct BDMUPRrs;
impl crate::RegisterSpec for BDMUPRrs {
    type Ux = u32;
}
///`read()` method returns [`bdmupr::R`](R) reader structure
impl crate::Readable for BDMUPRrs {}
///`write(|w| ..)` method takes [`bdmupr::W`](W) writer structure
impl crate::Writable for BDMUPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDMUPR to value 0
impl crate::Resettable for BDMUPRrs {}
