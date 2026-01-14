///Register `RAM_COM4` reader
pub type R = crate::R<RAM_COM4rs>;
///Register `RAM_COM4` writer
pub type W = crate::W<RAM_COM4rs>;
///Field `SEGS` reader - Segment states, one bit per segment, LSB: S00, MSB: S51
pub type SEGS_R = crate::FieldReader<u64>;
///Field `SEGS` writer - Segment states, one bit per segment, LSB: S00, MSB: S51
pub type SEGS_W<'a, REG> = crate::FieldWriter<'a, REG, 52, u64>;
impl R {
    ///Bits 0:51 - Segment states, one bit per segment, LSB: S00, MSB: S51
    #[inline(always)]
    pub fn segs(&self) -> SEGS_R {
        SEGS_R::new(self.bits & 0x000f_ffff_ffff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_COM4")
            .field("segs", &self.segs())
            .finish()
    }
}
impl W {
    ///Bits 0:51 - Segment states, one bit per segment, LSB: S00, MSB: S51
    #[inline(always)]
    pub fn segs(&mut self) -> SEGS_W<'_, RAM_COM4rs> {
        SEGS_W::new(self, 0)
    }
}
/**LCD display memory

You can [`read`](crate::Reg::read) this register and get [`ram_com4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:RAM_COM4)*/
pub struct RAM_COM4rs;
impl crate::RegisterSpec for RAM_COM4rs {
    type Ux = u64;
}
///`read()` method returns [`ram_com4::R`](R) reader structure
impl crate::Readable for RAM_COM4rs {}
///`write(|w| ..)` method takes [`ram_com4::W`](W) writer structure
impl crate::Writable for RAM_COM4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM_COM4 to value 0
impl crate::Resettable for RAM_COM4rs {}
