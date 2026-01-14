///Register `MTLRXQ0MPOCR` reader
pub type R = crate::R<MTLRXQ0MPOCRrs>;
///Register `MTLRXQ0MPOCR` writer
pub type W = crate::W<MTLRXQ0MPOCRrs>;
/**Field `OVFPKTCNT` reader - Overflow Packet Counter

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type OVFPKTCNT_R = crate::FieldReader<u16>;
///Field `OVFPKTCNT` writer - Overflow Packet Counter
pub type OVFPKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type OVFCNTOVF_R = crate::BitReader;
///Field `OVFCNTOVF` writer - Overflow Counter Overflow Bit
pub type OVFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `MISPKTCNT` reader - Missed Packet Counter

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MISPKTCNT_R = crate::FieldReader<u16>;
///Field `MISPKTCNT` writer - Missed Packet Counter
pub type MISPKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `MISCNTOVF` reader - Missed Packet Counter Overflow Bit

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MISCNTOVF_R = crate::BitReader;
///Field `MISCNTOVF` writer - Missed Packet Counter Overflow Bit
pub type MISCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Overflow Packet Counter
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Counter Overflow Bit
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:26 - Missed Packet Counter
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 27 - Missed Packet Counter Overflow Bit
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRXQ0MPOCR").finish()
    }
}
impl W {
    ///Bits 0:10 - Overflow Packet Counter
    #[inline(always)]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W<'_, MTLRXQ0MPOCRrs> {
        OVFPKTCNT_W::new(self, 0)
    }
    ///Bit 11 - Overflow Counter Overflow Bit
    #[inline(always)]
    pub fn ovfcntovf(&mut self) -> OVFCNTOVF_W<'_, MTLRXQ0MPOCRrs> {
        OVFCNTOVF_W::new(self, 11)
    }
    ///Bits 16:26 - Missed Packet Counter
    #[inline(always)]
    pub fn mispktcnt(&mut self) -> MISPKTCNT_W<'_, MTLRXQ0MPOCRrs> {
        MISPKTCNT_W::new(self, 16)
    }
    ///Bit 27 - Missed Packet Counter Overflow Bit
    #[inline(always)]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W<'_, MTLRXQ0MPOCRrs> {
        MISCNTOVF_W::new(self, 27)
    }
}
/**R0 queue 0 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq0mpocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq0mpocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ0MPOCR)*/
pub struct MTLRXQ0MPOCRrs;
impl crate::RegisterSpec for MTLRXQ0MPOCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrxq0mpocr::R`](R) reader structure
impl crate::Readable for MTLRXQ0MPOCRrs {}
///`write(|w| ..)` method takes [`mtlrxq0mpocr::W`](W) writer structure
impl crate::Writable for MTLRXQ0MPOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRXQ0MPOCR to value 0
impl crate::Resettable for MTLRXQ0MPOCRrs {}
