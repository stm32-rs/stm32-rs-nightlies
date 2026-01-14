///Register `MTLTXQ1UR` reader
pub type R = crate::R<MTLTXQ1URrs>;
///Register `MTLTXQ1UR` writer
pub type W = crate::W<MTLTXQ1URrs>;
/**Field `UFFRMCNT` reader - Underflow Packet Counter

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type UFFRMCNT_R = crate::FieldReader<u16>;
///Field `UFFRMCNT` writer - Underflow Packet Counter
pub type UFFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type UFCNTOVF_R = crate::BitReader;
///Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter
pub type UFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Underflow Packet Counter
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ1UR").finish()
    }
}
impl W {
    ///Bits 0:10 - Underflow Packet Counter
    #[inline(always)]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<'_, MTLTXQ1URrs> {
        UFFRMCNT_W::new(self, 0)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter
    #[inline(always)]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<'_, MTLTXQ1URrs> {
        UFCNTOVF_W::new(self, 11)
    }
}
/**T1 queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1ur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1ur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLTXQ1UR)*/
pub struct MTLTXQ1URrs;
impl crate::RegisterSpec for MTLTXQ1URrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq1ur::R`](R) reader structure
impl crate::Readable for MTLTXQ1URrs {}
///`write(|w| ..)` method takes [`mtltxq1ur::W`](W) writer structure
impl crate::Writable for MTLTXQ1URrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ1UR to value 0
impl crate::Resettable for MTLTXQ1URrs {}
