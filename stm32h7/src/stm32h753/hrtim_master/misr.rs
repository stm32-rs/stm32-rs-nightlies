#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISRrs>;
#[doc = "Field `MCMP1` reader - Master Compare 1 Interrupt Flag"]
pub type MCMP1_R = crate::BitReader;
#[doc = "Field `MCMP2` reader - Master Compare 2 Interrupt Flag"]
pub type MCMP2_R = crate::BitReader;
#[doc = "Field `MCMP3` reader - Master Compare 3 Interrupt Flag"]
pub type MCMP3_R = crate::BitReader;
#[doc = "Field `MCMP4` reader - Master Compare 4 Interrupt Flag"]
pub type MCMP4_R = crate::BitReader;
#[doc = "Field `MREP` reader - Master Repetition Interrupt Flag"]
pub type MREP_R = crate::BitReader;
#[doc = "Field `SYNC` reader - Sync Input Interrupt Flag"]
pub type SYNC_R = crate::BitReader;
#[doc = "Field `MUPD` reader - Master Update Interrupt Flag"]
pub type MUPD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Master Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync Input Interrupt Flag"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Update Interrupt Flag"]
    #[inline(always)]
    pub fn mupd(&self) -> MUPD_R {
        MUPD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Master Timer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISRrs {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
