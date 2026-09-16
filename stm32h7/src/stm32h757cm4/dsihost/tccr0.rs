///Register `TCCR0` reader
pub type R = crate::R<TCCR0rs>;
///Register `TCCR0` writer
pub type W = crate::W<TCCR0rs>;
///Field `LPRX_TOCNT` reader - Low-power reception timeout counter
pub type LPRX_TOCNT_R = crate::FieldReader<u16>;
///Field `LPRX_TOCNT` writer - Low-power reception timeout counter
pub type LPRX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HSTX_TOCNT` reader - High-speed transmission timeout counter
pub type HSTX_TOCNT_R = crate::FieldReader<u16>;
///Field `HSTX_TOCNT` writer - High-speed transmission timeout counter
pub type HSTX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-power reception timeout counter
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High-speed transmission timeout counter
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR0")
            .field("lprx_tocnt", &self.lprx_tocnt())
            .field("hstx_tocnt", &self.hstx_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-power reception timeout counter
    #[inline(always)]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W<'_, TCCR0rs> {
        LPRX_TOCNT_W::new(self, 0)
    }
    ///Bits 16:31 - High-speed transmission timeout counter
    #[inline(always)]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W<'_, TCCR0rs> {
        HSTX_TOCNT_W::new(self, 16)
    }
}
/**DSI Host timeout counter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`tccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:TCCR0)*/
pub struct TCCR0rs;
impl crate::RegisterSpec for TCCR0rs {
    type Ux = u32;
}
///`read()` method returns [`tccr0::R`](R) reader structure
impl crate::Readable for TCCR0rs {}
///`write(|w| ..)` method takes [`tccr0::W`](W) writer structure
impl crate::Writable for TCCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR0 to value 0
impl crate::Resettable for TCCR0rs {}
