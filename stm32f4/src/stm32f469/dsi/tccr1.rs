///Register `TCCR1` reader
pub type R = crate::R<TCCR1rs>;
///Register `TCCR1` writer
pub type W = crate::W<TCCR1rs>;
///Field `LPRX_TOCNT` reader - Low-power Reception Timeout Counter
pub type LPRX_TOCNT_R = crate::FieldReader<u16>;
///Field `LPRX_TOCNT` writer - Low-power Reception Timeout Counter
pub type LPRX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HSTX_TOCNT` reader - High-Speed Transmission Timeout Counter
pub type HSTX_TOCNT_R = crate::FieldReader<u16>;
///Field `HSTX_TOCNT` writer - High-Speed Transmission Timeout Counter
pub type HSTX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-power Reception Timeout Counter
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High-Speed Transmission Timeout Counter
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR1")
            .field("hstx_tocnt", &self.hstx_tocnt())
            .field("lprx_tocnt", &self.lprx_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-power Reception Timeout Counter
    #[inline(always)]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W<TCCR1rs> {
        LPRX_TOCNT_W::new(self, 0)
    }
    ///Bits 16:31 - High-Speed Transmission Timeout Counter
    #[inline(always)]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W<TCCR1rs> {
        HSTX_TOCNT_W::new(self, 16)
    }
}
/**DSI Host Timeout Counter Configuration Register1

You can [`read`](crate::Reg::read) this register and get [`tccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:TCCR1)*/
pub struct TCCR1rs;
impl crate::RegisterSpec for TCCR1rs {
    type Ux = u32;
}
///`read()` method returns [`tccr1::R`](R) reader structure
impl crate::Readable for TCCR1rs {}
///`write(|w| ..)` method takes [`tccr1::W`](W) writer structure
impl crate::Writable for TCCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TCCR1 to value 0
impl crate::Resettable for TCCR1rs {
    const RESET_VALUE: u32 = 0;
}
