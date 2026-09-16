///Register `TCCR2` reader
pub type R = crate::R<TCCR2rs>;
///Register `TCCR2` writer
pub type W = crate::W<TCCR2rs>;
///Field `LPRD_TOCNT` reader - Low-Power Read Timeout Counter
pub type LPRD_TOCNT_R = crate::FieldReader<u16>;
///Field `LPRD_TOCNT` writer - Low-Power Read Timeout Counter
pub type LPRD_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Low-Power Read Timeout Counter
    #[inline(always)]
    pub fn lprd_tocnt(&self) -> LPRD_TOCNT_R {
        LPRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR2")
            .field("lprd_tocnt", &self.lprd_tocnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Low-Power Read Timeout Counter
    #[inline(always)]
    pub fn lprd_tocnt(&mut self) -> LPRD_TOCNT_W<'_, TCCR2rs> {
        LPRD_TOCNT_W::new(self, 0)
    }
}
/**DSI Host Timeout Counter Configuration Register 2

You can [`read`](crate::Reg::read) this register and get [`tccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#DSI:TCCR2)*/
pub struct TCCR2rs;
impl crate::RegisterSpec for TCCR2rs {
    type Ux = u32;
}
///`read()` method returns [`tccr2::R`](R) reader structure
impl crate::Readable for TCCR2rs {}
///`write(|w| ..)` method takes [`tccr2::W`](W) writer structure
impl crate::Writable for TCCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR2 to value 0
impl crate::Resettable for TCCR2rs {}
