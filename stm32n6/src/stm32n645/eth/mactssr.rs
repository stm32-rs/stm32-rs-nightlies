///Register `MACTSSR` reader
pub type R = crate::R<MACTSSRrs>;
///Register `MACTSSR` writer
pub type W = crate::W<MACTSSRrs>;
/**Field `TSSOVF` reader - Timestamp Seconds Overflow

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TSSOVF_R = crate::BitReader;
///Field `TSSOVF` writer - Timestamp Seconds Overflow
pub type TSSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TSTARGT0` reader - Timestamp Target Time Reached

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TSTARGT0_R = crate::BitReader;
///Field `TSTARGT0` writer - Timestamp Target Time Reached
pub type TSTARGT0_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `AUXTSTRIG` reader - Auxiliary Timestamp Trigger Snapshot

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type AUXTSTRIG_R = crate::BitReader;
///Field `AUXTSTRIG` writer - Auxiliary Timestamp Trigger Snapshot
pub type AUXTSTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TSTRGTERR0` reader - Timestamp Target Time Error

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TSTRGTERR0_R = crate::BitReader;
///Field `TSTRGTERR0` writer - Timestamp Target Time Error
pub type TSTRGTERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TSTARGT1` reader - Timestamp Target Time Reached

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TSTARGT1_R = crate::BitReader;
///Field `TSTARGT1` writer - Timestamp Target Time Reached
pub type TSTARGT1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TSTRGTERR1` reader - Timestamp Target Time Error

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TSTRGTERR1_R = crate::BitReader;
///Field `TSTRGTERR1` writer - Timestamp Target Time Error
pub type TSTRGTERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TXTSSIS` reader - Tx Timestamp Status Interrupt Status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXTSSIS_R = crate::BitReader;
///Field `TXTSSIS` writer - Tx Timestamp Status Interrupt Status
pub type TXTSSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `ATSSTN` reader - Auxiliary Timestamp Snapshot Trigger Identifier

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type ATSSTN_R = crate::FieldReader;
///Field `ATSSTN` writer - Auxiliary Timestamp Snapshot Trigger Identifier
pub type ATSSTN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Field `ATSSTM` reader - Auxiliary Timestamp Snapshot Trigger Missed

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type ATSSTM_R = crate::BitReader;
///Field `ATSSTM` writer - Auxiliary Timestamp Snapshot Trigger Missed
pub type ATSSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATSNS` reader - Number of Auxiliary Timestamp Snapshots
pub type ATSNS_R = crate::FieldReader;
impl R {
    ///Bit 0 - Timestamp Seconds Overflow
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timestamp Target Time Reached
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Auxiliary Timestamp Trigger Snapshot
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp Target Time Error
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp Target Time Reached
    #[inline(always)]
    pub fn tstargt1(&self) -> TSTARGT1_R {
        TSTARGT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timestamp Target Time Error
    #[inline(always)]
    pub fn tstrgterr1(&self) -> TSTRGTERR1_R {
        TSTRGTERR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 15 - Tx Timestamp Status Interrupt Status
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:29 - Number of Auxiliary Timestamp Snapshots
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSSR")
            .field("atsns", &self.atsns())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timestamp Seconds Overflow
    #[inline(always)]
    pub fn tssovf(&mut self) -> TSSOVF_W<'_, MACTSSRrs> {
        TSSOVF_W::new(self, 0)
    }
    ///Bit 1 - Timestamp Target Time Reached
    #[inline(always)]
    pub fn tstargt0(&mut self) -> TSTARGT0_W<'_, MACTSSRrs> {
        TSTARGT0_W::new(self, 1)
    }
    ///Bit 2 - Auxiliary Timestamp Trigger Snapshot
    #[inline(always)]
    pub fn auxtstrig(&mut self) -> AUXTSTRIG_W<'_, MACTSSRrs> {
        AUXTSTRIG_W::new(self, 2)
    }
    ///Bit 3 - Timestamp Target Time Error
    #[inline(always)]
    pub fn tstrgterr0(&mut self) -> TSTRGTERR0_W<'_, MACTSSRrs> {
        TSTRGTERR0_W::new(self, 3)
    }
    ///Bit 4 - Timestamp Target Time Reached
    #[inline(always)]
    pub fn tstargt1(&mut self) -> TSTARGT1_W<'_, MACTSSRrs> {
        TSTARGT1_W::new(self, 4)
    }
    ///Bit 5 - Timestamp Target Time Error
    #[inline(always)]
    pub fn tstrgterr1(&mut self) -> TSTRGTERR1_W<'_, MACTSSRrs> {
        TSTRGTERR1_W::new(self, 5)
    }
    ///Bit 15 - Tx Timestamp Status Interrupt Status
    #[inline(always)]
    pub fn txtssis(&mut self) -> TXTSSIS_W<'_, MACTSSRrs> {
        TXTSSIS_W::new(self, 15)
    }
    ///Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier
    #[inline(always)]
    pub fn atsstn(&mut self) -> ATSSTN_W<'_, MACTSSRrs> {
        ATSSTN_W::new(self, 16)
    }
    ///Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed
    #[inline(always)]
    pub fn atsstm(&mut self) -> ATSSTM_W<'_, MACTSSRrs> {
        ATSSTM_W::new(self, 24)
    }
}
/**Timestamp status register

You can [`read`](crate::Reg::read) this register and get [`mactssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACTSSR)*/
pub struct MACTSSRrs;
impl crate::RegisterSpec for MACTSSRrs {
    type Ux = u32;
}
///`read()` method returns [`mactssr::R`](R) reader structure
impl crate::Readable for MACTSSRrs {}
///`write(|w| ..)` method takes [`mactssr::W`](W) writer structure
impl crate::Writable for MACTSSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSSR to value 0
impl crate::Resettable for MACTSSRrs {}
