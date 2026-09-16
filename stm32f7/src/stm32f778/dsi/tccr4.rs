///Register `TCCR4` reader
pub type R = crate::R<TCCR4rs>;
///Register `TCCR4` writer
pub type W = crate::W<TCCR4rs>;
///Field `LSWR_TOCNT` reader - Low-Power Write Timeout Counter
pub type LSWR_TOCNT_R = crate::FieldReader<u16>;
///Field `LSWR_TOCNT` writer - Low-Power Write Timeout Counter
pub type LSWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-Power Write Timeout Counter
    #[inline(always)]
    pub fn lswr_tocnt(&self) -> LSWR_TOCNT_R {
        LSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR4")
            .field("lswr_tocnt", &self.lswr_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-Power Write Timeout Counter
    #[inline(always)]
    pub fn lswr_tocnt(&mut self) -> LSWR_TOCNT_W<'_, TCCR4rs> {
        LSWR_TOCNT_W::new(self, 0)
    }
}
/**DSI Host Timeout Counter Configuration Register 4

You can [`read`](crate::Reg::read) this register and get [`tccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#DSI:TCCR4)*/
pub struct TCCR4rs;
impl crate::RegisterSpec for TCCR4rs {
    type Ux = u32;
}
///`read()` method returns [`tccr4::R`](R) reader structure
impl crate::Readable for TCCR4rs {}
///`write(|w| ..)` method takes [`tccr4::W`](W) writer structure
impl crate::Writable for TCCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR4 to value 0
impl crate::Resettable for TCCR4rs {}
