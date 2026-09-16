///Register `MTLTXQ0UR` reader
pub type R = crate::R<MTLTXQ0URrs>;
///Register `MTLTXQ0UR` writer
pub type W = crate::W<MTLTXQ0URrs>;
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
        f.debug_struct("MTLTXQ0UR").finish()
    }
}
impl W {
    ///Bits 0:10 - Underflow Packet Counter
    #[inline(always)]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<'_, MTLTXQ0URrs> {
        UFFRMCNT_W::new(self, 0)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter
    #[inline(always)]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<'_, MTLTXQ0URrs> {
        UFCNTOVF_W::new(self, 11)
    }
}
/**T0 queue 0 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0ur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq0ur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ0UR)*/
pub struct MTLTXQ0URrs;
impl crate::RegisterSpec for MTLTXQ0URrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq0ur::R`](R) reader structure
impl crate::Readable for MTLTXQ0URrs {}
///`write(|w| ..)` method takes [`mtltxq0ur::W`](W) writer structure
impl crate::Writable for MTLTXQ0URrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ0UR to value 0
impl crate::Resettable for MTLTXQ0URrs {}
