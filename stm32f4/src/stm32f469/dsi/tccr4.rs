///Register `TCCR4` reader
pub type R = crate::R<TCCR4rs>;
///Register `TCCR4` writer
pub type W = crate::W<TCCR4rs>;
///Field `HSWR_TOCNT` reader - High-Speed Write Timeout Counter
pub type HSWR_TOCNT_R = crate::FieldReader<u16>;
///Field `HSWR_TOCNT` writer - High-Speed Write Timeout Counter
pub type HSWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PM` reader - Presp Mode
pub type PM_R = crate::BitReader;
///Field `PM` writer - Presp Mode
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - High-Speed Write Timeout Counter
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 24 - Presp Mode
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR4")
            .field("pm", &self.pm())
            .field("hswr_tocnt", &self.hswr_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - High-Speed Write Timeout Counter
    #[inline(always)]
    #[must_use]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<TCCR4rs> {
        HSWR_TOCNT_W::new(self, 0)
    }
    ///Bit 24 - Presp Mode
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<TCCR4rs> {
        PM_W::new(self, 24)
    }
}
/**DSI Host Timeout Counter Configuration Register4

You can [`read`](crate::Reg::read) this register and get [`tccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:TCCR4)*/
pub struct TCCR4rs;
impl crate::RegisterSpec for TCCR4rs {
    type Ux = u32;
}
///`read()` method returns [`tccr4::R`](R) reader structure
impl crate::Readable for TCCR4rs {}
///`write(|w| ..)` method takes [`tccr4::W`](W) writer structure
impl crate::Writable for TCCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TCCR4 to value 0
impl crate::Resettable for TCCR4rs {
    const RESET_VALUE: u32 = 0;
}
