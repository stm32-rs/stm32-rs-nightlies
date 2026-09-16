///Register `TCCR3` reader
pub type R = crate::R<TCCR3rs>;
///Register `TCCR3` writer
pub type W = crate::W<TCCR3rs>;
///Field `HSWR_TOCNT` reader - High-Speed Write Timeout Counter
pub type HSWR_TOCNT_R = crate::FieldReader<u16>;
///Field `HSWR_TOCNT` writer - High-Speed Write Timeout Counter
pub type HSWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PM` reader - Presp mode
pub type PM_R = crate::BitReader;
///Field `PM` writer - Presp mode
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - High-Speed Write Timeout Counter
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 24 - Presp mode
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCCR3")
            .field("hswr_tocnt", &self.hswr_tocnt())
            .field("pm", &self.pm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - High-Speed Write Timeout Counter
    #[inline(always)]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<'_, TCCR3rs> {
        HSWR_TOCNT_W::new(self, 0)
    }
    ///Bit 24 - Presp mode
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, TCCR3rs> {
        PM_W::new(self, 24)
    }
}
/**DSI Host Timeout Counter Configuration Register 3

You can [`read`](crate::Reg::read) this register and get [`tccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:TCCR3)*/
pub struct TCCR3rs;
impl crate::RegisterSpec for TCCR3rs {
    type Ux = u32;
}
///`read()` method returns [`tccr3::R`](R) reader structure
impl crate::Readable for TCCR3rs {}
///`write(|w| ..)` method takes [`tccr3::W`](W) writer structure
impl crate::Writable for TCCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCCR3 to value 0
impl crate::Resettable for TCCR3rs {}
